use eframe::egui::{self, Ui};

use crate::{
    method::Method,
    requests::Request,
};

use super::{key_value_entry::KeyValueEntry, multiline_text::MultilineTextInput, text::TextInput};

pub struct EditView<'a> {
    request: &'a mut Request,
}

impl<'a> EditView<'a> {
    pub fn new(request: &'a mut Request) -> EditView<'a> {
        EditView {
            request,
        }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                egui::ComboBox::new("method", "")
                    .selected_text(format!("{:?}", self.request.method))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.request.method, Method::GET, "GET");
                        ui.selectable_value(&mut self.request.method, Method::POST, "POST");
                        ui.selectable_value(&mut self.request.method, Method::PUT, "PUT");
                        ui.selectable_value(&mut self.request.method, Method::DELETE, "DELETE");
                    });
                TextInput::new("URL:", &mut self.request.url).show(ui);
            });
            KeyValueEntry::new("Headers", &mut self.request.headers).show(ui);
            KeyValueEntry::new("QueryParams", &mut self.request.query_params).show(ui);

            MultilineTextInput::new("body", &mut self.request.body).show(ui);

        });
    }
}
