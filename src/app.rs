use eframe::egui::{self, Sense};
use eframe::emath::Align2;
use eframe::epaint::FontId;
use log::{error, info};

use crate::collection::Collection;

use crate::components::dropdown_selector::DropdownSelector;
use crate::components::edit_view::EditView;
use crate::components::select_list::SelectList;

use crate::views::collection_list::CollectionListView;

use crate::openapi::OpenAPI;
use crate::requests::{perform_request, Request};
use crate::views::request_list::RequestListView;

use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::{mpsc, Arc};

pub struct AppState {
    selected_request_index: usize,
    selected_collection_index: usize,
    requests: Vec<Request>,
    collection_list: Vec<Collection>,
    response: String,
    tx: mpsc::Sender<String>,
    rx: Arc<Mutex<mpsc::Receiver<String>>>,
}

fn create_clickable_row(ui: &mut egui::Ui, value_entry: String, row_height: f32) -> bool {
    let available_width = ui.available_size().x;
    let (rect, response) =
        ui.allocate_exact_size(egui::Vec2::new(available_width, row_height), Sense::click());
    let is_hovered = response.hovered();
    let is_clicked = response.clicked();

    // Draw background if hovered
    if is_hovered {
        ui.painter()
            .rect_filled(rect, 2.0, egui::Color32::from_gray(220));
    }

    let text_color = ui.style().visuals.text_color();

    let font_id = FontId::default();

    // Draw row content
    ui.painter().text(
        egui::Pos2::new(rect.min.x + 4.0, rect.center().y),
        Align2::LEFT_CENTER,
        value_entry,
        font_id,
        if is_hovered {
            egui::Color32::from_rgb(0, 0, 0)
        } else {
            text_color
        },
    );

    // Draw border
    if is_hovered {
        ui.painter().rect_stroke(
            rect,
            2.0,
            egui::Stroke::new(1.0, egui::Color32::from_gray(180)),
        );
    }

    is_clicked
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
            CollectionListView::new(&mut self.collection_list).show(ctx, ui);
            ui.set_min_width(200.0);
        });
        egui::SidePanel::left("request-side-panel").show(ctx, |ui| {
            RequestListView::new(
                &mut self.collection_list[self.selected_collection_index].name.clone(),
                &mut self.collection_list[self.selected_collection_index].collection,
                &mut self.selected_request_index,
            ).show(ui);
            ui.set_min_width(200.0);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            EditView::new(
                &mut self.requests[self.selected_request_index],
                self.response.clone(),
            )
            .show(ui);

            if ui.button("Send").clicked() {
                info!("Send button clicked");
                let tx = self.tx.clone();

                let req = self.requests[self.selected_request_index].clone();
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
            requests: collection.collection.clone(),
            collection_list: vec![collection],
            selected_request_index: 0,
            selected_collection_index: 0,
            tx,
            rx: Arc::new(Mutex::new(rx)),
        }
    }
}
