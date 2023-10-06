use eframe::egui::{TextEdit, Ui};

pub struct MultilineTextInput <'a>{
    value: &'a mut String,
    title: &'a str,
}

impl <'a> MultilineTextInput <'a> {
    pub fn new(title: &'a str, value: &'a mut String) -> MultilineTextInput <'a>{
        MultilineTextInput {
            title,
            value
        }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.label(self.title);
            ui.add(TextEdit::multiline(self.value));
        });
    }
}

