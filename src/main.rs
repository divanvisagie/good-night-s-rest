use std::{cell::RefCell, rc::Rc, sync::mpsc};

use components::{multiline_text::MultilineTextInput, text::TextInput};
use eframe::egui;
use log::{error, info};
use method::Method;

use std::sync::{Arc, Mutex};

mod components;
mod method;
mod requests;

struct AppState {
    url: Rc<RefCell<String>>,
    body: Rc<RefCell<String>>,
    method: Rc<RefCell<Method>>,
    tx: mpsc::Sender<String>,
    rx: Arc<Mutex<mpsc::Receiver<String>>>,
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("side-panel").show(ctx, |ui| {
            egui::widgets::global_dark_light_mode_switch(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ComboBox::from_label("Select one!")
                .selected_text(format!("{:?}", &*self.method.borrow()))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut *self.method.borrow_mut(), Method::GET, "GET");
                    ui.selectable_value(&mut *self.method.borrow_mut(), Method::POST, "POST");
                    ui.selectable_value(&mut *self.method.borrow_mut(), Method::PUT, "PUT");
                    ui.selectable_value(&mut *self.method.borrow_mut(), Method::DELETE, "DELETE");
                });

            TextInput::new("URL:".to_string(), self.url.clone()).show(ui);

            ui.label(&*self.url.borrow());

            MultilineTextInput::new("body".to_string(), self.body.clone()).show(ui);

            if ui.button("Send").clicked() {
                info!("Send button clicked");
                let tx = self.tx.clone();
                std::thread::spawn(move || {
                    // Your async or long-running code here
                    let message = "Task completed".to_string(); // Replace with actual message
                    tx.send(message).unwrap();
                });
            }
        });
        
        // Poll the status of the async task here and update your UI accordingly
        let rx = self.rx.lock().unwrap();
        if let Ok(message) = rx.try_recv() {
            // Process the message here, update UI, etc.
            info!("Message received: {}", message);
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            url: Rc::new(RefCell::new(String::new())),
            body: Rc::new(RefCell::new(String::new())),
            method: Rc::new(RefCell::new(Method::GET)),
            tx,
            rx: Arc::new(Mutex::new(rx)),
        }
    }
}

#[tokio::main]
async fn main() {
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
