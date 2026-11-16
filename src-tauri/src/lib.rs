mod commands;
mod global_drag;
mod jobs;
mod packs;
mod windows;

use std::path::{Path, PathBuf};
use std::sync::Arc;

use bergamot_core::{ActionRegistry, Dispatcher, SettingsStore};
use directories::BaseDirs;
use tauri::{AppHandle, Emitter, Manager, WindowEvent};

use commands::Actions;

fn clean_file_path(arg: &str, cwd: &Path) -> PathBuf {
    #[allow(unused_mut)]
    let mut clean = arg.strip_prefix("file://").unwrap_or(arg);
    #[cfg(windows)]
    {
        if clean.starts_with('/') && clean.len() >= 3 && clean.as_bytes()[2] == b':' {
            clean = &clean[1..];
        }
    }
    let path = Path::new(clean);
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        cwd.join(path)
    }
}

fn radial_files(args: impl IntoIterator<Item = String>, cwd: &Path) -> Option<Vec<PathBuf>> {
    let mut arguments = args.into_iter().skip(1);
    arguments.find(|arg| arg == "--radial-here")?;
    Some(arguments.map(|arg| clean_file_path(&arg, cwd)).collect())
}

fn clipboard_files(app: &AppHandle) -> Option<Vec<PathBuf>> {
    use tauri_plugin_clipboard_manager::ClipboardExt;
    let text = app.clipboard().read_text().ok()?;
    let paths: Vec<PathBuf> = text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| {
            let clean = line.strip_prefix("file://").unwrap_or(line);
            PathBuf::from(clean)
        })
        .filter(|path| path.exists())
        .collect();
    (!paths.is_empty()).then_some(paths)
}

pub(crate) fn register_shortcut(app: &AppHandle, shortcut_str: &str) -> Result<(), String> {
    use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

    let global_shortcut = app.global_shortcut();
    let _ = global_shortcut.unregister_all();

    let trimmed = shortcut_str.trim();
    if trimmed.is_empty() {
        return Ok(());
    }

    let handle = app.clone();
    global_shortcut
        .on_shortcut(trimmed, move |_app, _shortcut, event| {
            if event.state == ShortcutState::Pressed {
                let app_main = handle.clone();
                let _ = handle.run_on_main_thread(move || {
                    let files = clipboard_files(&app_main).unwrap_or_default();
                    let file_strings: Vec<String> = files
                        .iter()
                        .map(|path| path.to_string_lossy().into_owned())
                        .collect();
                    let _ = commands::present_files(
                        &app_main,
                        &app_main.state(),
                        file_strings,
                        windows::cursor_position(&app_main),
                        false,
                    );
                });
            }
        })
        .map_err(|error| {
            let msg = error.to_string();
            if msg.to_lowercase().contains("already")
                || msg.to_lowercase().contains("conflict")
                || msg.to_lowercase().contains("in use")
            {
                format!("Conflict detected: '{trimmed}' is already in use by the OS or another application.")
            } else {
                format!("Failed to register shortcut '{trimmed}': {msg}")
            }
        })
}

fn activate(
    app: &AppHandle,
    args: impl IntoIterator<Item = String>,
    cwd: &Path,
) -> Result<(), String> {
    let raw_args: Vec<String> = args.into_iter().collect();
    let is_clipboard = raw_args.iter().any(|arg| arg == "--radial-clipboard");
    let is_radial_here = raw_args.iter().any(|arg| arg == "--radial-here");

    let paths = if is_clipboard {
        clipboard_files(app).unwrap_or_default()
    } else if is_radial_here {
        let explicit = radial_files(raw_args, cwd).unwrap_or_default();
        if explicit.is_empty() {
            clipboard_files(app).unwrap_or_default()
        } else {
            explicit
        }
    } else {
        return windows::open_studio(app).map_err(|error| error.to_string());
    };

    let files: Vec<_> = paths
        .iter()
        .map(|path| path.to_string_lossy().into_owned())
        .collect();
    commands::present_files(
        app,
        &app.state(),
        files,
        windows::cursor_position(app),
        false,
    )
}

pub fn run() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|arg| arg == "--version" || arg == "-V") {
        println!("Bergamotto {}", env!("CARGO_PKG_VERSION"));
        return;
    }
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        println!("Bergamotto {}", env!("CARGO_PKG_VERSION"));
        println!("A cross-platform radial action launcher\n");
        println!("Usage: bergamot [OPTIONS] [FILES...]\n");
        println!("Options:");
        println!("  -V, --version            Print version information");
        println!("  -h, --help               Print help information");
        println!("  --radial-here <files...> Open radial wheel with specified files");
        println!("  --radial-clipboard       Open radial wheel with files from clipboard (ideal for Wayland hotkeys)");
        return;
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, args, cwd| {
            let app = app.clone();
            let handle = app.clone();
            let _ = handle.run_on_main_thread(move || {
                if let Err(error) = activate(&app, args, Path::new(&cwd)) {
                    eprintln!("Could not activate Bergamotto: {error}");
                }
            });
        }))
        .plugin(tauri_plugin_dialog::init())
        .plugin(
            tauri_plugin_opener::Builder::new()
                .open_js_links_on_click(false)
                .build(),
        )
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .manage(windows::WheelState::default())
        .manage(os_adapter::MonitorControl::default())
        .manage(jobs::Jobs::default())
        .manage(commands::DownloadRegistry::default())
        .setup(|app| {
            let dirs =
                BaseDirs::new().ok_or_else(|| std::io::Error::other("No user config directory"))?;
            let name = if cfg!(target_os = "linux") {
                "bergamot"
            } else {
                "Bergamot"
            };
            let config = dirs.config_dir().join(name);
            let settings = Arc::new(SettingsStore::open(config.join("settings.json"))?);
            let initial = settings.snapshot();
            app.manage(settings.clone());
            app.state::<os_adapter::MonitorControl>().configure(
                initial.global_hook_enabled,
                global_drag::native_modifier(initial.modifier_key),
            );
            let directory = config.join("actions");
            let handle = app.handle().clone();
            let registry = Arc::new(ActionRegistry::open_with_reload_handler(
                &directory,
                move || {
                    let _ = handle.emit("actions:updated", ());
                },
            )?);
            let dispatcher = Arc::new(Dispatcher::new(registry.clone())?.with_settings(settings));
            app.manage(Actions {
                directory,
                registry,
                dispatcher,
            });

            windows::create_tray(app.handle())?;
            global_drag::start(app.handle())?;
            let initial_shortcut = initial.global_shortcut.clone();
            if !initial_shortcut.is_empty() {
                if let Err(error) = register_shortcut(app.handle(), &initial_shortcut) {
                    eprintln!(
                        "Notice: Could not bind global shortcut '{initial_shortcut}': {error}"
                    );
                }
            }
            let args = std::env::args_os().map(|arg| arg.to_string_lossy().into_owned());
            activate(app.handle(), args, &std::env::current_dir()?)
                .map_err(std::io::Error::other)?;
            Ok(())
        })
        .on_window_event(|window, event| {
            if window.label() == "wheel" {
                global_drag::on_wheel_event(window.app_handle(), event);
            }
            if let WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::list_actions,
            commands::save_action,
            commands::reorder_actions,
            commands::create_action,
            commands::delete_action,
            commands::duplicate_action,
            commands::set_secret,
            commands::has_secret,
            commands::delete_secret,
            commands::get_settings,
            commands::save_settings,
            commands::test_global_shortcut,
            commands::check_action_dependencies,
            commands::get_managed_tools_status,
            commands::install_managed_tool,
            commands::uninstall_managed_tool,
            commands::cancel_managed_tool_download,
            packs::read_action_pack,
            packs::preview_action_pack,
            packs::import_action_pack,
            packs::export_action_pack,
            commands::match_actions,
            commands::invoke_action,
            commands::open_actions_folder,
            commands::open_third_party_licenses,
            commands::present_wheel,
            commands::wheel_state,
            global_drag::global_drag_enabled,
            global_drag::toggle_global_drag,
            global_drag::close_drag_wheel,
            jobs::cancel_job,
            jobs::dismiss_job,
            jobs::job_snapshot
        ])
        .run(tauri::generate_context!())
        .expect("Bergamotto could not start");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn radial_arguments_preserve_spaces_and_resolve_relative_paths() {
        let cwd = std::env::current_dir().unwrap();
        let sample_abs = if cfg!(windows) {
            PathBuf::from("C:\\tmp\\third.txt")
        } else {
            PathBuf::from("/tmp/third.txt")
        };
        let file_uri = if cfg!(windows) {
            "file:///C:/tmp/third.txt".to_string()
        } else {
            "file:///tmp/third.txt".to_string()
        };
        let args = [
            "bergamot".to_string(),
            "--radial-here".to_string(),
            "a b.txt".to_string(),
            "-second.txt".to_string(),
            file_uri,
        ];
        assert_eq!(
            radial_files(args, &cwd),
            Some(vec![
                cwd.join("a b.txt"),
                cwd.join("-second.txt"),
                sample_abs,
            ])
        );
        assert_eq!(radial_files(["bergamot".to_owned()], &cwd), None);
    }
}
