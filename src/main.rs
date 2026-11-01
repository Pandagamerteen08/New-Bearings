
#[path = "app/app.rs"]
mod app;

#[tauri::command]
fn get_app_info() -> String {
    format!("{} v{}", app::APP_NAME, app::APP_VERSION)
}

fn main() {
    let context = tauri::generate_context!();

    let result = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_app_info])
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
