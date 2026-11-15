use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ArchiveFormat {
    #[serde(rename = "zip")]
    Zip,
    #[serde(rename = "tar.gz")]
    TarGz,
    #[serde(rename = "tar.xz")]
    TarXz,
    #[serde(rename = "7z")]
    SevenZ,
    #[serde(rename = "appimage")]
    AppImage,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LicenseInfo {
    pub name: String,
    pub url: String,
    pub is_copyleft: bool,
    pub source_url: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationConfig {
    pub command: Vec<String>,
    #[serde(default)]
    pub encoders_command: Option<Vec<String>>,
    #[serde(default)]
    pub required_encoders: Vec<String>,
    #[serde(default)]
    pub formats_command: Option<Vec<String>>,
    #[serde(default)]
    pub required_formats: Vec<String>,
    #[serde(default)]
    pub test_script_args: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlatformAsset {
    pub version: String,
    pub url: String,
    pub sha256: String,
    pub archive_format: ArchiveFormat,
    pub binary_path: String,
    pub size_bytes: u64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub license: LicenseInfo,
    pub verification: VerificationConfig,
    pub platforms: HashMap<String, PlatformAsset>,
    #[serde(default)]
    pub manual_instructions: HashMap<String, String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedToolsManifest {
    pub schema_version: u32,
    pub tools: HashMap<String, ToolDefinition>,
}

#[derive(Debug, Error)]
pub enum ManifestError {
    #[error("Could not parse managed-tools.json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Invalid manifest: {0}")]
    Invalid(&'static str),
}

impl ManagedToolsManifest {
    pub fn load_embedded() -> Result<Self, ManifestError> {
        Self::parse(Self::embedded_str())
    }

    pub fn embedded_str() -> &'static str {
        include_str!("../../../../shared/managed-tools.json")
    }

    pub fn parse(json: &str) -> Result<Self, ManifestError> {
        let manifest: Self = serde_json::from_str(json)?;
        if manifest.schema_version != 1 {
            return Err(ManifestError::Invalid("Unsupported schemaVersion"));
        }
        Ok(manifest)
    }

    pub fn current_platform_key() -> Option<&'static str> {
        if cfg!(all(target_os = "windows", target_arch = "x86_64")) {
            Some("windows-x64")
        } else if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
            Some("linux-x64")
        } else if cfg!(all(target_os = "linux", target_arch = "aarch64")) {
            Some("linux-arm64")
        } else if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
            Some("macos-arm64")
        } else if cfg!(all(target_os = "macos", target_arch = "x86_64")) {
            Some("macos-x64")
        } else {
            None
        }
    }

    pub fn get_tool_asset(&self, tool: &str) -> Option<&PlatformAsset> {
        let key = Self::current_platform_key()?;
        self.tools.get(tool)?.platforms.get(key)
    }

    pub fn get_manual_instruction(&self, tool: &str) -> Option<&str> {
        let os_key = if cfg!(target_os = "windows") {
            "windows"
        } else if cfg!(target_os = "macos") {
            "macos"
        } else {
            "linux"
        };
        self.tools
            .get(tool)?
            .manual_instructions
            .get(os_key)
            .map(|s| s.as_str())
    }
}
