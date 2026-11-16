# bevel-theme

Design tokens for [egui](https://github.com/emilk/egui) desktop apps, in dark and light,
applied to a context in one call.

A `Theme` holds:

| Token group | What it covers |
| --- | --- |
| `Palette` | Surfaces (chrome, rail, panel, content, raised, input), borders, three levels of text, accent, selection. Muted text keeps 4.5:1 contrast on panels and content. |
| `Tones` | Status colours as background/foreground pairs, named by hue (`green`, `purple`, `red`, `amber`, `teal`, `gray`, `blue`, `orange`), so the app maps its own meanings onto them. |
| `Radii` | Small (chips), medium (buttons, inputs), large (cards, menus). |
| `Spacing` | Item spacing, button padding, control height, panel margin. |

`Theme::apply` writes the tokens into every egui style of the context and keeps a copy that
`Theme::get` reads back, for what egui's `Style` has no slot for (status tones, rail and chrome
surfaces).

```rust,ignore
use bevel_theme::{Chip, Theme};

// Once, or when the user switches between dark and light.
let theme = Theme::dark();
theme.apply(ctx);

// Anywhere in the UI.
let theme = Theme::get(ui.ctx());
egui::Frame::new().fill(theme.palette.panel).show(ui, |ui| {
    ui.add(Chip::new("MERGED", theme.tones.purple).strong());
});
```

Override the tokens you want before applying: `Theme { text_size: 14.0, ..Theme::light() }`.
`panel_frame`, `content_frame` and `card_frame` give `egui::Frame`s in the theme's surfaces.

## Features

| Feature | Default | Adds |
| --- | --- | --- |
| `widgets` | yes | `Theme::icon_button` and `Theme::outlined_icon_button`: [bevel-widgets](https://crates.io/crates/bevel-widgets) icon button styles from the tokens. |
| `fonts` | no | `fonts::install`: Inter (regular, medium, semibold) and JetBrains Mono, embedded (about 1.5 MB), with `fonts::medium` and `fonts::semibold` for the heavier weights. |

## License

The code is under MIT OR Apache-2.0. The fonts in `fonts/` are under the SIL Open Font License
1.1 (`fonts/Inter-OFL.txt`, `fonts/JetBrainsMono-OFL.txt`); they are only compiled in with the
`fonts` feature.
