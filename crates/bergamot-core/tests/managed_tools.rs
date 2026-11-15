use bergamot_core::descriptor::ExecutorConfig;
use bergamot_core::executors::process::{resolve_dependency_detailed, ToolSource};
use bergamot_core::managed_tools::download::is_allowed_host;
use bergamot_core::managed_tools::extractor::{
    extract_archive as extract_archive_async, ExtractionError,
};
use bergamot_core::managed_tools::manifest::{ArchiveFormat, ManagedToolsManifest};
use bergamot_core::{AppSettings, CancellationToken};
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use tempfile::tempdir;

async fn extract_archive(
    archive_path: &Path,
    format: ArchiveFormat,
    destination: &Path,
    max_files: usize,
    max_decompressed_bytes: u64,
) -> Result<(), ExtractionError> {
    extract_archive_async(
        archive_path,
        format,
        destination,
        max_files,
        max_decompressed_bytes,
        &CancellationToken::new(),
    )
    .await
}

fn shell_invocation(script: String) -> (std::path::PathBuf, Vec<String>) {
    #[cfg(unix)]
    {
        (
            Path::new("/bin/sh").to_path_buf(),
            vec!["-c".into(), script],
        )
    }
    #[cfg(windows)]
    {
        let shell = std::env::var_os("COMSPEC")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| Path::new("cmd.exe").to_path_buf());
        (shell, vec!["/C".into(), script])
    }
}

fn shell_output(output: &str) -> (std::path::PathBuf, Vec<String>) {
    let lines: Vec<_> = output.lines().collect();
    #[cfg(unix)]
    let script = format!(
        "printf '%s\\n' {}",
        lines
            .iter()
            .map(|line| format!("'{}'", line))
            .collect::<Vec<_>>()
            .join(" ")
    );
    #[cfg(windows)]
    let script = lines
        .iter()
        .map(|line| format!("echo {line}"))
        .collect::<Vec<_>>()
        .join(" & ");
    shell_invocation(script)
}

fn shell_exit(code: u8) -> Vec<String> {
    #[cfg(unix)]
    let script = format!("exit {code}");
    #[cfg(windows)]
    let script = format!("exit /B {code}");
    shell_invocation(script).1
}

#[tokio::test]
async fn embedded_manifest_loads_and_has_ffmpeg() {
    let manifest = ManagedToolsManifest::load_embedded().expect("Embedded manifest must parse");
    assert_eq!(manifest.schema_version, 1);
    let ffmpeg = manifest
        .tools
        .get("ffmpeg")
        .expect("ffmpeg must be defined");
    assert_eq!(ffmpeg.name, "FFmpeg");
    assert!(ffmpeg.license.is_copyleft);
    assert_eq!(ffmpeg.license.name, "GPL-3.0");

    let current_key = ManagedToolsManifest::current_platform_key();
    assert!(
        current_key.is_some(),
        "Current platform should be supported for ffmpeg"
    );
    let asset = manifest
        .get_tool_asset("ffmpeg")
        .expect("Asset should exist for current platform");
    assert!(!asset.url.is_empty());
    assert_eq!(asset.sha256.len(), 64);
}

#[tokio::test]
async fn download_allowed_hosts_allowlist() {
    assert!(is_allowed_host("github.com"));
    assert!(is_allowed_host("release-assets.githubusercontent.com"));
    assert!(is_allowed_host("objects.githubusercontent.com"));
    assert!(is_allowed_host("johnvansickle.com"));
    assert!(is_allowed_host("www.johnvansickle.com"));
    assert!(is_allowed_host("ffmpeg.martin-riedl.de"));
    assert!(is_allowed_host("evermeet.cx"));
    assert!(is_allowed_host("download.imagemagick.org"));
    assert!(!is_allowed_host("127.0.0.1"));
    assert!(!is_allowed_host("localhost"));

    assert!(!is_allowed_host("evil.com"));
    assert!(!is_allowed_host("raw.githubusercontent.com"));
    assert!(!is_allowed_host("malicious-site.org"));
}

#[tokio::test]
async fn zip_slip_path_traversal_is_blocked() {
    let temp = tempdir().unwrap();
    let zip_path = temp.path().join("slip.zip");
    let extract_dir = temp.path().join("extracted");

    // Create a zip with path traversal entry
    {
        let file = File::create(&zip_path).unwrap();
        let mut zip = zip::ZipWriter::new(file);
        let options = zip::write::SimpleFileOptions::default();
        zip.start_file("../evil.txt", options).unwrap();
        zip.write_all(b"malicious").unwrap();
        zip.finish().unwrap();
    }

    let err = extract_archive(&zip_path, ArchiveFormat::Zip, &extract_dir, 100, 1_000_000)
        .await
        .expect_err("Should block path traversal in zip");

    match err {
        ExtractionError::PathTraversal(_) => {}
        other => panic!("Expected PathTraversal error, got {:?}", other),
    }

    assert!(!temp.path().join("evil.txt").exists());
}

#[cfg(unix)]
#[tokio::test]
async fn zip_escaping_symlink_entry_is_rejected() {
    let temp = tempdir().unwrap();
    let archive_path = temp.path().join("symlink.zip");
    let destination = temp.path().join("extracted");
    let file = File::create(&archive_path).unwrap();
    let mut zip = zip::ZipWriter::new(file);
    zip.add_symlink(
        "escape",
        "../../outside",
        zip::write::SimpleFileOptions::default(),
    )
    .unwrap();
    zip.finish().unwrap();

    let error = extract_archive(&archive_path, ArchiveFormat::Zip, &destination, 10, 100)
        .await
        .expect_err("zip symlink entry must be rejected");
    assert!(
        matches!(error, ExtractionError::LinkEscapes(_)),
        "{error:?}"
    );
}

#[cfg(unix)]
#[tokio::test]
async fn zip_existing_symlink_parent_cannot_escape_destination() {
    use std::os::unix::fs::symlink;
    let temp = tempdir().unwrap();
    let archive_path = temp.path().join("write.zip");
    let destination = temp.path().join("extracted");
    let outside = temp.path().join("outside");
    fs::create_dir(&destination).unwrap();
    fs::create_dir(&outside).unwrap();
    symlink(&outside, destination.join("escape")).unwrap();
    let file = File::create(&archive_path).unwrap();
    let mut zip = zip::ZipWriter::new(file);
    zip.start_file("escape/payload", zip::write::SimpleFileOptions::default())
        .unwrap();
    zip.write_all(b"must not escape").unwrap();
    zip.finish().unwrap();

    assert!(
        extract_archive(&archive_path, ArchiveFormat::Zip, &destination, 10, 100)
            .await
            .is_err()
    );
    assert!(!outside.join("payload").exists());
}

#[cfg(unix)]
#[tokio::test]
async fn zip_existing_hardlink_target_is_not_overwritten() {
    let temp = tempdir().unwrap();
    let archive_path = temp.path().join("hardlink.zip");
    let destination = temp.path().join("extracted");
    fs::create_dir(&destination).unwrap();
    let outside = temp.path().join("outside.txt");
    fs::write(&outside, b"preserve outside file").unwrap();
    fs::hard_link(&outside, destination.join("payload.txt")).unwrap();

    let file = File::create(&archive_path).unwrap();
    let mut zip = zip::ZipWriter::new(file);
    zip.start_file("payload.txt", zip::write::SimpleFileOptions::default())
        .unwrap();
    zip.write_all(b"archive overwrite").unwrap();
    zip.finish().unwrap();

    assert!(
        extract_archive(&archive_path, ArchiveFormat::Zip, &destination, 10, 100)
            .await
            .is_err()
    );
    assert_eq!(fs::read(outside).unwrap(), b"preserve outside file");
}

#[tokio::test]
async fn tar_path_traversal_is_blocked() {
    let temp = tempdir().unwrap();
    let tar_gz_path = temp.path().join("slip.tar.gz");
    let extract_dir = temp.path().join("extracted");

    {
        let file = File::create(&tar_gz_path).unwrap();
        let mut enc = flate2::write::GzEncoder::new(file, flate2::Compression::default());
        let mut header = [0u8; 512];
        let name = b"../../evil.txt";
        header[..name.len()].copy_from_slice(name);
        header[100..108].copy_from_slice(b"0000644\0");
        header[124..136].copy_from_slice(b"00000000011\0");
        header[156] = b'0';
        header[257..263].copy_from_slice(b"ustar\0");
        header[263..265].copy_from_slice(b"00");
        header[148..156].copy_from_slice(b"        ");
        let sum: u32 = header.iter().map(|&b| b as u32).sum();
        let cksum = format!("{:06o}\0 ", sum);
        header[148..156].copy_from_slice(cksum.as_bytes());

        enc.write_all(&header).unwrap();
        enc.write_all(b"malicious").unwrap();
        enc.write_all(&[0u8; 503]).unwrap();
        enc.write_all(&[0u8; 1024]).unwrap();
        enc.finish().unwrap();
    }

    let err = extract_archive(
        &tar_gz_path,
        ArchiveFormat::TarGz,
        &extract_dir,
        100,
        1_000_000,
    )
    .await
    .expect_err("Should block path traversal in tar.gz");

    match err {
        ExtractionError::PathTraversal(_) => {}
        other => panic!("Expected PathTraversal error, got {:?}", other),
    }

    assert!(!temp.path().join("evil.txt").exists());
}

#[tokio::test]
async fn tar_escaping_symlink_is_blocked() {
    let temp = tempdir().unwrap();
    let tar_gz_path = temp.path().join("symlink_escape.tar.gz");
    let extract_dir = temp.path().join("extracted");

    {
        let file = File::create(&tar_gz_path).unwrap();
        let enc = flate2::write::GzEncoder::new(file, flate2::Compression::default());
        let mut tar = tar::Builder::new(enc);

        let mut header = tar::Header::new_gnu();
        header.set_entry_type(tar::EntryType::Symlink);
        header.set_size(0);
        header.set_cksum();
        tar.append_link(&mut header, "evil_link", "/etc/passwd")
            .unwrap();
        tar.finish().unwrap();
    }

    let err = extract_archive(
        &tar_gz_path,
        ArchiveFormat::TarGz,
        &extract_dir,
        100,
        1_000_000,
    )
    .await
    .expect_err("Should block escaping symlink in tar");

    match err {
        ExtractionError::LinkEscapes(_) => {}
        other => panic!("Expected LinkEscapes error, got {:?}", other),
    }
}

#[tokio::test]
async fn tar_escaping_hardlink_is_blocked() {
    let temp = tempdir().unwrap();
    let archive_path = temp.path().join("hardlink_escape.tar.gz");
    let destination = temp.path().join("extracted");
    let file = File::create(&archive_path).unwrap();
    let encoder = flate2::write::GzEncoder::new(file, flate2::Compression::default());
    let mut tar = tar::Builder::new(encoder);
    let mut header = tar::Header::new_gnu();
    header.set_entry_type(tar::EntryType::Link);
    header.set_size(0);
    header.set_cksum();
    tar.append_link(&mut header, "nested/evil", "../../outside")
        .unwrap();
    tar.finish().unwrap();
    tar.into_inner().unwrap().finish().unwrap();

    let error = extract_archive(
        &archive_path,
        ArchiveFormat::TarGz,
        &destination,
        100,
        1_000_000,
    )
    .await
    .expect_err("escaping hardlink must be rejected");
    assert!(
        matches!(error, ExtractionError::LinkEscapes(_)),
        "{error:?}"
    );
    assert!(!temp.path().join("outside").exists());
}

#[tokio::test]
async fn tar_symlink_parent_cannot_redirect_later_file_outside_destination() {
    let temp = tempdir().unwrap();
    let archive_path = temp.path().join("symlink_chain.tar.gz");
    let destination = temp.path().join("extracted");
    let outside = temp.path().join("outside");
    fs::create_dir(&outside).unwrap();
    let file = File::create(&archive_path).unwrap();
    let encoder = flate2::write::GzEncoder::new(file, flate2::Compression::default());
    let mut tar = tar::Builder::new(encoder);
    let mut link = tar::Header::new_gnu();
    link.set_entry_type(tar::EntryType::Symlink);
    link.set_size(0);
    link.set_cksum();
    tar.append_link(&mut link, "pivot", "../outside").unwrap();
    let data = b"must stay inside";
    let mut payload = tar::Header::new_gnu();
    payload.set_size(data.len() as u64);
    payload.set_cksum();
    tar.append_data(&mut payload, "pivot/payload", &data[..])
        .unwrap();
    tar.finish().unwrap();
    tar.into_inner().unwrap().finish().unwrap();

    let error = extract_archive(&archive_path, ArchiveFormat::TarGz, &destination, 10, 1_000)
        .await
        .expect_err("archive link chain must not escape destination");
    assert!(
        matches!(
            error,
            ExtractionError::PathTraversal(_) | ExtractionError::LinkEscapes(_)
        ),
        "{error:?}"
    );
    assert!(!outside.join("payload").exists());
}

#[tokio::test]
async fn extraction_limits_are_enforced() {
    let temp = tempdir().unwrap();
    let zip_path = temp.path().join("bomb.zip");
    let extract_dir = temp.path().join("extracted");

    {
        let file = File::create(&zip_path).unwrap();
        let mut zip = zip::ZipWriter::new(file);
        let options = zip::write::SimpleFileOptions::default();
        zip.start_file("large.txt", options).unwrap();
        zip.write_all(&vec![0u8; 100_000]).unwrap();
        zip.finish().unwrap();
    }

    // Test byte limit exceeded
    let err = extract_archive(
        &zip_path,
        ArchiveFormat::Zip,
        &extract_dir,
        10,
        1_000, // Limit to 1 KB
    )
    .await
    .expect_err("Should enforce max bytes");

    match err {
        ExtractionError::TooLarge(1_000) => {}
        other => panic!("Expected TooLarge, got {:?}", other),
    }
}

#[tokio::test]
async fn extraction_file_count_limit_is_enforced() {
    let temp = tempdir().unwrap();
    let archive_path = temp.path().join("many.zip");
    let destination = temp.path().join("extracted");
    let file = File::create(&archive_path).unwrap();
    let mut zip = zip::ZipWriter::new(file);
    for name in ["one", "two"] {
        zip.start_file(name, zip::write::SimpleFileOptions::default())
            .unwrap();
        zip.write_all(b"x").unwrap();
    }
    zip.finish().unwrap();

    let error = extract_archive(&archive_path, ArchiveFormat::Zip, &destination, 1, 100)
        .await
        .expect_err("file count limit must be enforced");
    assert!(
        matches!(error, ExtractionError::TooManyFiles(1)),
        "{error:?}"
    );
}

#[tokio::test]
async fn tar_file_count_limit_is_enforced() {
    let temp = tempdir().unwrap();
    let archive_path = temp.path().join("many.tar.gz");
    let destination = temp.path().join("extracted");
    let file = File::create(&archive_path).unwrap();
    let encoder = flate2::write::GzEncoder::new(file, flate2::Compression::default());
    let mut tar = tar::Builder::new(encoder);
    for name in ["one", "two"] {
        let data = b"x";
        let mut header = tar::Header::new_gnu();
        header.set_size(data.len() as u64);
        header.set_cksum();
        tar.append_data(&mut header, name, &data[..]).unwrap();
    }
    tar.finish().unwrap();
    tar.into_inner().unwrap().finish().unwrap();

    let error = extract_archive(&archive_path, ArchiveFormat::TarGz, &destination, 1, 100)
        .await
        .expect_err("tar file count limit must be enforced");
    assert!(
        matches!(error, ExtractionError::TooManyFiles(1)),
        "{error:?}"
    );
}

#[tokio::test]
async fn tar_decompressed_size_limit_is_enforced() {
    let temp = tempdir().unwrap();
    let archive_path = temp.path().join("large.tar.gz");
    let destination = temp.path().join("extracted");
    let file = File::create(&archive_path).unwrap();
    let encoder = flate2::write::GzEncoder::new(file, flate2::Compression::default());
    let mut tar = tar::Builder::new(encoder);
    let data = vec![b'x'; 2_048];
    let mut header = tar::Header::new_gnu();
    header.set_size(data.len() as u64);
    header.set_cksum();
    tar.append_data(&mut header, "large.bin", &data[..])
        .unwrap();
    tar.finish().unwrap();
    tar.into_inner().unwrap().finish().unwrap();

    let error = extract_archive(&archive_path, ArchiveFormat::TarGz, &destination, 10, 1_000)
        .await
        .expect_err("tar byte limit must be enforced");
    assert!(
        matches!(error, ExtractionError::TooLarge(1_000)),
        "{error:?}"
    );
}

#[tokio::test]
#[ignore = "manual real-archive benchmark; set BERGAMOT_FFMPEG_ARCHIVE"]
async fn real_ffmpeg_archive_extraction_benchmark() {
    let archive = std::env::var("BERGAMOT_FFMPEG_ARCHIVE").expect("archive path env var");
    let destination = tempdir().unwrap();
    extract_archive(
        std::path::Path::new(&archive),
        ArchiveFormat::TarXz,
        destination.path(),
        50_000,
        1_000_000_000,
    )
    .await
    .expect("real FFmpeg archive extraction");
}

#[tokio::test]
async fn ffmpeg_configuration_parsing_and_license_detection() {
    let valid_gpl3 = r#"ffmpeg version 7.1 Copyright (c) 2000-2024 the FFmpeg developers
built with gcc 14.2.0 (Rev1, Built by MSYS2 project)
configuration: --enable-gpl --enable-version3 --enable-libx264 --enable-libmp3lame
libavutil      59. 39.100 / 59. 39.100"#;

    let valid_gpl2 = r#"ffmpeg version 7.0 Copyright (c) 2000-2024 the FFmpeg developers
configuration: --enable-gpl --enable-libx264 --enable-libmp3lame
libavutil      59. 39.100 / 59. 39.100"#;

    let nonfree = r#"ffmpeg version 7.1 Copyright (c) 2000-2024 the FFmpeg developers
configuration: --enable-gpl --enable-version3 --enable-nonfree
libavutil      59. 39.100 / 59. 39.100"#;

    let no_gpl = r#"ffmpeg version 7.1 Copyright (c) 2000-2024 the FFmpeg developers
configuration: --enable-libx264
libavutil      59. 39.100 / 59. 39.100"#;

    // Test parse via verifier internal or helper
    use bergamot_core::managed_tools::verifier::VerificationError;

    let manifest = ManagedToolsManifest::load_embedded().unwrap();
    let ffmpeg_def = manifest.tools.get("ffmpeg").unwrap();
    let (fake_ffmpeg, version_command) = shell_output(valid_gpl3);
    let mut verification = ffmpeg_def.verification.clone();
    verification.command = version_command;
    verification.encoders_command = Some(shell_output("libx264\nlibmp3lame").1);

    let meta = bergamot_core::managed_tools::verify_tool(
        "ffmpeg",
        &fake_ffmpeg,
        &verification,
        &CancellationToken::new(),
    )
    .await
    .expect("Should verify valid GPLv3 build");
    assert_eq!(meta.detected_license, "GPL-3.0");

    verification.command = shell_output(valid_gpl2).1;
    let meta_gpl2 = bergamot_core::managed_tools::verify_tool(
        "ffmpeg",
        &fake_ffmpeg,
        &verification,
        &CancellationToken::new(),
    )
    .await
    .expect("Should verify valid GPLv2 build");
    assert_eq!(meta_gpl2.detected_license, "GPL-2.0");

    // Test nonfree rejection
    verification.command = shell_output(nonfree).1;
    let err = bergamot_core::managed_tools::verify_tool(
        "ffmpeg",
        &fake_ffmpeg,
        &verification,
        &CancellationToken::new(),
    )
    .await
    .expect_err("Should reject nonfree build");
    match err {
        VerificationError::InvalidLicense(msg) => assert!(msg.contains("nonfree")),
        other => panic!("Expected InvalidLicense for nonfree, got {:?}", other),
    }

    // Test missing gpl rejection
    verification.command = shell_output(no_gpl).1;
    let err = bergamot_core::managed_tools::verify_tool(
        "ffmpeg",
        &fake_ffmpeg,
        &verification,
        &CancellationToken::new(),
    )
    .await
    .expect_err("Should reject missing gpl build");
    match err {
        VerificationError::InvalidLicense(msg) => assert!(msg.contains("--enable-gpl")),
        other => panic!("Expected InvalidLicense for missing gpl, got {:?}", other),
    }
}

#[tokio::test]
async fn resolver_precedence_and_managed_tools_fallback() {
    let temp = tempdir().unwrap();
    let managed_tools_dir = temp.path().join("managed_tools");
    let custom_tools_dir = temp.path().join("custom_tools");
    fs::create_dir_all(&managed_tools_dir).unwrap();
    fs::create_dir_all(&custom_tools_dir).unwrap();

    let manifest = ManagedToolsManifest::load_embedded().unwrap();
    let asset = manifest.get_tool_asset("ffmpeg").unwrap();

    // Fake installed managed ffmpeg binary
    let managed_binary = managed_tools_dir
        .join("ffmpeg")
        .join(&asset.version)
        .join(&asset.binary_path);
    fs::create_dir_all(managed_binary.parent().unwrap()).unwrap();
    fs::write(&managed_binary, "#!/bin/sh\nexit 0").unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&managed_binary, fs::Permissions::from_mode(0o755)).unwrap();
    }

    let empty_path_dir = temp.path().join("empty_path");
    fs::create_dir_all(&empty_path_dir).unwrap();

    let mut env = BTreeMap::new();
    env.insert(
        "PATH".to_string(),
        empty_path_dir.to_string_lossy().into_owned(),
    );

    let executor = ExecutorConfig::Process {
        program: "ffmpeg".to_string(),
        args: vec![],
        working_directory: None,
        env,
        timeout_seconds: 30,
    };

    // 1. With managed tools directory configured and no overrides, resolves to managed binary
    let mut settings = AppSettings {
        managed_tools_dir: Some(managed_tools_dir.clone()),
        ..AppSettings::default()
    };

    let resolved = resolve_dependency_detailed(&executor, &settings, &CancellationToken::new())
        .await
        .unwrap()
        .expect("Should resolve managed tool");
    assert_eq!(resolved.path, managed_binary);
    assert_eq!(resolved.source, ToolSource::Managed);

    // 2. Custom paths take precedence over managed tools
    let custom_binary = custom_tools_dir.join(format!("ffmpeg{}", std::env::consts::EXE_SUFFIX));
    fs::write(&custom_binary, "#!/bin/sh\nexit 0").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&custom_binary, fs::Permissions::from_mode(0o755)).unwrap();
    }

    settings.custom_paths = vec![custom_tools_dir.to_string_lossy().into_owned()];
    let resolved_custom =
        resolve_dependency_detailed(&executor, &settings, &CancellationToken::new())
            .await
            .unwrap()
            .expect("Should resolve custom path");
    assert_eq!(resolved_custom.path, custom_binary);
    assert_eq!(resolved_custom.source, ToolSource::System);

    // 3. Tool override takes precedence over custom paths
    let override_binary = temp
        .path()
        .join(format!("ffmpeg-override{}", std::env::consts::EXE_SUFFIX));
    fs::write(&override_binary, "#!/bin/sh\nexit 0").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&override_binary, fs::Permissions::from_mode(0o755)).unwrap();
    }

    settings.tool_overrides.insert(
        "ffmpeg".to_string(),
        override_binary.to_string_lossy().into_owned(),
    );
    let resolved_override =
        resolve_dependency_detailed(&executor, &settings, &CancellationToken::new())
            .await
            .unwrap()
            .expect("Should resolve override");
    assert_eq!(resolved_override.path, override_binary);
    assert_eq!(resolved_override.source, ToolSource::System);
}

#[tokio::test]
async fn python3_manifest_and_verification() {
    let manifest = ManagedToolsManifest::load_embedded().expect("Manifest should parse");
    let python = manifest
        .tools
        .get("python3")
        .expect("python3 should be defined");
    assert_eq!(python.name, "Python 3");
    assert!(!python.license.is_copyleft);
    assert!(python.license.name.contains("PSF-2.0"));

    let asset = manifest
        .get_tool_asset("python3")
        .expect("Python asset for current platform");
    assert_eq!(asset.version, "3.12.16");
    assert_eq!(asset.sha256.len(), 64);

    let (shell, version_command) = shell_output("Python 3.12.16");
    let mut verification = python.verification.clone();
    verification.command = version_command;
    verification.test_script_args = Some(shell_output("OK").1);

    let meta = bergamot_core::managed_tools::verify_tool(
        "python3",
        &shell,
        &verification,
        &CancellationToken::new(),
    )
    .await
    .expect("Should verify valid python3");
    assert!(meta.detected_license.contains("PSF-2.0"));
    assert!(meta.version_output.contains("Python 3.12.16"));

    verification.test_script_args = Some(shell_exit(1));
    let err = bergamot_core::managed_tools::verify_tool(
        "python3",
        &shell,
        &verification,
        &CancellationToken::new(),
    )
    .await
    .expect_err("Should fail when stdlib test script fails");
    match err {
        bergamot_core::managed_tools::VerificationError::PythonScriptFailed(_) => {}
        other => panic!("Expected PythonScriptFailed, got {:?}", other),
    }
}

#[tokio::test]
async fn python3_managed_tool_resolution_and_stub_rejection() {
    let temp = tempdir().unwrap();
    let managed_tools_dir = temp.path().join("managed_tools");
    let custom_tools_dir = temp.path().join("custom_tools");
    fs::create_dir_all(&managed_tools_dir).unwrap();
    fs::create_dir_all(&custom_tools_dir).unwrap();

    let manifest = ManagedToolsManifest::load_embedded().unwrap();
    let asset = manifest.get_tool_asset("python3").unwrap();

    let managed_binary = managed_tools_dir
        .join("python3")
        .join(&asset.version)
        .join(&asset.binary_path);
    fs::create_dir_all(managed_binary.parent().unwrap()).unwrap();
    fs::write(&managed_binary, "#!/bin/sh\nexit 0\n").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&managed_binary, fs::Permissions::from_mode(0o755)).unwrap();
    }

    let mut env = BTreeMap::new();
    let empty_path = temp.path().join("empty_path");
    fs::create_dir_all(&empty_path).unwrap();
    env.insert(
        "PATH".to_string(),
        empty_path.to_string_lossy().into_owned(),
    );

    let executor = ExecutorConfig::Process {
        program: "python3".to_string(),
        args: vec![],
        working_directory: None,
        env,
        timeout_seconds: 30,
    };

    let settings = AppSettings {
        managed_tools_dir: Some(managed_tools_dir),
        ..AppSettings::default()
    };

    let resolved = resolve_dependency_detailed(&executor, &settings, &CancellationToken::new())
        .await
        .unwrap()
        .expect("Should resolve managed python3");
    assert_eq!(resolved.path, managed_binary);
    assert_eq!(resolved.source, ToolSource::Managed);

    let broken_stub = custom_tools_dir.join(format!("python3{}", std::env::consts::EXE_SUFFIX));
    fs::write(&broken_stub, "#!/bin/sh\nexit 1\n").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&broken_stub, fs::Permissions::from_mode(0o755)).unwrap();
    }

    let settings_with_stub = AppSettings {
        custom_paths: vec![custom_tools_dir.to_string_lossy().into_owned()],
        managed_tools_dir: Some(temp.path().join("nonexistent")),
        ..AppSettings::default()
    };

    assert!(
        resolve_dependency_detailed(&executor, &settings_with_stub, &CancellationToken::new(),)
            .await
            .is_err()
    );
}

#[tokio::test]
async fn imagemagick_manifest_and_verification() {
    let manifest = ManagedToolsManifest::load_embedded().expect("Manifest should parse");
    let magick = manifest
        .tools
        .get("magick")
        .expect("magick should be defined");
    assert_eq!(magick.name, "ImageMagick");
    assert!(!magick.license.is_copyleft);
    assert!(magick.license.name.contains("Apache-2.0"));

    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    {
        let asset = manifest.get_tool_asset("magick").expect("Linux x64 asset");
        assert_eq!(asset.version, "7.1.2-35");
        assert_eq!(asset.binary_path, "squashfs-root/AppRun");
        assert_eq!(asset.archive_format, ArchiveFormat::AppImage);
    }

    #[cfg(target_os = "macos")]
    {
        assert!(manifest.get_tool_asset("magick").is_none());
        assert!(manifest
            .get_manual_instruction("magick")
            .unwrap()
            .contains("brew"));
    }

    let (shell, version_command) = shell_output("Version: ImageMagick 7.1.2-35");
    let mut verification = magick.verification.clone();
    verification.command = version_command;
    verification.formats_command = Some(shell_output("WEBP* rw+ WebP Image Format").1);

    let meta = bergamot_core::managed_tools::verify_tool(
        "magick",
        &shell,
        &verification,
        &CancellationToken::new(),
    )
    .await
    .expect("Should verify valid magick with WEBP");
    assert!(meta.detected_license.contains("ImageMagick"));

    verification.formats_command = Some(shell_output("PNG* rw- PNG Format").1);
    let err = bergamot_core::managed_tools::verify_tool(
        "magick",
        &shell,
        &verification,
        &CancellationToken::new(),
    )
    .await
    .expect_err("Should fail when WEBP is missing");
    match err {
        bergamot_core::managed_tools::VerificationError::MissingFormats(missing) => {
            assert_eq!(missing, vec!["WEBP"]);
        }
        other => panic!("Expected MissingFormats, got {:?}", other),
    }
}

#[tokio::test]
#[ignore = "requires network to download real assets for platform verification"]
async fn test_live_download_and_verify_all_platform_tools() {
    let manifest = ManagedToolsManifest::load_embedded().unwrap();
    let platform_key =
        ManagedToolsManifest::current_platform_key().expect("Platform should be recognized");
    let temp = tempfile::tempdir().unwrap();

    for (tool_id, tool_def) in &manifest.tools {
        if let Some(_asset) = tool_def.platforms.get(platform_key) {
            println!("Testing download and verification for {tool_id} on {platform_key}...");
            let cancel = tokio_util::sync::CancellationToken::new();
            let outcome = bergamot_core::managed_tools::install_tool(
                &manifest,
                tool_id,
                temp.path(),
                |dl, total| {
                    if dl > 0 && dl % (5 * 1024 * 1024) == 0 {
                        println!("  progress: {dl} / {total:?}");
                    }
                },
                &cancel,
            )
            .await
            .unwrap_or_else(|e| panic!("Failed to download and verify {tool_id}: {e}"));

            println!("Successfully verified {tool_id}: {:?}", outcome.metadata);
        }
    }
}
