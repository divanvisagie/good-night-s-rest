use eframe::{
    egui::{self, Sense, Ui},
    emath::Align2,
    epaint::FontId,
};
use log::info;

use crate::collection::Collection;
use crate::requests::Request;

const HIGHLIGHT_COLOUR: egui::Color32 = egui::Color32::from_gray(220);
const BORDER_COLOUR: egui::Color32 = egui::Color32::from_gray(180);
const ROW_HEIGHT: f32 = 45.0;

pub struct RequestListView<'a> {
    pub collection: &'a mut Collection,
    pub selected_request_index: &'a mut usize,
}

impl<'a> RequestListView<'a> {
    pub fn new(
        collection: &'a mut Collection,
        selected_index: &'a mut usize,
    ) -> RequestListView<'a> {
        RequestListView {
            collection,
            selected_request_index: selected_index,
        }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        
        egui::ScrollArea::vertical().show(ui, |ui| {
            let mut selected_index: Option<usize> = None;
            for (index, request) in self.collection.requests.iter_mut().enumerate() {
                let text = format!("{} {}", request.method, request.url);
                if create_clickable_row(ui, text.clone(), ROW_HEIGHT) {
                    info!("Clicked Reqeust in row: {}", text);
                    selected_index = Some(index);
                }
            }
            if let Some(index) = selected_index {
                info!("Selected Request index: {}", index);
                *self.selected_request_index = index;
            }
            if ui.button("Add").clicked() {
                info!("Add button clicked");
                self.collection.requests.push(Request::new());
            }
        });
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
        ui.painter().rect_filled(rect, 2.0, HIGHLIGHT_COLOUR);
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
        ui.painter()
            .rect_stroke(rect, 2.0, egui::Stroke::new(1.0, BORDER_COLOUR));
    }

    is_clicked
}
