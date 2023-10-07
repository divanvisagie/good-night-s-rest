use eframe::egui::{self, Sense};
use eframe::emath::Align2;
use eframe::epaint::FontId;
use log::{error, info};

use crate::components::edit_view::EditView;
use crate::components::select_list::SelectList;
use crate::requests::{perform_request, Request};
use std::sync::Mutex;
use std::sync::{mpsc, Arc};

type Collection = Vec<Request>;

struct CollectionItem {
    name: String,
    collection: Collection,
}

pub struct AppState {
    selected_request_index: usize,
    selected_collection_index: usize,
    collection: Collection,
    collection_list: Vec<CollectionItem>,
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
impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // spread out as much as possible
                ui.style_mut().spacing.item_spacing = egui::vec2(0.0, 0.0);
                ui.heading(r#"🌙 Good Night's Rest 🌙"#);
                egui::widgets::global_dark_light_mode_switch(ui);
            });
        });
        egui::SidePanel::left("collection-side-panel").show(ctx, |ui| {
            ui.heading("Collections");
            ui.set_min_width(200.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                SelectList::new(
                    &mut self
                        .collection_list
                        .iter_mut()
                        .map(|c| c.name.clone())
                        .collect(),
                    |ui, i, item| {
                        let text = format!("{} {}", i, item);
                        if create_clickable_row(ui, text.clone(), 45.0) {
                            println!("Clicked row: {}", text);
                            //save the state of the current collection
                            self.collection_list[self.selected_collection_index].collection =
                                self.collection.clone();

                            // select new collection
                            self.selected_request_index = 0;
                            self.selected_collection_index = i;
                            self.collection = self.collection_list[i].collection.clone();
                        }
                    },
                )
                .show(ui);
                if ui.button("Add").clicked() {
                    info!("Add button clicked");
                    let collection = vec![Request::new()];
                    let collection_item = CollectionItem {
                        name: format!("Collection {}", self.collection_list.len() + 1),
                        collection: collection.clone(),
                    };
                    self.collection_list.push(collection_item);
                    self.collection = collection;
                    self.selected_request_index = 0;
                    self.response = String::new();
                }
            });
        });
        egui::SidePanel::left("request-side-panel").show(ctx, |ui| {
            let heading = format!(
                "{} Requests",
                self.collection_list[self.selected_collection_index].name,
            );
            ui.heading(heading);
            egui::ScrollArea::vertical().show(ui, |ui| {
                let mut current = self.collection[self.selected_request_index].clone();
                for (index, request) in self.collection.iter_mut().enumerate() {
                    let text = format!("{} {}", request.method, request.url);
                    if ui
                        .selectable_value(&mut current, request.clone(), text)
                        .clicked()
                    {
                        info!("Request selected: {}", request.url);
                        self.selected_request_index = index;
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
            EditView::new(
                &mut self.collection[self.selected_request_index],
                self.response.clone(),
            )
            .show(ui);

            if ui.button("Send").clicked() {
                info!("Send button clicked");
                let tx = self.tx.clone();

                let req = self.collection[self.selected_request_index].clone();
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
            selected_request_index: 0,
            selected_collection_index: 0,
            tx,
            rx: Arc::new(Mutex::new(rx)),
        }
    }
}
