use std::fs;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use bergamot_core::{ActionRegistry, Cardinality, DragContext, Platform, RegistryError};
use serde_json::{json, Value};
use tempfile::tempdir;

const VALID: &str = include_str!("../../../shared/fixtures/valid-process.json");
const INVALID: &str = include_str!("../../../shared/fixtures/invalid-action.json");
const DRAG: &str = include_str!("../../../shared/fixtures/drag-context.json");

#[test]
fn schema_validates_all_starters_and_fixtures() {
    for manifest in [
        VALID,
        include_str!("../../../shared/starter-pack/ffmpeg-compress.json"),
        include_str!("../../../shared/starter-pack/ffmpeg-extract-audio.json"),
        include_str!("../../../shared/starter-pack/imagemagick-webp.json"),
        include_str!("../../../shared/starter-pack/sha256-checksum.json"),
        include_str!("../../../shared/starter-pack/zip-files.json"),
        include_str!("../../../shared/starter-pack/webhook-upload.json"),
    ] {
        let descriptor = ActionRegistry::parse_manifest(manifest).unwrap();
        let serialized = serde_json::to_string(&descriptor).unwrap();
        assert_eq!(
            ActionRegistry::parse_manifest(&serialized).unwrap(),
            descriptor
        );
    }

    assert!(matches!(
        ActionRegistry::parse_manifest(INVALID),
        Err(RegistryError::InvalidManifest(_))
    ));
    assert!(matches!(
        ActionRegistry::parse_manifest("{ invalid JSON"),
        Err(RegistryError::Json(_))
    ));
}

#[test]
fn schema_rejects_invalid_variants_and_unsafe_names() {
    let base: Value = serde_json::from_str(VALID).unwrap();
    for (field, value) in [
        ("/schemaVersion", json!(2)),
        ("/id", json!("../escape")),
        ("/matching/extensions", json!(["TXT"])),
        ("/matching/extensions", json!(["txt", "*"])),
        (
            "/executor",
            json!({"type": "script", "interpreter": "python"}),
        ),
        (
            "/executor",
            json!({"type": "script", "interpreter": "python", "scriptBody": "print(1)", "scriptPath": "script.py"}),
        ),
        (
            "/executor",
            json!({"type": "process", "program": "tool", "timeoutSeconds": 0}),
        ),
        ("/output", json!({"mode": "file", "suffix": "/escape"})),
        ("/output", json!({"mode": "file", "extension": "../escape"})),
    ] {
        let mut action = base.clone();
        *action.pointer_mut(field).unwrap() = value;
        assert!(
            matches!(
                ActionRegistry::parse_manifest(&action.to_string()),
                Err(RegistryError::InvalidManifest(_))
            ),
            "invalid field {field} was accepted"
        );
    }
}

#[test]
fn registry_persists_crud_and_does_not_restore_deleted_starters() {
    let temp = tempdir().unwrap();
    let path = temp.path().join("actions");
    let registry = ActionRegistry::open(&path).unwrap();
    let starters = registry.list_actions();
    assert_eq!(starters.len(), 6);

    let mut descriptor = registry.import_action(VALID).unwrap();
    assert!(matches!(
        registry.import_action(VALID),
        Err(RegistryError::AlreadyExists(_))
    ));
    descriptor.label = "New label".to_owned();
    registry.save_action(&descriptor).unwrap();
    let exported = registry.export_action(&descriptor.id).unwrap();
    assert_eq!(
        ActionRegistry::parse_manifest(&exported).unwrap(),
        descriptor
    );
    let duplicate = registry
        .duplicate_action(&descriptor.id, "test.duplicate")
        .unwrap();
    assert_eq!(duplicate.id, "test.duplicate");
    assert_eq!(duplicate.label, descriptor.label);
    registry.delete_action(&duplicate.id).unwrap();
    assert!(registry.get_action(&duplicate.id).is_none());
    drop(registry);

    let registry = ActionRegistry::open(&path).unwrap();
    assert_eq!(registry.get_action(&descriptor.id), Some(descriptor));
    for action in registry.list_actions() {
        registry.delete_action(&action.id).unwrap();
    }
    drop(registry);
    assert!(ActionRegistry::open(&path)
        .unwrap()
        .list_actions()
        .is_empty());
}

#[test]
fn reload_handler_sees_external_manifest_changes() {
    let temp = tempdir().unwrap();
    let (updated, changes) = std::sync::mpsc::channel();
    let registry = ActionRegistry::open_with_reload_handler(temp.path(), move || {
        let _ = updated.send(());
    })
    .unwrap();

    // Drain initial seeding events if any
    while changes.recv_timeout(Duration::from_millis(100)).is_ok() {}

    let source = temp.path().join("external.tmp");
    fs::write(&source, VALID).unwrap();
    assert!(changes.recv_timeout(Duration::from_millis(100)).is_err());
    fs::rename(source, temp.path().join("external.json")).unwrap();
    changes.recv_timeout(Duration::from_secs(5)).unwrap();
    assert!(registry.get_action("test.text-files").is_some());
}

#[test]
fn registry_watches_external_create_replace_and_delete() {
    let temp = tempdir().unwrap();
    let path = temp.path().join("custom-name.json");
    fs::write(temp.path().join("invalid.json"), INVALID).unwrap();
    let registry = ActionRegistry::open(temp.path()).unwrap();
    assert!(registry.list_actions().is_empty());
    assert!(registry
        .take_reload_errors()
        .iter()
        .any(|error| error.contains("invalid.json")));

    fs::write(&path, VALID).unwrap();
    wait_until(|| registry.get_action("test.text-files").is_some());

    let mut action = registry.get_action("test.text-files").unwrap();
    action.label = "Edited externally".to_owned();
    let replacement = temp.path().join("editor-buffer.tmp");
    fs::write(&replacement, serde_json::to_vec(&action).unwrap()).unwrap();
    fs::rename(&replacement, &path).unwrap();
    wait_until(|| registry.get_action("test.text-files").as_ref() == Some(&action));

    action.label = "Edited through the registry".to_owned();
    registry.save_action(&action).unwrap();
    assert_eq!(
        ActionRegistry::parse_manifest(&fs::read_to_string(&path).unwrap()).unwrap(),
        action
    );

    fs::remove_file(&path).unwrap();
    wait_until(|| registry.get_action("test.text-files").is_none());
}

#[test]
fn duplicate_manifest_ids_cannot_be_overwritten_by_registry_operations() {
    let temp = tempdir().unwrap();
    for filename in ["first.json", "second.json"] {
        fs::write(temp.path().join(filename), VALID).unwrap();
    }
    let registry = ActionRegistry::open(temp.path()).unwrap();
    let action = ActionRegistry::parse_manifest(VALID).unwrap();
    assert!(registry.get_action(&action.id).is_none());
    assert!(registry.import_action(VALID).is_err());
    assert!(registry.save_action(&action).is_err());
    assert!(registry.delete_action(&action.id).is_err());
    assert_eq!(
        fs::read_to_string(temp.path().join("first.json")).unwrap(),
        VALID
    );
    assert_eq!(
        fs::read_to_string(temp.path().join("second.json")).unwrap(),
        VALID
    );
}

#[test]
fn matcher_filters_every_file_and_returns_ids_in_stable_order() {
    let temp = tempdir().unwrap();
    fs::write(temp.path().join("batch.json"), VALID).unwrap();
    let registry = ActionRegistry::open(temp.path()).unwrap();
    let mut action = ActionRegistry::parse_manifest(VALID).unwrap();
    action.id = "test.single".to_owned();
    action.matching.cardinality = Cardinality::Single;
    action.matching.extensions = vec!["txt".to_owned()];
    registry.save_action(&action).unwrap();
    action.id = "test.each".to_owned();
    action.matching.cardinality = Cardinality::Each;
    registry.save_action(&action).unwrap();
    action.id = "test.wildcard".to_owned();
    action.matching.extensions = vec!["*".to_owned()];
    action.platforms = vec![Platform::Linux];
    registry.save_action(&action).unwrap();
    action.id = "test.disabled".to_owned();
    action.enabled = false;
    registry.save_action(&action).unwrap();

    let ids = |context: &DragContext| {
        registry
            .available_actions(context)
            .into_iter()
            .map(|action| action.id)
            .collect::<Vec<_>>()
    };
    let mut context: DragContext = serde_json::from_str(DRAG).unwrap();
    assert_eq!(ids(&context), ["test.text-files", "test.wildcard"]);
    context.files.pop();
    assert_eq!(
        ids(&context),
        [
            "test.each",
            "test.single",
            "test.text-files",
            "test.wildcard"
        ]
    );
    context.files.push(PathBuf::from("other.txt"));
    assert_eq!(
        ids(&context),
        ["test.each", "test.text-files", "test.wildcard"]
    );
    context.files.push(PathBuf::from("README"));
    assert_eq!(ids(&context), ["test.wildcard"]);
    context.platform = Platform::Windows;
    assert!(ids(&context).is_empty());
    context.platform = Platform::Linux;
    context.files.clear();
    assert!(ids(&context).is_empty());

    let mut unicode = ActionRegistry::parse_manifest(VALID).unwrap();
    unicode.id = "test.unicode".to_owned();
    unicode.matching.extensions = vec!["éxt".to_owned()];
    registry.save_action(&unicode).unwrap();
    context.files.push(PathBuf::from("file.ÉXT"));
    assert_eq!(ids(&context), ["test.unicode", "test.wildcard"]);
}

#[test]
fn matcher_sorts_actions_by_sort_order_and_label_deterministically() {
    use bergamot_core::ActionMatcher;

    let mut action_a = ActionRegistry::parse_manifest(VALID).unwrap();
    action_a.id = "test.a".to_owned();
    action_a.label = "Beta".to_owned();
    action_a.sort_order = Some(10);

    let mut action_b = ActionRegistry::parse_manifest(VALID).unwrap();
    action_b.id = "test.b".to_owned();
    action_b.label = "Alpha".to_owned();
    action_b.sort_order = Some(10);

    let mut action_c = ActionRegistry::parse_manifest(VALID).unwrap();
    action_c.id = "test.c".to_owned();
    action_c.label = "First".to_owned();
    action_c.sort_order = Some(1);

    let mut action_d = ActionRegistry::parse_manifest(VALID).unwrap();
    action_d.id = "test.d".to_owned();
    action_d.label = "Default Zero".to_owned();
    action_d.sort_order = None; // Defaults to 0

    let context: DragContext = serde_json::from_str(DRAG).unwrap();
    let actions = vec![action_a, action_b, action_c, action_d];

    let sorted = ActionMatcher::available_actions(&actions, &context);
    let ids: Vec<_> = sorted.iter().map(|a| a.id.as_str()).collect();

    assert_eq!(ids, vec!["test.d", "test.c", "test.b", "test.a"]);
}

#[test]
fn backward_compatibility_with_manifests_omitting_sort_order_and_accent_color() {
    let descriptor = ActionRegistry::parse_manifest(VALID).unwrap();
    assert_eq!(descriptor.sort_order, None);
    assert_eq!(descriptor.accent_color, None);

    let json_with_customization = json!({
        "schemaVersion": 1,
        "id": "custom.order-color",
        "label": "Customized Action",
        "enabled": true,
        "sortOrder": 42,
        "accentColor": "#84cc16",
        "platforms": ["linux", "macos", "windows"],
        "matching": { "extensions": ["*"], "cardinality": "single" },
        "executor": { "type": "process", "program": "echo" },
        "output": { "mode": "none" }
    });
    let manifest_str = serde_json::to_string(&json_with_customization).unwrap();
    let parsed = ActionRegistry::parse_manifest(&manifest_str).unwrap();
    assert_eq!(parsed.sort_order, Some(42));
    assert_eq!(parsed.accent_color.as_deref(), Some("#84cc16"));
}

#[test]
fn reorder_actions_updates_sort_order_cleanly() {
    let dir = tempdir().unwrap();
    let registry = ActionRegistry::open(dir.path()).unwrap();

    let mut action1 = ActionRegistry::parse_manifest(VALID).unwrap();
    action1.id = "action.one".to_owned();
    action1.label = "Action One".to_owned();
    registry.save_action(&action1).unwrap();

    let mut action2 = ActionRegistry::parse_manifest(VALID).unwrap();
    action2.id = "action.two".to_owned();
    action2.label = "Action Two".to_owned();
    registry.save_action(&action2).unwrap();

    let mut action3 = ActionRegistry::parse_manifest(VALID).unwrap();
    action3.id = "action.three".to_owned();
    action3.label = "Action Three".to_owned();
    registry.save_action(&action3).unwrap();

    registry
        .reorder_actions(&[
            "action.three".to_owned(),
            "action.one".to_owned(),
            "action.two".to_owned(),
        ])
        .unwrap();

    let reloaded1 = registry.get_action("action.one").unwrap();
    let reloaded2 = registry.get_action("action.two").unwrap();
    let reloaded3 = registry.get_action("action.three").unwrap();

    assert_eq!(reloaded3.sort_order, Some(0));
    assert_eq!(reloaded1.sort_order, Some(1));
    assert_eq!(reloaded2.sort_order, Some(2));

    let context: DragContext = serde_json::from_str(DRAG).unwrap();
    let matched = registry.available_actions(&context);
    let matched_ids: Vec<_> = matched
        .iter()
        .map(|a| a.id.as_str())
        .filter(|id| id.starts_with("action."))
        .collect();
    assert_eq!(
        matched_ids,
        vec!["action.three", "action.one", "action.two"]
    );
}

fn wait_until(mut condition: impl FnMut() -> bool) {
    let deadline = Instant::now() + Duration::from_secs(10);
    while !condition() {
        assert!(
            Instant::now() < deadline,
            "watcher did not reload the registry"
        );
        std::thread::sleep(Duration::from_millis(20));
    }
}
