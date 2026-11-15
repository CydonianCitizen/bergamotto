pub mod download;
pub mod extractor;
pub mod installer;
pub mod manifest;
pub mod verifier;

use std::path::{Path, PathBuf};

pub use download::{download_file, is_allowed_host, DownloadError, ALLOWED_HOSTS};
pub use extractor::{extract_archive, ExtractionError};
pub use installer::{
    default_managed_tools_dir, install_tool, uninstall_tool, InstallError, InstallOutcome,
    InstalledToolRecord,
};
pub use manifest::{
    ArchiveFormat, LicenseInfo, ManagedToolsManifest, ManifestError, PlatformAsset, ToolDefinition,
    VerificationConfig,
};
pub use verifier::{verify_tool, VerificationError, VerifiedToolMetadata};

pub fn resolve_managed_binary(tools_base_dir: &Path, program: &str) -> Option<PathBuf> {
    let manifest = ManagedToolsManifest::load_embedded().ok()?;
    let canonical_tool = match program {
        "ffmpeg" => "ffmpeg",
        "python" | "python3" => "python3",
        "magick" | "convert" => "magick",
        other => other,
    };
    let tool_def = manifest.tools.get(canonical_tool)?;
    let platform_key = ManagedToolsManifest::current_platform_key()?;
    let asset = tool_def.platforms.get(platform_key)?;

    // 1. Direct path in specific version: <tools_base_dir>/<canonical_tool>/<version>/<binary_path>
    let binary = tools_base_dir
        .join(canonical_tool)
        .join(&asset.version)
        .join(&asset.binary_path);

    if is_executable_file(&binary) {
        return Some(binary);
    }

    // 2. Fallback: check any version directory installed under <tools_base_dir>/<canonical_tool>
    let tool_dir = tools_base_dir.join(canonical_tool);
    if let Ok(entries) = std::fs::read_dir(&tool_dir) {
        for entry in entries.flatten() {
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                let candidate = entry.path().join(&asset.binary_path);
                if is_executable_file(&candidate) {
                    return Some(candidate);
                }
            }
        }
    }

    None
}

fn is_executable_file(path: &Path) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(meta) = path.metadata() {
            return meta.is_file() && (meta.permissions().mode() & 0o111 != 0);
        }
        false
    }
    #[cfg(windows)]
    {
        if let Ok(meta) = path.metadata() {
            return meta.is_file();
        }
        false
    }
}
