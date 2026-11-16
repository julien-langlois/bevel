# How bevel is put together

## The crates and what they depend on

```text
                 your app
     ┌──────────────┼───────────────┬──────────────┐
     ▼              ▼               ▼              ▼
bevel-theme ──▶ bevel-widgets   bevel-window   (build.rs)
 (widgets          │    │        (egui opt.)       │
  feature)         ▼    ▼                          ▼
           bevel-i18n  bevel-fonts-build ◀─────────┘
```

- **bevel-fonts-build** has no dependency (the subsetter only with `subset`), and no egui:
  a build script or an engine crate below the UI can use it.
- **bevel-i18n** depends on ICU4X, not on egui: the translation layer works the same in a
  command-line tool.
- **bevel-widgets** uses both: the font stack reads which scripts each language needs from
  bevel-i18n and takes the faces bevel-fonts-build embedded; number fields format through
  bevel-i18n.
- **bevel-theme** turns its tokens into bevel-widgets styles (feature `widgets`, on by
  default).
- **bevel-window** stands alone. Without its `egui` feature it is pure decision and
  bookkeeping logic.

All five share one version and are released together.

## Who owns what

bevel's controls are *immediate-mode helpers*, like egui's own widgets: they are rebuilt every
frame from the app's state and keep nothing that matters between frames (at most egui's own
per-widget memory, such as a drag in progress).

| The app owns | bevel gives back |
| --- | --- |
| Colors, sizes, fonts (or takes `bevel-theme`'s) | Styles as plain values: `IconButtonStyle`, `ResizeHandleStyle`, `WidgetPalette` |
| Labels, already translated | Accessible names and roles reported to AccessKit |
| Which tab is selected, which panes are open, the split sizes | `egui::Response`s, a resize `delta`, `PaneRequest`s, docking `Action`s |
| Commands: what a click does | Nothing runs on its own: the app reads the response and acts |
| Assets: icons, fonts, catalogs | Loaders and tables for the app's own files |

A resize is a good example. The handle reports "the user moved the divider by 12 points";
`SplitLayout::resized` proposes a new size that respects the minimums; the app stores it, or
not:

```rust,ignore
{{#include ../../bevel-examples/src/lib.rs:split}}
```

## Build time and run time

Two pieces of bevel do part of their work in the app's `build.rs`, so the binary carries only
what the app needs:

| | At build time | At run time |
| --- | --- | --- |
| Fonts | `bevel-fonts-build` reads a font checkout, picks the faces, cuts CJK faces to the characters of the app's catalogs, and writes a table of faces | `bevel_widgets::fonts::UiFonts` orders the faces per UI language and installs them on the egui context |
| Translations | The app's `build.rs` embeds its catalogs as a static table | `bevel_i18n::I18n` parses a locale on first use, then looks texts up with fallback |

On the web, the font faces are served next to the wasm instead of embedded, and fetched when
the interface needs them: see [Fonts for every script](fonts.md).

## Never crash, never block

The libraries forbid `unwrap`, `expect`, `panic!` and friends outside tests (crate-level
`deny` lints). Invalid input returns `None` or an inert widget: a split too small for its
minimums, a non-finite size, a docking move that would lose a panel. Work that can be slow
(probing the Wayland compositor, reading `gsettings`, indexing system fonts) runs on a
background thread with a time limit, and the UI uses a safe default until it answers.
