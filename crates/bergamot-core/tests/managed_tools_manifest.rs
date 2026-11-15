use bergamot_core::managed_tools::ManagedToolsManifest;
use serde_json::Value;

fn managed_tools_schema_validator() -> jsonschema::Validator {
    let schema: Value = serde_json::from_str(include_str!(
        "../../../shared/schemas/managed-tools.schema.json"
    ))
    .unwrap();
    jsonschema::options()
        .should_validate_formats(true)
        .build(&schema)
        .unwrap()
}

#[test]
fn every_manifest_asset_records_hash_provenance() {
    let validator = managed_tools_schema_validator();
    let manifest = ManagedToolsManifest::load_embedded().expect("embedded manifest parses");
    assert!(!manifest.tools.is_empty());
    let source: Value = serde_json::from_str(ManagedToolsManifest::embedded_str()).unwrap();
    assert!(validator.is_valid(&source));

    for (tool_id, tool) in &manifest.tools {
        for (platform, asset) in &tool.platforms {
            assert_eq!(asset.sha256.len(), 64, "{tool_id}/{platform}");
            let date = asset.hash_source.rsplit_once(' ').map(|(_, date)| date);
            assert!(
                date.is_some_and(|date| {
                    let mut parts = date.split('-');
                    matches!((parts.next(), parts.next(), parts.next()), (Some(y), Some(m), Some(d))
                        if y.len() == 4 && m.len() == 2 && d.len() == 2
                        && y.bytes().chain(m.bytes()).chain(d.bytes()).all(|c| c.is_ascii_digit()))
                }),
                "{tool_id}/{platform} must include a YYYY-MM-DD date: {}",
                asset.hash_source
            );
        }
    }
}

#[test]
fn older_ffmpeg_pins_explain_version_selection() {
    let manifest = ManagedToolsManifest::load_embedded().unwrap();
    for (platform, asset) in &manifest.tools["ffmpeg"].platforms {
        if asset.version != "9.0.2" {
            assert!(
                asset
                    .version_reason
                    .as_deref()
                    .is_some_and(|reason| !reason.is_empty()),
                "ffmpeg/{platform} older pin needs a manifest versionReason"
            );
        }
    }
}

#[test]
fn manifest_rejects_asset_without_hash_provenance() {
    let validator = managed_tools_schema_validator();
    let mut manifest: serde_json::Value =
        serde_json::from_str(ManagedToolsManifest::embedded_str()).unwrap();
    manifest["tools"]["ffmpeg"]["platforms"]["windows-x64"]
        .as_object_mut()
        .unwrap()
        .remove("hashSource");
    assert!(!validator.is_valid(&manifest));
    assert!(ManagedToolsManifest::parse(&manifest.to_string()).is_err());
}

#[test]
fn manifest_schema_rejects_empty_or_undated_hash_provenance() {
    let validator = managed_tools_schema_validator();
    for invalid in ["", "SHA-256 calculated locally", "MD5 verified 2026-11-15"] {
        let mut manifest: Value =
            serde_json::from_str(ManagedToolsManifest::embedded_str()).unwrap();
        manifest["tools"]["ffmpeg"]["platforms"]["windows-x64"]["hashSource"] =
            Value::String(invalid.to_owned());
        assert!(
            !validator.is_valid(&manifest),
            "schema should reject hashSource {invalid:?}"
        );
    }
}
