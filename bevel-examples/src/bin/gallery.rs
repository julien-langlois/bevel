//! The gallery in a window: `cargo run -p bevel-examples --bin gallery`.
//!
//! `BEVEL_GALLERY_TITLEBAR=app` (or `=system`) forces who draws the window frame.

use bevel_examples::Gallery;
use bevel_theme::Mode;

// ANCHOR: main
fn main() -> eframe::Result {
    // Before the window exists: does the desktop draw its frame, or does the app?
    let choice = bevel_window::Choice::probe(bevel_window::Prefix::new("BEVEL_GALLERY"));
    let frame = choice.into_frame();
    let viewport = bevel_window::decorate(egui::ViewportBuilder::default().with_title("bevel gallery").with_inner_size([960.0, 600.0]), &frame);
    let options = eframe::NativeOptions { viewport, ..Default::default() };
    eframe::run_native(
        "bevel-gallery",
        options,
        Box::new(move |cc| {
            // Inter and JetBrains Mono (bevel-theme's `fonts` feature).
            bevel_theme::fonts::install(&cc.egui_ctx);
            Ok(Box::new(App { gallery: Gallery::new(Mode::Dark, frame), movable: None }))
        }),
    )
}

struct App {
    gallery: Gallery,
    /// The `NSWindow.movable` value last set (macOS).
    #[cfg_attr(not(target_os = "macos"), allow(dead_code))]
    movable: Option<bool>,
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.gallery.ui(ui);
        // macOS: AppKit moves a full-size-content window from a drag anywhere in the title
        // band unless told otherwise; this keeps the strip's controls clickable.
        #[cfg(target_os = "macos")]
        bevel_window::title_bar::sync_movable(ui.ctx(), &mut self.movable);
    }
}
// ANCHOR_END: main
