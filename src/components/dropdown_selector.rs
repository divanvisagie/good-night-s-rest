use eframe::egui::{TextEdit, Ui, self};

pub struct DropdownSelector<'a> {
    items: Vec<String>,
    selected_index: &'a mut usize,
}

impl<'a> DropdownSelector<'a> {
    pub fn new(items: Vec<String>, selected_index: &'a mut usize) -> DropdownSelector<'a> {
        DropdownSelector {
            items,
            selected_index,
        }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        egui::ComboBox::new("method", "")
            .selected_text(
                format!("{:?}", 
                    self.items[self.selected_index.clone()]
                )
            )
            .show_ui(ui, |ui| {
                for (i, item) in self.items.iter().enumerate() {
                    ui.selectable_value(self.selected_index, i, item);
                }
            });
    }
}
