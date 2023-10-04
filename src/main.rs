use std::{cell::RefCell, rc::Rc, sync::mpsc};

use components::{multiline_text::MultilineTextInput, text::TextInput, header_builder::HeaderBuilder};
use eframe::egui;
use log::{error, info};
use method::Method;

use std::sync::{Arc, Mutex};

use crate::requests::perform_request;

mod components;
mod method;
mod requests;

struct AppState {
    url: Rc<RefCell<String>>,
    body: Rc<RefCell<String>>,
    method: Rc<RefCell<Method>>,
    response: String,
    tx: mpsc::Sender<String>,
    headers: Vec<components::header_builder::Header>,
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

            HeaderBuilder::new(&mut self.headers).show(ui);

            MultilineTextInput::new("body".to_string(), self.body.clone()).show(ui);

            if ui.button("Send").clicked() {
                info!("Send button clicked");
                let tx = self.tx.clone();
                let url = self.url.borrow().clone();
                let method = self.method.borrow().clone();
                let body = self.body.borrow().clone();

                tokio::spawn(async move {
                    // Your async or long-running code here
                    //perform_request(url, method, body)
                    let result = perform_request(url.as_str(), method, body.as_str()).await;

                    match result {
                        Ok(response) => {
                            info!("Response: {}", response);
                            tx.send(response).unwrap();
                        }
                        Err(e) => {
                            error!("Error: {}", e);
                        }
                    }
                });
            }
            //rest of app
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut self.response.clone())
                        .font(egui::TextStyle::Monospace) // for cursor height
                        .code_editor()
                        .desired_rows(10)
                        .lock_focus(true)
                        .desired_width(f32::INFINITY)
                );
            });
        });

        // Poll the status of the async task here and update your UI accordingly
        let rx = self.rx.lock().unwrap();
        if let Ok(message) = rx.try_recv() {
            // Process the message here, update UI, etc.
            info!("Message received: {}", message);
            self.response = message;
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            url: Rc::new(RefCell::new("https://httpbin.org/json".to_string())),
            body: Rc::new(RefCell::new(String::new())),
            method: Rc::new(RefCell::new(Method::GET)),
            headers: vec![],
            tx,
            rx: Arc::new(Mutex::new(rx)),
            response: String::new(),
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
