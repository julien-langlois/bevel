#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! Design tokens for bevel apps, in dark and light.
//!
//! A [`Theme`] holds the surfaces, text colors, accent and status [`Tones`] of an app,
//! plus its [`Radii`] and [`Spacing`]. [`Theme::apply`] writes them into an egui
//! context once; components read the theme back for what egui's `Style` has no slot
//! for (status tones, the rail and chrome surfaces).
//!
//! Nothing here is specific to one app: an app picks [`Theme::dark`] or
//! [`Theme::light`] and overrides the tokens it wants (an accent, a radius).
//!
//! The [bevel guide](https://julien-langlois.github.io/bevel/theme.html) explains how the pieces fit,
//! with complete examples and screenshots.

use egui::{Color32, CornerRadius, FontFamily, FontId, Margin, Shadow, Stroke, TextStyle, Vec2};

mod chip;
#[cfg(feature = "fonts")]
pub mod fonts;

pub use chip::Chip;

#[cfg(feature = "widgets")]
mod widgets;

/// Whether the theme is meant for a dark or a light background.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Mode {
    Dark,
    Light,
}

/// Surfaces, borders, text and accent colors.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Palette {
    /// The window's own chrome: title bar and status bar.
    pub chrome: Color32,
    /// The icon rail at the window's edge.
    pub rail: Color32,
    /// Side panels (navigation, inspector).
    pub panel: Color32,
    /// The main content area.
    pub content: Color32,
    /// Popups, menus and cards raised above a panel.
    pub raised: Color32,
    /// Text fields and other inputs.
    pub input: Color32,
    /// Lines between regions.
    pub border: Color32,
    /// Lines inside a region (section separators, table rules).
    pub border_soft: Color32,
    /// Primary text.
    pub text: Color32,
    /// Secondary text: section headings, idle navigation items.
    pub text_secondary: Color32,
    /// Captions, hints and counts. Still meets 4.5:1 on `panel` and `content`.
    pub text_muted: Color32,
    /// Accent for links, focus, active icons and selection outlines.
    pub accent: Color32,
    /// Fill of primary buttons; white text on it meets 4.5:1.
    pub accent_fill: Color32,
    /// Fill of the selected row or item.
    pub selection: Color32,
    /// Background of the active rail item.
    pub rail_active: Color32,
}

/// A status color as a pair: a tinted background and a readable foreground.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tone {
    pub bg: Color32,
    pub fg: Color32,
}

/// The status tones, named by hue so apps map their own meanings onto them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tones {
    pub green: Tone,
    pub purple: Tone,
    pub red: Tone,
    pub amber: Tone,
    pub teal: Tone,
    pub gray: Tone,
    pub blue: Tone,
    pub orange: Tone,
}

/// Corner radii, smallest to largest.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Radii {
    /// Chips, badges and small tags.
    pub small: u8,
    /// Buttons, inputs and list items.
    pub medium: u8,
    /// Cards, menus and popups.
    pub large: u8,
}

/// Spacing between and inside controls, in points.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Spacing {
    /// Between items in a row or a column.
    pub item: Vec2,
    /// Inside buttons.
    pub button_padding: Vec2,
    /// Height of buttons and inputs.
    pub control_height: f32,
    /// Inside a panel or a card.
    pub panel_margin: i8,
}

/// Every token of one theme.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Theme {
    pub mode: Mode,
    pub palette: Palette,
    pub tones: Tones,
    pub radii: Radii,
    pub spacing: Spacing,
    /// Base text size, in points; headings and captions scale from it.
    pub text_size: f32,
}

const fn rgb(hex: u32) -> Color32 {
    Color32::from_rgb((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
}

/// A `fg` color at `alpha` (0–255) over transparent, as tone and selection tints.
const fn tint(hex: u32, alpha: u8) -> Color32 {
    Color32::from_rgba_unmultiplied_const((hex >> 16) as u8, (hex >> 8) as u8, hex as u8, alpha)
}

const fn tone(fg: u32, alpha: u8) -> Tone {
    Tone { bg: tint(fg, alpha), fg: rgb(fg) }
}

const RADII: Radii = Radii { small: 4, medium: 6, large: 10 };

const SPACING: Spacing = Spacing { item: Vec2::new(8.0, 6.0), button_padding: Vec2::new(10.0, 4.0), control_height: 28.0, panel_margin: 14 };

impl Theme {
    /// Neutral greys, a blue accent and tinted status tones.
    pub const fn dark() -> Self {
        Self {
            mode: Mode::Dark,
            palette: Palette {
                chrome: rgb(0x181818),
                rail: rgb(0x212121),
                panel: rgb(0x262626),
                content: rgb(0x1e1e1e),
                raised: rgb(0x2f2f2f),
                input: rgb(0x1b1b1b),
                border: rgb(0x363636),
                border_soft: rgb(0x2e2e2e),
                text: rgb(0xececec),
                text_secondary: rgb(0xb8b8b8),
                text_muted: rgb(0x929292),
                accent: rgb(0x3ba2ff),
                accent_fill: rgb(0x0b6fc6),
                selection: tint(0x3ba2ff, 38),
                rail_active: rgb(0x2f3a47),
            },
            tones: Tones {
                green: tone(0x6fdc8c, 41),
                purple: tone(0xc9b0ff, 46),
                red: tone(0xff8f86, 41),
                amber: tone(0xf5c060, 41),
                teal: tone(0x6fe0d4, 38),
                gray: Tone { bg: tint(0xffffff, 15), fg: rgb(0xa8a8a8) },
                blue: tone(0x7cc0ff, 41),
                orange: tone(0xffaa66, 41),
            },
            radii: RADII,
            spacing: SPACING,
            text_size: 13.0,
        }
    }

    /// The same structure as [`Theme::dark`] on white and light grey surfaces.
    pub const fn light() -> Self {
        Self {
            mode: Mode::Light,
            palette: Palette {
                chrome: rgb(0xe9e9e9),
                rail: rgb(0xf3f3f3),
                panel: rgb(0xffffff),
                content: rgb(0xf6f6f6),
                raised: rgb(0xffffff),
                input: rgb(0xf4f4f4),
                border: rgb(0xdcdcdc),
                border_soft: rgb(0xebebeb),
                text: rgb(0x1c1c1c),
                text_secondary: rgb(0x474747),
                text_muted: rgb(0x666666),
                accent: rgb(0x0b6fc6),
                accent_fill: rgb(0x0b6fc6),
                selection: tint(0x0b6fc6, 23),
                rail_active: rgb(0xe1ebf6),
            },
            tones: Tones {
                green: Tone { bg: rgb(0xe2f4e7), fg: rgb(0x1b6e35) },
                purple: Tone { bg: rgb(0xefe8fb), fg: rgb(0x5b33a8) },
                red: Tone { bg: rgb(0xfde6e4), fg: rgb(0xa3261c) },
                amber: Tone { bg: rgb(0xfbf0d9), fg: rgb(0x7a5000) },
                teal: Tone { bg: rgb(0xdcf3f0), fg: rgb(0x0d6a62) },
                gray: Tone { bg: rgb(0xefefef), fg: rgb(0x555555) },
                blue: Tone { bg: rgb(0xe1edf9), fg: rgb(0x0a5ea8) },
                orange: Tone { bg: rgb(0xfdeadb), fg: rgb(0x9a4a0c) },
            },
            radii: RADII,
            spacing: SPACING,
            text_size: 13.0,
        }
    }

    /// [`Theme::dark`] or [`Theme::light`].
    pub const fn for_mode(mode: Mode) -> Self {
        match mode {
            Mode::Dark => Self::dark(),
            Mode::Light => Self::light(),
        }
    }

    /// Writes the theme into `ctx`'s style and keeps a copy for [`Theme::get`].
    pub fn apply(&self, ctx: &egui::Context) {
        ctx.data_mut(|d| d.insert_temp(egui::Id::NULL.with("bevel-theme"), *self));
        let theme = *self;
        ctx.all_styles_mut(move |style| theme.apply_to_style(style));
    }

    /// The theme last applied to `ctx`, or [`Theme::dark`] when none was.
    pub fn get(ctx: &egui::Context) -> Self {
        ctx.data(|d| d.get_temp(egui::Id::NULL.with("bevel-theme"))).unwrap_or_else(Self::dark)
    }

    /// Writes the tokens into an egui `Style`.
    pub fn apply_to_style(&self, style: &mut egui::Style) {
        let p = &self.palette;
        let size = self.text_size;
        style.text_styles = [
            (TextStyle::Small, FontId::new(size - 1.5, FontFamily::Proportional)),
            (TextStyle::Body, FontId::new(size, FontFamily::Proportional)),
            (TextStyle::Button, FontId::new(size - 0.5, FontFamily::Proportional)),
            (TextStyle::Heading, FontId::new(size + 2.0, FontFamily::Proportional)),
            (TextStyle::Monospace, FontId::new(size - 1.0, FontFamily::Monospace)),
        ]
        .into();

        let s = &mut style.spacing;
        s.item_spacing = self.spacing.item;
        s.button_padding = self.spacing.button_padding;
        s.interact_size.y = self.spacing.control_height;
        s.window_margin = Margin::same(self.spacing.panel_margin);
        s.menu_margin = Margin::same(6);

        let v = &mut style.visuals;
        *v = match self.mode {
            Mode::Dark => egui::Visuals::dark(),
            Mode::Light => egui::Visuals::light(),
        };
        v.override_text_color = Some(p.text);
        v.hyperlink_color = p.accent;
        v.panel_fill = p.panel;
        v.window_fill = p.raised;
        v.window_stroke = Stroke::new(1.0, p.border);
        v.window_corner_radius = CornerRadius::same(self.radii.large);
        v.menu_corner_radius = CornerRadius::same(self.radii.large);
        v.extreme_bg_color = p.input;
        v.faint_bg_color = p.border_soft;
        v.code_bg_color = p.input;
        v.selection.bg_fill = p.selection;
        v.selection.stroke = Stroke::new(1.0, p.accent);
        v.window_shadow = Shadow { offset: [0, 6], blur: 18, spread: 0, color: Color32::from_black_alpha(46) };
        v.popup_shadow = v.window_shadow;

        let radius = CornerRadius::same(self.radii.medium);
        let w = &mut v.widgets;
        w.noninteractive.bg_fill = p.panel;
        w.noninteractive.weak_bg_fill = p.panel;
        w.noninteractive.bg_stroke = Stroke::new(1.0, p.border_soft);
        w.noninteractive.fg_stroke = Stroke::new(1.0, p.text_secondary);
        for (state, fill) in [(&mut w.inactive, p.input), (&mut w.hovered, p.raised), (&mut w.active, p.raised), (&mut w.open, p.raised)] {
            state.bg_fill = fill;
            state.weak_bg_fill = fill;
            state.bg_stroke = Stroke::new(1.0, p.border);
            state.fg_stroke = Stroke::new(1.0, p.text);
            state.corner_radius = radius;
            state.expansion = 0.0;
        }
        w.hovered.bg_stroke = Stroke::new(1.0, p.text_muted);
        w.active.bg_stroke = Stroke::new(1.0, p.accent);
        w.noninteractive.corner_radius = radius;
    }

    /// A frame for a side panel.
    pub fn panel_frame(&self) -> egui::Frame {
        egui::Frame::new().fill(self.palette.panel)
    }

    /// A frame for the main content area.
    pub fn content_frame(&self) -> egui::Frame {
        egui::Frame::new().fill(self.palette.content)
    }

    /// A frame for a card on the content area.
    pub fn card_frame(&self) -> egui::Frame {
        egui::Frame::new()
            .fill(self.palette.panel)
            .stroke(Stroke::new(1.0, self.palette.border_soft))
            .corner_radius(CornerRadius::same(self.radii.large))
            .inner_margin(Margin::same(16))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// WCAG relative luminance of an opaque color.
    fn luminance(c: Color32) -> f32 {
        let lin = |v: u8| {
            let v = f32::from(v) / 255.0;
            if v <= 0.03928 { v / 12.92 } else { ((v + 0.055) / 1.055).powf(2.4) }
        };
        0.2126 * lin(c.r()) + 0.7152 * lin(c.g()) + 0.0722 * lin(c.b())
    }

    fn contrast(a: Color32, b: Color32) -> f32 {
        let (la, lb) = (luminance(a), luminance(b));
        (la.max(lb) + 0.05) / (la.min(lb) + 0.05)
    }

    #[test]
    fn text_meets_contrast_on_every_surface() {
        for theme in [Theme::dark(), Theme::light()] {
            let p = theme.palette;
            for surface in [p.panel, p.content, p.chrome, p.rail] {
                for (name, text) in [("text", p.text), ("secondary", p.text_secondary), ("muted", p.text_muted)] {
                    let ratio = contrast(text, surface);
                    assert!(ratio >= 4.5, "{:?} {name} on {surface:?}: {ratio:.2}", theme.mode);
                }
            }
            assert!(contrast(Color32::WHITE, p.accent_fill) >= 4.5, "{:?} button text", theme.mode);
        }
    }

    #[test]
    fn apply_round_trips_through_the_context() {
        let ctx = egui::Context::default();
        Theme::light().apply(&ctx);
        assert_eq!(Theme::get(&ctx).mode, Mode::Light);
        assert_eq!(ctx.global_style().visuals.panel_fill, Theme::light().palette.panel);
    }
}
