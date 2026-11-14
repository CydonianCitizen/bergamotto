use std::path::PathBuf;
use std::sync::Arc;

use bergamot_core::{
    ActionDescriptor, ActionInvocation, ActionRegistry, Dispatcher, DragContext, Platform,
};
use tauri::{AppHandle, State};
use tauri_plugin_opener::OpenerExt;
use uuid::Uuid;

use crate::{jobs, windows};

pub struct Actions {
    pub directory: PathBuf,
    pub registry: Arc<ActionRegistry>,
    pub dispatcher: Arc<Dispatcher>,
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
) -> Result<String, String> {
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
        (Some(x), Some(y)) if x.is_finite() && y.is_finite() => Some((x, y)),
        (None, None) => None,
        _ => return Err("Provide both wheel coordinates in physical screen pixels".into()),
    };
    let actions = state.registry.available_actions(&context(files.clone())?);
    windows::show_wheel(
        &app,
        windows::WheelPresentation { files, actions },
        position,
    )
    .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn wheel_state(state: State<'_, windows::WheelState>) -> windows::WheelPresentation {
    state.0.lock().unwrap().clone()
}
