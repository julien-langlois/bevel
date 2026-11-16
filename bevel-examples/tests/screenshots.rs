//! The gallery, headless.
//!
//! `gallery_runs` runs in CI. `screenshots` writes the guide's images
//! (`book/src/images/gallery-{dark,light}.png`); run it by hand after a visual change:
//!
//! ```sh
//! cargo test -p bevel-examples --test screenshots -- --ignored
//! ```

use std::path::PathBuf;

use bevel_examples::Gallery;
use bevel_theme::Mode;
use bevel_window::{Frame, Layout};
use egui::UiBuilder;
use egui_kittest::Harness;
use egui_kittest::kittest::Queryable as _;

// ANCHOR: harness_and_test
/// The app draws its frame, with the window buttons on the right (`Layout::default`), so the
/// screenshots show the title bar the same on every machine.
fn harness(mode: Mode) -> Harness<'static, Gallery> {
    Harness::builder().with_size([960.0, 560.0]).with_context_setup(bevel_theme::fonts::install).build_ui_state(
        // kittest wraps the UI in an 8-point margin; eframe gives the app the whole window.
        |ui, gallery: &mut Gallery| {
            let window = ui.ctx().content_rect();
            ui.scope_builder(UiBuilder::new().max_rect(window), |ui| gallery.ui(ui));
        },
        Gallery::new(mode, Frame::App(Layout::default())),
    )
}

#[test]
fn gallery_runs() {
    for mode in [Mode::Dark, Mode::Light] {
        let mut harness = harness(mode);
        harness.run();
        // Accessible names come from the widgets' labels.
        assert!(harness.query_by_label("Close").is_some());
        assert!(harness.query_by_label("release-notes.md").is_some());
    }
}

#[test]
fn the_theme_button_switches_modes() {
    let mut harness = harness(Mode::Dark);
    harness.run();
    harness.get_by_label("Light theme").click();
    harness.run();
    assert_eq!(harness.state().mode, Mode::Light);
}
// ANCHOR_END: harness_and_test

#[test]
#[ignore = "writes book/src/images; run by hand after a visual change"]
fn screenshots() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../book/src/images");
    std::fs::create_dir_all(&dir).expect("create book/src/images");
    for (mode, name) in [(Mode::Dark, "gallery-dark.png"), (Mode::Light, "gallery-light.png")] {
        let mut harness = harness(mode);
        harness.run();
        let image = harness.render().expect("render (needs a GPU adapter or a software Vulkan driver)");
        image.save(dir.join(name)).expect("save the screenshot");
    }
}
