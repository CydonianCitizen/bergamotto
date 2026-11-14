use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use tempfile::NamedTempFile;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
pub enum ModifierKey {
    #[default]
    Shift,
    Ctrl,
    Alt,
    #[serde(rename = "Alt+Shift", alias = "AltShift")]
    AltShift,
    #[serde(rename = "Ctrl+Shift", alias = "CtrlShift")]
    CtrlShift,
    #[serde(rename = "Ctrl+Alt", alias = "CtrlAlt")]
    CtrlAlt,
}

impl std::fmt::Display for ModifierKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Shift => write!(f, "Shift"),
            Self::Ctrl => write!(f, "Ctrl"),
            Self::Alt => write!(f, "Alt"),
            Self::AltShift => write!(f, "Alt+Shift"),
            Self::CtrlShift => write!(f, "Ctrl+Shift"),
            Self::CtrlAlt => write!(f, "Ctrl+Alt"),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct AppSettings {
    pub global_hook_enabled: bool,
    pub modifier_key: ModifierKey,
    pub global_shortcut: String,
    pub custom_paths: Vec<String>,
    pub tool_overrides: HashMap<String, String>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            global_hook_enabled: true,
            modifier_key: ModifierKey::Shift,
            global_shortcut: "Alt+Shift+Space".to_string(),
            custom_paths: Vec::new(),
            tool_overrides: HashMap::new(),
        }
    }
}

impl AppSettings {
    fn validate(&self) -> Result<(), SettingsError> {
        if self.custom_paths.iter().any(|path| !absolute_path(path)) {
            return Err(SettingsError::Invalid(
                "Custom paths must be absolute directories",
            ));
        }
        std::env::join_paths(&self.custom_paths)
            .map_err(|_| SettingsError::Invalid("A custom directory contains a PATH separator"))?;
        if self.tool_overrides.iter().any(|(tool, path)| {
            tool.trim().is_empty() || tool.contains('\0') || !absolute_path(path)
        }) {
            return Err(SettingsError::Invalid(
                "Tool overrides need a tool name and an absolute executable path",
            ));
        }
        Ok(())
    }
}

fn absolute_path(path: &str) -> bool {
    !path.contains('\0') && Path::new(path).is_absolute()
}

#[derive(Debug, Error)]
pub enum SettingsError {
    #[error("Could not access settings.json: {0}")]
    Io(#[from] io::Error),
    #[error("Invalid settings.json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("{0}")]
    Invalid(&'static str),
}

pub struct SettingsStore {
    path: PathBuf,
    current: Mutex<AppSettings>,
}

impl SettingsStore {
    pub fn open(path: impl Into<PathBuf>) -> Result<Self, SettingsError> {
        let path = path.into();
        let settings = match fs::read(&path) {
            Ok(json) => serde_json::from_slice(&json)?,
            Err(error) if error.kind() == io::ErrorKind::NotFound => AppSettings::default(),
            Err(error) => return Err(error.into()),
        };
        settings.validate()?;
        Ok(Self {
            path,
            current: Mutex::new(settings),
        })
    }

    pub fn snapshot(&self) -> AppSettings {
        self.current.lock().unwrap().clone()
    }

    pub fn update(
        &self,
        change: impl FnOnce(&mut AppSettings),
    ) -> Result<AppSettings, SettingsError> {
        let mut current = self.current.lock().unwrap();
        let mut next = current.clone();
        change(&mut next);
        next.validate()?;

        let directory = self
            .path
            .parent()
            .filter(|path| !path.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."));
        fs::create_dir_all(directory)?;
        let mut temporary = NamedTempFile::new_in(directory)?;
        serde_json::to_writer_pretty(temporary.as_file_mut(), &next)?;
        temporary.write_all(b"\n")?;
        temporary.as_file().sync_all()?;
        temporary.persist(&self.path).map_err(|error| error.error)?;
        *current = next.clone();
        Ok(next)
    }
}
