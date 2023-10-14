use log::error;
use app::AppState;
use eframe::egui;

mod components;
mod method;
mod requests;
mod app;
mod collection;
mod openapi;

#[tokio::main]
async fn main() {
    env_logger::init();
    let mut window_options = eframe::NativeOptions::default();
    window_options.initial_window_size = Some(egui::Vec2::new(1280., 768.));
    window_options.resizable = true;
    window_options.decorated = true;

    match eframe::run_native(
        "🌙 Good Night's Rest 🌙",
        window_options,
        Box::new(|_cc| Box::<AppState>::default()),
    ) {
        Ok(_) => {}
        Err(e) => {
            error!(
                "There was an error while trying to initialise the window: {}",
                e
            );
        }
    }
}
