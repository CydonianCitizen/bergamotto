use bergamot_core::descriptor::ExecutorConfig;
use bergamot_core::executors::process::{resolve_dependency_detailed, ToolSource};
use bergamot_core::managed_tools::download::is_allowed_host;
use bergamot_core::managed_tools::extractor::{extract_archive, ExtractionError};
use bergamot_core::managed_tools::manifest::{ArchiveFormat, ManagedToolsManifest};
use bergamot_core::AppSettings;
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::Write;
use tempfile::tempdir;

#[test]
fn embedded_manifest_loads_and_has_ffmpeg() {
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

#[test]
fn download_allowed_hosts_allowlist() {
    assert!(is_allowed_host("github.com"));
    assert!(is_allowed_host("release-assets.githubusercontent.com"));
    assert!(is_allowed_host("objects.githubusercontent.com"));
    assert!(is_allowed_host("johnvansickle.com"));
    assert!(is_allowed_host("www.johnvansickle.com"));
    assert!(is_allowed_host("ffmpeg.martin-riedl.de"));
    assert!(is_allowed_host("evermeet.cx"));
    assert!(is_allowed_host("download.imagemagick.org"));
    assert!(is_allowed_host("127.0.0.1"));
    assert!(is_allowed_host("localhost"));

    assert!(!is_allowed_host("evil.com"));
    assert!(!is_allowed_host("raw.githubusercontent.com"));
    assert!(!is_allowed_host("malicious-site.org"));
}

#[test]
fn zip_slip_path_traversal_is_blocked() {
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
        .expect_err("Should block path traversal in zip");

    match err {
        ExtractionError::PathTraversal(_) => {}
        other => panic!("Expected PathTraversal error, got {:?}", other),
    }

    assert!(!temp.path().join("evil.txt").exists());
}

#[test]
fn tar_path_traversal_is_blocked() {
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
    .expect_err("Should block path traversal in tar.gz");

    match err {
        ExtractionError::PathTraversal(_) => {}
        other => panic!("Expected PathTraversal error, got {:?}", other),
    }

    assert!(!temp.path().join("evil.txt").exists());
}

#[test]
fn tar_escaping_symlink_is_blocked() {
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
    .expect_err("Should block escaping symlink in tar");

    match err {
        ExtractionError::LinkEscapes(_) => {}
        other => panic!("Expected LinkEscapes error, got {:?}", other),
    }
}

#[test]
fn extraction_limits_are_enforced() {
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
    .expect_err("Should enforce max bytes");

    match err {
        ExtractionError::TooLarge(1_000) => {}
        other => panic!("Expected TooLarge, got {:?}", other),
    }
}

#[test]
fn ffmpeg_configuration_parsing_and_license_detection() {
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

    // We can test verifier's parse_ffmpeg_configuration
    let temp = tempdir().unwrap();
    let fake_ffmpeg = temp.path().join("ffmpeg_test");
    // Write fake script that prints valid_gpl3
    fs::write(
        &fake_ffmpeg,
        format!("#!/bin/sh\ncat << 'EOF'\n{}\nEOF\n", valid_gpl3),
    )
    .unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&fake_ffmpeg, fs::Permissions::from_mode(0o755)).unwrap();
    }

    let manifest = ManagedToolsManifest::load_embedded().unwrap();
    let ffmpeg_def = manifest.tools.get("ffmpeg").unwrap();

    let meta =
        bergamot_core::managed_tools::verify_tool("ffmpeg", &fake_ffmpeg, &ffmpeg_def.verification)
            .expect("Should verify valid GPLv3 build");
    assert_eq!(meta.detected_license, "GPL-3.0");

    fs::write(
        &fake_ffmpeg,
        format!("#!/bin/sh\ncat << 'EOF'\n{}\nEOF\n", valid_gpl2),
    )
    .unwrap();
    let meta_gpl2 =
        bergamot_core::managed_tools::verify_tool("ffmpeg", &fake_ffmpeg, &ffmpeg_def.verification)
            .expect("Should verify valid GPLv2 build");
    assert_eq!(meta_gpl2.detected_license, "GPL-2.0");

    // Test nonfree rejection
    fs::write(
        &fake_ffmpeg,
        format!("#!/bin/sh\ncat << 'EOF'\n{}\nEOF\n", nonfree),
    )
    .unwrap();
    let err =
        bergamot_core::managed_tools::verify_tool("ffmpeg", &fake_ffmpeg, &ffmpeg_def.verification)
            .expect_err("Should reject nonfree build");
    match err {
        VerificationError::InvalidLicense(msg) => assert!(msg.contains("nonfree")),
        other => panic!("Expected InvalidLicense for nonfree, got {:?}", other),
    }

    // Test missing gpl rejection
    fs::write(
        &fake_ffmpeg,
        format!("#!/bin/sh\ncat << 'EOF'\n{}\nEOF\n", no_gpl),
    )
    .unwrap();
    let err =
        bergamot_core::managed_tools::verify_tool("ffmpeg", &fake_ffmpeg, &ffmpeg_def.verification)
            .expect_err("Should reject missing gpl build");
    match err {
        VerificationError::InvalidLicense(msg) => assert!(msg.contains("--enable-gpl")),
        other => panic!("Expected InvalidLicense for missing gpl, got {:?}", other),
    }
}

#[test]
fn resolver_precedence_and_managed_tools_fallback() {
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

    let resolved = resolve_dependency_detailed(&executor, &settings)
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
    let resolved_custom = resolve_dependency_detailed(&executor, &settings)
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
    let resolved_override = resolve_dependency_detailed(&executor, &settings)
        .unwrap()
        .expect("Should resolve override");
    assert_eq!(resolved_override.path, override_binary);
    assert_eq!(resolved_override.source, ToolSource::System);
}

#[test]
fn python3_manifest_and_verification() {
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

    let temp = tempdir().unwrap();
    let fake_py = temp.path().join("fake_python");
    fs::write(
        &fake_py,
        "#!/bin/sh\nif [ \"$1\" = \"--version\" ]; then echo 'Python 3.12.16'; exit 0; fi\nif [ \"$1\" = \"-c\" ]; then echo 'OK'; exit 0; fi\nexit 0\n",
    )
    .unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&fake_py, fs::Permissions::from_mode(0o755)).unwrap();
    }

    let meta = bergamot_core::managed_tools::verify_tool("python3", &fake_py, &python.verification)
        .expect("Should verify valid python3");
    assert!(meta.detected_license.contains("PSF-2.0"));
    assert!(meta.version_output.contains("Python 3.12.16"));

    let failing_py = temp.path().join("failing_python");
    fs::write(
        &failing_py,
        "#!/bin/sh\nif [ \"$1\" = \"--version\" ]; then echo 'Python 3.12.16'; exit 0; fi\nexit 1\n",
    )
    .unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&failing_py, fs::Permissions::from_mode(0o755)).unwrap();
    }

    let err =
        bergamot_core::managed_tools::verify_tool("python3", &failing_py, &python.verification)
            .expect_err("Should fail when stdlib test script fails");
    match err {
        bergamot_core::managed_tools::VerificationError::PythonScriptFailed(_) => {}
        other => panic!("Expected PythonScriptFailed, got {:?}", other),
    }
}

#[test]
fn python3_managed_tool_resolution_and_stub_rejection() {
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

    let resolved = resolve_dependency_detailed(&executor, &settings)
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

    assert!(resolve_dependency_detailed(&executor, &settings_with_stub).is_err());
}

#[test]
fn imagemagick_manifest_and_verification() {
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

    let temp = tempdir().unwrap();
    let fake_magick = temp.path().join("fake_magick");
    fs::write(
        &fake_magick,
        "#!/bin/sh\nif [ \"$1\" = \"-version\" ]; then echo 'Version: ImageMagick 7.1.2-35'; exit 0; fi\nif [ \"$1\" = \"-list\" ]; then echo 'WEBP* rw+ WebP Image Format'; exit 0; fi\nexit 0\n",
    )
    .unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&fake_magick, fs::Permissions::from_mode(0o755)).unwrap();
    }

    let meta =
        bergamot_core::managed_tools::verify_tool("magick", &fake_magick, &magick.verification)
            .expect("Should verify valid magick with WEBP");
    assert!(meta.detected_license.contains("ImageMagick"));

    let missing_webp_magick = temp.path().join("missing_webp_magick");
    fs::write(
        &missing_webp_magick,
        "#!/bin/sh\nif [ \"$1\" = \"-version\" ]; then echo 'Version: ImageMagick 7.1.2-35'; exit 0; fi\nif [ \"$1\" = \"-list\" ]; then echo 'PNG* rw- PNG Format'; exit 0; fi\nexit 0\n",
    )
    .unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&missing_webp_magick, fs::Permissions::from_mode(0o755)).unwrap();
    }

    let err = bergamot_core::managed_tools::verify_tool(
        "magick",
        &missing_webp_magick,
        &magick.verification,
    )
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
