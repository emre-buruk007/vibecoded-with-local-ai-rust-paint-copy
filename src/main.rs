//! Entry point for the rust-paint application.

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default().with_inner_size([1280.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Paint",
        options,
        Box::new(|cc| Ok(Box::new(rust_paint::app::PaintApp::new(cc)))),
    )
}
