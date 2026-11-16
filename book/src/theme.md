# Theme

`bevel-theme` is a set of design tokens: named values for every color, radius and spacing
an app's screens use, in a dark and a light variant.

| Token group | What it covers |
| --- | --- |
| `Palette` | Surfaces from the outside in (`chrome`, `rail`, `panel`, `content`, `raised`, `input`), `border` and `border_soft`, three levels of text, `accent`, `accent_fill`, `selection`, `rail_active` |
| `Tones` | Status colors as background/foreground pairs, named by hue: `green`, `purple`, `red`, `amber`, `teal`, `gray`, `blue`, `orange` |
| `Radii` | `small` (chips), `medium` (buttons, inputs), `large` (cards, menus) |
| `Spacing` | Item spacing, button padding, control height, panel margin |
| `text_size` | The body size; headings and captions are derived from it |

The palettes are checked for contrast: muted text keeps 4.5:1 on panels and content, and
white text keeps 4.5:1 on `accent_fill`.

## Applying it

```rust,ignore
{{#include ../../bevel-examples/src/lib.rs:theme}}
```

`apply` does two things:

1. It writes the tokens into **every egui style of the context** (dark and light), so egui's
   own widgets (buttons, checkboxes, text fields, sliders, scroll bars) follow them. The
   "egui controls" card of the gallery is plain egui.
2. It **stores a copy on the context**, which `Theme::get` returns anywhere in the UI. This
   is how components get at what egui's `Style` has no slot for: status tones, the rail and
   chrome surfaces.

To customize, start from a variant and change fields before applying:

```rust,ignore
let theme = Theme { text_size: 14.0, ..Theme::light() };
theme.apply(ctx);
```

`panel_frame`, `content_frame` and `card_frame` give `egui::Frame`s in the theme's surfaces.

## Chips

A `Chip` is a small rounded label in a status tone: badges, tags, states. Tones are named by
hue rather than meaning, so each app maps its own states onto them.

```rust,ignore
{{#include ../../bevel-examples/src/lib.rs:chips}}
```

`strong()` uses the semibold weight, `pill()` fully rounded ends, `glyph("●")` a leading
symbol, `font(…)` another size.

## Fonts

With the `fonts` feature, `bevel_theme::fonts::install(ctx)` installs Inter (regular, medium,
semibold) and JetBrains Mono, about 1.5 MB embedded. egui draws one weight per family, so the
heavier weights are their own families: `fonts::medium(size)` and `fonts::semibold(size)`
return a `FontId` in them. egui's default fonts stay behind them for symbols and emoji.

For scripts beyond Latin, Greek and Cyrillic, see [Fonts for every script](fonts.md).

## Styles for bevel-widgets

With the `widgets` feature (default), the theme builds the styles bevel-widgets' controls
take: `theme.icon_button()` for borderless icon buttons (rail, toolbars, window buttons) and
`theme.outlined_icon_button()` for buttons next to a field.

[API: `bevel_theme`](api/bevel_theme/index.html)
