use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use bergamot_core::descriptor::Platform;
use bergamot_core::executors::process::ToolSource;
use bergamot_core::managed_tools::manifest::ManagedToolsManifest;
use bergamot_core::{
    ActionDescriptor, ActionInvocation, ActionRegistry, AppSettings, CancellationToken, Dispatcher,
    DragContext, ExecutorConfig, KeyringSecretStore, ModifierKey, SecretError, SecretStore,
    SettingsStore,
};
use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, PhysicalPosition, Position, State};
use tauri_plugin_opener::OpenerExt;
use uuid::Uuid;

use crate::{jobs, windows};

#[derive(Default)]
pub struct DownloadRegistry(pub Mutex<HashMap<String, CancellationToken>>);

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
    global_shortcut: Option<String>,
    custom_paths: Vec<String>,
    tool_overrides: HashMap<String, String>,
) -> Result<(), String> {
    let handle = app.clone();
    let shortcut_to_apply = global_shortcut.clone();
    let (send, receive) = tokio::sync::oneshot::channel();
    app.run_on_main_thread(move || {
        let mut shortcut_err = None;
        if let Some(ref sc) = shortcut_to_apply {
            if let Err(err) = crate::register_shortcut(&handle, sc) {
                shortcut_err = Some(err);
            }
        }
        if let Some(err) = shortcut_err {
            let _ = send.send(Err(err));
            return;
        }
        let result = crate::global_drag::update_settings(&handle, |settings| {
            settings.modifier_key = modifier_key;
            if let Some(sc) = shortcut_to_apply {
                settings.global_shortcut = sc;
            }
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

#[tauri::command]
pub fn test_global_shortcut(app: AppHandle, shortcut: String) -> Result<(), String> {
    crate::register_shortcut(&app, &shortcut)
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolStatus {
    pub tool: String,
    pub path: Option<PathBuf>,
    pub message: Option<String>,
    pub source: ToolSource,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedToolInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub status: ToolSource,
    pub version: Option<String>,
    pub installed_path: Option<String>,
    pub license_name: String,
    pub license_url: String,
    pub is_copyleft: bool,
    pub source_url: String,
    pub estimated_size_bytes: u64,
    pub installed_size_bytes: Option<u64>,
    pub supported_on_current_platform: bool,
    pub manual_instructions: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolDownloadProgress {
    pub tool: String,
    pub downloaded_bytes: u64,
    pub total_bytes: Option<u64>,
    pub stage: String,
}

#[tauri::command]
pub async fn check_action_dependencies(
    actions: State<'_, Actions>,
    settings: State<'_, Arc<SettingsStore>>,
) -> Result<BTreeMap<String, ToolStatus>, String> {
    let actions = actions.registry.list_actions();
    let settings = settings.snapshot();
    let mut statuses = BTreeMap::new();
    for action in actions {
        let tool = match &action.executor {
            ExecutorConfig::Process { program, .. } => program.clone(),
            ExecutorConfig::Script { interpreter, .. } => interpreter.clone(),
            ExecutorConfig::Webhook { .. } => continue,
        };
        let cancel = CancellationToken::new();
        let (path, source, message) =
            match bergamot_core::executors::process::resolve_dependency_detailed(
                &action.executor,
                &settings,
                &cancel,
            )
            .await
            {
                Ok(Some(dep)) => (Some(dep.path), dep.source, None),
                Ok(None) => (None, ToolSource::System, None),
                Err(error) => (None, ToolSource::Missing, Some(error.to_string())),
            };
        statuses.insert(
            action.id,
            ToolStatus {
                tool,
                path,
                message,
                source,
            },
        );
    }
    Ok(statuses)
}

fn dir_size(path: &std::path::Path) -> u64 {
    let mut total = 0;
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(meta) = entry.metadata() {
                if meta.is_dir() {
                    total += dir_size(&entry.path());
                } else {
                    total += meta.len();
                }
            }
        }
    }
    total
}

#[tauri::command]
pub async fn get_managed_tools_status(
    settings: State<'_, Arc<SettingsStore>>,
) -> Result<Vec<ManagedToolInfo>, String> {
    let settings = settings.snapshot();
    let manifest = ManagedToolsManifest::load_embedded()
        .map_err(|e| format!("Could not load managed tools manifest: {e}"))?;

    let platform_key = ManagedToolsManifest::current_platform_key();
    let tools_dir = settings.managed_tools_dir();

    let mut result = Vec::new();
    for (tool_id, tool_def) in &manifest.tools {
        let asset = platform_key.and_then(|pk| tool_def.platforms.get(pk));
        let supported = asset.is_some();
        let manual_instructions = manifest.get_manual_instruction(tool_id).map(str::to_owned);

        let mut status = ToolSource::Missing;
        let mut installed_path: Option<String> = None;
        let mut version: Option<String> = None;
        let mut installed_size_bytes: Option<u64> = None;

        // Check if installed as managed tool
        if let (Some(ref tools_dir), Some(asset)) = (&tools_dir, asset) {
            let target_dir = tools_dir.join(tool_id).join(&asset.version);
            let target_bin = target_dir.join(&asset.binary_path);
            if target_bin.exists() {
                status = ToolSource::Managed;
                installed_path = Some(target_bin.to_string_lossy().into_owned());
                version = Some(asset.version.clone());
                installed_size_bytes = Some(dir_size(&target_dir));
            }
        }

        // If not managed, check if available on system
        if status == ToolSource::Missing {
            let executor = ExecutorConfig::Process {
                program: tool_id.clone(),
                args: vec![],
                working_directory: None,
                env: std::collections::BTreeMap::new(),
                timeout_seconds: 30,
            };
            let mut sys_settings = settings.clone();
            sys_settings.managed_tools_dir = None;
            let cancel = CancellationToken::new();
            if let Ok(Some(dep)) = bergamot_core::executors::process::resolve_dependency_detailed(
                &executor,
                &sys_settings,
                &cancel,
            )
            .await
            {
                status = ToolSource::System;
                installed_path = Some(dep.path.to_string_lossy().into_owned());
            }
        }

        result.push(ManagedToolInfo {
            id: tool_id.clone(),
            name: tool_def.name.clone(),
            description: tool_def.description.clone(),
            status,
            version,
            installed_path,
            license_name: tool_def.license.name.clone(),
            license_url: tool_def.license.url.clone(),
            is_copyleft: tool_def.license.is_copyleft,
            source_url: tool_def.license.source_url.clone(),
            estimated_size_bytes: asset.map(|a| a.size_bytes).unwrap_or(0),
            installed_size_bytes,
            supported_on_current_platform: supported,
            manual_instructions,
        });
    }

    Ok(result)
}

#[tauri::command]
pub async fn install_managed_tool(
    app: AppHandle,
    settings: State<'_, Arc<SettingsStore>>,
    registry: State<'_, DownloadRegistry>,
    tool: String,
) -> Result<ManagedToolInfo, String> {
    let manifest = ManagedToolsManifest::load_embedded()
        .map_err(|e| format!("Could not load manifest: {e}"))?;

    let snapshot = settings.snapshot();
    let tools_dir = snapshot
        .managed_tools_dir()
        .ok_or_else(|| "Could not determine local tools directory".to_string())?;

    let cancel = CancellationToken::new();
    {
        let mut map = registry.0.lock().map_err(|_| "Lock poisoned")?;
        map.insert(tool.clone(), cancel.clone());
    }

    let tool_name_for_event = tool.clone();
    let app_handle = app.clone();
    let on_progress = move |downloaded: u64, total: Option<u64>| {
        let _ = app_handle.emit(
            "managed-tool:progress",
            ToolDownloadProgress {
                tool: tool_name_for_event.clone(),
                downloaded_bytes: downloaded,
                total_bytes: total,
                stage: "downloading".to_string(),
            },
        );
    };

    let install_res = bergamot_core::managed_tools::install_tool(
        &manifest,
        &tool,
        &tools_dir,
        on_progress,
        &cancel,
    )
    .await;

    if let Ok(mut map) = registry.0.lock() {
        map.remove(&tool);
    }

    match install_res {
        Ok(_) => {
            let _ = app.emit(
                "managed-tool:progress",
                ToolDownloadProgress {
                    tool: tool.clone(),
                    downloaded_bytes: 0,
                    total_bytes: None,
                    stage: "completed".to_string(),
                },
            );
            let _ = app.emit("actions:updated", ());
            let list = get_managed_tools_status(settings).await?;
            list.into_iter()
                .find(|t| t.id == tool)
                .ok_or_else(|| "Tool status not found".to_string())
        }
        Err(e) => {
            let _ = app.emit(
                "managed-tool:progress",
                ToolDownloadProgress {
                    tool: tool.clone(),
                    downloaded_bytes: 0,
                    total_bytes: None,
                    stage: "failed".to_string(),
                },
            );
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub async fn uninstall_managed_tool(
    app: AppHandle,
    settings: State<'_, Arc<SettingsStore>>,
    tool: String,
) -> Result<ManagedToolInfo, String> {
    let manifest = ManagedToolsManifest::load_embedded()
        .map_err(|e| format!("Could not load managed tools manifest: {e}"))?;
    validate_managed_tool_id(&manifest, &tool)?;

    let snapshot = settings.snapshot();
    let tools_dir = snapshot
        .managed_tools_dir()
        .ok_or_else(|| "Could not determine local tools directory".to_string())?;

    bergamot_core::managed_tools::uninstall_tool(&tools_dir, &tool)
        .map_err(|e| format!("Failed to uninstall tool: {e}"))?;

    let _ = app.emit("actions:updated", ());
    let list = get_managed_tools_status(settings).await?;
    list.into_iter()
        .find(|t| t.id == tool)
        .ok_or_else(|| "Tool status not found".to_string())
}

fn validate_managed_tool_id(manifest: &ManagedToolsManifest, tool: &str) -> Result<(), String> {
    if manifest.tools.contains_key(tool) {
        Ok(())
    } else {
        Err(format!(
            "Tool '{tool}' is not defined in the managed tools manifest"
        ))
    }
}

#[tauri::command]
pub fn cancel_managed_tool_download(
    registry: State<'_, DownloadRegistry>,
    tool: String,
) -> Result<(), String> {
    let map = registry.0.lock().map_err(|_| "Lock poisoned")?;
    if let Some(token) = map.get(&tool) {
        token.cancel();
    }
    Ok(())
}

pub fn context(files: Vec<String>) -> Result<DragContext, String> {
    Ok(DragContext {
        platform: Platform::current().ok_or("Unsupported platform")?,
        files: files.into_iter().map(PathBuf::from).collect(),
    })
}

#[cfg(test)]
mod managed_tool_uninstall_tests {
    use super::{validate_managed_tool_id, ManagedToolsManifest};

    #[test]
    fn uninstall_guard_rejects_ids_missing_from_embedded_manifest() {
        let manifest = ManagedToolsManifest::load_embedded().unwrap();

        assert!(validate_managed_tool_id(&manifest, "ffmpeg").is_ok());
        assert!(validate_managed_tool_id(&manifest, "../outside").is_err());
    }
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
pub fn reorder_actions(state: State<'_, Actions>, ordered_ids: Vec<String>) -> Result<(), String> {
    state
        .registry
        .reorder_actions(&ordered_ids)
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
pub fn open_third_party_licenses(app: AppHandle) -> Result<(), String> {
    let path = app
        .path()
        .resolve(
            "THIRD_PARTY_LICENSES.txt",
            tauri::path::BaseDirectory::Resource,
        )
        .map_err(|error| format!("Could not resolve third-party license notices: {error}"))?;
    if !path.is_file() {
        return Err("The third-party license notice file is missing from this installation".into());
    }
    app.opener()
        .open_path(path.to_string_lossy().into_owned(), None::<&str>)
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
