use bergamot_core::{ActionDescriptor, ActionRegistry, PackCollision, RegistryError};
use tempfile::tempdir;

const ACTION: &str = include_str!("../../../shared/fixtures/valid-process.json");

#[test]
fn export_round_trips_single_and_enabled_actions() {
    let temp = tempdir().unwrap();
    let registry = ActionRegistry::open(temp.path().join("source")).unwrap();
    let action = registry.import_action(ACTION).unwrap();
    let single = registry.export_pack(Some(&action.id)).unwrap();
    let destination = ActionRegistry::open(temp.path().join("destination")).unwrap();
    let imported = destination
        .import_pack(&single, PackCollision::Reject)
        .unwrap();
    assert_eq!(imported[0].id, action.id);
    let enabled: Vec<ActionDescriptor> =
        serde_json::from_str(&registry.export_pack(None).unwrap()).unwrap();
    assert!(enabled.iter().any(|entry| entry.id == action.id));
    assert!(enabled.iter().all(|entry| entry.enabled));
}

#[test]
fn invalid_or_repeated_entries_leave_registry_untouched() {
    let temp = tempdir().unwrap();
    let registry = ActionRegistry::open(temp.path()).unwrap();
    let action = ActionRegistry::parse_manifest(ACTION).unwrap();
    for pack in [
        format!("[{ACTION}, {{\"id\": \"bad\"}}]"),
        format!("[{ACTION}, {ACTION}]"),
    ] {
        assert!(registry
            .import_pack(&pack, PackCollision::Overwrite)
            .is_err());
        assert!(registry.get_action(&action.id).is_none());
    }
    assert!(registry.import_pack("[]", PackCollision::Reject).is_err());
}

#[test]
fn collision_preview_duplicate_and_overwrite_preserve_intended_manifests() {
    let temp = tempdir().unwrap();
    let registry = ActionRegistry::open(temp.path()).unwrap();
    let original = registry.import_action(ACTION).unwrap();
    let mut replacement = original.clone();
    replacement.label = "Replacement".into();
    let document = serde_json::to_string(&replacement).unwrap();
    let preview = registry.preview_pack(&document).unwrap();
    assert_eq!(preview.collisions, std::slice::from_ref(&original.id));
    assert_eq!(preview.count, 1);
    assert!(matches!(
        registry.import_pack(&document, PackCollision::Reject),
        Err(RegistryError::AlreadyExists(_))
    ));
    let duplicates = registry
        .import_pack(&document, PackCollision::Duplicate)
        .unwrap();
    assert_ne!(duplicates[0].id, original.id);
    assert_eq!(duplicates[0].label, replacement.label);
    assert_eq!(
        registry.get_action(&original.id).unwrap().label,
        original.label
    );
    registry
        .import_pack(&document, PackCollision::Overwrite)
        .unwrap();
    assert_eq!(
        registry.get_action(&original.id).unwrap().label,
        replacement.label
    );
}

#[test]
fn filesystem_conflict_prevents_partial_import() {
    let temp = tempdir().unwrap();
    let registry = ActionRegistry::open(temp.path()).unwrap();
    let action = ActionRegistry::parse_manifest(ACTION).unwrap();
    let mut blocked = action.clone();
    blocked.id = "blocked".into();
    std::fs::create_dir(temp.path().join("action-blocked.json")).unwrap();
    let document = serde_json::to_string(&[&action, &blocked]).unwrap();
    assert!(registry
        .import_pack(&document, PackCollision::Reject)
        .is_err());
    assert!(registry.get_action(&action.id).is_none());
    assert!(!temp
        .path()
        .join(format!("action-{}.json", action.id))
        .exists());
}
