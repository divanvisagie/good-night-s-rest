use eframe::egui;
use log::{error, info};

use crate::components::edit_view::EditView;
use crate::requests::{perform_request, Request};
use std::sync::Mutex;
use std::sync::{mpsc, Arc};

type Collection = Vec<Request>;

struct CollectionItem {
    name: String,
    collection: Collection,
}

pub struct AppState {
    selected_index: usize,
    collection: Collection,
    collection_list: Vec<CollectionItem>,
    response: String,
    tx: mpsc::Sender<String>,
    rx: Arc<Mutex<mpsc::Receiver<String>>>,
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("first-side-panel").show(ctx, |ui| {
            ui.label("Collections");
            ui.set_min_width(200.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                for (index, collection_item) in self.collection_list.iter_mut().enumerate() {
                    if ui
                        .selectable_value(
                            &mut self.selected_index,
                            index,
                            collection_item.name.clone(),
                        )
                        .clicked()
                    {
                        info!("Collection selected: {}", collection_item.name);
                        self.collection = collection_item.collection.clone();
                        self.selected_index = 0;
                        self.response = String::new();
                    }
                }
                if ui.button("Add").clicked() {
                    info!("Add button clicked");
                    let collection = vec![Request::new()];
                    let collection_item = CollectionItem {
                        name: format!("Collection {}", self.collection_list.len() + 1),
                        collection: collection.clone(),
                    };
                    self.collection_list.push(collection_item);
                    self.collection = collection;
                    self.selected_index = 0;
                    self.response = String::new();
                }
            });
        });
        egui::SidePanel::left("second-side-panel").show(ctx, |ui| {
            ui.label("Requests");
            egui::ScrollArea::vertical().show(ui, |ui| {
                let mut current = self.collection[self.selected_index].clone();
                for (index, request) in self.collection.iter_mut().enumerate() {
                    let text = format!("{} {}", request.method, request.url);
                    if ui
                        .selectable_value(&mut current, request.clone(), text)
                        .clicked()
                    {
                        info!("Request selected: {}", request.url);
                        self.selected_index = index;
                        self.response = String::new();
                    }
                }
                if ui.button("Add").clicked() {
                    info!("Add button clicked");
                    self.collection.push(Request::new());
                }
            });
            ui.set_min_width(200.0);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::widgets::global_dark_light_mode_switch(ui);
            EditView::new(&mut self.collection[self.selected_index], self.response.clone()).show(ui);

            if ui.button("Send").clicked() {
                info!("Send button clicked");
                let tx = self.tx.clone();

                let req = self.collection[self.selected_index].clone();
                tokio::spawn(async move {
                    // Your async or long-running code here
                    // perform_request(url, method, body)
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
        let collection = vec![Request::new()];
        let collection_item = CollectionItem {
            name: "Collection 1".to_string(),
            collection: collection.clone(),
        };
        Self {
            response: String::new(),
            collection: collection.clone(),
            collection_list: vec![collection_item],
            selected_index: 0,
            tx,
            rx: Arc::new(Mutex::new(rx)),
        }
    }
}
