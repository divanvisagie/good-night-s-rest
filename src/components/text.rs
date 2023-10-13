
use eframe::egui::{TextEdit, Ui, TextStyle};
use std::option::Option;

pub struct TextInput<'a> {
    value: &'a mut String,
    title: &'a str,
    text_style: Option<TextStyle>,
}

impl<'a> TextInput<'a> {
    pub fn new(title: &'a str, value: &'a mut String, text_style: Option<TextStyle>) -> TextInput<'a> {
        TextInput {
            title,
            value,
            text_style,
        }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.label(self.title);

            if let Some(style) = self.text_style {
                ui.style_mut().body_text_style = style;
            }

            ui.add(TextEdit::singleline(self.value));

            // Reset to default text style if changed
            if self.text_style.is_some() {
                ui.style_mut().body_text_style = TextStyle::Body;
            }
        });
    }
}

