use eframe::egui::Ui;

pub struct SelectList<'a, F>
where
    F: FnMut(&mut Ui, usize, &mut String),
{
    items: &'a mut Vec<String>,
    render_item: F,
}

impl<'a, F> SelectList<'a, F>
where
    F: FnMut(&mut Ui, usize, &mut String),
{
    pub fn new(
        items: &'a mut Vec<String>,
        render_item: F,
    ) -> SelectList<'a, F> {
        SelectList {
            items,
            render_item,
        }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            let render_item = &mut self.render_item;
            for (i, item) in self.items.iter_mut().enumerate() {
                render_item(ui, i, item);
            }
        });
    }
}

