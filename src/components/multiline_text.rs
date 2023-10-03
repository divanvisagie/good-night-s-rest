use std::{cell::RefCell, rc::Rc};

use eframe::egui::{self, TextEdit, Ui};

pub struct MultilineTextInput {
    value: Rc<RefCell<String>>,
    title: String,
}

impl MultilineTextInput {
    pub fn new(title: String, value: Rc<RefCell<String>>) -> MultilineTextInput {
        MultilineTextInput {
            title,
            value
        }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.label(&self.title);
            ui.add(TextEdit::multiline(&mut *self.value.borrow_mut()));
        });
    }
}

