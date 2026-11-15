pub mod process;
pub mod webhook;

use thiserror::Error;

/// Maximum bytes captured per process output stream before the child is stopped.
pub const MAX_CAPTURED_OUTPUT_BYTES: u64 = 8 * 1024 * 1024;
/// Maximum webhook response bytes streamed to a staged file.
pub const MAX_WEBHOOK_FILE_BYTES: u64 = 1024 * 1024 * 1024;
/// Maximum stdout and stderr bytes retained for one dispatched action result.
pub const MAX_AGGREGATE_OUTPUT_BYTES: u64 = 16 * 1024 * 1024;

#[derive(Debug, Error)]
pub enum ExecutionError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("action was cancelled")]
    Cancelled,
    #[error("action timed out after {0} seconds")]
    TimedOut(u32),
    #[error("invalid execution configuration: {0}")]
    InvalidConfig(String),
    #[error("process exited unsuccessfully ({code:?}): {stderr}")]
    ProcessFailed { code: Option<i32>, stderr: String },
    #[error("{stream} exceeded the {limit}-byte output limit")]
    OutputLimit { stream: &'static str, limit: u64 },
    #[error("webhook request failed: {0}")]
    Webhook(String),
    #[error("credential could not be retrieved: {0}")]
    Secret(String),
    #[error("action {0:?} was not found")]
    ActionNotFound(String),
    #[error("action {0:?} is unavailable for the selected files or platform")]
    ActionUnavailable(String),
}

#[derive(Debug)]
pub struct ExecutionOutput {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}
