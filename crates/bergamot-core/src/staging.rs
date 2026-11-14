use std::ffi::{OsStr, OsString};
use std::fs;
use std::io;
use std::path::{absolute, Path, PathBuf};

use tempfile::{TempDir, TempPath};

use super::executors::ExecutionError;
use crate::descriptor::{OutputConfig, OutputMode};

/// A unique output path on the same filesystem as the intended destination.
/// Dropping this guard removes unpublished output and any other staged files.
pub struct StagedFile {
    temp_path: TempPath,
    directory: TempDir,
    input: PathBuf,
    base_name: OsString,
    extension: Option<OsString>,
    overwrite_original: bool,
}

impl StagedFile {
    pub fn new(input: &Path, config: &OutputConfig) -> Result<Self, ExecutionError> {
        if config.mode != OutputMode::File {
            return Err(ExecutionError::InvalidConfig(
                "file staging requires output mode `file`".to_owned(),
            ));
        }

        let input = absolute(input)?;
        let parent = input.parent().ok_or_else(|| {
            ExecutionError::InvalidConfig("the input has no parent directory".to_owned())
        })?;
        let mut base_name = input
            .file_stem()
            .unwrap_or_else(|| OsStr::new("output"))
            .to_os_string();
        let suffix = config.suffix.as_deref().unwrap_or_default();
        if !is_safe_filename_part(suffix, true) {
            return Err(ExecutionError::InvalidConfig(
                "output suffix contains characters invalid in a filename".to_owned(),
            ));
        }
        base_name.push(suffix);

        let extension = match config.extension.as_deref() {
            Some(extension) => {
                let extension = extension.trim_start_matches('.');
                if extension.is_empty() || !is_safe_filename_part(extension, false) {
                    return Err(ExecutionError::InvalidConfig(
                        "output extension contains characters invalid in a filename".to_owned(),
                    ));
                }
                Some(OsString::from(extension))
            }
            None => input.extension().map(OsStr::to_os_string),
        };

        let id = uuid::Uuid::new_v4();
        let directory = tempfile::Builder::new()
            .prefix(".bergamot-tmp-")
            .tempdir_in(parent)?;
        let temp_name = append_extension(
            OsString::from(format!(".bergamot-tmp-{id}")),
            extension.as_deref(),
        );
        let temp_path = TempPath::try_from_path(directory.path().join(temp_name))?;
        Ok(Self {
            temp_path,
            directory,
            input,
            base_name,
            extension,
            overwrite_original: config.overwrite_original,
        })
    }

    pub fn path(&self) -> &Path {
        self.temp_path.as_ref()
    }

    pub fn commit(self) -> Result<PathBuf, ExecutionError> {
        let metadata = fs::symlink_metadata(self.path())?;
        if !metadata.file_type().is_file() {
            return Err(ExecutionError::InvalidConfig(
                "staged output must be a regular file".to_owned(),
            ));
        }

        let Self {
            temp_path,
            directory: _directory,
            input,
            base_name,
            extension,
            overwrite_original,
        } = self;

        if overwrite_original {
            temp_path
                .persist(&input)
                .map_err(|error| ExecutionError::Io(error.error))?;
            return Ok(input);
        }

        let parent = input.parent().unwrap_or_else(|| Path::new("."));
        let mut index = 0_u64;
        let mut temp_path = temp_path;
        loop {
            let name = if index == 0 {
                append_extension(base_name.clone(), extension.as_deref())
            } else {
                let mut stem = base_name.clone();
                stem.push(format!(" ({index})"));
                append_extension(stem, extension.as_deref())
            };
            let destination = parent.join(name);

            match temp_path.persist_noclobber(&destination) {
                Ok(()) => return Ok(destination),
                Err(error) if error.error.kind() == io::ErrorKind::AlreadyExists => {
                    temp_path = error.path;
                    index = index.checked_add(1).ok_or_else(|| {
                        ExecutionError::InvalidConfig("output collision count overflow".to_owned())
                    })?;
                }
                Err(error) => return Err(ExecutionError::Io(error.error)),
            }
        }
    }
}

fn append_extension(mut name: OsString, extension: Option<&OsStr>) -> OsString {
    if let Some(extension) = extension {
        name.push(".");
        name.push(extension);
    }
    name
}

fn is_safe_filename_part(value: &str, allow_empty: bool) -> bool {
    (allow_empty || !value.is_empty())
        && !value.chars().any(|character| {
            character.is_control()
                || matches!(
                    character,
                    '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*'
                )
        })
}
