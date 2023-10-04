use std::{cell::RefCell, rc::Rc};

use eframe::egui::{TextEdit, Ui};

pub struct TextInput {
    value: Rc<RefCell<String>>,
    title: String,
}

impl TextInput {
    pub fn new(title: String, value: Rc<RefCell<String>>) -> TextInput {
        TextInput {
            title,
            value
        }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.label(&self.title);
            ui.add(TextEdit::singleline(&mut *self.value.borrow_mut()));
        });
    }
}

