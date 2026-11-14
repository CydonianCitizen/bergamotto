use std::sync::Mutex;

use bergamot_core::ActionDescriptor;
use serde::Serialize;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{
    AppHandle, Emitter, LogicalPosition, LogicalSize, Manager, Monitor, PhysicalPosition, Position,
    WebviewWindow,
};

const WHEEL_RADIUS: f64 = 210.0;

#[derive(Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WheelPresentation {
    pub files: Vec<String>,
    pub actions: Vec<ActionDescriptor>,
    pub drag_session: Option<u64>,
    pub modifier_key: bergamot_core::ModifierKey,
}

#[derive(Default)]
pub struct WheelSession {
    pub presentation: WheelPresentation,
    pub received_drop: bool,
    serial: u64,
}

pub type WheelState = Mutex<WheelSession>;

fn window(app: &AppHandle, label: &str) -> tauri::Result<WebviewWindow> {
    app.get_webview_window(label)
        .ok_or(tauri::Error::WindowNotFound)
}

fn active_monitor(app: &AppHandle, position: Option<(f64, f64)>) -> Option<Monitor> {
    position
        .or_else(|| app.cursor_position().ok().map(|point| (point.x, point.y)))
        .and_then(|(x, y)| app.monitor_from_point(x, y).ok().flatten())
        .or_else(|| app.primary_monitor().ok().flatten())
}

pub fn open_studio(app: &AppHandle) -> tauri::Result<()> {
    let studio = window(app, "studio")?;
    studio.unminimize()?;
    studio.show()?;
    studio.set_focus()
}

pub fn show_wheel(
    app: &AppHandle,
    files: Vec<String>,
    actions: Vec<ActionDescriptor>,
    position: Option<Position>,
    native_drag: bool,
) -> tauri::Result<()> {
    let wheel = window(app, "wheel")?;
    match position {
        Some(Position::Logical(point)) => {
            set_logical_wheel_position(
                &wheel,
                LogicalPosition::new(point.x - WHEEL_RADIUS, point.y - WHEEL_RADIUS),
            )?;
        }
        Some(Position::Physical(point)) => {
            let scale = if cfg!(target_os = "windows") {
                app.monitor_from_point(point.x.into(), point.y.into())?
                    .map(|monitor| monitor.scale_factor())
                    .unwrap_or(wheel.scale_factor()?)
            } else {
                wheel.scale_factor()?
            };
            let radius = WHEEL_RADIUS * scale;
            wheel.set_position(PhysicalPosition::new(
                f64::from(point.x) - radius,
                f64::from(point.y) - radius,
            ))?;
        }
        None => center_wheel(app, &wheel)?,
    }
    wheel.set_focusable(!native_drag)?;
    let presentation = {
        let state = app.state::<WheelState>();
        let mut state = state.lock().unwrap();
        state.serial += 1;
        state.received_drop = false;
        state.presentation = WheelPresentation {
            files,
            actions,
            drag_session: native_drag.then_some(state.serial),
            modifier_key: app
                .state::<std::sync::Arc<bergamot_core::SettingsStore>>()
                .snapshot()
                .modifier_key,
        };
        state.presentation.clone()
    };
    app.emit_to("wheel", "wheel:present", presentation)?;
    if native_drag {
        return show_without_activation(&wheel);
    }
    wheel.show()?;
    wheel.set_focus()
}

fn center_wheel(app: &AppHandle, wheel: &WebviewWindow) -> tauri::Result<()> {
    #[cfg(target_os = "linux")]
    if os_adapter::is_wayland() {
        return wheel.center();
    }
    let Some(monitor) = active_monitor(app, None) else {
        return wheel.center();
    };
    let area = monitor.work_area();
    let radius = WHEEL_RADIUS * monitor.scale_factor();
    wheel.set_position(PhysicalPosition::new(
        f64::from(area.position.x) + f64::from(area.size.width) / 2.0 - radius,
        f64::from(area.position.y) + f64::from(area.size.height) / 2.0 - radius,
    ))
}

fn set_logical_wheel_position(
    wheel: &WebviewWindow,
    point: LogicalPosition<f64>,
) -> tauri::Result<()> {
    #[cfg(target_os = "macos")]
    {
        let window = wheel.ns_window()?.cast::<objc2_app_kit::NSWindow>();
        let height = core_graphics::display::CGDisplay::main()
            .bounds()
            .size
            .height;
        // SAFETY: Logical wheel positions come from the native monitor or CLI on the main thread.
        unsafe {
            (*window)
                .setFrameTopLeftPoint(objc2_foundation::NSPoint::new(point.x, height - point.y))
        };
        Ok(())
    }
    #[cfg(not(target_os = "macos"))]
    wheel.set_position(point)
}

fn show_without_activation(wheel: &WebviewWindow) -> tauri::Result<()> {
    #[cfg(target_os = "macos")]
    {
        let window = wheel.ns_window()?.cast::<objc2_app_kit::NSWindow>();
        // SAFETY: The wheel owns this NSWindow and the drag event runs on the main thread.
        unsafe { (*window).orderFrontRegardless() };
        Ok(())
    }
    #[cfg(not(target_os = "macos"))]
    wheel.show()
}

pub fn cursor_position(app: &AppHandle) -> Option<Position> {
    #[cfg(target_os = "macos")]
    let position = {
        let _ = app;
        let (x, y) = os_adapter::cursor_position();
        Some(LogicalPosition::new(x, y).into())
    };
    #[cfg(not(target_os = "macos"))]
    let position = {
        #[cfg(target_os = "linux")]
        if os_adapter::is_wayland() {
            return None;
        }
        app.cursor_position().ok().map(Into::into)
    };
    position
}

pub fn hide_drag_wheel(app: &AppHandle, session: u64, unless_dropped: bool) -> tauri::Result<()> {
    let state = app.state::<WheelState>();
    let mut state = state.lock().unwrap();
    if state.presentation.drag_session == Some(session) && !(unless_dropped && state.received_drop)
    {
        state.presentation = WheelPresentation::default();
        drop(state);
        window(app, "wheel")?.hide()?;
    }
    Ok(())
}

pub fn update_hud(app: &AppHandle, count: usize, reposition: bool) -> tauri::Result<()> {
    let hud = window(app, "hud")?;
    if count == 0 {
        return hud.hide();
    }
    let monitor = reposition.then(|| active_monitor(app, None)).flatten();
    let height = 100.0 * count.min(3) as f64;
    hud.set_size(LogicalSize::new(420.0, height))?;
    if let Some(monitor) = monitor {
        let scale = monitor.scale_factor();
        let area = monitor.work_area();
        hud.set_position(PhysicalPosition::new(
            f64::from(area.position.x) + (f64::from(area.size.width) - 420.0 * scale) / 2.0,
            f64::from(area.position.y) + 12.0 * scale,
        ))?;
    }
    hud.show()
}

pub fn create_tray(app: &AppHandle) -> tauri::Result<()> {
    let studio = MenuItem::with_id(app, "studio", "Open Action Studio", true, None::<&str>)?;
    let wheel = MenuItem::with_id(app, "wheel", "Test Radial Wheel", true, None::<&str>)?;
    let hook = MenuItem::with_id(
        app,
        "global-drag",
        crate::global_drag::menu_label(
            &app.state::<std::sync::Arc<bergamot_core::SettingsStore>>()
                .snapshot(),
        ),
        true,
        None::<&str>,
    )?;
    let quit = MenuItem::with_id(app, "quit", "Quit Bergamotto", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&studio, &wheel, &hook, &quit])?;
    app.manage(hook);
    let mut tray = TrayIconBuilder::with_id("bergamotto")
        .menu(&menu)
        .tooltip("Bergamotto")
        .on_menu_event(|app, event| {
            let result = match event.id().as_ref() {
                "studio" => open_studio(app),
                "wheel" => show_wheel(app, Vec::new(), Vec::new(), None, false),
                "global-drag" => crate::global_drag::toggle(app)
                    .map_err(std::io::Error::other)
                    .map_err(Into::into),
                "quit" => {
                    app.state::<crate::jobs::Jobs>().quit(app);
                    return;
                }
                _ => return,
            };
            if let Err(error) = result {
                eprintln!("Tray action failed: {error}");
            }
        });
    if let Some(icon) = app.default_window_icon() {
        tray = tray.icon(icon.clone());
    }
    tray.build(app)?;
    Ok(())
}
