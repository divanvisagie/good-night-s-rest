use eframe::egui::{self, Sense, Ui};
use eframe::emath::Align2;
use eframe::epaint::FontId;
use log::info;

use crate::collection::Collection;
use crate::requests::Request;

pub struct CollectionListView<'a> {
    collection_list: &'a mut Vec<Collection>,
    selected_collection_index: &'a mut usize,
}

/// Collection view
///
/// Used as the parent view for all ui related
/// to manipulating a collection
impl<'a> CollectionListView<'a> {
    pub fn new(
        collection_list: &'a mut Vec<Collection>,
        selected_collection_index: &'a mut usize,
    ) -> CollectionListView<'a> {
        CollectionListView {
            collection_list,
            selected_collection_index,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, _ui: &mut Ui) -> bool {
        let mut clicked = false;
        egui::SidePanel::left("collection-side-panel").show(ctx, |ui| {
            ui.heading("Collections");
            ui.set_min_width(200.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                let mut selected_index: Option<usize> = None;
                for (index, request) in self.collection_list.iter_mut().enumerate() {
                    let text = format!("{}", request.name);
                    if create_clickable_row(ui, text.clone(), 45.0) {
                        info!("Clicked Reqeust in row: {}", text);
                        selected_index = Some(index);
                    }

                    if let Some(index) = selected_index {
                        clicked = true;
                        info!("Selected Collection index: {}", index);
                        *self.selected_collection_index = index;
                    }
                }

                if ui.button("Add").clicked() {
                    info!("Add button clicked");
                    // create a new collection
                    let collection = vec![Request::new()];
                    let collection_item = Collection {
                        name: format!("Collection {}", self.collection_list.len() + 1),
                        requests: collection.clone(),
                    };
                    self.collection_list.push(collection_item);
                }
            });
        });
        clicked
    }
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
