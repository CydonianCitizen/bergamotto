use std::sync::Mutex;

use bergamot_core::ActionDescriptor;
use serde::Serialize;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{AppHandle, Emitter, LogicalSize, Manager, Monitor, PhysicalPosition, WebviewWindow};

#[derive(Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WheelPresentation {
    pub files: Vec<String>,
    pub actions: Vec<ActionDescriptor>,
}

#[derive(Default)]
pub struct WheelState(pub Mutex<WheelPresentation>);

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
    presentation: WheelPresentation,
    position: Option<(f64, f64)>,
) -> tauri::Result<()> {
    let wheel = window(app, "wheel")?;
    if let Some(monitor) = active_monitor(app, position) {
        let area = monitor.work_area();
        let scale = monitor.scale_factor();
        let size = 420.0 * scale;
        let left = f64::from(area.position.x);
        let top = f64::from(area.position.y);
        let width = f64::from(area.size.width);
        let height = f64::from(area.size.height);
        let (x, y) = position.unwrap_or((left + width / 2.0, top + height / 2.0));
        wheel.set_position(PhysicalPosition::new(x - size / 2.0, y - size / 2.0))?;
    }
    *app.state::<WheelState>().0.lock().unwrap() = presentation.clone();
    app.emit_to("wheel", "wheel:present", presentation)?;
    wheel.show()?;
    wheel.set_focus()
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
    let quit = MenuItem::with_id(app, "quit", "Quit Bergamotto", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&studio, &wheel, &quit])?;
    let mut tray = TrayIconBuilder::with_id("bergamotto")
        .menu(&menu)
        .tooltip("Bergamotto")
        .on_menu_event(|app, event| {
            let result = match event.id().as_ref() {
                "studio" => open_studio(app),
                "wheel" => show_wheel(app, WheelPresentation::default(), None),
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
