use bergamot_core::managed_tools::extractor::{extract_archive, ExtractionError};
use bergamot_core::managed_tools::manifest::ArchiveFormat;
use bergamot_core::CancellationToken;
use sevenz_rust2::{ArchiveEntry, ArchiveWriter};
use std::fs;
use std::io::Cursor;
use std::path::Path;
use tempfile::tempdir;

fn create_7z(path: &Path, entries: &[(&str, &[u8])]) {
    let mut writer = ArchiveWriter::create(path).expect("create 7z fixture");
    for (name, contents) in entries {
        writer
            .push_archive_entry(ArchiveEntry::new_file(name), Some(Cursor::new(*contents)))
            .expect("write 7z entry");
    }
    writer.finish().expect("finish 7z fixture");
}

async fn extract_7z(
    archive: &Path,
    destination: &Path,
    max_files: usize,
    max_bytes: u64,
) -> Result<(), ExtractionError> {
    extract_archive(
        archive,
        ArchiveFormat::SevenZ,
        destination,
        max_files,
        max_bytes,
        &CancellationToken::new(),
    )
    .await
}

#[tokio::test]
async fn sevenz_extracts_valid_nested_layout_once() {
    let temp = tempdir().unwrap();
    let archive = temp.path().join("valid.7z");
    let destination = temp.path().join("extracted");
    create_7z(&archive, &[("nested/payload.txt", b"valid payload")]);

    extract_7z(&archive, &destination, 10, 1_000)
        .await
        .expect("valid 7z archive should extract");

    assert_eq!(
        fs::read(destination.join("nested/payload.txt")).unwrap(),
        b"valid payload"
    );
}

#[tokio::test]
async fn sevenz_rejects_absolute_and_parent_paths_in_both_separator_styles() {
    let temp = tempdir().unwrap();

    for (index, name) in [
        "../escape.txt",
        "/absolute.txt",
        r"..\escape.txt",
        r"C:\escape.txt",
    ]
    .into_iter()
    .enumerate()
    {
        let archive = temp.path().join(format!("unsafe-{index}.7z"));
        let destination = temp.path().join(format!("destination-{index}"));
        create_7z(&archive, &[(name, b"must not escape")]);

        let result = extract_7z(&archive, &destination, 10, 1_000).await;

        assert!(
            result.is_err(),
            "unsafe archive entry {name:?} was accepted"
        );
    }
}

#[cfg(unix)]
#[tokio::test]
async fn sevenz_rejects_preexisting_symlink_parent_escape() {
    use std::os::unix::fs::symlink;

    let temp = tempdir().unwrap();
    let archive = temp.path().join("symlink-parent.7z");
    let destination = temp.path().join("extracted");
    let outside = temp.path().join("outside");
    fs::create_dir(&destination).unwrap();
    fs::create_dir(&outside).unwrap();
    symlink(&outside, destination.join("escape")).unwrap();
    create_7z(&archive, &[("escape/payload.txt", b"must not escape")]);

    let result = extract_7z(&archive, &destination, 10, 1_000).await;

    assert!(result.is_err(), "7z extraction followed a symlink parent");
    assert!(!outside.join("payload.txt").exists());
}

#[tokio::test]
async fn sevenz_does_not_overwrite_a_colliding_output() {
    let temp = tempdir().unwrap();
    let archive = temp.path().join("collision.7z");
    let destination = temp.path().join("extracted");
    let collision = destination.join("payload");
    fs::create_dir_all(&destination).unwrap();
    fs::write(&collision, b"existing contents").unwrap();
    create_7z(&archive, &[("payload", b"archive contents")]);

    let result = extract_7z(&archive, &destination, 10, 1_000).await;

    assert!(result.is_err(), "colliding output path was accepted");
    assert_eq!(fs::read(collision).unwrap(), b"existing contents");
}

#[tokio::test]
async fn sevenz_enforces_file_count_and_expanded_byte_limits() {
    let temp = tempdir().unwrap();
    let archive = temp.path().join("limits.7z");
    let destination = temp.path().join("extracted");
    create_7z(&archive, &[("one.txt", b"1234"), ("two.txt", b"5678")]);

    assert!(extract_7z(&archive, &destination, 1, 1_000).await.is_err());

    let destination = temp.path().join("size-limited");
    assert!(extract_7z(&archive, &destination, 10, 4).await.is_err());
}
