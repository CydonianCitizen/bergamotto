use bergamot_core::{MemorySecretStore, SecretError, SecretStore};

#[test]
fn secrets_can_be_set_replaced_and_deleted_by_key() {
    let store = MemorySecretStore::default();
    assert!(!store.has_secret("webhook").unwrap());
    assert_eq!(store.get_secret("webhook").unwrap(), None);
    store.set_secret("webhook", "first").unwrap();
    store.set_secret("other", "private").unwrap();
    store.set_secret("webhook", "updated").unwrap();
    assert!(store.has_secret("webhook").unwrap());
    assert_eq!(
        store.get_secret("webhook").unwrap().as_deref(),
        Some("updated")
    );
    store.delete_secret("webhook").unwrap();
    store.delete_secret("webhook").unwrap();
    assert!(!store.has_secret("webhook").unwrap());
    assert!(store.has_secret("other").unwrap());
}

#[test]
fn invalid_credentials_never_enter_the_store() {
    let store = MemorySecretStore::default();
    assert!(matches!(
        store.set_secret("  ", "token"),
        Err(SecretError::InvalidKey)
    ));
    assert!(matches!(
        store.set_secret("webhook", ""),
        Err(SecretError::EmptySecret)
    ));
    assert!(!store.has_secret("webhook").unwrap());
}
