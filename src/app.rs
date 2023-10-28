use eframe::egui::{self, Window};
use eframe::epaint::Vec2;
use log::{error, info};

use crate::collection::Collection;

use crate::components::dropdown_selector::DropdownSelector;
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
    selected_server_index: usize,
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
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::TopBottomPanel::top("top-panel").show_inside(ui, |ui| {
                egui::SidePanel::left("top-left-panel").show_inside(ui, |ui| {
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
                    });
                });
                egui::SidePanel::right("top-right-panel")
                    .resizable(false)
                    .default_width(50.0)
                    .show_inside(ui, |ui| {
                        egui::widgets::global_dark_light_mode_switch(ui);
                    });
            });
            egui::SidePanel::left("collection-side-panel")
                .resizable(true)
                .default_width(200.0)
                .width_range(200.0..=400.0)
                .show_inside(ui, |ui| {
                    CollectionListView::new(
                        &mut self.collection_list,
                        &mut self.selected_collection_index,
                        &mut self.selected_request_index,
                        &mut self.selected_server_index,
                    )
                    .show(ui)
                });
            egui::CentralPanel::default().show_inside(ui, |ui| {
                egui::TopBottomPanel::top("top-of-request-panel").show_inside(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(
                            &mut self.collection_list[self.selected_collection_index].name,
                        );
                        DropdownSelector::new(
                            self.collection_list[self.selected_collection_index]
                                .servers
                                .iter()
                                .map(|r| format!("{}", r))
                                .collect(),
                            &mut self.selected_server_index,
                        )
                        .show(ui);
                    });
                });
                egui::SidePanel::left("request-side-panel").show_inside(ui, |ui| {
                    RequestListView::new(
                        &mut self.collection_list[self.selected_collection_index],
                        &mut self.selected_request_index,
                    )
                    .show(ui);
                });
                egui::CentralPanel::default().show_inside(ui, |ui| {
                    // Get the screen rectangle for the CentralPanel
                    let screen_rect = ui.max_rect();

                    // Define window's position and size based on the CentralPanel's screen rectangle
                    // Adjust these values as needed
                    let width = screen_rect.width();
                    let window_pos =  screen_rect.min + Vec2::new(0.0, 0.0);
                    let window_size: Vec2 = Vec2::new(width, 300.0);
                                                                //
                    Window::new("Request")
                        .fixed_pos(window_pos)
                        .fixed_size(window_size)
                        .show(ctx, |ui| {
                            ui.text_edit_singleline(
                                &mut self.collection_list[self.selected_collection_index].servers
                                    [self.selected_server_index],
                            );
                            EditView::new(
                                &mut self.collection_list[self.selected_collection_index].requests
                                    [self.selected_request_index]
                            )
                            .show(ui);

                            if ui.button("Send").clicked() {
                                info!("Send button clicked");
                                let tx = self.tx.clone();

                                let req = self.collection_list[self.selected_collection_index]
                                    .requests[self.selected_request_index]
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
                    let response_window_pos = screen_rect.min + Vec2::new(0.0, 305.0);

                    Window::new("Response")
                        .fixed_pos(response_window_pos)
                        .fixed_size(window_size)
                        .show(ctx, |ui| {
                            ui.text_edit_multiline(&mut self.response);
                        });
                });
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

        let path = String::from("./test_data/test.yaml");
        let openapi = OpenAPI::load_from_yaml_file(path);
        let collection = Collection::from_openapi_format(openapi);
        Self {
            response: String::new(),
            collection_list: vec![collection],
            selected_request_index: 0,
            selected_collection_index: 0,
            selected_server_index: 0,
            tx,
            rx: Arc::new(Mutex::new(rx)),
        }
    }
}
