use std::{cell::RefCell, rc::Rc};

use components::text::TextInput;
use eframe::egui;
use log::{debug, error, log_enabled, info, Level};
mod components;

struct AppState {
    url: Rc<RefCell<String>>
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("side-panel").show(ctx, |ui| {
            egui::widgets::global_dark_light_mode_switch(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            TextInput::new("URL:".to_string(), self.url.clone()).show(ui);
        });
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            url: Rc::new(RefCell::new(String::new()))
        }
    }
}

fn main() {
    env_logger::init();
    info!(">> start the app");
    let mut window_options = eframe::NativeOptions::default();
    window_options.initial_window_size = Some(egui::Vec2::new(1280., 768.));
    window_options.resizable = true;
    window_options.decorated = true;

    match eframe::run_native(
        "Arcanaeum",
        window_options,
        Box::new(|_cc| Box::<AppState>::default()),
    ) {
        Ok(_) => {}
        Err(e) => {
            error!("There was an error while trying to initialise the window: {}", e);
        }
    }
}
