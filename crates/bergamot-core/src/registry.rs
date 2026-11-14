use std::collections::{BTreeMap, HashSet};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock, RwLock, Weak};

use jsonschema::Validator;
use notify::event::{AccessKind, AccessMode};
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use serde_json::Value;
use tempfile::NamedTempFile;

use crate::descriptor::{ActionDescriptor, DragContext};

const ACTION_SCHEMA: &str = include_str!("../../../shared/schemas/action.schema.json");
const STARTER_PACK: &[(&str, &[u8])] = &[
    (
        "ffmpeg-compress.json",
        include_bytes!("../../../shared/starter-pack/ffmpeg-compress.json"),
    ),
    (
        "imagemagick-convert.json",
        include_bytes!("../../../shared/starter-pack/imagemagick-convert.json"),
    ),
    (
        "python-script.json",
        include_bytes!("../../../shared/starter-pack/python-script.json"),
    ),
    (
        "webhook-upload.json",
        include_bytes!("../../../shared/starter-pack/webhook-upload.json"),
    ),
];
const STARTER_MARKER: &str = ".bergamot-starters-seeded";
const STARTER_INITIALIZING_MARKER: &str = ".bergamot-starters-initializing";

static SCHEMA_VALIDATOR: OnceLock<Result<Validator, String>> = OnceLock::new();

#[derive(Debug, thiserror::Error)]
pub enum RegistryError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("invalid JSON manifest: {0}")]
    Json(#[from] serde_json::Error),
    #[error("manifest schema is invalid: {0}")]
    InvalidSchema(String),
    #[error("manifest failed schema validation: {0}")]
    InvalidManifest(String),
    #[error("action `{0}` was not found")]
    NotFound(String),
    #[error("action `{0}` already exists")]
    AlreadyExists(String),
    #[error("multiple manifests use action id `{0}")]
    DuplicateId(String),
    #[error("file watcher failed: {0}")]
    Watcher(String),
    #[error("registry lock was poisoned")]
    LockPoisoned,
}

#[derive(Default)]
struct RegistryState {
    actions: BTreeMap<String, ActionDescriptor>,
    files: BTreeMap<String, PathBuf>,
    duplicate_ids: HashSet<String>,
}

pub struct ActionRegistry {
    actions_dir: PathBuf,
    state: Arc<RwLock<RegistryState>>,
    operation_lock: Arc<Mutex<()>>,
    reload_errors: Arc<Mutex<Vec<String>>>,
    _watcher: RecommendedWatcher,
}

impl ActionRegistry {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, RegistryError> {
        let actions_dir = path.as_ref().to_path_buf();
        fs::create_dir_all(&actions_dir)?;
        let actions_dir = fs::canonicalize(actions_dir)?;
        initialize_starters(&actions_dir)?;

        let state = Arc::new(RwLock::new(RegistryState::default()));
        let operation_lock = Arc::new(Mutex::new(()));
        let reload_errors = Arc::new(Mutex::new(Vec::new()));

        let mut watcher = create_watcher(
            &actions_dir,
            Arc::downgrade(&state),
            Arc::downgrade(&operation_lock),
            Arc::downgrade(&reload_errors),
        )?;
        watcher
            .watch(&actions_dir, RecursiveMode::NonRecursive)
            .map_err(|error| RegistryError::Watcher(error.to_string()))?;

        let registry = Self {
            actions_dir,
            state,
            operation_lock,
            reload_errors,
            _watcher: watcher,
        };
        {
            let _guard = registry
                .operation_lock
                .lock()
                .map_err(|_| RegistryError::LockPoisoned)?;
            registry.refresh_locked()?;
        }
        Ok(registry)
    }

    pub fn parse_manifest(manifest: &str) -> Result<ActionDescriptor, RegistryError> {
        let value: Value = serde_json::from_str(manifest)?;
        validate_manifest_value(&value)?;
        Ok(serde_json::from_value(value)?)
    }

    pub fn get_action(&self, id: &str) -> Option<ActionDescriptor> {
        self.state
            .read()
            .ok()
            .and_then(|state| state.actions.get(id).cloned())
    }

    pub fn list_actions(&self) -> Vec<ActionDescriptor> {
        self.state
            .read()
            .map(|state| state.actions.values().cloned().collect())
            .unwrap_or_default()
    }

    pub fn available_actions(&self, context: &DragContext) -> Vec<ActionDescriptor> {
        crate::matcher::available_actions(&self.list_actions(), context)
    }

    pub fn save_action(&self, action: &ActionDescriptor) -> Result<(), RegistryError> {
        let _guard = self
            .operation_lock
            .lock()
            .map_err(|_| RegistryError::LockPoisoned)?;
        self.save_action_locked(action, true)
    }

    pub fn delete_action(&self, id: &str) -> Result<(), RegistryError> {
        let _guard = self
            .operation_lock
            .lock()
            .map_err(|_| RegistryError::LockPoisoned)?;
        self.refresh_locked()?;
        let state = self
            .state
            .read()
            .map_err(|_| RegistryError::LockPoisoned)?;
        if state.duplicate_ids.contains(id) {
            return Err(RegistryError::DuplicateId(id.to_owned()));
        }
        let file_path = state
            .files
            .get(id)
            .cloned()
            .ok_or_else(|| RegistryError::NotFound(id.to_owned()))?;
        drop(state);

        let current = fs::read_to_string(&file_path)?;
        if Self::parse_manifest(&current)?.id != id {
            self.refresh_locked()?;
            return Err(RegistryError::NotFound(id.to_owned()));
        }
        fs::remove_file(file_path)?;
        self.refresh_locked()?;
        Ok(())
    }

    pub fn duplicate_action(
        &self,
        source_id: &str,
        new_id: impl Into<String>,
    ) -> Result<ActionDescriptor, RegistryError> {
        let new_id = new_id.into();
        let _guard = self
            .operation_lock
            .lock()
            .map_err(|_| RegistryError::LockPoisoned)?;
        self.refresh_locked()?;
        let state = self
            .state
            .read()
            .map_err(|_| RegistryError::LockPoisoned)?;
        if state.duplicate_ids.contains(source_id) {
            return Err(RegistryError::DuplicateId(source_id.to_owned()));
        }
        let mut action = state
            .actions
            .get(source_id)
            .cloned()
            .ok_or_else(|| RegistryError::NotFound(source_id.to_owned()))?;
        drop(state);
        action.id = new_id;
        self.save_action_locked(&action, false)?;
        Ok(action)
    }

    pub fn import_action(&self, manifest: &str) -> Result<ActionDescriptor, RegistryError> {
        let action = Self::parse_manifest(manifest)?;
        let _guard = self
            .operation_lock
            .lock()
            .map_err(|_| RegistryError::LockPoisoned)?;
        self.save_action_locked(&action, false)?;
        Ok(action)
    }

    pub fn export_action(&self, id: &str) -> Result<String, RegistryError> {
        let state = self
            .state
            .read()
            .map_err(|_| RegistryError::LockPoisoned)?;
        if state.duplicate_ids.contains(id) {
            return Err(RegistryError::DuplicateId(id.to_owned()));
        }
        let action = state
            .actions
            .get(id)
            .cloned()
            .ok_or_else(|| RegistryError::NotFound(id.to_owned()))?;
        drop(state);
        Ok(serde_json::to_string_pretty(&action)?)
    }

    pub fn take_reload_errors(&self) -> Vec<String> {
        self.reload_errors
            .lock()
            .map(|mut errors| std::mem::take(&mut *errors))
            .unwrap_or_else(|_| vec![RegistryError::LockPoisoned.to_string()])
    }

    fn save_action_locked(
        &self,
        action: &ActionDescriptor,
        allow_replace: bool,
    ) -> Result<(), RegistryError> {
        let value = serde_json::to_value(action)?;
        validate_manifest_value(&value)?;

        self.refresh_locked()?;
        let state = self
            .state
            .read()
            .map_err(|_| RegistryError::LockPoisoned)?;
        if state.duplicate_ids.contains(&action.id) {
            return Err(RegistryError::DuplicateId(action.id.clone()));
        }
        let existing_path = state.files.get(&action.id).cloned();
        drop(state);
        if existing_path.is_some() && !allow_replace {
            return Err(RegistryError::AlreadyExists(action.id.clone()));
        }
        let target = existing_path
            .clone()
            .unwrap_or_else(|| self.actions_dir.join(format!("action-{}.json", action.id)));

        if existing_path.is_some() {
            let current = fs::read_to_string(&target)?;
            if Self::parse_manifest(&current)?.id != action.id {
                self.refresh_locked()?;
                return Err(RegistryError::NotFound(action.id.clone()));
            }
        }

        let mut temporary = NamedTempFile::new_in(&self.actions_dir)?;
        serde_json::to_writer_pretty(temporary.as_file_mut(), action)?;
        temporary.as_file_mut().write_all(b"\n")?;
        temporary.as_file_mut().sync_all()?;
        if existing_path.is_some() {
            temporary
                .persist(&target)
                .map_err(|error| RegistryError::Io(error.error))?;
        } else {
            temporary
                .persist_noclobber(&target)
                .map_err(|error| {
                    if error.error.kind() == io::ErrorKind::AlreadyExists {
                        RegistryError::AlreadyExists(action.id.clone())
                    } else {
                        RegistryError::Io(error.error)
                    }
                })?;
        }

        self.refresh_locked()?;
        Ok(())
    }

    fn refresh_locked(&self) -> Result<(), RegistryError> {
        let (state, errors) = scan_directory(&self.actions_dir)?;
        *self
            .state
            .write()
            .map_err(|_| RegistryError::LockPoisoned)? = state;
        *self
            .reload_errors
            .lock()
            .map_err(|_| RegistryError::LockPoisoned)? = errors;
        Ok(())
    }
}

fn validate_manifest_value(value: &Value) -> Result<(), RegistryError> {
    let validator = schema_validator()?;
    let errors = validator
        .iter_errors(value)
        .map(|error| error.to_string())
        .collect::<Vec<_>>();
    if errors.is_empty() {
        Ok(())
    } else {
        Err(RegistryError::InvalidManifest(errors.join("; ")))
    }
}

fn schema_validator() -> Result<&'static Validator, RegistryError> {
    let result = SCHEMA_VALIDATOR.get_or_init(|| {
        let schema: Value =
            serde_json::from_str(ACTION_SCHEMA).map_err(|error| error.to_string())?;
        jsonschema::options()
            .should_validate_formats(true)
            .build(&schema)
            .map_err(|error| error.to_string())
    });
    result
        .as_ref()
        .map_err(|error| RegistryError::InvalidSchema(error.clone()))
}

fn initialize_starters(actions_dir: &Path) -> Result<(), RegistryError> {
    let marker_path = actions_dir.join(STARTER_MARKER);
    if !marker_path.exists() {
        let initializing_path = actions_dir.join(STARTER_INITIALIZING_MARKER);
        let directory_is_empty = fs::read_dir(actions_dir)?.next().transpose()?.is_none();
        let should_seed = initializing_path.exists() || directory_is_empty;
        if should_seed {
            match OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&initializing_path)
            {
                Ok(_) => {}
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
                Err(error) => return Err(error.into()),
            }
            for (filename, contents) in STARTER_PACK {
                let path = actions_dir.join(filename);
                if !path.exists() {
                    write_starter_atomically(actions_dir, &path, contents)?;
                }
            }
            if !marker_path.exists() {
                match fs::rename(&initializing_path, &marker_path) {
                    Ok(()) => {}
                    Err(_error) if marker_path.exists() => {}
                    Err(error) => return Err(error.into()),
                }
            }
        } else {
            match OpenOptions::new().write(true).create_new(true).open(marker_path) {
                Ok(mut marker) => marker.write_all(b"1\n")?,
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
                Err(error) => return Err(error.into()),
            }
        }
    }
    Ok(())
}

fn scan_directory(directory: &Path) -> Result<(RegistryState, Vec<String>), RegistryError> {
    let mut errors = Vec::new();
    let mut paths = Vec::new();
    for entry in fs::read_dir(directory)? {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                errors.push(error.to_string());
                continue;
            }
        };
        let path = entry.path();
        let is_json = path
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| extension.eq_ignore_ascii_case("json"));
        if is_json {
            match entry.file_type() {
                Ok(kind) if kind.is_file() => paths.push(path),
                Ok(_) => {}
                Err(error) => errors.push(format!("{}: {error}", path.display())),
            }
        }
    }
    paths.sort();

    let mut state = RegistryState::default();
    for path in paths {
        let manifest = match fs::read_to_string(&path) {
            Ok(manifest) => manifest,
            Err(error) => {
                errors.push(format!("{}: {error}", path.display()));
                continue;
            }
        };
        let action = match ActionRegistry::parse_manifest(&manifest) {
            Ok(action) => action,
            Err(error) => {
                errors.push(format!("{}: {error}", path.display()));
                continue;
            }
        };
        if state.duplicate_ids.contains(&action.id) {
            errors.push(format!(
                "{}: duplicate action id `{}`",
                path.display(),
                action.id
            ));
            continue;
        }
        if state.actions.contains_key(&action.id) {
            errors.push(format!(
                "{}: duplicate action id `{}`",
                path.display(),
                action.id
            ));
            state.actions.remove(&action.id);
            state.files.remove(&action.id);
            state.duplicate_ids.insert(action.id);
            continue;
        }
        state.files.insert(action.id.clone(), path);
        state.actions.insert(action.id.clone(), action);
    }
    Ok((state, errors))
}

fn create_watcher(
    directory: &Path,
    state: Weak<RwLock<RegistryState>>,
    operation_lock: Weak<Mutex<()>>,
    reload_errors: Weak<Mutex<Vec<String>>>,
) -> Result<RecommendedWatcher, RegistryError> {
    let directory = directory.to_path_buf();
    RecommendedWatcher::new(
        move |event: notify::Result<notify::Event>| {
            let Some(state) = state.upgrade() else {
                return;
            };
            let Some(operation_lock) = operation_lock.upgrade() else {
                return;
            };
            let Some(reload_errors) = reload_errors.upgrade() else {
                return;
            };
            let Ok(_guard) = operation_lock.lock() else {
                return;
            };

            let event = match event {
                Ok(event) => event,
                Err(error) => {
                    if let Ok(mut errors) = reload_errors.lock() {
                        append_reload_error(&mut errors, error.to_string());
                    }
                    return;
                }
            };
            if let EventKind::Access(access) = event.kind {
                if access != AccessKind::Close(AccessMode::Write) {
                    return;
                }
            }

            match scan_directory(&directory) {
                Ok((next_state, errors)) => {
                    if let Ok(mut current_state) = state.write() {
                        *current_state = next_state;
                    }
                    if let Ok(mut current_errors) = reload_errors.lock() {
                        *current_errors = errors;
                    }
                }
                Err(error) => {
                    if let Ok(mut errors) = reload_errors.lock() {
                        append_reload_error(&mut errors, error.to_string());
                    }
                }
            }
        },
        Config::default(),
    )
    .map_err(|error| RegistryError::Watcher(error.to_string()))
}

fn append_reload_error(errors: &mut Vec<String>, message: String) {
    if !errors.contains(&message) {
        const MAX_RELOAD_ERRORS: usize = 256;
        if errors.len() == MAX_RELOAD_ERRORS {
            errors.remove(0);
        }
        errors.push(message);
    }
}

fn write_starter_atomically(
    directory: &Path,
    target: &Path,
    contents: &[u8],
) -> Result<(), RegistryError> {
    let mut temporary = NamedTempFile::new_in(directory)?;
    temporary.write_all(contents)?;
    temporary.as_file().sync_all()?;
    match temporary.persist_noclobber(target) {
        Ok(_) => Ok(()),
        Err(error) if error.error.kind() == io::ErrorKind::AlreadyExists => Ok(()),
        Err(error) => Err(error.error.into()),
    }
}
