use serde::{Deserialize, Serialize};

use super::*;

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PackCollision {
    Reject,
    Overwrite,
    Duplicate,
}

#[derive(Debug, Serialize)]
pub struct PackPreview {
    pub count: usize,
    pub collisions: Vec<String>,
}

impl ActionRegistry {
    /// Exports a single saved action, or all enabled actions, as a JSON array.
    pub fn export_pack(&self, id: Option<&str>) -> Result<String, RegistryError> {
        let state = self.state.read().map_err(|_| RegistryError::LockPoisoned)?;
        let actions = match id {
            Some(id) => {
                if state.duplicate_ids.contains(id) {
                    return Err(RegistryError::DuplicateId(id.to_owned()));
                }
                vec![state
                    .actions
                    .get(id)
                    .ok_or_else(|| RegistryError::NotFound(id.to_owned()))?]
            }
            None => state
                .actions
                .values()
                .filter(|action| action.enabled)
                .collect(),
        };
        if actions.is_empty() {
            return Err(RegistryError::InvalidManifest(
                "No enabled actions to export".into(),
            ));
        }
        Ok(serde_json::to_string_pretty(&actions)? + "\n")
    }

    pub fn preview_pack(&self, document: &str) -> Result<PackPreview, RegistryError> {
        let actions = parse_pack(document)?;
        let _guard = self
            .operation_lock
            .lock()
            .map_err(|_| RegistryError::LockPoisoned)?;
        self.refresh_locked()?;
        let state = self.state.read().map_err(|_| RegistryError::LockPoisoned)?;
        let collisions = actions
            .iter()
            .filter(|action| has_id(&state, &action.id))
            .map(|action| action.id.clone())
            .collect();
        Ok(PackPreview {
            count: actions.len(),
            collisions,
        })
    }

    /// Validates and stages every manifest before publishing. Rolls back committed files on error.
    pub fn import_pack(
        &self,
        document: &str,
        collision: PackCollision,
    ) -> Result<Vec<ActionDescriptor>, RegistryError> {
        let mut actions = parse_pack(document)?;
        let _guard = self
            .operation_lock
            .lock()
            .map_err(|_| RegistryError::LockPoisoned)?;
        self.refresh_locked()?;
        let state = self.state.read().map_err(|_| RegistryError::LockPoisoned)?;
        let mut reserved: HashSet<String> =
            actions.iter().map(|action| action.id.clone()).collect();
        let mut staged = Vec::with_capacity(actions.len());
        for action in &mut actions {
            if has_id(&state, &action.id) {
                match collision {
                    PackCollision::Reject => {
                        return Err(RegistryError::AlreadyExists(action.id.clone()))
                    }
                    PackCollision::Overwrite if state.duplicate_ids.contains(&action.id) => {
                        return Err(RegistryError::DuplicateId(action.id.clone()))
                    }
                    PackCollision::Overwrite => {}
                    PackCollision::Duplicate => loop {
                        let prefix =
                            action.id[..action.id.len().min(100)].trim_end_matches(['-', '.', '_']);
                        let id = format!(
                            "{prefix}.copy-{}",
                            &uuid::Uuid::new_v4().simple().to_string()[..12]
                        );
                        if !has_id(&state, &id) && reserved.insert(id.clone()) {
                            action.id = id;
                            break;
                        }
                    },
                }
            }
            staged.push(PendingManifest::new(
                &self.actions_dir,
                state.files.get(&action.id),
                action,
            )?);
        }
        drop(state);

        let mut committed = Vec::with_capacity(staged.len());
        let result = (|| {
            for pending in staged {
                committed.push(pending.publish()?);
            }
            self.refresh_locked()
        })();
        if let Err(original) = result {
            let rollback_errors = committed
                .into_iter()
                .rev()
                .filter_map(|manifest| manifest.restore().err().map(|error| error.to_string()))
                .collect::<Vec<_>>();
            self.refresh_locked()?;
            if !rollback_errors.is_empty() {
                return Err(RegistryError::PackRollback {
                    original: original.to_string(),
                    rollback: rollback_errors.join(", "),
                });
            }
            return Err(original);
        }
        Ok(actions)
    }
}

fn parse_pack(document: &str) -> Result<Vec<ActionDescriptor>, RegistryError> {
    let values = match serde_json::from_str(document)? {
        Value::Array(values) => values,
        value => vec![value],
    };
    if values.is_empty() {
        return Err(RegistryError::InvalidManifest(
            "Action pack is empty".into(),
        ));
    }
    let mut seen = HashSet::new();
    let mut actions = Vec::with_capacity(values.len());
    for (index, value) in values.into_iter().enumerate() {
        validate_manifest_value(&value).map_err(|error| {
            RegistryError::InvalidManifest(format!("Action {}: {error}", index + 1))
        })?;
        let action: ActionDescriptor = serde_json::from_value(value)?;
        if !seen.insert(action.id.clone()) {
            return Err(RegistryError::DuplicateId(action.id));
        }
        actions.push(action);
    }
    Ok(actions)
}

fn has_id(state: &RegistryState, id: &str) -> bool {
    state.files.contains_key(id) || state.duplicate_ids.contains(id)
}

struct PendingManifest {
    target: PathBuf,
    original: Option<NamedTempFile>,
    replacement: NamedTempFile,
    replacement_contents: Vec<u8>,
}

impl PendingManifest {
    fn new(
        directory: &Path,
        existing: Option<&PathBuf>,
        action: &ActionDescriptor,
    ) -> Result<Self, RegistryError> {
        let target = existing
            .cloned()
            .unwrap_or_else(|| directory.join(format!("action-{}.json", action.id)));
        let original = if existing.is_some() {
            let current = fs::read_to_string(&target)?;
            if ActionRegistry::parse_manifest(&current)?.id != action.id {
                return Err(RegistryError::NotFound(action.id.clone()));
            }
            Some(stage(directory, current.as_bytes())?)
        } else {
            match target.symlink_metadata() {
                Ok(_) => return Err(RegistryError::AlreadyExists(action.id.clone())),
                Err(error) if error.kind() == io::ErrorKind::NotFound => {}
                Err(error) => return Err(error.into()),
            }
            None
        };
        let mut replacement_contents = serde_json::to_vec_pretty(action)?;
        replacement_contents.push(b'\n');
        let replacement = stage(directory, &replacement_contents)?;
        Ok(Self {
            target,
            original,
            replacement,
            replacement_contents,
        })
    }

    fn publish(self) -> Result<CommittedManifest, RegistryError> {
        if let Some(original) = &self.original {
            if fs::read(&self.target)? != fs::read(original.path())? {
                return Err(io::Error::other(format!(
                    "{} changed during import",
                    self.target.display()
                ))
                .into());
            }
            self.replacement
                .persist(&self.target)
                .map_err(|error| error.error)?;
        } else {
            self.replacement
                .persist_noclobber(&self.target)
                .map_err(|error| error.error)?;
        }
        Ok(CommittedManifest {
            target: self.target,
            original: self.original,
            replacement_contents: self.replacement_contents,
        })
    }
}

struct CommittedManifest {
    target: PathBuf,
    original: Option<NamedTempFile>,
    replacement_contents: Vec<u8>,
}

impl CommittedManifest {
    fn restore(self) -> io::Result<()> {
        if fs::read(&self.target)? != self.replacement_contents {
            return Err(io::Error::other(format!(
                "{} changed during import",
                self.target.display()
            )));
        }
        if let Some(original) = self.original {
            original
                .persist(&self.target)
                .map_err(|error| error.error)?;
        } else {
            fs::remove_file(self.target)?;
        }
        Ok(())
    }
}

fn stage(directory: &Path, contents: &[u8]) -> io::Result<NamedTempFile> {
    let mut file = NamedTempFile::new_in(directory)?;
    file.write_all(contents)?;
    file.as_file().sync_all()?;
    Ok(file)
}
