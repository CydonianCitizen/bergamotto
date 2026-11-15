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
