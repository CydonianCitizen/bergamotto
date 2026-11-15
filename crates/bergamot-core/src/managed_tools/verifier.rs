use std::io;
use std::path::Path;
use std::process::Command;
use thiserror::Error;

use super::manifest::VerificationConfig;

#[derive(Debug, Error)]
pub enum VerificationError {
    #[error("I/O error during verification: {0}")]
    Io(#[from] io::Error),
    #[error("Verification command failed with status {code:?}: {stderr}")]
    CommandFailed { code: Option<i32>, stderr: String },
    #[error("FFmpeg configuration violation: {0}")]
    InvalidLicense(String),
    #[error("FFmpeg is missing required encoders: {0:?}")]
    MissingEncoders(Vec<String>),
    #[error("ImageMagick is missing required formats: {0:?}")]
    MissingFormats(Vec<String>),
    #[error("Python verification script failed: {0}")]
    PythonScriptFailed(String),
}

#[derive(Clone, Debug)]
pub struct VerifiedToolMetadata {
    pub version_output: String,
    pub detected_license: String,
}

pub fn verify_tool(
    tool_name: &str,
    binary_path: &Path,
    config: &VerificationConfig,
) -> Result<VerifiedToolMetadata, VerificationError> {
    let output = Command::new(binary_path).args(&config.command).output()?;

    if !output.status.success() {
        return Err(VerificationError::CommandFailed {
            code: output.status.code(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        });
    }

    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let mut detected_license = "Unknown".to_string();

    if tool_name == "ffmpeg" {
        let (license, config_line) = parse_ffmpeg_configuration(&stdout)?;
        detected_license = license;

        if let Some(ref encoders_cmd) = config.encoders_command {
            let enc_output = Command::new(binary_path).args(encoders_cmd).output()?;
            if !enc_output.status.success() {
                return Err(VerificationError::CommandFailed {
                    code: enc_output.status.code(),
                    stderr: String::from_utf8_lossy(&enc_output.stderr).into_owned(),
                });
            }
            let enc_stdout = String::from_utf8_lossy(&enc_output.stdout);
            let mut missing = Vec::new();
            for req in &config.required_encoders {
                if !enc_stdout.contains(req) && !config_line.contains(req) {
                    missing.push(req.clone());
                }
            }
            if !missing.is_empty() {
                return Err(VerificationError::MissingEncoders(missing));
            }
        }
    } else if tool_name == "python3" {
        detected_license = "PSF-2.0 (with bundled third-party licenses)".to_string();

        if let Some(ref script_args) = config.test_script_args {
            let test_output = Command::new(binary_path).args(script_args).output()?;
            if !test_output.status.success() {
                return Err(VerificationError::PythonScriptFailed(
                    String::from_utf8_lossy(&test_output.stderr).into_owned(),
                ));
            }
        }
    } else if tool_name == "magick" {
        detected_license = "ImageMagick (Apache-2.0 compatible)".to_string();

        if let Some(ref fmt_cmd) = config.formats_command {
            let fmt_output = Command::new(binary_path).args(fmt_cmd).output()?;
            if !fmt_output.status.success() {
                return Err(VerificationError::CommandFailed {
                    code: fmt_output.status.code(),
                    stderr: String::from_utf8_lossy(&fmt_output.stderr).into_owned(),
                });
            }
            let fmt_stdout = String::from_utf8_lossy(&fmt_output.stdout);
            let mut missing = Vec::new();
            for req in &config.required_formats {
                if !fmt_stdout.to_uppercase().contains(&req.to_uppercase()) {
                    missing.push(req.clone());
                }
            }
            if !missing.is_empty() {
                return Err(VerificationError::MissingFormats(missing));
            }
        }
    }

    Ok(VerifiedToolMetadata {
        version_output: stdout,
        detected_license,
    })
}

fn parse_ffmpeg_configuration(version_output: &str) -> Result<(String, String), VerificationError> {
    let config_line = version_output
        .lines()
        .find(|l| l.contains("configuration:"))
        .ok_or_else(|| {
            VerificationError::InvalidLicense(
                "Missing 'configuration:' line in ffmpeg -version".into(),
            )
        })?;

    if !config_line.contains("--enable-gpl") {
        return Err(VerificationError::InvalidLicense(
            "FFmpeg build is missing --enable-gpl".into(),
        ));
    }

    if config_line.contains("--enable-nonfree") {
        return Err(VerificationError::InvalidLicense(
            "FFmpeg build contains disallowed --enable-nonfree flag".into(),
        ));
    }

    let gpl_version = if config_line.contains("--enable-version3") {
        "GPL-3.0"
    } else {
        "GPL-2.0"
    };

    Ok((gpl_version.to_string(), config_line.to_string()))
}
