use std::{sync::Arc, time::Duration};

use bergamot_core::{AppSettings, ModifierKey, SettingsStore};
use os_adapter::{DragSessionEvent, MonitorControl};
use tauri::menu::MenuItem;
use tauri::{AppHandle, DragDropEvent, Emitter, Manager, State, WindowEvent};

use crate::{commands, windows};

pub fn start(app: &AppHandle) -> std::io::Result<()> {
    let handle = app.clone();
    app.state::<MonitorControl>()
        .start(move |event, generation| {
            if matches!(event, DragSessionEvent::DragMoved { .. }) {
                // The wheel stays anchored so the user can reach a slice.
                return;
            }
            let app = handle.clone();
            let _ = handle.run_on_main_thread(move || {
                if app.state::<MonitorControl>().accepts(generation) {
                    if let Err(error) = handle_event(&app, event) {
                        eprintln!("Global Shift+Drag wheel failed: {error}");
                    }
                }
            });
        })
}

fn active_session(app: &AppHandle) -> Option<u64> {
    app.state::<windows::WheelState>()
        .lock()
        .unwrap()
        .presentation
        .drag_session
}

fn hide_after(app: &AppHandle, session: u64, delay: Duration, unless_dropped: bool) {
    let handle = app.clone();
    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(delay).await;
        let app = handle.clone();
        let _ = handle.run_on_main_thread(move || {
            if let Err(error) = windows::hide_drag_wheel(&app, session, unless_dropped) {
                eprintln!("Could not hide the drag wheel: {error}");
            }
        });
    });
}

fn handle_event(app: &AppHandle, event: DragSessionEvent) -> Result<(), String> {
    match event {
        DragSessionEvent::DragStarted {
            files,
            cursor_x,
            cursor_y,
        } => {
            let files = files
                .into_iter()
                .map(|path| path.to_string_lossy().into_owned())
                .collect();
            #[cfg(target_os = "macos")]
            let position = tauri::LogicalPosition::new(cursor_x, cursor_y).into();
            #[cfg(not(target_os = "macos"))]
            let position = tauri::PhysicalPosition::new(cursor_x, cursor_y).into();
            commands::present_files(app, &app.state(), files, Some(position), true)?;
        }
        DragSessionEvent::DragModifierReleased => {
            if let Some(session) = active_session(app) {
                windows::hide_drag_wheel(app, session, false).map_err(|error| error.to_string())?;
            }
        }
        DragSessionEvent::DragEnded => {
            if let Some(session) = active_session(app) {
                // The mouse-up poll and the incoming native drop are independent.
                hide_after(app, session, Duration::from_millis(200), true);
            }
        }
        DragSessionEvent::DragMoved { .. } => {}
    }
    Ok(())
}

pub fn on_wheel_event(app: &AppHandle, event: &WindowEvent) {
    if let WindowEvent::DragDrop(DragDropEvent::Drop { .. }) = event {
        let state = app.state::<windows::WheelState>();
        let mut state = state.lock().unwrap();
        if let Some(session) = state.presentation.drag_session {
            state.received_drop = true;
            // The frontend normally closes it after hit testing. Also handle an unready webview.
            hide_after(app, session, Duration::from_secs(2), false);
        }
    }
}

pub fn native_modifier(key: ModifierKey) -> os_adapter::ModifierKey {
    match key {
        ModifierKey::Shift => os_adapter::ModifierKey::Shift,
        ModifierKey::Ctrl => os_adapter::ModifierKey::Ctrl,
        ModifierKey::Alt => os_adapter::ModifierKey::Alt,
    }
}

pub fn menu_label(settings: &AppSettings) -> String {
    let state = if settings.global_hook_enabled {
        "Enabled"
    } else {
        "Disabled"
    };
    format!("Global {:?}+Drag Hook: {state}", settings.modifier_key)
}

pub fn update_settings(
    app: &AppHandle,
    change: impl FnOnce(&mut AppSettings),
) -> Result<(), String> {
    let settings = app
        .state::<Arc<SettingsStore>>()
        .update(change)
        .map_err(|error| error.to_string())?;
    let changed = app.state::<MonitorControl>().configure(
        settings.global_hook_enabled,
        native_modifier(settings.modifier_key),
    );
    if changed {
        if let Some(session) = active_session(app) {
            windows::hide_drag_wheel(app, session, false).map_err(|error| error.to_string())?;
        }
    }
    app.state::<MenuItem<tauri::Wry>>()
        .set_text(menu_label(&settings))
        .map_err(|error| error.to_string())?;
    app.emit("settings:changed", settings)
        .map_err(|error| error.to_string())
}

pub fn toggle(app: &AppHandle) -> Result<(), String> {
    update_settings(app, |settings| {
        settings.global_hook_enabled = !settings.global_hook_enabled
    })
}

#[tauri::command]
pub fn global_drag_enabled(control: State<'_, MonitorControl>) -> bool {
    control.enabled()
}

#[tauri::command]
pub async fn toggle_global_drag(app: AppHandle) -> Result<(), String> {
    let handle = app.clone();
    let (send, receive) = tokio::sync::oneshot::channel();
    app.run_on_main_thread(move || {
        let _ = send.send(toggle(&handle));
    })
    .map_err(|error| error.to_string())?;
    receive
        .await
        .map_err(|_| "Could not toggle global drag".to_owned())?
}

#[tauri::command]
pub fn close_drag_wheel(app: AppHandle, session: u64) -> Result<(), String> {
    let handle = app.clone();
    app.run_on_main_thread(move || {
        if let Err(error) = windows::hide_drag_wheel(&handle, session, false) {
            eprintln!("Could not hide the drag wheel: {error}");
        }
    })
    .map_err(|error| error.to_string())
}
