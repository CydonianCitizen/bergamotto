use std::sync::Arc;

use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::descriptor::{
    ActionInvocation, ActionProgress, ActionResult, Cardinality, ExecutorConfig, OutputMode,
};
use crate::executors::process;
use crate::executors::webhook::{KeyringSecretProvider, SecretProvider, WebhookExecutor};
use crate::executors::ExecutionError;
use crate::registry::ActionRegistry;
use crate::staging::StagedFile;

pub struct Dispatcher {
    registry: Arc<ActionRegistry>,
    webhook: WebhookExecutor,
}

impl Dispatcher {
    pub fn new(registry: Arc<ActionRegistry>) -> Result<Self, ExecutionError> {
        Self::with_secret_provider(registry, Arc::new(KeyringSecretProvider))
    }

    pub fn with_secret_provider(
        registry: Arc<ActionRegistry>,
        secrets: Arc<dyn SecretProvider>,
    ) -> Result<Self, ExecutionError> {
        Ok(Self {
            registry,
            webhook: WebhookExecutor::new(secrets)?,
        })
    }

    /// Runs one job for `single` or `batch`, and one job per file for `each`.
    /// The caller can drain progress concurrently. Dropping the receiver detaches it.
    /// A batch uses the first input as the basis for its staged filename.
    /// If a later job fails, files already committed by earlier jobs remain available.
    pub async fn dispatch(
        &self,
        invocation: &ActionInvocation,
        progress: mpsc::Sender<ActionProgress>,
        cancel: &CancellationToken,
    ) -> Result<ActionResult, ExecutionError> {
        if cancel.is_cancelled() {
            return Err(ExecutionError::Cancelled);
        }
        let action = self
            .registry
            .get_action(&invocation.action_id)
            .ok_or_else(|| ExecutionError::ActionNotFound(invocation.action_id.clone()))?;
        if !action.matches(&invocation.context) {
            return Err(ExecutionError::ActionUnavailable(action.id));
        }

        let files = &invocation.context.files;
        let job_size = match action.matching.cardinality {
            Cardinality::Each => 1,
            Cardinality::Single | Cardinality::Batch => files.len(),
        };
        let jobs = files.chunks(job_size);
        let total = jobs.len();
        let mut result = ActionResult {
            action_id: action.id,
            completed: 0,
            output_files: Vec::new(),
            stdout: Vec::new(),
            stderr: Vec::new(),
            clipboard: None,
        };
        report_progress(&progress, &result, total, cancel).await;
        let mut text_output = Vec::new();

        for job in jobs {
            if cancel.is_cancelled() {
                return Err(ExecutionError::Cancelled);
            }
            let staged = if action.output.mode == OutputMode::File {
                Some(StagedFile::new(&job[0], &action.output)?)
            } else {
                None
            };
            let mut output = match &action.executor {
                ExecutorConfig::Webhook { .. } => {
                    self.webhook.execute(&action.executor, job, cancel).await?
                }
                _ => {
                    let path = staged.as_ref().map(StagedFile::path);
                    process::execute(&action.executor, job, path, cancel).await?
                }
            };
            if cancel.is_cancelled() {
                return Err(ExecutionError::Cancelled);
            }
            if let Some(mut staged) = staged {
                if matches!(&action.executor, ExecutorConfig::Webhook { .. }) {
                    let body = std::mem::take(&mut output.stdout);
                    staged = tokio::task::spawn_blocking(move || {
                        std::fs::write(staged.path(), body)?;
                        Ok::<_, std::io::Error>(staged)
                    })
                    .await
                    .map_err(std::io::Error::other)??;
                }
                if cancel.is_cancelled() {
                    return Err(ExecutionError::Cancelled);
                }
                result.output_files.push(staged.commit()?);
            }
            if matches!(
                action.output.mode,
                OutputMode::Stdout | OutputMode::Clipboard
            ) {
                text_output.extend(output.stdout);
            }
            result.stderr.extend(output.stderr);
            result.completed += 1;
            report_progress(&progress, &result, total, cancel).await;
        }

        match action.output.mode {
            OutputMode::Stdout => result.stdout = text_output,
            OutputMode::Clipboard => {
                result.clipboard = Some(String::from_utf8_lossy(&text_output).into_owned());
            }
            _ => {}
        }
        Ok(result)
    }
}

async fn report_progress(
    progress: &mpsc::Sender<ActionProgress>,
    result: &ActionResult,
    total: usize,
    cancel: &CancellationToken,
) {
    let event = ActionProgress {
        action_id: result.action_id.clone(),
        completed: result.completed,
        total,
    };
    tokio::select! {
        _ = progress.send(event) => {},
        _ = cancel.cancelled() => {},
    }
}
