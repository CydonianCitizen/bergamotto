use bergamot_core::{AppSettings, ModifierKey, SettingsStore};

#[test]
fn settings_round_trip_and_failed_updates_keep_previous_values() {
    let temp = tempfile::tempdir().unwrap();
    let file = temp.path().join("new config/settings.json");
    let settings = SettingsStore::open(&file).unwrap();
    assert_eq!(settings.snapshot(), AppSettings::default());
    let custom = AppSettings {
        global_hook_enabled: false,
        modifier_key: ModifierKey::Alt,
        global_shortcut: "Ctrl+Alt+B".into(),
        custom_paths: vec![temp.path().to_string_lossy().into_owned()],
        tool_overrides: [(
            "example-tool".into(),
            temp.path().join("test bin").to_string_lossy().into_owned(),
        )]
        .into(),
        managed_tools_dir: None,
    };
    settings
        .update(|settings| *settings = custom.clone())
        .unwrap();
    assert_eq!(SettingsStore::open(&file).unwrap().snapshot(), custom);
    assert!(settings
        .update(|settings| settings.custom_paths.push("relative/path".into()))
        .is_err());
    assert_eq!(settings.snapshot(), custom);
    assert_eq!(SettingsStore::open(&file).unwrap().snapshot(), custom);
    let json = std::fs::read_to_string(&file).unwrap();
    assert!(json.contains("\"globalHookEnabled\": false"));
    assert!(json.contains("\"modifierKey\": \"Alt\""));

    // Test two-key modifiers like Alt+Shift
    settings
        .update(|settings| settings.modifier_key = ModifierKey::AltShift)
        .unwrap();
    let json_dual = std::fs::read_to_string(&file).unwrap();
    assert!(json_dual.contains("\"modifierKey\": \"Alt+Shift\""));
    assert_eq!(
        SettingsStore::open(&file).unwrap().snapshot().modifier_key,
        ModifierKey::AltShift
    );
}

#[test]
fn older_settings_use_defaults_for_missing_fields() {
    let temp = tempfile::tempdir().unwrap();
    let file = temp.path().join("settings.json");
    std::fs::write(&file, r#"{"globalHookEnabled":false}"#).unwrap();
    assert_eq!(
        SettingsStore::open(file).unwrap().snapshot(),
        AppSettings {
            global_hook_enabled: false,
            ..AppSettings::default()
        }
    );
}
