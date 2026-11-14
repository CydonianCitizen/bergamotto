use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;
use std::sync::Arc;

use bergamot_core::{
    ActionDescriptor, ActionInvocation, ActionRegistry, AppSettings, Dispatcher, DragContext,
    ExecutorConfig, KeyringSecretStore, ModifierKey, Platform, SecretError, SecretStore,
    SettingsStore,
};
use serde::Serialize;
use tauri::{AppHandle, Manager, PhysicalPosition, Position, State};
use tauri_plugin_opener::OpenerExt;
use uuid::Uuid;

use crate::{jobs, windows};

pub struct Actions {
    pub directory: PathBuf,
    pub registry: Arc<ActionRegistry>,
    pub dispatcher: Arc<Dispatcher>,
}

async fn access_secret<T: Send + 'static>(
    operation: impl FnOnce(KeyringSecretStore) -> Result<T, SecretError> + Send + 'static,
) -> Result<T, String> {
    tauri::async_runtime::spawn_blocking(move || operation(KeyringSecretStore))
        .await
        .map_err(|_| "Credential store operation failed".to_owned())?
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn set_secret(key: String, value: String) -> Result<(), String> {
    access_secret(move |store| store.set_secret(&key, &value)).await
}

#[tauri::command]
pub async fn has_secret(key: String) -> Result<bool, String> {
    access_secret(move |store| store.has_secret(&key)).await
}

#[tauri::command]
pub async fn delete_secret(key: String) -> Result<(), String> {
    access_secret(move |store| store.delete_secret(&key)).await
}

#[tauri::command]
pub fn get_settings(settings: State<'_, Arc<SettingsStore>>) -> AppSettings {
    settings.snapshot()
}

#[tauri::command]
pub async fn save_settings(
    app: AppHandle,
    modifier_key: ModifierKey,
    custom_paths: Vec<String>,
    tool_overrides: HashMap<String, String>,
) -> Result<(), String> {
    let handle = app.clone();
    let (send, receive) = tokio::sync::oneshot::channel();
    app.run_on_main_thread(move || {
        let result = crate::global_drag::update_settings(&handle, |settings| {
            settings.modifier_key = modifier_key;
            settings.custom_paths = custom_paths;
            settings.tool_overrides = tool_overrides;
        });
        let _ = send.send(result);
    })
    .map_err(|error| error.to_string())?;
    receive
        .await
        .map_err(|_| "Could not save settings".to_owned())?
}

#[derive(Serialize)]
pub struct ToolStatus {
    tool: String,
    path: Option<PathBuf>,
    message: Option<String>,
}

#[tauri::command]
pub async fn check_action_dependencies(
    actions: State<'_, Actions>,
    settings: State<'_, Arc<SettingsStore>>,
) -> Result<BTreeMap<String, ToolStatus>, String> {
    let actions = actions.registry.list_actions();
    let settings = settings.snapshot();
    tauri::async_runtime::spawn_blocking(move || {
        actions
            .into_iter()
            .filter_map(|action| {
                let tool = match &action.executor {
                    ExecutorConfig::Process { program, .. } => program.clone(),
                    ExecutorConfig::Script { interpreter, .. } => interpreter.clone(),
                    ExecutorConfig::Webhook { .. } => return None,
                };
                let (path, message) = match bergamot_core::executors::process::resolve_dependency(
                    &action.executor,
                    &settings,
                ) {
                    Ok(path) => (path, None),
                    Err(error) => (None, Some(error.to_string())),
                };
                Some((
                    action.id,
                    ToolStatus {
                        tool,
                        path,
                        message,
                    },
                ))
            })
            .collect()
    })
    .await
    .map_err(|_| "Could not check action dependencies".to_owned())
}

pub fn context(files: Vec<String>) -> Result<DragContext, String> {
    Ok(DragContext {
        platform: Platform::current().ok_or("Unsupported platform")?,
        files: files.into_iter().map(PathBuf::from).collect(),
    })
}

#[tauri::command]
pub fn list_actions(state: State<'_, Actions>) -> Vec<ActionDescriptor> {
    state.registry.list_actions()
}

#[tauri::command]
pub fn save_action(state: State<'_, Actions>, action: ActionDescriptor) -> Result<(), String> {
    state
        .registry
        .save_action(&action)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn create_action(state: State<'_, Actions>, action: ActionDescriptor) -> Result<(), String> {
    let manifest = serde_json::to_string(&action).map_err(|error| error.to_string())?;
    state
        .registry
        .import_action(&manifest)
        .map(|_| ())
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn delete_action(state: State<'_, Actions>, id: String) -> Result<(), String> {
    state
        .registry
        .delete_action(&id)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn duplicate_action(state: State<'_, Actions>, id: String) -> Result<ActionDescriptor, String> {
    let id_prefix = id
        .get(..id.len().min(100))
        .unwrap_or(&id)
        .trim_end_matches(['-', '.', '_']);
    let new_id = format!(
        "{id_prefix}.copy-{}",
        &Uuid::new_v4().simple().to_string()[..12]
    );
    state
        .registry
        .duplicate_action(&id, new_id)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn match_actions(
    state: State<'_, Actions>,
    files: Vec<String>,
) -> Result<Vec<ActionDescriptor>, String> {
    Ok(state.registry.available_actions(&context(files)?))
}

#[tauri::command]
pub fn invoke_action(
    app: AppHandle,
    state: State<'_, Actions>,
    action_id: String,
    files: Vec<String>,
    drag_session: Option<u64>,
) -> Result<String, String> {
    if let Some(session) = drag_session {
        let wheel = app.state::<windows::WheelState>();
        let wheel = wheel.lock().unwrap();
        if wheel.presentation.drag_session != Some(session) || !wheel.received_drop {
            return Err("This drag session has ended".into());
        }
    }
    let context = context(files)?;
    let action = state
        .registry
        .get_action(&action_id)
        .ok_or("Action was removed")?;
    if !action.matches(&context) {
        return Err("This action does not match the selected files".into());
    }
    let invocation = ActionInvocation { action_id, context };
    let total = match action.matching.cardinality {
        bergamot_core::Cardinality::Each => invocation.context.files.len(),
        _ => 1,
    };
    jobs::start(
        app,
        state.dispatcher.clone(),
        invocation,
        action.label,
        total,
    )
}

#[tauri::command]
pub fn open_actions_folder(app: AppHandle, state: State<'_, Actions>) -> Result<(), String> {
    app.opener()
        .open_path(state.directory.to_string_lossy().into_owned(), None::<&str>)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn present_wheel(
    app: AppHandle,
    state: State<'_, Actions>,
    files: Vec<String>,
    x: Option<f64>,
    y: Option<f64>,
) -> Result<(), String> {
    let position = match (x, y) {
        (Some(x), Some(y)) if x.is_finite() && y.is_finite() => {
            Some(PhysicalPosition::new(x, y).into())
        }
        (None, None) => None,
        _ => return Err("Provide both wheel coordinates in physical screen pixels".into()),
    };
    present_files(&app, &state, files, position, false)
}

pub fn present_files(
    app: &AppHandle,
    state: &Actions,
    files: Vec<String>,
    position: Option<Position>,
    native_drag: bool,
) -> Result<(), String> {
    let actions = state.registry.available_actions(&context(files.clone())?);
    windows::show_wheel(app, files, actions, position, native_drag)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn wheel_state(state: State<'_, windows::WheelState>) -> windows::WheelPresentation {
    state.lock().unwrap().presentation.clone()
}
