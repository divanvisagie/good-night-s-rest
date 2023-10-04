
use eframe::egui::{self, TextEdit, Ui};
use log::info;

pub struct Header {
    pub title: String,
    pub value: String,
}
pub struct HeaderBuilder <'a> {
    headers: &'a mut Vec<Header>,
}

impl <'a> HeaderBuilder <'a> {
    pub fn new(headers: &'a mut Vec<Header>) -> HeaderBuilder<'a> {
        HeaderBuilder { headers }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        ui.label("Headers");
        for header in self.headers.iter_mut() {
            ui.horizontal(|ui| {
                ui.add(TextEdit::singleline(&mut header.title));
                ui.add(TextEdit::singleline(&mut header.value));
            });
        }
        if ui.button("Add").clicked() {
            info!("Add button clicked");
            self.headers.push(Header {
                title: "".to_string(),
                value: "".to_string(),
            });
        }
    }
}
