//! Resize bands along the edges of a window whose frame the app draws.
//!
//! With [`Frame::App`] the window is undecorated, and an undecorated window has no edges to resize
//! it from: the desktop's frame used to provide them. GNOME's Mutter, for one, leaves them to the
//! app, as it does the title bar. [`edges`] puts them back: a thin band along each side and an L
//! at each corner, inside the window. Hovering one shows the matching resize cursor; pressing and
//! dragging hands the resize to the window system (`ViewportCommand::BeginResize`), which then
//! runs it as for any native window (Wayland's `xdg_toplevel.resize`, X11's
//! `_NET_WM_MOVERESIZE`, Windows' `WM_NCLBUTTONDOWN`).
//!
//! Call it once per frame, **after** the window's UI: egui gives the pointer to the widget
//! registered last, so the bands then win over the content they lie on, the title bar's empty
//! area and its window buttons included (as at a native window's edges). In-window panels in
//! layers above the UI's (an `egui::Window`, a popup) keep the pointer where they reach the edge.
//! `Viewports` calls it for each window it shows.
//!
//! The bands stand down where the window cannot be resized from its edges:
//!
//! | When | Why |
//! | --- | --- |
//! | [`Frame::Desktop`] | The desktop's frame has its own edges |
//! | Maximized or full screen | Nothing to resize, as with a native window |
//! | macOS | AppKit keeps the frame and resizes the window itself; winit has no `BeginResize` there |
//!
//! ```ignore
//! fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
//!     draw_the_app(ui);
//!     // Last: the bands lie over the content at the window's edges.
//!     bevel_window::resize::edges(ui, &self.frame);
//! }
//! ```
//!
//! [`Edges`] sets how deep the bands reach and how far a corner's grab runs along its edges.

use egui::{CursorIcon, Pos2, Rect, ResizeDirection, Sense, Ui, ViewportCommand};

use crate::Frame;

/// How far the bands reach into the window from its edges, in points: GTK's resize border for a
/// window without a shadow.
pub const DEPTH: f32 = 6.0;
/// How far a corner's grab runs along each of its two edges, in points.
pub const CORNER: f32 = 16.0;

/// The bands that resize `rect` from its edges: an L at each corner, `depth` deep and `corner`
/// along both edges, then the four edges between them. A corner wins over the edges it joins.
///
/// `corner` is at least `depth`, and both are kept to half of `rect` so opposite bands never
/// overlap in a tiny window.
pub fn bands(rect: Rect, depth: f32, corner: f32) -> [(ResizeDirection, Rect); 12] {
    use ResizeDirection as D;
    let depth = depth.max(0.0).min(rect.width() / 2.0).min(rect.height() / 2.0);
    let corner = corner.max(depth).min(rect.width() / 2.0).min(rect.height() / 2.0);
    let (l, r, t, b, e, c) = (rect.left(), rect.right(), rect.top(), rect.bottom(), depth, corner);
    let band = |x0: f32, y0: f32, x1: f32, y1: f32| Rect::from_min_max(Pos2::new(x0, y0), Pos2::new(x1, y1));
    [
        (D::NorthWest, band(l, t, l + c, t + e)),
        (D::NorthWest, band(l, t, l + e, t + c)),
        (D::NorthEast, band(r - c, t, r, t + e)),
        (D::NorthEast, band(r - e, t, r, t + c)),
        (D::SouthWest, band(l, b - e, l + c, b)),
        (D::SouthWest, band(l, b - c, l + e, b)),
        (D::SouthEast, band(r - c, b - e, r, b)),
        (D::SouthEast, band(r - e, b - c, r, b)),
        (D::North, band(l + c, t, r - c, t + e)),
        (D::South, band(l + c, b - e, r - c, b)),
        (D::West, band(l, t + c, l + e, b - c)),
        (D::East, band(r - e, t + c, r, b - c)),
    ]
}

/// Which way a press at `pos` resizes `rect`, if it lies in one of its [`bands`].
pub fn direction_at(rect: Rect, pos: Pos2, depth: f32, corner: f32) -> Option<ResizeDirection> {
    bands(rect, depth, corner).into_iter().find(|(_, band)| band.contains(pos)).map(|(direction, _)| direction)
}

/// The pointer shown over a band that resizes the window `direction`.
pub fn cursor(direction: ResizeDirection) -> CursorIcon {
    use {CursorIcon as C, ResizeDirection as D};
    match direction {
        D::North => C::ResizeNorth,
        D::South => C::ResizeSouth,
        D::East => C::ResizeEast,
        D::West => C::ResizeWest,
        D::NorthEast => C::ResizeNorthEast,
        D::NorthWest => C::ResizeNorthWest,
        D::SouthEast => C::ResizeSouthEast,
        D::SouthWest => C::ResizeSouthWest,
    }
}

/// The resize bands of the window, with their sizes. [`edges`] shows them with the defaults.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Edges {
    depth: f32,
    corner: f32,
}

impl Default for Edges {
    fn default() -> Self {
        Self { depth: DEPTH, corner: CORNER }
    }
}

impl Edges {
    /// How far the bands reach into the window ([`DEPTH`] by default).
    pub fn depth(mut self, depth: f32) -> Self {
        self.depth = depth;
        self
    }

    /// How far a corner's grab runs along its edges ([`CORNER`] by default).
    pub fn corner(mut self, corner: f32) -> Self {
        self.corner = corner;
        self
    }

    /// Shows the bands over the edges of `ui`'s window for this frame, when the app draws the
    /// window's `frame` and the window can be resized from its edges (see the [module](self)).
    /// Call it after the window's UI.
    ///
    /// Returns the direction a resize started in this frame, if one did.
    pub fn show(self, ui: &Ui, frame: &Frame) -> Option<ResizeDirection> {
        if !active(ui, frame) {
            return None;
        }
        let id = ui.scope_id().with("bevel_window_resize_edges");
        let mut started = None;
        for (i, (direction, band)) in bands(ui.ctx().viewport_rect(), self.depth, self.corner).into_iter().enumerate() {
            let response = ui.interact(band, id.with(i), Sense::drag()).on_hover_cursor(cursor(direction));
            if response.drag_started() {
                ui.ctx().send_viewport_cmd(ViewportCommand::BeginResize(direction));
                started = Some(direction);
            }
        }
        started
    }
}

/// Shows the window's resize bands with the default sizes: [`Edges::show`].
pub fn edges(ui: &Ui, frame: &Frame) -> Option<ResizeDirection> {
    Edges::default().show(ui, frame)
}

/// Whether the window resizes from the app's bands this frame.
fn active(ui: &Ui, frame: &Frame) -> bool {
    if cfg!(target_os = "macos") || frame.decorations() {
        return false;
    }
    let (maximized, fullscreen) = ui.input(|i| (i.viewport().maximized, i.viewport().fullscreen));
    !maximized.unwrap_or(false) && !fullscreen.unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Layout;
    use egui::{Context, Event, PointerButton, RawInput, ViewportId, pos2, vec2};

    const WINDOW: Rect = Rect { min: Pos2::ZERO, max: pos2(400.0, 300.0) };

    #[test]
    fn corners_win_over_the_edges_they_join() {
        let at = |x, y| direction_at(WINDOW, pos2(x, y), DEPTH, CORNER);
        use ResizeDirection as D;
        assert_eq!(at(1.0, 1.0), Some(D::NorthWest));
        assert_eq!(at(10.0, 2.0), Some(D::NorthWest), "along the top, within the corner's grab");
        assert_eq!(at(2.0, 10.0), Some(D::NorthWest), "along the side, within the corner's grab");
        assert_eq!(at(399.0, 299.0), Some(D::SouthEast));
        assert_eq!(at(395.0, 5.0), Some(D::NorthEast));
        assert_eq!(at(3.0, 297.0), Some(D::SouthWest));
        assert_eq!(at(200.0, 2.0), Some(D::North));
        assert_eq!(at(200.0, 298.0), Some(D::South));
        assert_eq!(at(2.0, 150.0), Some(D::West));
        assert_eq!(at(398.0, 150.0), Some(D::East));
        assert_eq!(at(200.0, 150.0), None);
        assert_eq!(at(200.0, DEPTH + 1.0), None, "just inside the band");
    }

    #[test]
    fn bands_stay_apart_in_a_tiny_window() {
        let tiny = Rect::from_min_size(Pos2::ZERO, vec2(10.0, 8.0));
        let bands = bands(tiny, DEPTH, CORNER);
        for (_, band) in bands {
            assert!(tiny.contains_rect(band), "{band:?}");
        }
        assert!(bands.iter().filter(|(_, b)| b.is_positive()).all(|(_, b)| b.width() <= 5.0 || b.height() <= 4.0));
    }

    #[test]
    fn every_direction_has_its_cursor() {
        use ResizeDirection as D;
        let all = [D::North, D::South, D::East, D::West, D::NorthEast, D::NorthWest, D::SouthEast, D::SouthWest];
        let cursors: std::collections::HashSet<_> = all.iter().map(|&d| format!("{:?}", cursor(d))).collect();
        assert_eq!(cursors.len(), all.len());
    }

    /// A window drawn by the app, its content a button filling it, the bands over it.
    struct Window {
        ctx: Context,
        time: f64,
        frame: Frame,
        maximized: bool,
        fullscreen: bool,
    }

    impl Window {
        fn new(frame: Frame) -> Self {
            Self { ctx: Context::default(), time: 0.0, frame, maximized: false, fullscreen: false }
        }

        /// Runs one frame with `events`; returns the viewport commands it sent and the cursor.
        fn frame(&mut self, events: Vec<Event>) -> (Vec<ViewportCommand>, CursorIcon) {
            self.time += 1.0 / 60.0;
            let mut input = RawInput { screen_rect: Some(WINDOW), time: Some(self.time), events, ..RawInput::default() };
            let viewport = input.viewports.entry(ViewportId::ROOT).or_default();
            viewport.maximized = Some(self.maximized);
            viewport.fullscreen = Some(self.fullscreen);
            let frame = self.frame.clone();
            let mut out = self.ctx.run_ui(input, |ui| {
                // Content that takes drags right up to the edges, as a title bar or a canvas does.
                let _ = ui.put(WINDOW, egui::Button::new("content").sense(Sense::click_and_drag()));
                edges(ui, &frame);
            });
            // No renderer here to take the font atlas.
            out.textures_delta.clear();
            let commands = out.viewport_output.get(&ViewportId::ROOT).map(|v| v.commands.clone()).unwrap_or_default();
            (commands, out.platform_output.cursor_icon)
        }

        /// Moves the pointer to `p` and lets hover settle; returns the cursor shown there.
        fn hover(&mut self, p: Pos2) -> CursorIcon {
            let mut cursor = CursorIcon::Default;
            for _ in 0..3 {
                cursor = self.frame(vec![Event::PointerMoved(p)]).1;
            }
            cursor
        }

        /// Presses at `p`, drags 30 points right and releases; returns the resizes it began.
        fn drag(&mut self, p: Pos2) -> Vec<ResizeDirection> {
            self.hover(p);
            let button = |pos, pressed| Event::PointerButton { pos, button: PointerButton::Primary, pressed, modifiers: egui::Modifiers::NONE };
            let mut commands = self.frame(vec![button(p, true)]).0;
            for dx in [5.0, 10.0, 20.0, 30.0] {
                commands.extend(self.frame(vec![Event::PointerMoved(p + vec2(dx, 0.0))]).0);
            }
            commands.extend(self.frame(vec![button(p + vec2(30.0, 0.0), false)]).0);
            commands
                .into_iter()
                .filter_map(|c| match c {
                    ViewportCommand::BeginResize(direction) => Some(direction),
                    _ => None,
                })
                .collect()
        }
    }

    fn app_frame() -> Frame {
        Frame::App(Layout::default())
    }

    #[test]
    #[cfg_attr(target_os = "macos", ignore = "AppKit resizes the window itself")]
    fn a_drag_on_an_edge_resizes_the_window_once() {
        let mut window = Window::new(app_frame());
        assert_eq!(window.drag(pos2(2.0, 150.0)), vec![ResizeDirection::West]);
        assert_eq!(window.drag(pos2(398.0, 298.0)), vec![ResizeDirection::SouthEast]);
        assert_eq!(window.drag(pos2(200.0, 1.0)), vec![ResizeDirection::North], "over the content's own drag");
        assert_eq!(window.drag(pos2(200.0, 150.0)), vec![], "the content keeps the middle");
    }

    #[test]
    #[cfg_attr(target_os = "macos", ignore = "AppKit resizes the window itself")]
    fn the_edges_show_the_resize_cursor() {
        let mut window = Window::new(app_frame());
        assert_eq!(window.hover(pos2(398.0, 150.0)), CursorIcon::ResizeEast);
        assert_eq!(window.hover(pos2(2.0, 2.0)), CursorIcon::ResizeNorthWest);
        assert_ne!(window.hover(pos2(200.0, 150.0)), CursorIcon::ResizeEast);
    }

    #[test]
    fn the_bands_stand_down_when_the_window_cannot_be_resized() {
        let mut desktop = Window::new(Frame::Desktop);
        assert_eq!(desktop.drag(pos2(2.0, 150.0)), vec![], "the desktop's frame has its own edges");
        let mut maximized = Window::new(app_frame());
        maximized.maximized = true;
        assert_eq!(maximized.drag(pos2(2.0, 150.0)), vec![]);
        let mut fullscreen = Window::new(app_frame());
        fullscreen.fullscreen = true;
        assert_eq!(fullscreen.drag(pos2(2.0, 150.0)), vec![]);
    }
}
