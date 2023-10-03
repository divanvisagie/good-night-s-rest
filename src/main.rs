use std::{cell::RefCell, rc::Rc};

use components::{multiline_text::MultilineTextInput, text::TextInput};
use eframe::egui;
use log::{info, error};
mod components;

struct AppState {
    url: Rc<RefCell<String>>,
    body: Rc<RefCell<String>>,
    method: Rc<RefCell<Method>>,
}

#[derive(Debug, PartialEq)]
enum Method {
    GET,
    POST,
    PUT,
    DELETE,
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("side-panel").show(ctx, |ui| {
            egui::widgets::global_dark_light_mode_switch(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ComboBox::from_label("Select one!")
                .selected_text(format!("{:?}", & *self.method.borrow()))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut *self.method.borrow_mut(), Method::GET, "GET");
                    ui.selectable_value(&mut *self.method.borrow_mut(), Method::POST, "POST");
                    ui.selectable_value(&mut *self.method.borrow_mut(), Method::PUT, "PUT");
                    ui.selectable_value(&mut *self.method.borrow_mut(), Method::DELETE, "DELETE");
                });
            TextInput::new("URL:".to_string(), self.url.clone()).show(ui);
            ui.label(&*self.url.borrow());
            MultilineTextInput::new("body".to_string(), self.body.clone()).show(ui);
           if  ui.button ("Send") .clicked() {
               info!("Send button clicked");
           }
        });
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            url: Rc::new(RefCell::new(String::new())),
            body: Rc::new(RefCell::new(String::new())),
            method: Rc::new(RefCell::new(Method::GET)),
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
        "Vivus",
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
