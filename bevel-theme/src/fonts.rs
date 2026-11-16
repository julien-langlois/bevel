//! Bundled typefaces: Inter for text (regular, medium, semibold) and JetBrains Mono
//! for code, both under the SIL Open Font License (see `fonts/*-OFL.txt`).
//!
//! egui draws one weight per family, so the heavier weights are their own named
//! families: [`medium`] and [`semibold`] build a [`FontId`] in them. Each family
//! keeps egui's default fonts as fallbacks for symbols and emoji, and eframe's
//! system font provider (when enabled) still covers other scripts.

use std::sync::Arc;

use egui::{FontData, FontDefinitions, FontFamily, FontId};

const INTER_REGULAR: &[u8] = include_bytes!("../fonts/Inter-Regular.ttf");
const INTER_MEDIUM: &[u8] = include_bytes!("../fonts/Inter-Medium.ttf");
const INTER_SEMIBOLD: &[u8] = include_bytes!("../fonts/Inter-SemiBold.ttf");
const JETBRAINS_MONO: &[u8] = include_bytes!("../fonts/JetBrainsMono-Regular.ttf");

const MEDIUM: &str = "bevel-medium";
const SEMIBOLD: &str = "bevel-semibold";

/// Makes Inter the proportional font and JetBrains Mono the monospace one, and
/// adds the [`medium`] and [`semibold`] families. Call it once, before [`medium`]
/// or [`semibold`] are used.
pub fn install(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    for (name, data) in [
        ("Inter-Regular", INTER_REGULAR),
        ("Inter-Medium", INTER_MEDIUM),
        ("Inter-SemiBold", INTER_SEMIBOLD),
        ("JetBrainsMono-Regular", JETBRAINS_MONO),
    ] {
        fonts.font_data.insert(name.to_owned(), Arc::new(FontData::from_static(data)));
    }
    let fallbacks = fonts.families.get(&FontFamily::Proportional).cloned().unwrap_or_default();
    let with_fallbacks = |first: &str| std::iter::once(first.to_owned()).chain(fallbacks.iter().cloned()).collect::<Vec<_>>();
    fonts.families.insert(FontFamily::Proportional, with_fallbacks("Inter-Regular"));
    fonts.families.insert(FontFamily::Name(MEDIUM.into()), with_fallbacks("Inter-Medium"));
    fonts.families.insert(FontFamily::Name(SEMIBOLD.into()), with_fallbacks("Inter-SemiBold"));
    fonts.families.entry(FontFamily::Monospace).or_default().insert(0, "JetBrainsMono-Regular".to_owned());
    ctx.set_fonts(fonts);
}

/// Inter Medium at `size`: emphasis in body text, active items.
pub fn medium(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(MEDIUM.into()))
}

/// Inter SemiBold at `size`: headings, titles, badges.
pub fn semibold(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(SEMIBOLD.into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_weights_resolve_once_installed() {
        let ctx = egui::Context::default();
        install(&ctx);
        let mut output = ctx.run_ui(Default::default(), |ui| {
            for font in [medium(13.0), semibold(13.0), FontId::monospace(12.0)] {
                let _galley = ui.painter().layout_no_wrap("Merge ● requests".into(), font, egui::Color32::WHITE);
            }
        });
        output.textures_delta.clear();
    }
}
