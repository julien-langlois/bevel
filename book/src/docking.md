# Panels, panes and docking

`bevel-widgets` has two levels of panel management. Pick the one that matches the app.

## Panes around a document: `panes`

Document apps (an editor, a viewer) usually have one central document and task panes around
it. `panes::PaneHost` draws each open pane as an egui side or bottom panel, or as a floating
frame, with a title bar (title, ▾ menu, ×) and a resizable inner edge. Call it before the
app's `CentralPanel`: the document is never a pane.

The app owns which panes are open and where: every frame the host takes the open list and
each pane's `PaneState`, and returns `PaneRequest`s (`Close`, `Place` on an edge or floating, `Resize`, `FloatRect`,
`Focus`, `Before` to reorder) for the app to run through its own commands. `panes::Rail`
draws an icon strip at one edge that opens one pane at a time.

## A full docking layout: `docking`

Tool-heavy apps (a photo or vector editor) let the user rearrange every panel: split, tab,
stack, float. `docking::Layout<P>` holds that arrangement independently of the panels'
contents: a tree of split, tab and accordion nodes plus floating tab groups, where `P` is the
app's panel id type.

Every change goes through one transactional path:

```rust,ignore
// `action` comes from the renderer (a drop, a tab drag) or from the app's own commands.
match layout.apply(action) {
    // Committed. `closed` lists the panels the action closed, for the app to release.
    Ok(closed) => release(closed),
    // Rejected: the layout is unchanged.
    Err(error) => log::warn!("{error:?}"),
}
```

`apply` validates the complete layout, applies the action to a copy and commits only a valid
result: a rejected move cannot lose the panel being moved. `apply_with_permissions` also
checks the app's policy per panel (can it close, float, move). Moves, tab insertion and reorder,
side splits, detaching, activating, closing, revealing and resizing all use it.

- `Layout::location` remembers where a panel was before it is hidden; `restore` puts it back
  if that place still exists.
- With the `serde` feature the layout can be saved and loaded; validate a loaded layout before
  rendering it.
- `docking::DockArea` is an optional renderer: it takes the layout, styles, labels, panel
  permissions and a body callback per panel, and returns actions. An app with its own chrome
  can keep it and use only the model and `resolve_drop` (which drop zone the pointer is over).
- With `native-windows`, floating groups can move to their own OS windows (egui viewports).
- With `testing`, `docking::testing` drives a docking surface headlessly, for the app's own
  tests of its workflows.

[API: `bevel_widgets::docking`](api/bevel_widgets/docking/index.html) ·
[`bevel_widgets::panes`](api/bevel_widgets/panes/index.html)
