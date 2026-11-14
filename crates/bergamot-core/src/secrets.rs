use std::collections::HashMap;
use std::sync::Mutex;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum SecretError {
    #[error("Credential key must contain text and cannot contain NUL characters")]
    InvalidKey,
    #[error("Credential value cannot be empty")]
    EmptySecret,
    #[error("{0}")]
    Unavailable(&'static str),
}

/// Blocking credential access. Async callers should use a blocking thread.
pub trait SecretStore: Send + Sync {
    fn set_secret(&self, key: &str, value: &str) -> Result<(), SecretError>;
    fn get_secret(&self, key: &str) -> Result<Option<String>, SecretError>;
    fn delete_secret(&self, key: &str) -> Result<(), SecretError>;

    fn has_secret(&self, key: &str) -> Result<bool, SecretError> {
        Ok(self.get_secret(key)?.is_some())
    }
}

#[derive(Default)]
pub struct MemorySecretStore(Mutex<HashMap<String, String>>);

impl SecretStore for MemorySecretStore {
    fn set_secret(&self, key: &str, value: &str) -> Result<(), SecretError> {
        validate_value(key, value)?;
        self.0
            .lock()
            .unwrap()
            .insert(key.to_owned(), value.to_owned());
        Ok(())
    }

    fn get_secret(&self, key: &str) -> Result<Option<String>, SecretError> {
        validate_key(key)?;
        Ok(self.0.lock().unwrap().get(key).cloned())
    }

    fn delete_secret(&self, key: &str) -> Result<(), SecretError> {
        validate_key(key)?;
        self.0.lock().unwrap().remove(key);
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct KeyringSecretStore;

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
mod native {
    use super::*;

    const SERVICE: &str = "bergamotto";
    const LEGACY_SERVICE: &str = "bergamot";
    static KEYRING_ACCESS: Mutex<()> = Mutex::new(());

    impl SecretStore for KeyringSecretStore {
        fn set_secret(&self, key: &str, value: &str) -> Result<(), SecretError> {
            validate_value(key, value)?;
            let _guard = KEYRING_ACCESS.lock().unwrap();
            keyring::Entry::new(SERVICE, key)
                .and_then(|entry| entry.set_password(value))
                .map_err(store_error)
        }

        fn get_secret(&self, key: &str) -> Result<Option<String>, SecretError> {
            validate_key(key)?;
            let _guard = KEYRING_ACCESS.lock().unwrap();
            match get_password(SERVICE, key)? {
                Some(value) => Ok(Some(value)),
                None => get_password(LEGACY_SERVICE, key),
            }
        }

        fn delete_secret(&self, key: &str) -> Result<(), SecretError> {
            validate_key(key)?;
            let _guard = KEYRING_ACCESS.lock().unwrap();
            // Remove the previous service's entry first so a failed delete cannot revive it.
            for service in [LEGACY_SERVICE, SERVICE] {
                optional(
                    keyring::Entry::new(service, key).and_then(|entry| entry.delete_credential()),
                )?;
            }
            Ok(())
        }
    }

    fn get_password(service: &str, key: &str) -> Result<Option<String>, SecretError> {
        optional(keyring::Entry::new(service, key).and_then(|entry| entry.get_password()))
    }

    fn optional<T>(result: keyring::Result<T>) -> Result<Option<T>, SecretError> {
        match result {
            Ok(value) => Ok(Some(value)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(error) => Err(store_error(error)),
        }
    }

    fn store_error(error: keyring::Error) -> SecretError {
        let message = match error {
            keyring::Error::TooLong(_, _) => "Credential exceeds the OS credential store's size limit",
            keyring::Error::BadEncoding(_) => "Stored credential is not valid UTF-8",
            keyring::Error::Ambiguous(_) => "Multiple entries match this credential key in the OS credential store",
            _ if cfg!(target_os = "linux") => "Cannot access the D-Bus Secret Service. Start or unlock a Secret Service provider in this desktop session.",
            _ => "Cannot access the OS credential store. Check that it is available and unlocked.",
        };
        SecretError::Unavailable(message)
    }
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
impl SecretStore for KeyringSecretStore {
    fn set_secret(&self, _: &str, _: &str) -> Result<(), SecretError> {
        Err(SecretError::Unavailable(
            "No OS credential store on this platform",
        ))
    }

    fn get_secret(&self, _: &str) -> Result<Option<String>, SecretError> {
        Err(SecretError::Unavailable(
            "No OS credential store on this platform",
        ))
    }

    fn delete_secret(&self, _: &str) -> Result<(), SecretError> {
        Err(SecretError::Unavailable(
            "No OS credential store on this platform",
        ))
    }
}

fn validate_key(key: &str) -> Result<(), SecretError> {
    if key.trim().is_empty() || key.contains('\0') {
        return Err(SecretError::InvalidKey);
    }
    Ok(())
}

fn validate_value(key: &str, value: &str) -> Result<(), SecretError> {
    validate_key(key)?;
    if value.is_empty() {
        return Err(SecretError::EmptySecret);
    }
    Ok(())
}
