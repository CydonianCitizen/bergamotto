//! Headless action manifests, matching, and execution for Bergamot.

pub mod descriptor;
pub mod dispatcher;
pub mod executors;
pub mod managed_tools;
pub mod matcher;
pub mod registry;
pub mod secrets;
pub mod settings;
pub mod staging;

pub use descriptor::{
    ActionDescriptor, ActionInvocation, ActionProgress, ActionResult, Cardinality, DragContext,
    ExecutorConfig, MatchingRule, OutputConfig, OutputMode, Platform,
};
pub use dispatcher::Dispatcher;
pub use executors::ExecutionError;
pub use matcher::ActionMatcher;
pub use registry::{ActionRegistry, PackCollision, PackPreview, RegistryError};
pub use secrets::{KeyringSecretStore, MemorySecretStore, SecretError, SecretStore};
pub use settings::{AppSettings, ModifierKey, SettingsStore};
pub use tokio_util::sync::CancellationToken;
