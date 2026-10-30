#[path = "app/app.rs"]
mod app;

fn main() {
    if let Err(error) = app::run() {
        eprintln!("Recall could not start: {error}");
        std::process::exit(1);
    }
}
