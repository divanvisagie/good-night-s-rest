use eframe::egui;
use log::{error, info};

use crate::components::edit_view::EditView;
use crate::requests::{perform_request, Request};
use std::sync::Mutex;
use std::sync::{mpsc, Arc};

pub struct AppState {
    response: String,
    tx: mpsc::Sender<String>,
    request: Request,
    rx: Arc<Mutex<mpsc::Receiver<String>>>,
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("side-panel").show(ctx, |ui| {
            egui::widgets::global_dark_light_mode_switch(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            EditView::new(&mut self.request, self.response.clone()).show(ui);

            if ui.button("Send").clicked() {
                info!("Send button clicked");
                let tx = self.tx.clone();

                let req = self.request.clone();
                tokio::spawn(async move {
                    // Your async or long-running code here
                    //perform_request(url, method, body)
                    let result = perform_request(req).await;

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
            request: Request::new(),
            response: String::new(),
            tx,
            rx: Arc::new(Mutex::new(rx)),
        }
    }
}
