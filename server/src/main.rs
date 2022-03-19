use eframe::{epi, egui, epaint};

#[derive(Default)]
struct MarketsTerminal {}

// Main app panel
impl epi::App for MarketsTerminal {
    fn name(&self) -> &str {
        "Transcend Capital - Markets Terminal"
    }

    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
        });
    }   
}

fn main() {
    let app = MarketsTerminal::default();
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(epaint::Vec2::new(540., 320.));
    eframe::run_native(Box::new(app), native_options);
}
