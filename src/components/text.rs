
use eframe::egui::{TextEdit, Ui};

pub struct TextInput <'a> {
    value: &'a mut String,
    title: &'a str,
}

impl <'a> TextInput <'a> {
    pub fn new(title: &'a str, value: &'a mut String) -> TextInput <'a> {
        TextInput {
            title,
            value
        }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.label(self.title);
            ui.add(TextEdit::singleline(self.value));
        });
    }
}

