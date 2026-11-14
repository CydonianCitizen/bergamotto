use std::collections::BTreeMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    Macos,
    Windows,
    Linux,
}

impl Platform {
    pub fn current() -> Option<Self> {
        match std::env::consts::OS {
            "macos" => Some(Self::Macos),
            "windows" => Some(Self::Windows),
            "linux" => Some(Self::Linux),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Cardinality {
    Single,
    Each,
    Batch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionDescriptor {
    pub schema_version: u32,
    pub id: String,
    pub label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<String>,
    pub enabled: bool,
    pub platforms: Vec<Platform>,
    pub matching: MatchingRule,
    pub executor: ExecutorConfig,
    pub output: OutputConfig,
}

impl ActionDescriptor {
    pub fn matches(&self, context: &DragContext) -> bool {
        if !self.enabled || !self.platforms.contains(&context.platform) || context.files.is_empty()
        {
            return false;
        }
        if self.matching.cardinality == Cardinality::Single && context.files.len() != 1 {
            return false;
        }
        if self
            .matching
            .extensions
            .iter()
            .any(|extension| extension == "*")
        {
            return true;
        }

        context.files.iter().all(|file| {
            file.extension()
                .and_then(|extension| extension.to_str())
                .is_some_and(|extension| {
                    let lowercase = extension.to_lowercase();
                    self.matching.extensions.contains(&lowercase)
                })
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchingRule {
    pub extensions: Vec<String>,
    pub cardinality: Cardinality,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "type",
    rename_all = "lowercase",
    rename_all_fields = "camelCase"
)]
pub enum ExecutorConfig {
    Process {
        program: String,
        #[serde(default)]
        args: Vec<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        working_directory: Option<PathBuf>,
        #[serde(default)]
        env: BTreeMap<String, String>,
        #[serde(default = "default_timeout_seconds")]
        timeout_seconds: u32,
    },
    Script {
        interpreter: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        script_body: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        script_path: Option<PathBuf>,
        #[serde(default = "default_timeout_seconds")]
        timeout_seconds: u32,
    },
    Webhook {
        method: String,
        url: String,
        #[serde(default)]
        headers: BTreeMap<String, String>,
        #[serde(default = "default_file_field_name")]
        file_field_name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        credential_key: Option<String>,
        #[serde(default = "default_timeout_seconds")]
        timeout_seconds: u32,
    },
}

fn default_timeout_seconds() -> u32 {
    60
}

fn default_file_field_name() -> String {
    "file".to_owned()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputMode {
    None,
    File,
    Clipboard,
    Stdout,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputConfig {
    pub mode: OutputMode,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extension: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[serde(default)]
    pub overwrite_original: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DragContext {
    pub platform: Platform,
    pub files: Vec<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionInvocation {
    pub action_id: String,
    pub context: DragContext,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionProgress {
    pub action_id: String,
    pub completed: usize,
    pub total: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionResult {
    pub action_id: String,
    pub completed: usize,
    pub output_files: Vec<PathBuf>,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub clipboard: Option<String>,
}
