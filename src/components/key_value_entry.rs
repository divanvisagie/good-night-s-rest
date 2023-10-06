use eframe::egui::{TextEdit, Ui};
use log::info;

#[derive(Clone)]
pub struct KeyValuePair {
    pub key: String,
    pub value: String,
}
pub struct KeyValueEntry <'a> {
    title: &'a str,
    pairs: &'a mut Vec<(String, String)>,
}

impl <'a> KeyValueEntry <'a> {
    pub fn new(title: &'a str, pairs: &'a mut Vec<(String, String)>) -> KeyValueEntry<'a> {
        KeyValueEntry { pairs, title }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        ui.label(self.title);
        for header in self.pairs.iter_mut() {
            ui.horizontal(|ui| {
                ui.add(TextEdit::singleline(&mut header.0));
                ui.add(TextEdit::singleline(&mut header.1));
            });
        }
        if ui.button("Add").clicked() {
            info!("Add button clicked");
            self.pairs.push(
                (String::from(""), String::from(""))
            );
        }
    }
}
