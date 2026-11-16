//! The bevel guide's examples.
//!
//! [`Gallery`] is one screen built from the bevel crates: the theme and its chips, a title bar
//! drawn by the app (window buttons, tabs, theme switch), an icon rail, a side panel and two
//! cards in a resizable split. The guide (`book/`) includes its code between the `ANCHOR`
//! comments, `src/bin/gallery.rs` runs it in a window, and `tests/screenshots.rs` renders it.

use bevel_theme::{Chip, Mode, Theme};
use bevel_widgets::buttons::IconButton;
use bevel_widgets::layout::{ResizeHandle, ResizeHandleStyle, SplitAxis, SplitSize, split_rect};
use bevel_widgets::tabs::{self, Tab};
use bevel_window::{Button, Frame};
use egui::{
    Align, Align2, Color32, CornerRadius, Layout, Margin, Painter, Rect, Response, RichText, Sense, Stroke, StrokeKind, Ui, UiBuilder, Vec2, vec2,
};

const DOCUMENTS: [&str; 3] = ["overview.md", "release-notes.md", "roadmap.md"];
const RAIL: [(&str, Icon); 3] = [("Files", Icon::Files), ("Search", Icon::Search), ("Settings", Icon::Settings)];
const PROJECTS: [&str; 5] = ["bevel", "egui", "wgpu", "winit", "icu4x"];

/// The gallery's state: everything the app owns, bevel keeps none of it.
pub struct Gallery {
    pub mode: Mode,
    /// Who draws the window frame (`bevel_window::Choice::probe` at startup).
    pub frame: Frame,
    applied: Option<Mode>,
    tab: usize,
    rail: usize,
    project: usize,
    split: SplitSize,
    search: String,
    notify: bool,
}

impl Gallery {
    pub fn new(mode: Mode, frame: Frame) -> Self {
        Self { mode, frame, applied: None, tab: 0, rail: 0, project: 0, split: SplitSize::Ratio(0.55), search: String::new(), notify: true }
    }

    /// One frame of the whole screen.
    pub fn ui(&mut self, ui: &mut Ui) {
        // ANCHOR: theme
        // Apply the theme once, and again when the user switches between dark and light.
        if self.applied != Some(self.mode) {
            Theme::for_mode(self.mode).apply(ui.ctx());
            self.applied = Some(self.mode);
        }
        // Anywhere in the UI, the theme last applied comes back from the context.
        let theme = Theme::get(ui.ctx());
        // ANCHOR_END: theme

        self.title_bar(ui, &theme);
        self.rail(ui, &theme);
        self.side_panel(ui, &theme);
        egui::CentralPanel::default().frame(theme.content_frame().inner_margin(Margin::same(16))).show(ui, |ui| self.content(ui, &theme));

        // ANCHOR: resize_edges
        // Last, so the edges win over the content under them: an undecorated window gets its
        // resize bands back (nothing happens when the desktop draws the frame).
        bevel_window::resize::edges(ui, &self.frame);
        // ANCHOR_END: resize_edges
    }

    // ANCHOR: title_bar
    fn title_bar(&mut self, ui: &mut Ui, theme: &Theme) {
        egui::Panel::top("title_bar").exact_size(40.0).frame(egui::Frame::new().fill(theme.palette.chrome)).show(ui, |ui| {
            let bar = ui.max_rect();
            // Before the bar's controls: they sit on top and keep their own clicks, and a drag
            // on the empty part moves the window.
            bevel_window::title_bar::empty_area(ui, bar);
            ui.horizontal_centered(|ui| {
                // Only when the app draws the frame: the buttons go where the desktop puts them.
                if let Some(layout) = self.frame.layout() {
                    window_buttons(ui, theme, &layout.left);
                }
                ui.add_space(10.0);
                ui.label(RichText::new("bevel").font(bevel_theme::fonts::semibold(14.0)));
                ui.add_space(14.0);
                self.tabs(ui, theme);
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if let Some(layout) = self.frame.layout() {
                        let right: Vec<Button> = layout.right.iter().rev().copied().collect();
                        window_buttons(ui, theme, &right);
                    }
                    let (label, next) = match self.mode {
                        Mode::Dark => ("Light theme", Mode::Light),
                        Mode::Light => ("Dark theme", Mode::Dark),
                    };
                    if icon_button(ui, theme, label, Icon::Theme, false).clicked() {
                        self.mode = next;
                    }
                });
            });
        });
    }
    // ANCHOR_END: title_bar

    // ANCHOR: tabs
    fn tabs(&mut self, ui: &mut Ui, theme: &Theme) {
        let font = egui::TextStyle::Body.resolve(ui.style());
        let padding = 28.0;
        let natural: Vec<f32> =
            DOCUMENTS.iter().map(|name| ui.painter().layout_no_wrap((*name).to_owned(), font.clone(), Color32::WHITE).size().x + padding).collect();
        // bevel only assigns widths: which tabs fit, which go to an overflow menu.
        let fit = tabs::fit(&natural, self.tab, (ui.available_width() - 120.0).max(0.0), 72.0, 28.0);
        for (index, width) in fit.shown {
            let (rect, _) = ui.allocate_exact_size(vec2(width, 28.0), Sense::hover());
            let selected = index == self.tab;
            // The tab reports its role, label and selection to AccessKit and handles the
            // keyboard; the app paints it.
            let response = Tab::new(ui.scope_id().with(("document", index)), DOCUMENTS[index], selected)
                .focus_stroke(Stroke::new(1.0, theme.palette.accent))
                .show_at(ui, rect, |ui, response| {
                    let fill = match (selected, response.hovered()) {
                        (true, _) => theme.palette.content,
                        (false, true) => theme.palette.raised,
                        (false, false) => Color32::TRANSPARENT,
                    };
                    ui.painter().rect_filled(response.rect, theme.radii.medium, fill);
                    let color = if selected { theme.palette.text } else { theme.palette.text_secondary };
                    ui.painter().text(response.rect.center(), Align2::CENTER_CENTER, DOCUMENTS[index], font.clone(), color);
                });
            if response.clicked() {
                self.tab = index;
            }
        }
    }
    // ANCHOR_END: tabs

    fn rail(&mut self, ui: &mut Ui, theme: &Theme) {
        egui::Panel::left("rail")
            .exact_size(48.0)
            .resizable(false)
            .frame(egui::Frame::new().fill(theme.palette.rail).inner_margin(Margin::symmetric(0, 8)))
            .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    for (index, (label, icon)) in RAIL.into_iter().enumerate() {
                        if icon_button(ui, theme, label, icon, index == self.rail).clicked() {
                            self.rail = index;
                        }
                    }
                });
            });
    }

    fn side_panel(&mut self, ui: &mut Ui, theme: &Theme) {
        egui::Panel::left("projects").exact_size(210.0).resizable(false).frame(theme.panel_frame().inner_margin(Margin::same(12))).show(ui, |ui| {
            ui.add(egui::TextEdit::singleline(&mut self.search).hint_text("Filter projects").desired_width(f32::INFINITY));
            ui.add_space(10.0);
            ui.label(RichText::new("PROJECTS").small().color(theme.palette.text_muted));
            for (index, name) in PROJECTS.into_iter().enumerate() {
                if name.contains(self.search.trim()) && ui.selectable_label(index == self.project, name).clicked() {
                    self.project = index;
                }
            }
        });
    }

    // ANCHOR: split
    fn content(&mut self, ui: &mut Ui, theme: &Theme) {
        let minimum = 180.0;
        let Some(split) = split_rect(ui.max_rect(), SplitAxis::Horizontal, self.split, 12.0, minimum) else { return };
        let style = ResizeHandleStyle { hovered: theme.palette.border, dragged: theme.palette.accent, focused: theme.palette.accent };
        // Pointer drags, arrow keys and screen readers all come back as one delta; the app
        // decides what it does with it.
        let resize = ResizeHandle::new(ui.scope_id().with("columns"), SplitAxis::Horizontal, split.divider, "Resize the columns", style).show(ui);
        if resize.delta != 0.0
            && let Some(size) = split.resized(self.split, resize.delta, minimum)
        {
            self.split = size;
        }
        ui.scope_builder(UiBuilder::new().max_rect(split.first), |ui| self.merge_requests(ui, theme));
        ui.scope_builder(UiBuilder::new().max_rect(split.second), |ui| self.controls(ui, theme));
    }
    // ANCHOR_END: split

    // ANCHOR: chips
    fn merge_requests(&mut self, ui: &mut Ui, theme: &Theme) {
        theme.card_frame().show(ui, |ui| {
            ui.set_min_size(ui.available_size());
            ui.label(RichText::new("Merge requests").font(bevel_theme::fonts::semibold(15.0)));
            ui.add_space(8.0);
            let t = &theme.tones;
            let rows = [
                ("Theme: light palette contrast", Chip::new("MERGED", t.purple).strong()),
                ("Window: Wayland probe timeout", Chip::new("OPEN", t.green).strong()),
                ("Docking: restore closed panels", Chip::new("REVIEW", t.amber)),
                ("i18n: Persian digits in fields", Chip::new("CONFLICT", t.red)),
                ("Fonts: serve CJK faces on the web", Chip::new("DRAFT", t.gray)),
                ("Tabs: keep the selected tab shown", Chip::new("CI passed", t.blue).pill()),
            ];
            for (title, chip) in rows {
                ui.horizontal(|ui| {
                    ui.add(chip);
                    ui.label(title);
                });
                ui.add_space(4.0);
            }
        });
    }
    // ANCHOR_END: chips

    fn controls(&mut self, ui: &mut Ui, theme: &Theme) {
        theme.card_frame().show(ui, |ui| {
            ui.set_min_size(ui.available_size());
            ui.label(RichText::new("egui controls, themed").font(bevel_theme::fonts::semibold(15.0)));
            ui.label(RichText::new("Theme::apply styles egui's own widgets too.").color(theme.palette.text_secondary));
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                let publish = egui::Button::new(RichText::new("Publish").color(Color32::WHITE)).fill(theme.palette.accent_fill);
                ui.add(publish);
                let _ = ui.button("Cancel");
            });
            ui.add_space(6.0);
            ui.checkbox(&mut self.notify, "Notify reviewers");
            ui.add(egui::ProgressBar::new(0.62).text("Pipeline 62 %").fill(theme.palette.accent));
            ui.add_space(6.0);
            ui.label(RichText::new("cargo add bevel-theme").monospace().color(theme.palette.text_secondary));
        });
    }
}

// ANCHOR: icon_button
/// An icon button in the theme's style; the icon itself is the app's (painted here).
fn icon_button(ui: &mut Ui, theme: &Theme, label: &str, icon: Icon, selected: bool) -> Response {
    let style = theme.icon_button();
    // `label` is the accessible name: screen readers announce it, tests find the button by it.
    IconButton::new(label, Vec2::splat(32.0), &style).selected(selected).show(ui, |ui, rect, tint| paint_icon(ui.painter(), rect, icon, tint))
}
// ANCHOR_END: icon_button

// ANCHOR: window_buttons
/// Minimize, maximize and close, in the order the desktop gives (`bevel_window::Layout`).
fn window_buttons(ui: &mut Ui, theme: &Theme, buttons: &[Button]) {
    for button in buttons {
        let (label, icon) = match button {
            Button::Minimize => ("Minimize", Icon::Minimize),
            Button::Maximize => ("Maximize", Icon::Maximize),
            Button::Close => ("Close", Icon::Close),
        };
        if icon_button(ui, theme, label, icon, false).clicked() {
            let command = match button {
                Button::Minimize => egui::ViewportCommand::Minimized(true),
                Button::Maximize => egui::ViewportCommand::Maximized(!ui.input(|i| i.viewport().maximized.unwrap_or(false))),
                Button::Close => egui::ViewportCommand::Close,
            };
            ui.ctx().send_viewport_cmd(command);
        }
    }
}
// ANCHOR_END: window_buttons

#[derive(Clone, Copy)]
enum Icon {
    Theme,
    Minimize,
    Maximize,
    Close,
    Files,
    Search,
    Settings,
}

/// Simple vector icons, so the example needs no image assets.
fn paint_icon(painter: &Painter, rect: Rect, icon: Icon, tint: Color32) {
    let c = rect.center();
    let s = rect.width().min(rect.height()) * 0.22;
    let stroke = Stroke::new(1.4, tint);
    match icon {
        Icon::Theme => {
            painter.circle_stroke(c, s, stroke);
            painter.add(egui::Shape::convex_polygon(semicircle(c, s), tint, Stroke::NONE));
        }
        Icon::Minimize => {
            painter.line_segment([c + vec2(-s, 0.0), c + vec2(s, 0.0)], stroke);
        }
        Icon::Maximize => {
            painter.rect_stroke(Rect::from_center_size(c, Vec2::splat(2.0 * s)), CornerRadius::ZERO, stroke, StrokeKind::Middle);
        }
        Icon::Close => {
            painter.line_segment([c + vec2(-s, -s), c + vec2(s, s)], stroke);
            painter.line_segment([c + vec2(-s, s), c + vec2(s, -s)], stroke);
        }
        Icon::Files => {
            let page = Rect::from_center_size(c, vec2(1.6 * s, 2.1 * s));
            painter.rect_stroke(page, CornerRadius::same(2), stroke, StrokeKind::Middle);
            for k in 0..3 {
                let y = page.top() + page.height() * (0.3 + 0.2 * k as f32);
                painter.line_segment([egui::pos2(page.left() + 0.3 * s, y), egui::pos2(page.right() - 0.3 * s, y)], stroke);
            }
        }
        Icon::Search => {
            painter.circle_stroke(c + vec2(-0.25 * s, -0.25 * s), 0.75 * s, stroke);
            painter.line_segment([c + vec2(0.3 * s, 0.3 * s), c + vec2(s, s)], stroke);
        }
        Icon::Settings => {
            painter.circle_stroke(c, 0.45 * s, stroke);
            for k in 0..8 {
                let angle = k as f32 * std::f32::consts::TAU / 8.0;
                let dir = Vec2::angled(angle);
                painter.line_segment([c + dir * 0.75 * s, c + dir * 1.05 * s], stroke);
            }
        }
    }
}

/// The right half of a circle, for the theme icon.
fn semicircle(center: egui::Pos2, radius: f32) -> Vec<egui::Pos2> {
    (0..=16).map(|k| center + Vec2::angled(-std::f32::consts::FRAC_PI_2 + k as f32 * std::f32::consts::PI / 16.0) * radius).collect()
}
