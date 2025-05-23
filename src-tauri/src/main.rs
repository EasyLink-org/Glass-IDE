// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod menu;

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg_attr(
                not(any(target_os = "macos", target_os = "windows")),
                allow(unused_variables)
            )]
            let window = match app.get_webview_window("main") {
                Some(win) => win,
                None => {
                    eprintln!(
                        "Error: Could not get main window. Visual effects will not be applied."
                    );
                    // Exit setup gracefully, allowing the app to continue running without these effects.
                    return Ok(());
                }
            };

            #[cfg(target_os = "macos")]
            {
                use window_vibrancy::{NSVisualEffectMaterial, NSVisualEffectState};
                if let Err(err) = window_vibrancy::apply_vibrancy(
                    &window,
                    NSVisualEffectMaterial::FullScreenUI,
                    Some(NSVisualEffectState::Active),
                    Some(8.0),
                ) {
                    eprintln!("Couldn't apply vibrancy effect: {}", err);
                }
            }

            #[cfg(target_os = "windows")]
            {
                if let Err(err) = window_vibrancy::apply_blur(&window, Some((18, 18, 18, 125))) {
                    eprintln!("Couldn't apply blur effect: {}", err);
                }
            }

            // Build native menu via helper module
            menu::attach(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application");
}
