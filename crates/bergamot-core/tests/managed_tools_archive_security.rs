#![cfg(unix)]

use bergamot_core::managed_tools::extractor::{extract_archive, ExtractionError};
use bergamot_core::managed_tools::manifest::ArchiveFormat;
#[cfg(target_os = "linux")]
use bergamot_core::managed_tools::manifest::ManagedToolsManifest;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use tempfile::tempdir;

#[cfg(target_os = "linux")]
fn assert_normalized_tree(path: &Path) {
    use std::os::unix::fs::PermissionsExt;
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let metadata = fs::symlink_metadata(&path).unwrap();
        if metadata.file_type().is_symlink() {
            assert_eq!(metadata.permissions().mode() & 0o777, 0o777, "{path:?}");
        } else if metadata.is_dir() {
            assert_normalized_tree(&path);
        } else if metadata.is_file() {
            let permissions = metadata.permissions().mode() & 0o7777;
            let expected = if permissions & 0o111 != 0 {
                0o755
            } else {
                0o644
            };
            assert_eq!(permissions, expected, "{path:?}");
            assert_eq!(
                permissions & 0o022,
                0,
                "{path:?} must not be writable by others"
            );
        }
    }
}

#[cfg(target_os = "linux")]
fn assert_python_runs(binary: &Path) {
    let output = std::process::Command::new(binary)
        .args(["-c", "import sys; print(sys.version_info.major)"])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "3");
}

enum TarEntry<'a> {
    File {
        path: &'a str,
        contents: &'a [u8],
        mode: u32,
    },
    Symlink {
        path: &'a str,
        target: &'a str,
    },
    Hardlink {
        path: &'a str,
        target: &'a str,
        mode: u32,
    },
    Directory {
        path: &'a str,
        mode: u32,
    },
}

fn create_tar_gz(path: &Path, entries: &[TarEntry<'_>]) {
    let file = File::create(path).unwrap();
    let encoder = flate2::write::GzEncoder::new(file, flate2::Compression::default());
    let mut tar = tar::Builder::new(encoder);
    for item in entries {
        match item {
            TarEntry::File {
                path,
                contents,
                mode,
            } => {
                let mut header = tar::Header::new_gnu();
                header.set_entry_type(tar::EntryType::Regular);
                header.set_mode(*mode);
                header.set_size(contents.len() as u64);
                header.set_cksum();
                tar.append_data(&mut header, path, *contents).unwrap();
            }
            TarEntry::Symlink { path, target } => {
                let mut header = tar::Header::new_gnu();
                header.set_entry_type(tar::EntryType::Symlink);
                header.set_mode(0o777);
                header.set_size(0);
                header.set_cksum();
                tar.append_link(&mut header, path, target).unwrap();
            }
            TarEntry::Hardlink { path, target, mode } => {
                let mut header = tar::Header::new_gnu();
                header.set_entry_type(tar::EntryType::Link);
                header.set_mode(*mode);
                header.set_size(0);
                header.set_cksum();
                tar.append_link(&mut header, path, target).unwrap();
            }
            TarEntry::Directory { path, mode } => {
                let mut header = tar::Header::new_gnu();
                header.set_entry_type(tar::EntryType::Directory);
                header.set_mode(*mode);
                header.set_size(0);
                header.set_cksum();
                tar.append_data(&mut header, path, &[][..]).unwrap();
            }
        }
    }
    tar.finish().unwrap();
    tar.into_inner().unwrap().finish().unwrap();
}

#[cfg(unix)]
fn assert_tar_entry_modes(archive_path: &Path, expected: &[(&str, u32)]) {
    let decoder = flate2::read::GzDecoder::new(File::open(archive_path).unwrap());
    let mut archive = tar::Archive::new(decoder);
    let mut actual = std::collections::HashMap::new();
    for entry in archive.entries().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path().unwrap().into_owned();
        actual.insert(
            path.to_string_lossy().into_owned(),
            entry.header().mode().unwrap(),
        );
    }
    for (path, mode) in expected {
        assert_eq!(actual.get(*path), Some(mode), "archive mode for {path}");
    }
}

#[cfg(unix)]
fn patch_zip_permissions(archive_path: &Path, changes: &[(&str, u32)]) {
    let mut bytes = fs::read(archive_path).unwrap();
    let eocd = bytes
        .windows(4)
        .rposition(|window| window == b"PK\x05\x06")
        .expect("ZIP end-of-central-directory record");
    let entry_count = u16::from_le_bytes(bytes[eocd + 10..eocd + 12].try_into().unwrap());
    let central_offset =
        u32::from_le_bytes(bytes[eocd + 16..eocd + 20].try_into().unwrap()) as usize;
    let mut offset = central_offset;
    let mut patched = std::collections::HashSet::new();

    for _ in 0..entry_count {
        assert_eq!(&bytes[offset..offset + 4], b"PK\x01\x02");
        let name_len =
            u16::from_le_bytes(bytes[offset + 28..offset + 30].try_into().unwrap()) as usize;
        let extra_len =
            u16::from_le_bytes(bytes[offset + 30..offset + 32].try_into().unwrap()) as usize;
        let comment_len =
            u16::from_le_bytes(bytes[offset + 32..offset + 34].try_into().unwrap()) as usize;
        let name_start = offset + 46;
        let name = std::str::from_utf8(&bytes[name_start..name_start + name_len])
            .unwrap()
            .to_owned();
        if let Some((_, permissions)) = changes.iter().find(|(path, _)| *path == name) {
            let attrs = u32::from_le_bytes(bytes[offset + 38..offset + 42].try_into().unwrap());
            let mode = (attrs >> 16) as u16;
            assert_ne!(mode & 0o170000, 0, "fixture should have a Unix file type");
            let patched_mode = (mode & !0o7777) | (*permissions as u16 & 0o7777);
            let patched_attrs = (u32::from(patched_mode) << 16) | (attrs & 0xffff);
            bytes[offset + 38..offset + 42].copy_from_slice(&patched_attrs.to_le_bytes());
            patched.insert(name);
        }
        offset = name_start + name_len + extra_len + comment_len;
    }

    assert_eq!(patched.len(), changes.len(), "all fixture entries patched");
    fs::write(archive_path, bytes).unwrap();

    let mut archive = zip::ZipArchive::new(File::open(archive_path).unwrap()).unwrap();
    for (name, permissions) in changes {
        let entry = archive.by_name(name).unwrap();
        let mode = entry
            .unix_mode()
            .expect("patched Unix mode must be readable");
        assert_eq!(mode & 0o7777, *permissions, "fixture mode for {name}");
    }
}

#[cfg(unix)]
fn mode(path: &Path) -> u32 {
    use std::os::unix::fs::PermissionsExt;
    fs::symlink_metadata(path).unwrap().permissions().mode() & 0o7777
}

#[cfg(unix)]
#[test]
fn tar_normalizes_regular_modes_without_chmod_through_symlinks() {
    let temp = tempdir().unwrap();
    let archive = temp.path().join("modes.tar.gz");
    let destination = temp.path().join("extracted");
    create_tar_gz(
        &archive,
        &[
            TarEntry::File {
                path: "target",
                contents: b"executable",
                mode: 0o775,
            },
            TarEntry::File {
                path: "data",
                contents: b"ordinary",
                mode: 0o666,
            },
            TarEntry::Symlink {
                path: "link",
                target: "target",
            },
        ],
    );

    extract_archive(&archive, ArchiveFormat::TarGz, &destination, 10, 1_000).unwrap();

    assert_eq!(mode(&destination.join("target")), 0o755);
    assert_eq!(mode(&destination.join("data")), 0o644);
    assert!(destination.join("link").is_symlink());
    assert_eq!(
        fs::read_link(destination.join("link")).unwrap(),
        PathBuf::from("target")
    );
    assert_eq!(mode(&destination.join("link")), 0o777);
    assert_eq!(mode(&destination.join("target")), 0o755);
}

#[cfg(unix)]
#[test]
fn tar_strips_special_bits_and_group_other_writes_from_files() {
    let temp = tempdir().unwrap();
    let archive = temp.path().join("special-modes.tar.gz");
    let destination = temp.path().join("extracted");
    create_tar_gz(
        &archive,
        &[
            TarEntry::File {
                path: "executable",
                contents: b"executable",
                mode: 0o7777,
            },
            TarEntry::File {
                path: "ordinary",
                contents: b"ordinary",
                mode: 0o7666,
            },
        ],
    );
    assert_tar_entry_modes(&archive, &[("executable", 0o7777), ("ordinary", 0o7666)]);

    extract_archive(&archive, ArchiveFormat::TarGz, &destination, 10, 1_000).unwrap();

    assert_eq!(mode(&destination.join("executable")), 0o755);
    assert_eq!(mode(&destination.join("ordinary")), 0o644);
}

#[cfg(unix)]
#[test]
fn tar_normalizes_created_directories_and_preserves_existing_root_permissions() {
    let temp = tempdir().unwrap();
    let archive = temp.path().join("directories.tar.gz");
    let destination = temp.path().join("extracted");
    fs::create_dir(&destination).unwrap();
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(&destination, fs::Permissions::from_mode(0o700)).unwrap();
    create_tar_gz(
        &archive,
        &[
            TarEntry::Directory {
                path: ".",
                mode: 0o7777,
            },
            TarEntry::Directory {
                path: "nested/",
                mode: 0o7777,
            },
            TarEntry::File {
                path: "implicit/payload",
                contents: b"inside",
                mode: 0o644,
            },
        ],
    );

    extract_archive(&archive, ArchiveFormat::TarGz, &destination, 10, 1_000).unwrap();

    assert_eq!(mode(&destination), 0o700);
    assert_eq!(mode(&destination.join("nested")), 0o755);
    assert_eq!(mode(&destination.join("implicit")), 0o755);
}

#[cfg(unix)]
#[test]
fn tar_rejects_non_directory_entries_at_the_extraction_root() {
    let temp = tempdir().unwrap();
    let archive = temp.path().join("root-file.tar.gz");
    let destination = temp.path().join("extracted");
    create_tar_gz(
        &archive,
        &[TarEntry::File {
            path: ".",
            contents: b"not a root directory",
            mode: 0o644,
        }],
    );

    let error = extract_archive(&archive, ArchiveFormat::TarGz, &destination, 10, 1_000)
        .expect_err("non-directory entries at the extraction root must be rejected");
    assert!(
        matches!(error, ExtractionError::PathTraversal(_)),
        "{error:?}"
    );
}

#[cfg(unix)]
#[test]
fn tar_rejects_chained_symlink_escape_without_touching_outside_target() {
    let temp = tempdir().unwrap();
    let archive = temp.path().join("escape.tar.gz");
    let destination = temp.path().join("extracted");
    let outside = temp.path().join("outside");
    fs::write(&outside, b"outside contents").unwrap();
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(&outside, fs::Permissions::from_mode(0o640)).unwrap();

    create_tar_gz(
        &archive,
        &[
            TarEntry::Symlink {
                path: "d",
                target: ".",
            },
            TarEntry::Symlink {
                path: "bad",
                target: "d/../outside",
            },
        ],
    );

    let error = extract_archive(&archive, ArchiveFormat::TarGz, &destination, 10, 1_000)
        .expect_err("a symlink chain that climbs above the root must be rejected");
    assert!(
        matches!(error, ExtractionError::LinkEscapes(_)),
        "{error:?}"
    );
    assert_eq!(fs::read(&outside).unwrap(), b"outside contents");
    assert_eq!(mode(&outside), 0o640);
}

#[cfg(unix)]
#[test]
fn tar_accepts_forward_confined_symlink_chains_with_parent_components() {
    let temp = tempdir().unwrap();
    let archive = temp.path().join("forward-links.tar.gz");
    let destination = temp.path().join("extracted");
    create_tar_gz(
        &archive,
        &[
            TarEntry::Symlink {
                path: "sub/first",
                target: "../middle",
            },
            TarEntry::Symlink {
                path: "middle",
                target: "target",
            },
            TarEntry::File {
                path: "target",
                contents: b"inside",
                mode: 0o775,
            },
        ],
    );

    extract_archive(&archive, ArchiveFormat::TarGz, &destination, 10, 1_000).unwrap();

    assert_eq!(fs::read(destination.join("sub/first")).unwrap(), b"inside");
    assert_eq!(mode(&destination.join("target")), 0o755);
}

#[cfg(unix)]
#[test]
fn tar_rejects_symlink_cycles() {
    let temp = tempdir().unwrap();
    let archive = temp.path().join("cycle.tar.gz");
    let destination = temp.path().join("extracted");
    create_tar_gz(
        &archive,
        &[
            TarEntry::Symlink {
                path: "first",
                target: "second",
            },
            TarEntry::Symlink {
                path: "second",
                target: "first",
            },
        ],
    );

    let error = extract_archive(&archive, ArchiveFormat::TarGz, &destination, 10, 1_000)
        .expect_err("symlink cycles must be rejected");
    assert!(
        matches!(error, ExtractionError::LinkEscapes(_)),
        "{error:?}"
    );
}

#[cfg(unix)]
#[test]
fn tar_rejects_symlink_chains_beyond_the_expansion_limit() {
    let temp = tempdir().unwrap();
    let archive = temp.path().join("long-chain.tar.gz");
    let destination = temp.path().join("extracted");
    let file = File::create(&archive).unwrap();
    let encoder = flate2::write::GzEncoder::new(file, flate2::Compression::default());
    let mut tar = tar::Builder::new(encoder);
    for index in 0..42 {
        let path = format!("link-{index}");
        let target = format!("link-{}", index + 1);
        let mut header = tar::Header::new_gnu();
        header.set_entry_type(tar::EntryType::Symlink);
        header.set_mode(0o777);
        header.set_size(0);
        header.set_cksum();
        tar.append_link(&mut header, path, target).unwrap();
    }
    let data = b"inside";
    let mut header = tar::Header::new_gnu();
    header.set_size(data.len() as u64);
    header.set_cksum();
    tar.append_data(&mut header, "link-42", &data[..]).unwrap();
    tar.finish().unwrap();
    tar.into_inner().unwrap().finish().unwrap();

    let error = extract_archive(&archive, ArchiveFormat::TarGz, &destination, 100, 1_000)
        .expect_err("excessively long symlink chains must be bounded");
    assert!(
        matches!(error, ExtractionError::LinkEscapes(_)),
        "{error:?}"
    );
}

#[cfg(unix)]
#[test]
fn tar_bounds_repeated_symlink_expansions_within_one_target() {
    let temp = tempdir().unwrap();
    let archive = temp.path().join("repeated-expansions.tar.gz");
    let destination = temp.path().join("extracted");
    let repeated_target = vec!["leaf"; 41].join("/");
    create_tar_gz(
        &archive,
        &[
            TarEntry::Symlink {
                path: "first",
                target: &repeated_target,
            },
            TarEntry::Symlink {
                path: "leaf",
                target: ".",
            },
        ],
    );

    let error = extract_archive(&archive, ArchiveFormat::TarGz, &destination, 100, 1_000)
        .expect_err("repeated symlink expansions must consume a shared bound");
    assert!(
        matches!(error, ExtractionError::LinkEscapes(_)),
        "{error:?}"
    );
}

#[cfg(unix)]
#[test]
fn tar_does_not_chmod_a_hardlink_to_an_outside_inode() {
    let temp = tempdir().unwrap();
    let archive = temp.path().join("hardlink.tar.gz");
    let destination = temp.path().join("extracted");
    fs::create_dir(&destination).unwrap();
    let outside = temp.path().join("outside");
    fs::write(&outside, b"outside contents").unwrap();
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(&outside, fs::Permissions::from_mode(0o600)).unwrap();
    fs::hard_link(&outside, destination.join("existing")).unwrap();
    create_tar_gz(
        &archive,
        &[TarEntry::Hardlink {
            path: "new-link",
            target: "existing",
            mode: 0o775,
        }],
    );

    extract_archive(&archive, ArchiveFormat::TarGz, &destination, 10, 1_000).unwrap();

    assert_eq!(fs::read(&outside).unwrap(), b"outside contents");
    assert_eq!(mode(&outside), 0o600);
}

#[cfg(unix)]
#[test]
fn tar_hardlink_target_resolution_honors_symlink_parent_before_parent_dir() {
    let temp = tempdir().unwrap();
    let archive = temp.path().join("hardlink-escape.tar.gz");
    let destination = temp.path().join("extracted");
    let outside = temp.path().join("outside");
    fs::write(&outside, b"outside contents").unwrap();
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(&outside, fs::Permissions::from_mode(0o600)).unwrap();
    create_tar_gz(
        &archive,
        &[
            TarEntry::Symlink {
                path: "d",
                target: ".",
            },
            TarEntry::Hardlink {
                path: "bad",
                target: "d/../outside",
                mode: 0o775,
            },
        ],
    );

    let error = extract_archive(&archive, ArchiveFormat::TarGz, &destination, 10, 1_000)
        .expect_err("hardlink targets must resolve symlink parents before '..'");
    assert!(
        matches!(error, ExtractionError::LinkEscapes(_)),
        "{error:?}"
    );
    assert_eq!(fs::read(&outside).unwrap(), b"outside contents");
    assert_eq!(mode(&outside), 0o600);
}

#[cfg(unix)]
#[test]
fn zip_normalizes_regular_modes_and_accepts_confined_forward_symlinks() {
    let temp = tempdir().unwrap();
    let archive = temp.path().join("modes.zip");
    let destination = temp.path().join("extracted");
    let file = File::create(&archive).unwrap();
    let mut zip = zip::ZipWriter::new(file);
    zip.add_symlink(
        "sub/first",
        "../middle",
        zip::write::SimpleFileOptions::default().unix_permissions(0o777),
    )
    .unwrap();
    zip.add_symlink(
        "middle",
        "target",
        zip::write::SimpleFileOptions::default().unix_permissions(0o777),
    )
    .unwrap();
    zip.start_file(
        "target",
        zip::write::SimpleFileOptions::default().unix_permissions(0o775),
    )
    .unwrap();
    zip.write_all(b"executable").unwrap();
    zip.start_file(
        "data",
        zip::write::SimpleFileOptions::default().unix_permissions(0o666),
    )
    .unwrap();
    zip.write_all(b"ordinary").unwrap();
    zip.finish().unwrap();

    extract_archive(&archive, ArchiveFormat::Zip, &destination, 10, 1_000).unwrap();

    assert_eq!(
        fs::read(destination.join("sub/first")).unwrap(),
        b"executable"
    );
    assert_eq!(mode(&destination.join("target")), 0o755);
    assert_eq!(mode(&destination.join("data")), 0o644);
    assert_eq!(mode(&destination.join("sub/first")), 0o777);
}

#[cfg(unix)]
#[test]
fn zip_strips_special_bits_and_group_other_writes_from_files() {
    let temp = tempdir().unwrap();
    let archive = temp.path().join("special-modes.zip");
    let destination = temp.path().join("extracted");
    let file = File::create(&archive).unwrap();
    let mut zip = zip::ZipWriter::new(file);
    zip.start_file(
        "executable",
        zip::write::SimpleFileOptions::default().unix_permissions(0o755),
    )
    .unwrap();
    zip.write_all(b"executable").unwrap();
    zip.start_file(
        "ordinary",
        zip::write::SimpleFileOptions::default().unix_permissions(0o644),
    )
    .unwrap();
    zip.write_all(b"ordinary").unwrap();
    zip.finish().unwrap();
    patch_zip_permissions(&archive, &[("executable", 0o7777), ("ordinary", 0o7666)]);

    extract_archive(&archive, ArchiveFormat::Zip, &destination, 10, 1_000).unwrap();

    assert_eq!(mode(&destination.join("executable")), 0o755);
    assert_eq!(mode(&destination.join("ordinary")), 0o644);
}

#[cfg(unix)]
#[test]
fn zip_normalizes_created_directories_without_widening_existing_root() {
    let temp = tempdir().unwrap();
    let archive = temp.path().join("directories.zip");
    let destination = temp.path().join("extracted");
    fs::create_dir(&destination).unwrap();
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(&destination, fs::Permissions::from_mode(0o700)).unwrap();
    fs::create_dir(destination.join("existing")).unwrap();
    fs::set_permissions(
        destination.join("existing"),
        fs::Permissions::from_mode(0o700),
    )
    .unwrap();

    let file = File::create(&archive).unwrap();
    let mut zip = zip::ZipWriter::new(file);
    zip.add_directory(
        "nested/",
        zip::write::SimpleFileOptions::default().unix_permissions(0o777),
    )
    .unwrap();
    zip.add_directory(
        "existing/",
        zip::write::SimpleFileOptions::default().unix_permissions(0o777),
    )
    .unwrap();
    zip.start_file(
        "implicit/payload",
        zip::write::SimpleFileOptions::default().unix_permissions(0o644),
    )
    .unwrap();
    zip.write_all(b"inside").unwrap();
    zip.start_file(
        "existing/payload",
        zip::write::SimpleFileOptions::default().unix_permissions(0o644),
    )
    .unwrap();
    zip.write_all(b"keep existing directory mode").unwrap();
    zip.finish().unwrap();

    extract_archive(&archive, ArchiveFormat::Zip, &destination, 10, 1_000).unwrap();

    assert_eq!(mode(&destination), 0o700);
    assert_eq!(mode(&destination.join("nested")), 0o755);
    assert_eq!(mode(&destination.join("implicit")), 0o755);
    assert_eq!(mode(&destination.join("existing")), 0o700);
}

#[cfg(unix)]
#[test]
fn zip_rejects_chained_symlink_escape_without_touching_outside_target() {
    let temp = tempdir().unwrap();
    let archive = temp.path().join("escape.zip");
    let destination = temp.path().join("extracted");
    let outside = temp.path().join("outside");
    fs::write(&outside, b"outside contents").unwrap();
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(&outside, fs::Permissions::from_mode(0o640)).unwrap();

    let file = File::create(&archive).unwrap();
    let mut zip = zip::ZipWriter::new(file);
    for (path, target) in [("d", "."), ("bad", "d/../outside")] {
        zip.add_symlink(
            path,
            target,
            zip::write::SimpleFileOptions::default().unix_permissions(0o777),
        )
        .unwrap();
    }
    zip.finish().unwrap();

    let error = extract_archive(&archive, ArchiveFormat::Zip, &destination, 10, 1_000)
        .expect_err("a symlink chain that climbs above the root must be rejected");
    assert!(
        matches!(error, ExtractionError::LinkEscapes(_)),
        "{error:?}"
    );
    assert_eq!(fs::read(&outside).unwrap(), b"outside contents");
    assert_eq!(mode(&outside), 0o640);
}

#[cfg(unix)]
#[test]
fn zip_rejects_symlink_cycles() {
    let temp = tempdir().unwrap();
    let archive = temp.path().join("cycle.zip");
    let destination = temp.path().join("extracted");
    let file = File::create(&archive).unwrap();
    let mut zip = zip::ZipWriter::new(file);
    for (path, target) in [("first", "second"), ("second", "first")] {
        zip.add_symlink(
            path,
            target,
            zip::write::SimpleFileOptions::default().unix_permissions(0o777),
        )
        .unwrap();
    }
    zip.finish().unwrap();

    let error = extract_archive(&archive, ArchiveFormat::Zip, &destination, 10, 1_000)
        .expect_err("symlink cycles must be rejected");
    assert!(
        matches!(error, ExtractionError::LinkEscapes(_)),
        "{error:?}"
    );
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[tokio::test]
#[ignore = "downloads and installs the pinned production Python archive"]
async fn live_python_install_runs_and_has_normalized_archive_permissions() {
    use bergamot_core::managed_tools::install_tool;
    use tokio_util::sync::CancellationToken;

    let manifest = ManagedToolsManifest::load_embedded().unwrap();
    let asset = manifest.get_tool_asset("python3").unwrap();
    assert_eq!(
        asset.sha256,
        "c2f87d4b4e8db3abddb0f116c6ccfc503d0cbdbb9ee8faacddad1b679ec12e99"
    );
    let tools_dir = tempdir().unwrap();
    let installed = install_tool(
        &manifest,
        "python3",
        tools_dir.path(),
        |_, _| {},
        &CancellationToken::new(),
    )
    .await
    .expect("production Python install should finish and verify");

    assert_python_runs(&installed.binary_path);

    let version_root = installed
        .binary_path
        .parent()
        .and_then(Path::parent)
        .and_then(Path::parent)
        .expect("Python install layout should include bin/python");
    assert_normalized_tree(version_root);
    let python3 = version_root.join("python/bin/python3");
    assert!(fs::symlink_metadata(&python3)
        .unwrap()
        .file_type()
        .is_symlink());
    assert_eq!(mode(&python3), 0o777);
    use std::os::unix::fs::PermissionsExt;
    assert_eq!(
        fs::metadata(&python3).unwrap().permissions().mode() & 0o777,
        0o755
    );
}

#[cfg(target_os = "linux")]
#[test]
#[ignore = "requires BERGAMOT_PYTHON_ARCHIVE to point at the pinned production archive"]
fn cached_python_archive_extracts_and_runs_with_normalized_permissions() {
    use bergamot_core::managed_tools::verify_tool;
    use sha2::Digest;

    let archive = std::env::var_os("BERGAMOT_PYTHON_ARCHIVE").expect("archive path env var");
    let archive = PathBuf::from(archive);
    let manifest = ManagedToolsManifest::load_embedded().unwrap();
    let python = manifest.tools.get("python3").unwrap();
    let asset = manifest.get_tool_asset("python3").unwrap();
    let digest = sha2::Sha256::digest(fs::read(&archive).unwrap());
    assert_eq!(format!("{digest:x}"), asset.sha256);

    let extracted = tempdir().unwrap();
    extract_archive(
        &archive,
        asset.archive_format,
        extracted.path(),
        bergamot_core::managed_tools::extractor::DEFAULT_MAX_FILES,
        bergamot_core::managed_tools::extractor::DEFAULT_MAX_DECOMPRESSED_BYTES,
    )
    .unwrap();
    let binary = extracted.path().join(&asset.binary_path);
    verify_tool("python3", &binary, &python.verification).unwrap();
    assert_python_runs(&binary);
    assert_normalized_tree(extracted.path());

    let python3 = extracted.path().join("python/bin/python3");
    use std::os::unix::fs::PermissionsExt;
    assert!(fs::symlink_metadata(&python3)
        .unwrap()
        .file_type()
        .is_symlink());
    assert_eq!(mode(&python3), 0o777);
    assert_eq!(
        fs::metadata(&python3).unwrap().permissions().mode() & 0o777,
        0o755
    );
}
