use std::collections::{HashMap, HashSet};
use std::ffi::OsString;
use std::fs::{self, File};
use std::io::{self, BufReader, Read};
use std::path::{Path, PathBuf};
use thiserror::Error;
#[cfg(unix)]
use tokio::process::Command;
use tokio_util::sync::CancellationToken;

#[cfg(unix)]
use crate::executors::process::run_command;
#[cfg(unix)]
use crate::executors::ExecutionError;

use super::manifest::ArchiveFormat;

pub const DEFAULT_MAX_FILES: usize = 50_000;
pub const DEFAULT_MAX_DECOMPRESSED_BYTES: u64 = 1_000_000_000; // 1 GB
/// Maximum time allowed for a self-extracting AppImage to unpack its contents.
pub const APPIMAGE_TIMEOUT_SECONDS: u32 = 300;

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
    #[error("Extraction was cancelled")]
    Cancelled,
    #[error("AppImage extraction timed out after {0} seconds")]
    AppImageTimedOut(u32),
}

pub async fn extract_archive(
    archive_path: &Path,
    format: ArchiveFormat,
    destination: &Path,
    max_files: usize,
    max_decompressed_bytes: u64,
    cancel: &CancellationToken,
) -> Result<(), ExtractionError> {
    if cancel.is_cancelled() {
        return Err(ExtractionError::Cancelled);
    }

    if format == ArchiveFormat::AppImage {
        fs::create_dir_all(destination)?;
        let canonical_dest = destination.canonicalize()?;
        return extract_appimage(archive_path, &canonical_dest, cancel).await;
    }

    let archive_path = archive_path.to_path_buf();
    let destination = destination.to_path_buf();
    tokio::task::spawn_blocking(move || {
        extract_archive_blocking(
            &archive_path,
            format,
            &destination,
            max_files,
            max_decompressed_bytes,
        )
    })
    .await
    .map_err(io::Error::other)??;

    if cancel.is_cancelled() {
        return Err(ExtractionError::Cancelled);
    }
    Ok(())
}

fn extract_archive_blocking(
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
        ArchiveFormat::AppImage => unreachable!("AppImage extraction uses the async runner"),
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
    if zip.len() > max_files {
        return Err(ExtractionError::TooManyFiles(max_files));
    }

    let mut symlinks = HashMap::new();
    let mut directories = HashSet::new();
    let mut preflight_bytes = 0_u64;
    for i in 0..zip.len() {
        let mut entry = zip.by_index(i)?;
        preflight_bytes = preflight_bytes.saturating_add(entry.size());
        if preflight_bytes > max_bytes {
            return Err(ExtractionError::TooLarge(max_bytes));
        }
        let unix_mode = entry.unix_mode();
        let enclosed = entry
            .enclosed_name()
            .ok_or_else(|| ExtractionError::PathTraversal(zip_entry_path_for_error(&entry)))?;
        let relative = relative_archive_path(&enclosed)?;
        if entry.is_dir() || unix_mode.is_some_and(|mode| mode & 0o170000 == 0o040000) {
            directories.insert(relative.clone());
        }
        let mut parent = relative.parent();
        while let Some(path) = parent {
            if !path.as_os_str().is_empty() {
                directories.insert(path.to_path_buf());
            }
            parent = path.parent();
        }
        if unix_mode.is_some_and(|mode| mode & 0o170000 == 0o120000) {
            let path = relative;
            let target = read_zip_link_target(&mut entry, &path)?;
            if symlinks.insert(path.clone(), target).is_some() {
                return Err(ExtractionError::LinkEscapes(path));
            }
        }
    }
    validate_symlink_targets(dest, &symlinks)?;

    let mut total_files = 0;
    let mut total_bytes: u64 = 0;

    for i in 0..zip.len() {
        total_files += 1;
        if total_files > max_files {
            return Err(ExtractionError::TooManyFiles(max_files));
        }

        let mut entry = zip.by_index(i)?;
        let unix_mode = entry.unix_mode();
        let is_symlink = unix_mode.is_some_and(|mode| mode & 0o170000 == 0o120000);
        let enclosed = entry
            .enclosed_name()
            .ok_or_else(|| ExtractionError::PathTraversal(zip_entry_path_for_error(&entry)))?
            .to_path_buf();

        let relative = relative_archive_path(&enclosed)?;
        let is_dir = entry.is_dir() || unix_mode.is_some_and(|mode| mode & 0o170000 == 0o040000);
        if relative.as_os_str().is_empty() {
            if is_dir {
                let remaining = max_bytes.saturating_sub(total_bytes);
                if entry.size() > remaining {
                    return Err(ExtractionError::TooLarge(max_bytes));
                }
                total_bytes = total_bytes.saturating_add(entry.size());
                continue;
            }
            return Err(ExtractionError::PathTraversal(enclosed));
        }
        let outpath = dest.join(&relative);
        if is_symlink {
            ensure_safe_output_path(dest, &outpath, false)?;
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent)?;
            }
            let remaining = max_bytes.saturating_sub(total_bytes);
            if entry.size() > remaining {
                return Err(ExtractionError::TooLarge(max_bytes));
            }
            let target = read_zip_link_target(&mut entry, &relative)?;
            total_bytes = total_bytes.saturating_add(entry.size());
            let target_is_dir =
                resolve_link_target(dest, &relative, &target, &symlinks).is_ok_and(|resolved| {
                    directories.contains(&resolved) || dest.join(resolved).is_dir()
                });
            create_symlink(&target, &outpath, target_is_dir)?;
            continue;
        }

        let existed = path_exists(&outpath)?;
        ensure_safe_output_path(dest, &outpath, is_dir)?;

        if is_dir {
            let remaining = max_bytes.saturating_sub(total_bytes);
            if entry.size() > remaining {
                return Err(ExtractionError::TooLarge(max_bytes));
            }
            total_bytes = total_bytes.saturating_add(entry.size());
            if !existed {
                create_extraction_dir(&outpath)?;
            }
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
            total_bytes = total_bytes.saturating_add(written);
            if total_bytes > max_bytes {
                return Err(ExtractionError::TooLarge(max_bytes));
            }

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mode = if unix_mode.is_some_and(|mode| mode & 0o111 != 0) {
                    0o755
                } else {
                    0o644
                };
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))?;
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
    let mut gz = flate2::read::GzDecoder::new(BufReader::new(file));
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
    io::copy(&mut gz, &mut limited)?;
    io::Seek::seek(&mut decompressed_tar, io::SeekFrom::Start(0))?;

    let archive = tar::Archive::new(decompressed_tar);
    unpack_tar_safely(archive, dest, max_files, max_bytes)
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

    let archive = tar::Archive::new(decompressed_tar);
    unpack_tar_safely(archive, dest, max_files, max_bytes)
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

fn unpack_tar_safely<R: io::Read + io::Seek>(
    mut archive: tar::Archive<R>,
    dest: &Path,
    max_files: usize,
    max_bytes: u64,
) -> Result<(), ExtractionError> {
    let symlinks = collect_tar_symlinks(&mut archive, dest, max_files)?;
    let mut reader = archive.into_inner();
    io::Seek::seek(&mut reader, io::SeekFrom::Start(0))?;
    let mut archive = tar::Archive::new(reader);

    let mut total_files = 0;
    let mut total_bytes: u64 = 0;

    for entry_res in archive.entries()? {
        total_files += 1;
        if total_files > max_files {
            return Err(ExtractionError::TooManyFiles(max_files));
        }

        let mut entry = entry_res?;
        let path = entry.path()?.into_owned();
        let relative = relative_archive_path(&path)?;

        let is_dir = entry.header().entry_type().is_dir();
        if relative.as_os_str().is_empty() {
            if is_dir {
                let size = entry.size();
                total_bytes = total_bytes.saturating_add(size);
                if total_bytes > max_bytes {
                    return Err(ExtractionError::TooLarge(max_bytes));
                }
                continue;
            }
            return Err(ExtractionError::PathTraversal(path));
        }

        let outpath = dest.join(&relative);
        let existed = path_exists(&outpath)?;
        ensure_safe_output_path(dest, &outpath, is_dir)?;

        let is_hardlink = entry.header().entry_type().is_hard_link();
        #[cfg(unix)]
        let (is_symlink, mode) = (
            entry.header().entry_type().is_symlink(),
            entry.header().mode().ok(),
        );

        // Hardlinks use archive-root-relative targets, unlike symlinks.
        if is_hardlink {
            if let Some(target) = entry.link_name()? {
                resolve_confined_target(dest, &target, &symlinks)
                    .map_err(|()| ExtractionError::LinkEscapes(target.into_owned()))?;
            }
        }

        let size = entry.size();
        total_bytes = total_bytes.saturating_add(size);
        if total_bytes > max_bytes {
            return Err(ExtractionError::TooLarge(max_bytes));
        }

        if is_dir {
            if !existed {
                create_extraction_dir(&outpath)?;
            }
            continue;
        }

        entry.set_mask(0o7022);
        entry.unpack_in(dest)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if !is_symlink && !is_hardlink {
                if let Ok(metadata) = fs::symlink_metadata(&outpath) {
                    if metadata.file_type().is_file() {
                        let permissions = if mode.is_some_and(|mode| mode & 0o111 != 0) {
                            0o755
                        } else {
                            0o644
                        };
                        fs::set_permissions(&outpath, fs::Permissions::from_mode(permissions))?;
                    }
                }
            }
        }
    }

    Ok(())
}

fn collect_tar_symlinks<R: io::Read>(
    archive: &mut tar::Archive<R>,
    dest: &Path,
    max_files: usize,
) -> Result<HashMap<PathBuf, PathBuf>, ExtractionError> {
    let mut links = HashMap::new();
    let mut total_files = 0;
    for entry_res in archive.entries()? {
        total_files += 1;
        if total_files > max_files {
            return Err(ExtractionError::TooManyFiles(max_files));
        }
        let entry = entry_res?;
        if entry.header().entry_type().is_symlink() {
            let entry_path = entry.path()?;
            let path = relative_archive_path(&entry_path)?;
            let target = entry
                .link_name()?
                .ok_or_else(|| ExtractionError::LinkEscapes(path.clone()))?
                .into_owned();
            if links.insert(path.clone(), target).is_some() {
                return Err(ExtractionError::LinkEscapes(path));
            }
        }
    }
    validate_symlink_targets(dest, &links)?;
    Ok(links)
}

fn extract_7z(
    archive_path: &Path,
    dest: &Path,
    max_files: usize,
    max_bytes: u64,
) -> Result<(), ExtractionError> {
    let file = File::open(archive_path)?;
    let mut archive = sevenz_rust2::ArchiveReader::new(file, sevenz_rust2::Password::empty())
        .map_err(|error| ExtractionError::SevenZ(error.to_string()))?;
    let mut total_files = 0;
    let mut total_bytes: u64 = 0;
    let mut callback_error = None;

    archive
        .for_each_entries(|entry, reader| {
            total_files += 1;
            if total_files > max_files {
                callback_error = Some(ExtractionError::TooManyFiles(max_files));
                return Err(io::Error::other("too many files in 7z archive").into());
            }

            let relative = match relative_7z_archive_path(entry.name()) {
                Ok(relative) => relative,
                Err(error) => {
                    callback_error = Some(error);
                    return Err(io::Error::other("unsafe path in 7z archive").into());
                }
            };
            if relative.as_os_str().is_empty() {
                if entry.is_directory() {
                    return Ok(true);
                }
                callback_error = Some(ExtractionError::PathTraversal(PathBuf::from(entry.name())));
                return Err(io::Error::other("empty file path in 7z archive").into());
            }

            let outpath = dest.join(&relative);
            let is_dir = entry.is_directory();
            let existed = path_exists(&outpath)?;
            if let Err(error) = ensure_safe_output_path(dest, &outpath, is_dir) {
                callback_error = Some(error);
                return Err(io::Error::other("unsafe output path in 7z archive").into());
            }

            if is_dir {
                if !existed {
                    create_extraction_dir(&outpath)?;
                }
            } else {
                let mut outfile = fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(&outpath)?;
                let remaining = max_bytes.saturating_sub(total_bytes);
                let written =
                    io::copy(&mut reader.take(remaining.saturating_add(1)), &mut outfile)?;
                total_bytes = total_bytes.saturating_add(written);
                if total_bytes > max_bytes {
                    callback_error = Some(ExtractionError::TooLarge(max_bytes));
                    return Err(io::Error::other("7z archive exceeds expanded-byte limit").into());
                }
            }
            Ok(true)
        })
        .map_err(|error| {
            callback_error.unwrap_or_else(|| ExtractionError::SevenZ(error.to_string()))
        })
}

fn relative_7z_archive_path(name: &str) -> Result<PathBuf, ExtractionError> {
    let normalized = name.replace('\\', "/");
    let bytes = normalized.as_bytes();
    if bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':' {
        return Err(ExtractionError::PathTraversal(PathBuf::from(name)));
    }
    relative_archive_path(Path::new(&normalized))
}

fn zip_entry_path_for_error<R: Read + ?Sized>(entry: &zip::read::ZipFile<'_, R>) -> PathBuf {
    entry
        .name()
        .map(|name| PathBuf::from(name.into_owned()))
        .unwrap_or_else(|_| PathBuf::from(String::from_utf8_lossy(entry.name_raw()).into_owned()))
}

async fn extract_appimage(
    archive_path: &Path,
    dest: &Path,
    cancel: &CancellationToken,
) -> Result<(), ExtractionError> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(archive_path, fs::Permissions::from_mode(0o755));
        let mut command = Command::new(archive_path);
        command.arg("--appimage-extract").current_dir(dest);
        let output = run_command(&mut command, APPIMAGE_TIMEOUT_SECONDS, cancel)
            .await
            .map_err(map_appimage_process_error)?;

        if !output.status.success() {
            return Err(ExtractionError::AppImageFailed(format!(
                "AppImage --appimage-extract exited with status {:?}",
                output.status.code()
            )));
        }
        Ok(())
    }
    #[cfg(not(unix))]
    {
        let _ = (archive_path, dest, cancel);
        Err(ExtractionError::AppImageFailed(
            "AppImage extraction is only supported on Unix".into(),
        ))
    }
}

#[cfg(unix)]
fn map_appimage_process_error(error: ExecutionError) -> ExtractionError {
    match error {
        ExecutionError::Io(error) => ExtractionError::Io(error),
        ExecutionError::Cancelled => ExtractionError::Cancelled,
        ExecutionError::TimedOut(seconds) => ExtractionError::AppImageTimedOut(seconds),
        other => ExtractionError::AppImageFailed(other.to_string()),
    }
}

const MAX_SYMLINK_TARGET_BYTES: u64 = 64 * 1024;
const MAX_SYMLINK_EXPANSIONS: usize = 40;

fn read_zip_link_target(reader: &mut impl Read, path: &Path) -> Result<PathBuf, ExtractionError> {
    let mut bytes = Vec::new();
    (&mut *reader)
        .take(MAX_SYMLINK_TARGET_BYTES + 1)
        .read_to_end(&mut bytes)?;
    if bytes.len() as u64 > MAX_SYMLINK_TARGET_BYTES {
        return Err(ExtractionError::TooLarge(MAX_SYMLINK_TARGET_BYTES));
    }
    let target =
        String::from_utf8(bytes).map_err(|_| ExtractionError::LinkEscapes(path.to_path_buf()))?;
    if target.is_empty() {
        return Err(ExtractionError::LinkEscapes(path.to_path_buf()));
    }
    Ok(PathBuf::from(target))
}

fn relative_archive_path(path: &Path) -> Result<PathBuf, ExtractionError> {
    if has_root_or_prefix(path)
        || path
            .components()
            .any(|component| component == std::path::Component::ParentDir)
    {
        return Err(ExtractionError::PathTraversal(path.to_path_buf()));
    }

    let mut relative = PathBuf::new();
    for component in path.components() {
        match component {
            std::path::Component::Normal(component) => relative.push(component),
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir
            | std::path::Component::RootDir
            | std::path::Component::Prefix(_) => {
                return Err(ExtractionError::PathTraversal(path.to_path_buf()));
            }
        }
    }
    Ok(relative)
}

fn has_root_or_prefix(path: &Path) -> bool {
    path.components().any(|component| {
        matches!(
            component,
            std::path::Component::RootDir | std::path::Component::Prefix(_)
        )
    })
}

fn validate_symlink_targets(
    dest: &Path,
    links: &HashMap<PathBuf, PathBuf>,
) -> Result<(), ExtractionError> {
    for (link_path, target) in links {
        if target.as_os_str().is_empty() || has_root_or_prefix(target) {
            return Err(ExtractionError::LinkEscapes(dest.join(link_path)));
        }
        resolve_link_target(dest, link_path, target, links)
            .map_err(|()| ExtractionError::LinkEscapes(dest.join(link_path)))?;
    }
    Ok(())
}

fn resolve_link_target(
    dest: &Path,
    link_path: &Path,
    target: &Path,
    links: &HashMap<PathBuf, PathBuf>,
) -> Result<PathBuf, ()> {
    if target.as_os_str().is_empty() || has_root_or_prefix(target) {
        return Err(());
    }
    let mut resolved = Vec::new();
    let mut active = HashSet::new();
    let mut remaining_expansions = MAX_SYMLINK_EXPANSIONS;
    active.insert(link_path.to_path_buf());
    if let Some(parent) = link_path.parent() {
        resolve_link_components(
            dest,
            &mut resolved,
            parent,
            links,
            &mut active,
            &mut remaining_expansions,
        )?;
    }
    resolve_link_components(
        dest,
        &mut resolved,
        target,
        links,
        &mut active,
        &mut remaining_expansions,
    )?;
    Ok(resolved.iter().collect())
}

fn resolve_confined_target(
    dest: &Path,
    target: &Path,
    links: &HashMap<PathBuf, PathBuf>,
) -> Result<PathBuf, ()> {
    if has_root_or_prefix(target) {
        return Err(());
    }
    let mut resolved = Vec::new();
    let mut active = HashSet::new();
    let mut remaining_expansions = MAX_SYMLINK_EXPANSIONS;
    resolve_link_components(
        dest,
        &mut resolved,
        target,
        links,
        &mut active,
        &mut remaining_expansions,
    )?;
    Ok(resolved.iter().collect())
}

fn resolve_link_components(
    dest: &Path,
    resolved: &mut Vec<OsString>,
    path: &Path,
    links: &HashMap<PathBuf, PathBuf>,
    active: &mut HashSet<PathBuf>,
    remaining_expansions: &mut usize,
) -> Result<(), ()> {
    for component in path.components() {
        match component {
            std::path::Component::Normal(name) => {
                resolved.push(name.to_os_string());
                let relative: PathBuf = resolved.iter().collect();
                let target = links.get(&relative).cloned().or_else(|| {
                    let disk_path = dest.join(&relative);
                    fs::symlink_metadata(&disk_path)
                        .ok()
                        .filter(|metadata| metadata.file_type().is_symlink())
                        .and_then(|_| fs::read_link(disk_path).ok())
                });
                if let Some(target) = target {
                    if target.as_os_str().is_empty() || has_root_or_prefix(&target) {
                        return Err(());
                    }
                    if *remaining_expansions == 0
                        || active.len() >= MAX_SYMLINK_EXPANSIONS
                        || !active.insert(relative.clone())
                    {
                        return Err(());
                    }
                    *remaining_expansions -= 1;
                    resolved.pop();
                    let result = resolve_link_components(
                        dest,
                        resolved,
                        &target,
                        links,
                        active,
                        remaining_expansions,
                    );
                    active.remove(&relative);
                    result?;
                }
            }
            std::path::Component::ParentDir => {
                if resolved.pop().is_none() {
                    return Err(());
                }
            }
            std::path::Component::CurDir => {}
            std::path::Component::RootDir | std::path::Component::Prefix(_) => return Err(()),
        }
    }
    Ok(())
}

fn create_symlink(target: &Path, link: &Path, target_is_dir: bool) -> io::Result<()> {
    #[cfg(unix)]
    {
        let _ = target_is_dir;
        std::os::unix::fs::symlink(target, link)
    }
    #[cfg(windows)]
    {
        if target_is_dir {
            std::os::windows::fs::symlink_dir(target, link)
        } else {
            std::os::windows::fs::symlink_file(target, link)
        }
    }
    #[cfg(not(any(unix, windows)))]
    {
        let _ = (target, link, target_is_dir);
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "symbolic links are not supported on this platform",
        ))
    }
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
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                create_extraction_dir(&current)?;
            }
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

fn path_exists(path: &Path) -> io::Result<bool> {
    match fs::symlink_metadata(path) {
        Ok(_) => Ok(true),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(error),
    }
}

fn create_extraction_dir(path: &Path) -> io::Result<()> {
    fs::create_dir(path)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o755))?;
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
