# Window chrome

An egui app either keeps the desktop's window frame (title bar, borders, window buttons) or
turns it off and draws its own. Neither is right everywhere: on KDE the desktop's frame is
expected, on GNOME (Wayland) there is none unless the app draws it, and winit's fallback
frame matches no desktop. `bevel-window` makes that decision in one place.

## Who draws the frame

| Platform | Frame |
| --- | --- |
| macOS, Windows | the desktop (the OS) |
| Linux/BSD on X11 | the desktop (the window manager) |
| Linux/BSD on Wayland **with** `zxdg_decoration_manager_v1` (KDE, Sway, Hyprland, COSMIC) | the desktop (the compositor) |
| Linux/BSD on Wayland **without** it (GNOME's Mutter) | the app |

```rust,ignore
let choice = bevel_window::Choice::probe(bevel_window::Prefix::new("MYAPP"));
let frame = choice.into_frame();
let viewport = bevel_window::decorate(egui::ViewportBuilder::default(), &frame);
```

`Choice::probe` runs once, before the window is created. On Wayland it opens a short-lived
connection on its own thread and reads the compositor's globals, with a 500 ms limit; on a
failure or a timeout it keeps the desktop's frame. It never touches the connection winit will
use.

`Frame` is either `Frame::Desktop` or `Frame::App(Layout)`. When the app draws the frame,
`Layout` says where the desktop puts the window buttons (`left` and `right`, from GNOME's
`button-layout`), so minimize, maximize and close land where the user expects them.

### Overriding

`MYAPP_TITLEBAR=app` forces the app's frame, `MYAPP_TITLEBAR=system` (or `desktop`) the
desktop's: the prefix is the one passed to `Prefix::new` (`BEVEL` by default). An app with a
"system title bar" preference checks `choice.overridden()` and its own setting.

## The app's own title bar

When the app draws over the title bar (a `Frame::App` window, or a macOS window with a hidden
title bar and full-size content), its top strip has to behave like one: a drag moves the
window, a double-click maximizes, and the controls on the strip still take their own clicks.

```rust,ignore
{{#include ../../bevel-examples/src/lib.rs:title_bar}}
```

`title_bar::empty_area` registers the strip's empty part, before the controls so they sit on
top:

| Where the drag or double-click starts | What happens |
| --- | --- |
| The strip's empty part | A drag moves the window; a double-click follows the system setting (macOS: zoom, minimize or nothing; elsewhere zoom) |
| A control on the strip | The control gets it |
| An in-window panel over the strip (a floating dock, an `egui::Window`) | The panel gets it |
| A modal's backdrop or a tooltip | The window moves, as under the system's own title bar |
| Anything, in full screen | Nothing happens to the window |

The window buttons are the app's to draw, in its own style, in the order `Layout` gives:

```rust,ignore
{{#include ../../bevel-examples/src/lib.rs:window_buttons}}
```

On **macOS**, AppKit moves a full-size-content window from a drag anywhere in the title band,
controls included, unless the window is not movable. Call `title_bar::sync_movable` once per
frame after the UI (see [Getting started](getting-started.md)): it turns the flag off while the
pointer is over a control.

## Resizing from the edges

An undecorated window has no edges to resize it from. `resize::edges` puts them back: a band
along each side and an L at each corner, inside the window. Call it last, so it wins over the
content under the edges:

```rust,ignore
{{#include ../../bevel-examples/src/lib.rs:resize_edges}}
```

The bands stand down when the desktop draws the frame, when the window is maximized or full
screen, and on macOS (AppKit resizes the window itself).

## One window per document

For apps that open each document in its own window, the crate keeps the shared bookkeeping:

| Type | What it does |
| --- | --- |
| `WindowList<T>` | Open windows with the app's data per window, in opening order and most-recent order; the active window, and which window to focus when one closes |
| `WindowId` | A window's id, never reused while the app runs: the key for per-window state |
| `cascade` | Places a new window one title bar right of and below the most recent one, staying on the monitor |
| `WindowMenu` | The Window menu: titles in opening order, `Name:1`, `Name:2` for windows on the same document, dirty and active marks |
| `Viewports` (`egui`) | Shows the windows as egui immediate viewports, with the frame decision and cascade placement, and reports focus and close requests |

Close and quit policy stay the app's: a window closes when the app removes it from the list.

[API: `bevel_window`](api/bevel_window/index.html)
