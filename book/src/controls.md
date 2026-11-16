# Controls: buttons, tabs, splits

The controls in `bevel-widgets` take their colors from the app (or from
[`bevel-theme`](theme.md)), report themselves to screen readers through AccessKit, and work
with the keyboard. They never run a command: the app reads the response.

## Icon buttons

`buttons::IconButton` is a square button whose icon the app paints. It takes a style with
visuals for the normal, hovered, pressed and selected states; selection wins over press and
hover.

```rust,ignore
{{#include ../../bevel-examples/src/lib.rs:icon_button}}
```

- The **label** is the accessible name: screen readers announce it, and tests find the button
  by it (`harness.get_by_label("Close")`). Attach a tooltip to the response if you want one.
- The painter gets the button's rectangle and the tint for the current state; the icon can be
  an SVG from `icons::SvgIconSet`, an image, or shapes as in the gallery.
- Mouse, Enter and Space activate it; a disabled parent disables it, for painting, input and
  accessibility alike.

## SVG icon catalogs

`icons::SvgIconSet` prepares an app's embedded SVG icons once (substituting `currentColor`
and the stroke width) and hands egui their bytes under a URI per icon. The app installs
`egui_extras`' SVG loader and keeps its own assets; bevel bundles no icons.

## Tabs

Tabs are two separate jobs: **fitting** them in a strip, and **interacting** with each one.

- `tabs::fit(natural_widths, selected, available, minimum, overflow_width)` shrinks the widest
  tabs first, down to `minimum`, then sends the rest to an overflow menu. The selected tab
  always stays visible. `tabs::fit_whole` keeps every visible tab at its natural width
  instead.
- `tabs::Tab` registers one tab at a rectangle the app chose, reports the tab role, label and
  selection, handles the keyboard, and lets the app paint it.

```rust,ignore
{{#include ../../bevel-examples/src/lib.rs:tabs}}
```

The returned response carries clicks, double-clicks and drag phases, so a strip can reorder,
tear off or collapse tabs the way its app does.

## Splits and resize handles

`layout::split_rect` divides a rectangle in two along an axis, with a gap and a minimum for
each side. The size is a `SplitSize`: a ratio, a fixed first side, or a fixed second side.
`layout::ResizeHandle` is the divider: pointer drags, the arrow keys while it has focus, and
screen readers' increment and decrement actions all come back as one `delta`.

```rust,ignore
{{#include ../../bevel-examples/src/lib.rs:split}}
```

`SplitLayout::resized` applies the delta while keeping the sizing mode (a ratio stays a ratio)
and the minimums. Tiny or invalid geometry gives `None` rather than a panic, and the app keeps
its stored size.

## Toolbar overflow

`layout::fit_toolbar` keeps an ordered prefix of toolbar groups at their natural widths and
reports which groups go to an overflow menu, reserving the menu button's width. It never
compresses a control or skips a wide group.

## Number fields

`localized::NumberField` and `localized::LocalizedDragValue` write numbers the way the user's
region does (digits, decimal mark, grouping) and read back what people type, with
[`bevel-i18n`](i18n.md) doing the formatting. Text that is not a number never changes the
value: it stays in the field, outlined, with the reason in a tooltip.

[API: `bevel_widgets`](api/bevel_widgets/index.html)
