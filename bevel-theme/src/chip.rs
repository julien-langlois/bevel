//! A small rounded label in a status tone: badges, tags, state pills.

use egui::{CornerRadius, FontId, Response, Role, Sense, TextStyle, Ui, Vec2, Widget, WidgetInfo};

use crate::{Theme, Tone};

/// A tone-colored label, optionally led by a glyph (`●`, `◐`…).
///
/// ```ignore
/// let theme = Theme::get(ui.ctx());
/// ui.add(Chip::new("MERGED", theme.tones.purple).strong());
/// ```
#[derive(Clone, Debug)]
pub struct Chip<'a> {
    text: &'a str,
    glyph: Option<&'a str>,
    tone: Tone,
    strong: bool,
    pill: bool,
    font: Option<FontId>,
}

impl<'a> Chip<'a> {
    pub fn new(text: &'a str, tone: Tone) -> Self {
        Self { text, glyph: None, tone, strong: false, pill: false, font: None }
    }

    /// A glyph drawn before the text, in the same color.
    pub fn glyph(mut self, glyph: &'a str) -> Self {
        self.glyph = Some(glyph);
        self
    }

    /// Bold, tighter text: for state badges (`OPEN`, `MERGED`).
    pub fn strong(mut self) -> Self {
        self.strong = true;
        self
    }

    /// Draws the text in `font` (e.g. a semibold face) instead of the small text style.
    /// A `strong` chip with a font of its own is not faux-bolded.
    pub fn font(mut self, font: FontId) -> Self {
        self.font = Some(font);
        self
    }

    /// Fully rounded ends instead of the small radius.
    pub fn pill(mut self) -> Self {
        self.pill = true;
        self
    }
}

impl Widget for Chip<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui.ctx());
        let faux_bold = self.strong && self.font.is_none();
        let font = self.font.unwrap_or_else(|| {
            let size = ui.style().text_styles.get(&TextStyle::Small).map_or(11.0, |f| f.size);
            FontId::proportional(size)
        });
        let label = match self.glyph {
            Some(glyph) => format!("{glyph} {}", self.text),
            None => self.text.to_owned(),
        };
        let mut job = egui::text::LayoutJob::simple_singleline(label, font, self.tone.fg);
        if self.strong
            && let Some(section) = job.sections.first_mut()
        {
            section.format.extra_letter_spacing = 0.3;
        }
        let galley = ui.painter().layout_job(job);
        let padding = Vec2::new(7.0, 2.0);
        let (rect, response) = ui.allocate_exact_size(galley.size() + padding * 2.0, Sense::hover());
        if ui.is_rect_visible(rect) {
            let radius =
                if self.pill { CornerRadius::same((rect.height() / 2.0).round().min(255.0) as u8) } else { CornerRadius::same(theme.radii.small) };
            ui.painter().rect_filled(rect, radius, self.tone.bg);
            let pos = rect.min + padding;
            if faux_bold {
                // Faux bold: the default font set has no bold face.
                ui.painter().galley(pos + Vec2::new(0.4, 0.0), galley.clone(), self.tone.fg);
            }
            ui.painter().galley(pos, galley, self.tone.fg);
        }
        let text = self.text.to_owned();
        response.widget_info(|| WidgetInfo::labeled(Role::Label, true, &text));
        response
    }
}
