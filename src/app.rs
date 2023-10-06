use eframe::egui;
use log::{error, info};

use crate::components::edit_view::EditView;
use crate::requests::{perform_request, Request};
use std::sync::Mutex;
use std::sync::{mpsc, Arc};

pub struct AppState {
    selected_index: usize,
    collection: Vec<Request>,
    request: Request,
    response: String,
    tx: mpsc::Sender<String>,
    rx: Arc<Mutex<mpsc::Receiver<String>>>,
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("side-panel").show(ctx, |ui| {
            egui::widgets::global_dark_light_mode_switch(ui);
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (index, request) in self.collection.iter_mut().enumerate() {
                    if ui
                        .selectable_value(&mut self.request, request.clone(), request.url.clone())
                        .clicked()
                    {
                        info!("Request selected: {}", request.url);
                        self.selected_index = index;
                    }
                }
                if ui.button("Add").clicked() {
                    info!("Add button clicked");
                    self.collection.push(Request::new());
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            EditView::new(&mut self.collection[self.selected_index], self.response.clone()).show(ui);

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
            selected_index: 0,
            collection: vec![Request::new()],
            tx,
            rx: Arc::new(Mutex::new(rx)),
        }
    }
}
