# Right-to-left interfaces

In Arabic, Hebrew, Persian or Urdu, an interface reads from the right, and desktop apps
mirror their chrome: the panel on the left moves to the right, rows and menu bars start at
the right, menus open under their button's right edge, back arrows point right. Content keeps
its geometry: a map, a timeline, a media player's controls, a chart do not mirror.

egui has no direction for a whole context: panels, popups and hand-placed positions name a
side. `bevel_widgets::direction` lets call sites name the *reading start* and *end* instead.

## Setting the direction

```rust,ignore
use bevel_widgets::direction::{self, Setting};

// At startup and whenever the UI language changes.
let dir = direction::resolve(ui_language, Setting::Language); // or the user's Ltr / Rtl choice
direction::apply(ctx, dir);
```

`resolve` takes, in order: the `BEVEL_UI_DIRECTION` environment variable (`rtl` or `ltr`, to
check a mirrored layout before any right-to-left translation exists), then the app's
`Setting`, then the language's script (through `bevel_i18n::direction`). `apply` stores the
result on the context. Call it again after installing a whole new `egui::Style`.

## Using it

Every helper reads the direction from a `Context`, a `Ui` or a `Direction`:

- `direction::start_panel` / `end_panel`: an egui side panel at the reading start or end;
- `direction::row`: a horizontal row that starts at the reading start; rows nested in it follow;
- `direction::start_x`, `end_x`, `start_align`: positions and alignments for hand-painted UI;
- `direction::popup`: a popup's `RectAlign`, flipped right to left so menus open under their
  button's reading-start edge;
- `direction::icon`: which icon to draw for a name right to left: its mirror-image counterpart
  (`SWAPPED_ICONS`), the same drawing flipped (`FLIPPED_ICONS`, with `Icon::uv`), or the icon
  as it is (`KEPT_ICONS`);
- `direction::unmirrored_row`: content that keeps its left-to-right geometry (`Unmirrored`
  lists the cases).

Screen tests run Arabic laid out from the right: see [Testing an app's screens](testing.md).

> **Known limitation.** egui from crates.io does not reorder bidirectional text: a run of
> Latin letters or digits inside Arabic text (a file name, a year) can show in logical rather
> than visual order. The chrome mirrors; mixed-direction text is egui's to fix.

[API: `bevel_widgets::direction`](api/bevel_widgets/direction/index.html)
