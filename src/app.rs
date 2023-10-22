use eframe::egui::{self};
use log::{error, info};

use crate::collection::Collection;

use crate::components::edit_view::EditView;

use crate::views::collection_list::CollectionListView;

use crate::openapi::OpenAPI;
use crate::requests::perform_request;
use crate::views::request_list::RequestListView;

use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::{mpsc, Arc};

pub struct AppState {
    selected_request_index: usize,
    selected_collection_index: usize,
    collection_list: Vec<Collection>,
    response: String,
    tx: mpsc::Sender<String>,
    rx: Arc<Mutex<mpsc::Receiver<String>>>,
}

fn handle_import_selected_file(path: Option<PathBuf>) -> Option<Collection> {
    match path {
        Some(path_buf) => {
            let file_path = String::from(path_buf.to_str().unwrap());
            log::info!("Selected File: {}", file_path);
            let openapi = OpenAPI::load_from_yaml_file(file_path);
            let collection = Collection::from_openapi_format(openapi);
            Some(collection)
        }
        None => {
            log::error!("No file selected");
            None
        }
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Import").clicked() {
                    let res = rfd::FileDialog::new()
                        .set_directory("~")
                        .add_filter("YAML", &["yaml", "yml"])
                        .pick_file();
                    let col = handle_import_selected_file(res);
                    if let Some(collection) = col {
                        self.collection_list.push(collection);
                    }
                }
                egui::widgets::global_dark_light_mode_switch(ui);
            });
        });
        egui::SidePanel::left("collection-side-panel").show(ctx, |ui| {
            ui.set_min_width(200.0);
            if CollectionListView::new(
                &mut self.collection_list,
                &mut self.selected_collection_index,
            )
            .show(ctx, ui) {
                self.selected_request_index = 0;
            }
        });
        egui::SidePanel::left("request-side-panel").show(ctx, |ui| {
            ui.set_min_width(200.0);
            RequestListView::new(
                &mut self.collection_list[self.selected_collection_index],
                &mut self.selected_request_index,
            )
            .show(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            EditView::new(
                &mut self.collection_list[self.selected_collection_index].requests
                    [self.selected_request_index],
                self.response.clone(),
            )
            .show(ui);

            if ui.button("Send").clicked() {
                info!("Send button clicked");
                let tx = self.tx.clone();

                let req = self.collection_list[self.selected_collection_index].requests
                    [self.selected_request_index]
                    .clone();

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

        let path = String::from("./test_data/test.yaml");
        let openapi = OpenAPI::load_from_yaml_file(path);
        let collection = Collection::from_openapi_format(openapi);
        Self {
            response: String::new(),
            collection_list: vec![collection],
            selected_request_index: 0,
            selected_collection_index: 0,
            tx,
            rx: Arc::new(Mutex::new(rx)),
        }
    }
}
