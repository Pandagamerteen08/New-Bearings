//! New Bearings: application startup, module registration and the Tauri
//! command surface.
//!
//! Commands stay thin: they validate nothing themselves, they hand the request
//! to [`app::Session`], which owns the application state and its stored data.

#[path = "app/app.rs"]
mod app;
#[path = "learning/learning.rs"]
mod learning;
#[path = "lessons/lessons.rs"]
mod lessons;
#[path = "progress/progress.rs"]
mod progress;
#[path = "setup/setup.rs"]
mod setup;
#[path = "storage/storage.rs"]
mod storage;

use tauri::Manager;

use app::{AnswerOutcome, Session};
use learning::{LearningError, LearningSnapshot};
use setup::{SetupError, SetupSnapshot};

#[tauri::command]
fn get_app_info() -> String {
    format!("{} v{}", app::APP_NAME, app::APP_VERSION)
}

#[tauri::command]
fn get_setup_state(session: tauri::State<'_, Session>) -> SetupSnapshot {
    session.setup_snapshot()
}

#[tauri::command]
fn begin_setup(session: tauri::State<'_, Session>) -> Result<SetupSnapshot, SetupError> {
    session.begin_setup()
}

#[tauri::command]
fn return_to_welcome(session: tauri::State<'_, Session>) -> Result<SetupSnapshot, SetupError> {
    session.return_to_welcome()
}

#[tauri::command]
fn start_learning(session: tauri::State<'_, Session>) -> Result<SetupSnapshot, SetupError> {
    session.start_learning()
}

#[tauri::command]
fn get_learning_state(
    session: tauri::State<'_, Session>,
) -> Result<LearningSnapshot, LearningError> {
    session.learning_snapshot()
}

#[tauri::command]
fn open_learning_path(
    session: tauri::State<'_, Session>,
    path_id: String,
) -> Result<LearningSnapshot, LearningError> {
    session.open_path(&path_id)
}

#[tauri::command]
fn open_learning_lesson(
    session: tauri::State<'_, Session>,
    path_id: String,
    lesson_id: String,
) -> Result<LearningSnapshot, LearningError> {
    session.open_lesson(&path_id, &lesson_id)
}

#[tauri::command]
fn go_back_in_learning(
    session: tauri::State<'_, Session>,
) -> Result<LearningSnapshot, LearningError> {
    session.go_back()
}

#[tauri::command]
fn submit_lesson_answer(
    session: tauri::State<'_, Session>,
    path_id: String,
    lesson_id: String,
    check_id: String,
    option: usize,
) -> Result<AnswerOutcome, LearningError> {
    session.submit_answer(&path_id, &lesson_id, &check_id, option)
}

fn main() {
    let context = tauri::generate_context!();

    let result = tauri::Builder::default()
        .setup(|application| {
            // Learner data belongs in the platform's application data
            // directory, never beside the executable or in the source tree.
            let directory = application.path().app_data_dir()?;
            application.manage(Session::open(storage::AppStorage::new(directory)));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_app_info,
            get_setup_state,
            begin_setup,
            return_to_welcome,
            start_learning,
            get_learning_state,
            open_learning_path,
            open_learning_lesson,
            go_back_in_learning,
            submit_lesson_answer
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
