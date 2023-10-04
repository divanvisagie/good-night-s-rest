use eframe::egui::{TextEdit, Ui};
use log::info;

#[derive(Clone)]
pub struct KeyValuePair {
    pub key: String,
    pub value: String,
}
pub struct KeyValueEntry <'a> {
    title: &'a str,
    headers: &'a mut Vec<KeyValuePair>,
}

impl <'a> KeyValueEntry <'a> {
    pub fn new(title: &'a str, headers: &'a mut Vec<KeyValuePair>) -> KeyValueEntry<'a> {
        KeyValueEntry { headers, title }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        ui.label(self.title);
        for header in self.headers.iter_mut() {
            ui.horizontal(|ui| {
                ui.add(TextEdit::singleline(&mut header.key));
                ui.add(TextEdit::singleline(&mut header.value));
            });
        }
        if ui.button("Add").clicked() {
            info!("Add button clicked");
            self.headers.push(KeyValuePair {
                key: "".to_string(),
                value: "".to_string(),
            });
        }
    }
}
