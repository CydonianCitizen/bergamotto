mod commands;
mod jobs;
mod windows;

use std::path::{Path, PathBuf};
use std::sync::Arc;

use bergamot_core::{ActionRegistry, Dispatcher};
use directories::BaseDirs;
use tauri::{AppHandle, Emitter, Manager, WindowEvent};

use commands::Actions;

fn radial_files(args: impl IntoIterator<Item = String>, cwd: &Path) -> Option<Vec<PathBuf>> {
    let mut arguments = args.into_iter().skip(1);
    arguments.find(|arg| arg == "--radial-here")?;
    Some(arguments.map(|arg| cwd.join(arg)).collect())
}

fn activate(
    app: &AppHandle,
    args: impl IntoIterator<Item = String>,
    cwd: &Path,
) -> Result<(), String> {
    if let Some(paths) = radial_files(args, cwd) {
        let files: Vec<_> = paths
            .iter()
            .map(|path| path.to_string_lossy().into_owned())
            .collect();
        commands::present_wheel(app.clone(), app.state(), files, None, None)
    } else {
        windows::open_studio(app).map_err(|error| error.to_string())
    }
}

pub fn run() {
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
        .manage(windows::WheelState::default())
        .manage(jobs::Jobs::default())
        .setup(|app| {
            let dirs =
                BaseDirs::new().ok_or_else(|| std::io::Error::other("No user config directory"))?;
            let name = if cfg!(target_os = "linux") {
                "bergamot"
            } else {
                "Bergamot"
            };
            let directory = dirs.config_dir().join(name).join("actions");
            let handle = app.handle().clone();
            let registry = Arc::new(ActionRegistry::open_with_reload_handler(
                &directory,
                move || {
                    let _ = handle.emit("actions:updated", ());
                },
            )?);
            let dispatcher = Arc::new(Dispatcher::new(registry.clone())?);
            app.manage(Actions {
                directory,
                registry,
                dispatcher,
            });

            windows::create_tray(app.handle())?;
            let args = std::env::args_os().map(|arg| arg.to_string_lossy().into_owned());
            activate(app.handle(), args, &std::env::current_dir()?)
                .map_err(std::io::Error::other)?;
            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::list_actions,
            commands::save_action,
            commands::create_action,
            commands::delete_action,
            commands::duplicate_action,
            commands::match_actions,
            commands::invoke_action,
            commands::open_actions_folder,
            commands::present_wheel,
            commands::wheel_state,
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
        let args = ["bergamot", "--radial-here", "a b.txt", "-second.txt"];
        assert_eq!(
            radial_files(args.map(str::to_owned), &cwd),
            Some(vec![cwd.join("a b.txt"), cwd.join("-second.txt")])
        );
        assert_eq!(radial_files(["bergamot".to_owned()], &cwd), None);
    }
}
