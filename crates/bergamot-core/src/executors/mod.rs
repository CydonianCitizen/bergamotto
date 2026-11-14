pub mod process;
pub mod webhook;

use thiserror::Error;

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
