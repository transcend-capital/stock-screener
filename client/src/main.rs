use eframe::{epi, egui};

#[derive(Default)]
struct TranscendMarkets {}
impl epi::App for TranscendMarkets {
    fn name(&self) -> &str {
        "Transcend Markets"
    }

    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
        });
    }
}

fn main() {
    let app = TranscendMarkets::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
