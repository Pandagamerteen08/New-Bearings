#[path = "app/app.rs"]
mod app;
#[path = "setup/setup.rs"]
mod setup;

use setup::{SetupError, SetupSnapshot, SetupStore};

#[tauri::command]
fn get_app_info() -> String {
    format!("{} v{}", app::APP_NAME, app::APP_VERSION)
}

#[tauri::command]
fn get_setup_state(setup: tauri::State<'_, SetupStore>) -> SetupSnapshot {
    setup.snapshot()
}

#[tauri::command]
fn begin_setup(setup: tauri::State<'_, SetupStore>) -> Result<SetupSnapshot, SetupError> {
    setup.begin()
}

#[tauri::command]
fn return_to_welcome(setup: tauri::State<'_, SetupStore>) -> Result<SetupSnapshot, SetupError> {
    setup.return_to_welcome()
}

fn main() {
    let context = tauri::generate_context!();

    let result = tauri::Builder::default()
        .manage(SetupStore::new())
        .invoke_handler(tauri::generate_handler![
            get_app_info,
            get_setup_state,
            begin_setup,
            return_to_welcome
        ])
        .run(context);

    if let Err(error) = result {
        eprintln!(
            "{} v{} could not start: {error}",
            app::APP_NAME,
            app::APP_VERSION
        );

        std::process::exit(1);
    }
}
