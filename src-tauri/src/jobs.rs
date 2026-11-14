use std::sync::{Arc, Mutex};
use std::time::Duration;

use bergamot_core::{
    ActionInvocation, ActionProgress, CancellationToken, Dispatcher, ExecutionError,
};
use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tokio::sync::mpsc;
use uuid::Uuid;

#[derive(Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
enum Status {
    Running,
    Cancelling,
    Succeeded,
    Cancelled,
    Failed,
}

impl Status {
    fn active(self) -> bool {
        matches!(self, Self::Running | Self::Cancelling)
    }
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct JobProgress {
    job_id: String,
    label: String,
    #[serde(flatten)]
    progress: ActionProgress,
    fraction: f64,
    status: Status,
    message: String,
}

struct Job {
    update: JobProgress,
    cancel: CancellationToken,
}

#[derive(Default)]
struct JobList {
    entries: Vec<Job>,
    revision: u64,
    quitting: bool,
}

#[derive(Default)]
pub struct Jobs(Mutex<JobList>);

#[derive(Serialize, Clone)]
pub struct JobSnapshot {
    revision: u64,
    jobs: Vec<JobProgress>,
}

impl JobList {
    fn snapshot(&self) -> JobSnapshot {
        JobSnapshot {
            revision: self.revision,
            jobs: self
                .entries
                .iter()
                .rev()
                .map(|job| job.update.clone())
                .collect(),
        }
    }

    fn publish(
        &mut self,
        app: &AppHandle,
        update: Option<JobProgress>,
        resize: bool,
        reposition: bool,
    ) {
        self.revision += 1;
        if let Some(update) = update {
            let _ = app.emit("job:progress", update);
        }
        let _ = app.emit_to("hud", "jobs:updated", self.snapshot());
        if resize {
            if let Err(error) = crate::windows::update_hud(app, self.entries.len(), reposition) {
                eprintln!("Could not update the progress HUD: {error}");
            }
        }
    }
}

impl Jobs {
    pub fn quit(&self, app: &AppHandle) {
        let mut list = self.0.lock().unwrap();
        list.quitting = true;
        if !list.entries.iter().any(|job| job.update.status.active()) {
            drop(list);
            app.exit(0);
            return;
        }
        for job in &mut list.entries {
            if job.update.status.active() {
                job.update.status = Status::Cancelling;
                job.update.message = "Cancelling…".to_owned();
                job.cancel.cancel();
            }
        }
        list.publish(app, None, false, false);
    }
}

pub fn start(
    app: AppHandle,
    dispatcher: Arc<Dispatcher>,
    invocation: ActionInvocation,
    label: String,
    total: usize,
) -> Result<String, String> {
    let job_id = Uuid::new_v4().to_string();
    let cancel = CancellationToken::new();
    let update = JobProgress {
        job_id: job_id.clone(),
        label,
        progress: ActionProgress {
            action_id: invocation.action_id.clone(),
            completed: 0,
            total,
        },
        fraction: 0.0,
        status: Status::Running,
        message: "Starting…".to_owned(),
    };
    {
        let jobs = app.state::<Jobs>();
        let mut list = jobs.0.lock().unwrap();
        if list.quitting {
            return Err("Bergamotto is quitting".into());
        }
        let reposition = !list.entries.iter().any(|job| job.update.status.active());
        list.entries.push(Job {
            update: update.clone(),
            cancel: cancel.clone(),
        });
        list.publish(&app, Some(update), true, reposition);
    }

    tauri::async_runtime::spawn(run(app, dispatcher, invocation, job_id.clone(), cancel));
    Ok(job_id)
}

async fn run(
    app: AppHandle,
    dispatcher: Arc<Dispatcher>,
    invocation: ActionInvocation,
    job_id: String,
    cancel: CancellationToken,
) {
    let (sender, mut receiver) = mpsc::channel(8);
    let mut dispatch = std::pin::pin!(dispatcher.dispatch(&invocation, sender, &cancel));
    let outcome = loop {
        tokio::select! {
            biased;
            Some(progress) = receiver.recv() => {
                let app = app.clone();
                let job_id = job_id.clone();
                let _ = app.clone().run_on_main_thread(move || {
                    change_job(&app, &job_id, |update| {
                        update.fraction = progress.completed as f64 / progress.total.max(1) as f64;
                        if update.status == Status::Running {
                            update.message = format!("{} of {} complete", progress.completed, progress.total);
                        }
                        update.progress = progress;
                    });
                });
            }
            outcome = &mut dispatch => break outcome,
        }
    };

    let finish_app = app.clone();
    let _ = app.run_on_main_thread(move || {
        let (status, message, completed) = match outcome {
            Ok(result) => {
                let copied = result.clipboard.filter(|text| !text.is_empty());
                match copied
                    .map(|text| finish_app.clipboard().write_text(text))
                    .transpose()
                {
                    Ok(copied) => (
                        Status::Succeeded,
                        if copied.is_some() {
                            "Copied to clipboard"
                        } else {
                            "Completed"
                        }
                        .to_owned(),
                        Some(result.completed),
                    ),
                    Err(error) => (
                        Status::Failed,
                        format!("Clipboard: {error}"),
                        Some(result.completed),
                    ),
                }
            }
            Err(ExecutionError::Cancelled) => (Status::Cancelled, "Cancelled".to_owned(), None),
            Err(error) => (Status::Failed, error.to_string(), None),
        };
        change_job(&finish_app, &job_id, |update| {
            update.status = status;
            update.message = message;
            if let Some(completed) = completed {
                update.progress.completed = completed;
                update.fraction = 1.0;
            }
        });
        let should_exit = {
            let jobs = finish_app.state::<Jobs>();
            let list = jobs.0.lock().unwrap();
            list.quitting && !list.entries.iter().any(|job| job.update.status.active())
        };
        if should_exit {
            finish_app.exit(0);
        } else if status != Status::Failed {
            tauri::async_runtime::spawn(async move {
                tokio::time::sleep(Duration::from_millis(2500)).await;
                let app = finish_app.clone();
                let _ = finish_app.run_on_main_thread(move || remove_job(&app, &job_id));
            });
        }
    });
}

fn change_job(app: &AppHandle, id: &str, change: impl FnOnce(&mut JobProgress)) {
    let jobs = app.state::<Jobs>();
    let mut list = jobs.0.lock().unwrap();
    if let Some(job) = list.entries.iter_mut().find(|job| job.update.job_id == id) {
        change(&mut job.update);
        let update = job.update.clone();
        list.publish(app, Some(update), false, false);
    }
}

fn remove_job(app: &AppHandle, id: &str) {
    let jobs = app.state::<Jobs>();
    let mut list = jobs.0.lock().unwrap();
    list.entries.retain(|job| job.update.job_id != id);
    list.publish(app, None, true, false);
}

#[tauri::command]
pub fn cancel_job(app: AppHandle, jobs: State<'_, Jobs>, job_id: String) -> Result<(), String> {
    let mut list = jobs.0.lock().unwrap();
    let Some(job) = list
        .entries
        .iter_mut()
        .find(|job| job.update.job_id == job_id)
    else {
        return Ok(());
    };
    if job.update.status.active() {
        job.cancel.cancel();
        job.update.status = Status::Cancelling;
        job.update.message = "Cancelling…".to_owned();
        let update = job.update.clone();
        list.publish(&app, Some(update), false, false);
    }
    Ok(())
}

#[tauri::command]
pub fn dismiss_job(app: AppHandle, jobs: State<'_, Jobs>, job_id: String) {
    let mut list = jobs.0.lock().unwrap();
    let old_len = list.entries.len();
    list.entries
        .retain(|job| job.update.job_id != job_id || job.update.status.active());
    if old_len != list.entries.len() {
        list.publish(&app, None, true, false);
    }
}

#[tauri::command]
pub fn job_snapshot(jobs: State<'_, Jobs>) -> JobSnapshot {
    jobs.0.lock().unwrap().snapshot()
}
