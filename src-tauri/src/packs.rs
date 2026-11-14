use std::fs;
use std::io::Write;
use std::path::Path;

use bergamot_core::{PackCollision, PackPreview};
use tauri::{AppHandle, State};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_dialog::DialogExt;

use crate::commands::Actions;

#[tauri::command]
pub async fn read_action_pack(app: AppHandle, clipboard: bool) -> Result<Option<String>, String> {
    if clipboard {
        let document = app
            .clipboard()
            .read_text()
            .map_err(|error| error.to_string())?;
        return Ok(Some(document));
    }
    tauri::async_runtime::spawn_blocking(move || {
        let selection = app
            .dialog()
            .file()
            .set_title("Import action pack")
            .add_filter("JSON", &["json"])
            .blocking_pick_file();
        let Some(selection) = selection else {
            return Ok(None);
        };
        let path = selection.into_path().map_err(|error| error.to_string())?;
        fs::read_to_string(path)
            .map(Some)
            .map_err(|error| error.to_string())
    })
    .await
    .map_err(|_| "Could not read action pack".to_owned())?
}

#[tauri::command]
pub async fn preview_action_pack(
    state: State<'_, Actions>,
    document: String,
) -> Result<PackPreview, String> {
    let registry = state.registry.clone();
    tauri::async_runtime::spawn_blocking(move || {
        registry
            .preview_pack(&document)
            .map_err(|error| error.to_string())
    })
    .await
    .map_err(|_| "Could not validate action pack".to_owned())?
}

#[tauri::command]
pub async fn import_action_pack(
    state: State<'_, Actions>,
    document: String,
    collision: PackCollision,
) -> Result<Vec<String>, String> {
    let registry = state.registry.clone();
    tauri::async_runtime::spawn_blocking(move || {
        registry
            .import_pack(&document, collision)
            .map(|actions| actions.into_iter().map(|action| action.id).collect())
            .map_err(|error| error.to_string())
    })
    .await
    .map_err(|_| "Could not import action pack".to_owned())?
}

#[tauri::command]
pub async fn export_action_pack(
    app: AppHandle,
    state: State<'_, Actions>,
    id: Option<String>,
    clipboard: bool,
) -> Result<bool, String> {
    let document = state
        .registry
        .export_pack(id.as_deref())
        .map_err(|error| error.to_string())?;
    if clipboard {
        app.clipboard()
            .write_text(document)
            .map_err(|error| error.to_string())?;
        return Ok(true);
    }
    tauri::async_runtime::spawn_blocking(move || {
        let name = id.as_deref().unwrap_or("actions");
        let selection = app
            .dialog()
            .file()
            .set_title("Export action pack")
            .add_filter("Bergamot action pack", &["json"])
            .set_file_name(format!("{name}.bergamot-pack.json"))
            .blocking_save_file();
        let Some(selection) = selection else {
            return Ok(false);
        };
        let path = selection.into_path().map_err(|error| error.to_string())?;
        write_document(&path, &document).map_err(|error| error.to_string())?;
        Ok(true)
    })
    .await
    .map_err(|_| "Could not export action pack".to_owned())?
}

fn write_document(path: &Path, document: &str) -> std::io::Result<()> {
    let parent = path
        .parent()
        .ok_or_else(|| std::io::Error::other("Choose a destination folder"))?;
    let mut file = tempfile::NamedTempFile::new_in(parent)?;
    file.write_all(document.as_bytes())?;
    file.as_file().sync_all()?;
    file.persist(path).map_err(|error| error.error)?;
    Ok(())
}
