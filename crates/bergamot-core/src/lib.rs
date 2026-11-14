//! Headless action manifests, matching, and execution for Bergamot.

pub mod descriptor;
pub mod dispatcher;
pub mod executors;
pub mod matcher;
pub mod registry;
pub mod staging;

pub use descriptor::{
    ActionDescriptor, ActionInvocation, ActionProgress, ActionResult, Cardinality, DragContext,
    ExecutorConfig, MatchingRule, OutputConfig, OutputMode, Platform,
};
pub use dispatcher::Dispatcher;
pub use executors::ExecutionError;
pub use registry::{ActionRegistry, RegistryError};
pub use tokio_util::sync::CancellationToken;
