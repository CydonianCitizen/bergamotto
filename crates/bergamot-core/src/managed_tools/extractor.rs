use std::fs::{self, File};
use std::io::{self, BufReader, Read};
use std::path::{Path, PathBuf};
use thiserror::Error;

use super::manifest::ArchiveFormat;

pub const DEFAULT_MAX_FILES: usize = 50_000;
pub const DEFAULT_MAX_DECOMPRESSED_BYTES: u64 = 1_000_000_000; // 1 GB

#[derive(Debug, Error)]
pub enum ExtractionError {
    #[error("I/O error during extraction: {0}")]
    Io(#[from] io::Error),
    #[error("Zip extraction error: {0}")]
    Zip(#[from] zip::result::ZipError),
    #[error("LZMA/XZ decompression error: {0}")]
    Lzma(#[from] lzma_rs::error::Error),
    #[error("7z decompression error: {0}")]
    SevenZ(String),
    #[error("Path traversal detected: {0}")]
    PathTraversal(PathBuf),
    #[error("Link target escapes destination directory: {0}")]
    LinkEscapes(PathBuf),
    #[error("Archive exceeds maximum allowed file count of {0}")]
    TooManyFiles(usize),
    #[error("Archive exceeds maximum decompressed size of {0} bytes")]
    TooLarge(u64),
    #[error("AppImage extraction failed: {0}")]
    AppImageFailed(String),
}

pub fn extract_archive(
    archive_path: &Path,
    format: ArchiveFormat,
    destination: &Path,
    max_files: usize,
    max_decompressed_bytes: u64,
) -> Result<(), ExtractionError> {
    fs::create_dir_all(destination)?;
    let canonical_dest = destination.canonicalize()?;

    match format {
        ArchiveFormat::Zip => extract_zip(
            archive_path,
            &canonical_dest,
            max_files,
            max_decompressed_bytes,
        ),
        ArchiveFormat::TarGz => extract_tar_gz(
            archive_path,
            &canonical_dest,
            max_files,
            max_decompressed_bytes,
        ),
        ArchiveFormat::TarXz => extract_tar_xz(
            archive_path,
            &canonical_dest,
            max_files,
            max_decompressed_bytes,
        ),
        ArchiveFormat::SevenZ => extract_7z(
            archive_path,
            &canonical_dest,
            max_files,
            max_decompressed_bytes,
        ),
        ArchiveFormat::AppImage => extract_appimage(archive_path, &canonical_dest),
    }
}

fn extract_zip(
    archive_path: &Path,
    dest: &Path,
    max_files: usize,
    max_bytes: u64,
) -> Result<(), ExtractionError> {
    let file = File::open(archive_path)?;
    let mut zip = zip::ZipArchive::new(file)?;
    let mut total_files = 0;
    let mut total_bytes: u64 = 0;

    for i in 0..zip.len() {
        total_files += 1;
        if total_files > max_files {
            return Err(ExtractionError::TooManyFiles(max_files));
        }

        let mut entry = zip.by_index(i)?;
        if entry
            .unix_mode()
            .is_some_and(|mode| mode & 0o170000 == 0o120000)
        {
            return Err(ExtractionError::LinkEscapes(PathBuf::from(entry.name())));
        }
        let enclosed = entry
            .enclosed_name()
            .ok_or_else(|| ExtractionError::PathTraversal(PathBuf::from(entry.name())))?
            .to_path_buf();

        let outpath = dest.join(enclosed);
        ensure_safe_output_path(dest, &outpath, entry.is_dir())?;

        if entry.is_dir() {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut outfile = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&outpath)?;
            let remaining = max_bytes.saturating_sub(total_bytes);
            let written = io::copy(
                &mut entry.by_ref().take(remaining.saturating_add(1)),
                &mut outfile,
            )?;
            total_bytes += written;
            if total_bytes > max_bytes {
                return Err(ExtractionError::TooLarge(max_bytes));
            }

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Some(mode) = entry.unix_mode() {
                    fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))?;
                }
            }
        }
    }

    Ok(())
}

fn extract_tar_gz(
    archive_path: &Path,
    dest: &Path,
    max_files: usize,
    max_bytes: u64,
) -> Result<(), ExtractionError> {
    let file = File::open(archive_path)?;
    let gz = flate2::read::GzDecoder::new(BufReader::new(file));
    let mut archive = tar::Archive::new(gz);
    unpack_tar_safely(&mut archive, dest, max_files, max_bytes)
}

fn extract_tar_xz(
    archive_path: &Path,
    dest: &Path,
    max_files: usize,
    max_bytes: u64,
) -> Result<(), ExtractionError> {
    let mut xz_file = BufReader::new(File::open(archive_path)?);
    let mut decompressed_tar = tempfile::tempfile()?;
    let tar_overhead = (max_files as u64)
        .saturating_mul(1024)
        .saturating_add(1024 * 1024);
    let stream_limit = max_bytes.saturating_add(tar_overhead);
    let mut limited = LimitedWriter {
        file: &mut decompressed_tar,
        written: 0,
        limit: stream_limit,
    };
    lzma_rs::xz_decompress(&mut xz_file, &mut limited)?;
    io::Seek::seek(&mut decompressed_tar, io::SeekFrom::Start(0))?;

    let mut archive = tar::Archive::new(decompressed_tar);
    unpack_tar_safely(&mut archive, dest, max_files, max_bytes)
}

struct LimitedWriter<'a> {
    file: &'a mut File,
    written: u64,
    limit: u64,
}

impl io::Write for LimitedWriter<'_> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if self.written.saturating_add(bytes.len() as u64) > self.limit {
            return Err(io::Error::other(
                "decompressed tar stream exceeds safety limit",
            ));
        }
        let written = self.file.write(bytes)?;
        self.written += written as u64;
        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
    }
}

fn unpack_tar_safely<R: io::Read>(
    archive: &mut tar::Archive<R>,
    dest: &Path,
    max_files: usize,
    max_bytes: u64,
) -> Result<(), ExtractionError> {
    let mut total_files = 0;
    let mut total_bytes: u64 = 0;

    for entry_res in archive.entries()? {
        total_files += 1;
        if total_files > max_files {
            return Err(ExtractionError::TooManyFiles(max_files));
        }

        let mut entry = entry_res?;
        let path = entry.path()?.into_owned();

        // Check for path traversal in entry path
        if path.is_absolute()
            || path
                .components()
                .any(|c| c == std::path::Component::ParentDir)
        {
            return Err(ExtractionError::PathTraversal(path));
        }

        let outpath = dest.join(&path);
        let is_dir = entry.header().entry_type().is_dir();
        ensure_safe_output_path(dest, &outpath, is_dir)?;

        let (is_link, mode) = {
            let header = entry.header();
            (
                header.entry_type().is_symlink() || header.entry_type().is_hard_link(),
                header.mode().ok(),
            )
        };

        // Check symlinks and hardlinks
        if is_link {
            if let Some(target) = entry.link_name()? {
                let target_path = if target.is_absolute() {
                    target.into_owned()
                } else if entry.header().entry_type().is_hard_link() {
                    dest.join(&target)
                } else if let Some(parent) = outpath.parent() {
                    parent.join(&target)
                } else {
                    dest.join(&target)
                };

                let normalized = normalize_path(&target_path);
                if !normalized.starts_with(dest) {
                    return Err(ExtractionError::LinkEscapes(target_path));
                }
            }
        }

        let size = entry.size();
        total_bytes += size;
        if total_bytes > max_bytes {
            return Err(ExtractionError::TooLarge(max_bytes));
        }

        entry.unpack_in(dest)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = mode {
                if outpath.exists() && (mode & 0o111 != 0) {
                    let _ = fs::set_permissions(&outpath, fs::Permissions::from_mode(mode | 0o755));
                }
            }
        }
    }

    Ok(())
}

fn extract_7z(
    archive_path: &Path,
    dest: &Path,
    max_files: usize,
    max_bytes: u64,
) -> Result<(), ExtractionError> {
    let mut total_files = 0;
    let mut total_bytes: u64 = 0;

    sevenz_rust::decompress_file_with_extract_fn(archive_path, dest, |entry, reader, out_dir| {
        total_files += 1;
        if total_files > max_files {
            return Err(sevenz_rust::Error::other(format!(
                "Too many files in archive (limit {max_files})"
            )));
        }

        let path = PathBuf::from(entry.name());
        if path.is_absolute()
            || path
                .components()
                .any(|c| c == std::path::Component::ParentDir)
        {
            return Err(sevenz_rust::Error::other(format!(
                "Path traversal in 7z: {:?}",
                entry.name()
            )));
        }

        let outpath = out_dir.join(&path);
        if entry.is_directory() {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut outfile = File::create(&outpath)?;
            let remaining = max_bytes.saturating_sub(total_bytes);
            let written = io::copy(&mut reader.take(remaining.saturating_add(1)), &mut outfile)?;
            total_bytes += written;
            if total_bytes > max_bytes {
                return Err(sevenz_rust::Error::other(format!(
                    "Archive too large (limit {max_bytes} bytes)"
                )));
            }
        }
        Ok(true)
    })
    .map_err(|e| ExtractionError::SevenZ(e.to_string()))
}

fn extract_appimage(archive_path: &Path, dest: &Path) -> Result<(), ExtractionError> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(archive_path, fs::Permissions::from_mode(0o755));
        let status = std::process::Command::new(archive_path)
            .arg("--appimage-extract")
            .current_dir(dest)
            .status()?;

        if !status.success() {
            return Err(ExtractionError::AppImageFailed(format!(
                "AppImage --appimage-extract exited with status {:?}",
                status.code()
            )));
        }
        Ok(())
    }
    #[cfg(not(unix))]
    {
        Err(ExtractionError::AppImageFailed(
            "AppImage extraction is only supported on Unix".into(),
        ))
    }
}

fn normalize_path(path: &Path) -> PathBuf {
    let mut components = Vec::new();
    for component in path.components() {
        match component {
            std::path::Component::ParentDir => {
                components.pop();
            }
            std::path::Component::CurDir => {}
            c => components.push(c),
        }
    }
    components.iter().collect()
}

fn ensure_safe_output_path(
    dest: &Path,
    output: &Path,
    allow_existing_dir: bool,
) -> Result<(), ExtractionError> {
    if output == dest {
        return Ok(());
    }
    let relative = output
        .strip_prefix(dest)
        .map_err(|_| ExtractionError::PathTraversal(output.to_path_buf()))?;
    let mut current = dest.to_path_buf();
    let components: Vec<_> = relative.components().collect();
    for component in components.iter().take(components.len().saturating_sub(1)) {
        current.push(component);
        match fs::symlink_metadata(&current) {
            Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => {
                return Err(ExtractionError::PathTraversal(current));
            }
            Ok(_) => {}
            Err(error) if error.kind() == io::ErrorKind::NotFound => fs::create_dir(&current)?,
            Err(error) => return Err(error.into()),
        }
    }
    let canonical_parent = output.parent().unwrap_or(dest).canonicalize()?;
    if !canonical_parent.starts_with(dest) {
        return Err(ExtractionError::PathTraversal(output.to_path_buf()));
    }
    if let Ok(metadata) = fs::symlink_metadata(output) {
        if !(allow_existing_dir && metadata.is_dir() && !metadata.file_type().is_symlink()) {
            return Err(ExtractionError::PathTraversal(output.to_path_buf()));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::LimitedWriter;
    use std::io::Write;

    #[test]
    fn limited_writer_rejects_bytes_beyond_expanded_tar_budget() {
        let mut file = tempfile::tempfile().unwrap();
        let mut writer = LimitedWriter {
            file: &mut file,
            written: 0,
            limit: 4,
        };

        assert_eq!(writer.write(b"four").unwrap(), 4);
        assert!(writer.write(b"x").is_err());
        assert_eq!(writer.written, 4);
    }
}
