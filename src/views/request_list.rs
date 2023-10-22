use std::borrow::BorrowMut;

use eframe::{
    egui::{self, Sense, Ui},
    emath::Align2,
    epaint::FontId,
};
use log::info;

use crate::{components::dropdown_selector::DropdownSelector, requests::Request};

pub struct RequestListView<'a> {
    pub collection_name: &'a mut String,
    pub requests: &'a mut Vec<Request>,
    pub selected_request_index: &'a mut usize,
}

impl<'a> RequestListView<'a> {
    pub fn new(
        collection_name: &'a mut String,
        requests: &'a mut Vec<Request>,
        selected_index: &'a mut usize,
    ) -> RequestListView<'a> {
        RequestListView {
            collection_name,
            requests,
            selected_request_index: selected_index,
        }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        ui.text_edit_singleline(self.collection_name);

        DropdownSelector::new(
            self.requests
                .iter()
                .map(|r| format!("{} {}", r.method, r.url))
                .collect(),
            &mut self.selected_request_index,
        );

        egui::ScrollArea::vertical().show(ui, |ui| {
            let mut current = self.requests[self.selected_request_index.clone()].clone();
            let mut selected_index: Option<usize> = None;
            for (index, request) in self.requests.iter_mut().enumerate() {
                let text = format!("{} {}", request.method, request.url);
                if ui
                    .selectable_value(&mut current, request.clone(), text)
                    .clicked()
                {
                    info!("Request selected: {}", request.url);
                    selected_index = Some(index);
                }
            }
            if let Some(index) = selected_index {
                info!("Selected index: {}", index);
                *self.selected_request_index = index;
            }
            if ui.button("Add").clicked() {
                info!("Add button clicked");
                self.requests.push(Request::new());
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
