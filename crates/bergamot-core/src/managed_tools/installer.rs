use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use thiserror::Error;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use super::download::{self, DownloadError};
use super::extractor::{self, ExtractionError, DEFAULT_MAX_DECOMPRESSED_BYTES, DEFAULT_MAX_FILES};
use super::manifest::{ManagedToolsManifest, PlatformAsset, ToolDefinition};
use super::verifier::{self, VerificationError, VerifiedToolMetadata};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstalledToolRecord {
    pub tool: String,
    pub version: String,
    pub detected_license: String,
    pub binary_path: String,
}

pub fn default_managed_tools_dir() -> Option<PathBuf> {
    let dirs = directories::BaseDirs::new()?;
    let name = if cfg!(target_os = "linux") {
        "bergamot"
    } else {
        "Bergamot"
    };
    Some(dirs.data_local_dir().join(name).join("tools"))
}

#[derive(Debug, Error)]
pub enum InstallError {
    #[error("Tool '{0}' is not defined in manifest")]
    ToolNotFound(String),
    #[error("Platform '{0}' is not supported for tool '{1}'")]
    PlatformNotSupported(String, String),
    #[error("Download error: {0}")]
    Download(#[from] DownloadError),
    #[error("Extraction error: {0}")]
    Extraction(#[from] ExtractionError),
    #[error("Verification error: {0}")]
    Verification(#[from] VerificationError),
    #[error("I/O error during installation: {0}")]
    Io(#[from] io::Error),
    #[error("Expected binary was not found after extraction at: {0}")]
    BinaryNotFound(PathBuf),
    #[error("Installation was cancelled")]
    Cancelled,
}

pub struct InstallOutcome {
    pub binary_path: PathBuf,
    pub metadata: VerifiedToolMetadata,
}

pub async fn install_tool<F>(
    manifest: &ManagedToolsManifest,
    tool_name: &str,
    tools_base_dir: &Path,
    on_progress: F,
    cancel: &CancellationToken,
) -> Result<InstallOutcome, InstallError>
where
    F: Fn(u64, Option<u64>) + Send + Sync + 'static,
{
    let tool_def: &ToolDefinition = manifest
        .tools
        .get(tool_name)
        .ok_or_else(|| InstallError::ToolNotFound(tool_name.to_owned()))?;

    let platform_key = ManagedToolsManifest::current_platform_key().ok_or_else(|| {
        InstallError::PlatformNotSupported("unknown".into(), tool_name.to_owned())
    })?;

    let asset: &PlatformAsset = tool_def.platforms.get(platform_key).ok_or_else(|| {
        InstallError::PlatformNotSupported(platform_key.to_owned(), tool_name.to_owned())
    })?;

    fs::create_dir_all(tools_base_dir)?;

    let target_dir = tools_base_dir.join(tool_name).join(&asset.version);
    let target_binary = target_dir.join(&asset.binary_path);

    if target_binary.exists() {
        if let Ok(metadata) =
            verifier::verify_tool(tool_name, &target_binary, &tool_def.verification)
        {
            return Ok(InstallOutcome {
                binary_path: target_binary,
                metadata,
            });
        }
    }

    let uuid_str = Uuid::new_v4().simple().to_string();
    let tmp_archive = tools_base_dir.join(format!(".download-{uuid_str}.tmp"));
    let tmp_extract = tools_base_dir.join(format!(".extract-{uuid_str}"));

    // Guard to clean up temporary paths
    struct CleanupGuard {
        archive: PathBuf,
        extract: PathBuf,
        active: bool,
    }
    impl Drop for CleanupGuard {
        fn drop(&mut self) {
            if self.active {
                let _ = fs::remove_file(&self.archive);
                let _ = fs::remove_dir_all(&self.extract);
            }
        }
    }
    let mut guard = CleanupGuard {
        archive: tmp_archive.clone(),
        extract: tmp_extract.clone(),
        active: true,
    };

    // 1. Download
    download::download_file(
        &asset.url,
        &asset.sha256,
        &tmp_archive,
        asset.size_bytes.saturating_mul(4).max(300_000_000),
        on_progress,
        cancel,
    )
    .await?;

    if cancel.is_cancelled() {
        return Err(InstallError::Cancelled);
    }

    // 2. Extract
    extractor::extract_archive(
        &tmp_archive,
        asset.archive_format,
        &tmp_extract,
        DEFAULT_MAX_FILES,
        DEFAULT_MAX_DECOMPRESSED_BYTES,
    )?;

    let _ = fs::remove_file(&tmp_archive);

    if cancel.is_cancelled() {
        return Err(InstallError::Cancelled);
    }

    // 3. Verify
    let candidate_binary = tmp_extract.join(&asset.binary_path);
    if !candidate_binary.exists() {
        return Err(InstallError::BinaryNotFound(candidate_binary));
    }

    let metadata = verifier::verify_tool(tool_name, &candidate_binary, &tool_def.verification)?;

    // 4. Atomic installation move
    if let Some(parent) = target_dir.parent() {
        fs::create_dir_all(parent)?;
    }
    if target_dir.exists() {
        let _ = fs::remove_dir_all(&target_dir);
    }

    if fs::rename(&tmp_extract, &target_dir).is_err() {
        // Fallback if cross-device or directory replacement fails
        copy_dir_recursive(&tmp_extract, &target_dir)?;
        let _ = fs::remove_dir_all(&tmp_extract);
    }

    let record = InstalledToolRecord {
        tool: tool_name.to_owned(),
        version: asset.version.clone(),
        detected_license: metadata.detected_license.clone(),
        binary_path: asset.binary_path.clone(),
    };
    if let Ok(record_json) = serde_json::to_string_pretty(&record) {
        let _ = fs::write(target_dir.join(".bergamot-tool.json"), record_json);
    }

    guard.active = false;

    Ok(InstallOutcome {
        binary_path: target_binary,
        metadata,
    })
}

pub fn uninstall_tool(tools_base_dir: &Path, tool_name: &str) -> io::Result<()> {
    let tool_dir = tools_base_dir.join(tool_name);
    if tool_dir.exists() {
        fs::remove_dir_all(&tool_dir)?;
    }
    Ok(())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}
