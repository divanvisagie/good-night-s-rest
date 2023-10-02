use eframe::egui;
struct AppState {}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("side-panel").show(ctx, |ui| {
            egui::widgets::global_dark_light_mode_switch(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {});
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {}
    }
}

fn main() {
    let mut window_options = eframe::NativeOptions::default();
    window_options.initial_window_size = Some(egui::Vec2::new(1280., 768.));
    window_options.resizable = true;
    window_options.decorated = true;

    match eframe::run_native(
        "Arcanaeum",
        window_options,
        Box::new(|_cc| Box::<AppState>::default()),
    ) {
        Ok(_) => {}
        Err(e) => {
        }
    }
}
