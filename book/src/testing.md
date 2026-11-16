# Testing an app's screens

egui apps can be tested headlessly with [egui_kittest](https://crates.io/crates/egui_kittest):
it runs the UI without a window, finds widgets by their accessible names, clicks and types,
and renders screenshots. bevel's controls report names and roles to AccessKit, so they are
easy to reach from such tests.

## Finding and driving bevel's controls

The gallery's tests show the pattern: build a harness around the app's UI, run a frame, and
query by label.

```rust,ignore
{{#include ../../bevel-examples/tests/screenshots.rs:harness_and_test}}
```

Icon buttons are found by their label (`"Close"`, `"Light theme"`), tabs by their text, resize
handles by theirs, all independent of the pixels drawn.

## Screens in every test language

With the `screen-testing` feature, `bevel_widgets::testing::Screens` renders a screen once per
test language: Arabic, Japanese, Traditional Chinese (Taiwan), Korean, Thai and Vietnamese.
Each run installs the app's font stack for that language with system fonts off, so:

- a snapshot looks the same on every machine;
- a character that no face draws fails the test, naming it;
- Arabic is laid out from the right.

```rust,ignore
use bevel_widgets::testing::{KOREAN, Screens, compose};

let screens = Screens::new("rename").size([320.0, 150.0]).fonts(|fonts| fonts.bundled_faces(FACES));
// Compares tests/snapshots/rename-ar.png, rename-ja.png, …; UPDATE_SNAPSHOTS=1 rewrites them.
screens.snapshots(|ui, locale| rename_dialog(ui, locale));

// IME smoke test: a Korean input method composing 한 into the focused field.
let mut harness = screens.harness_state("ko", |ui, locale, dialog: &mut Dialog| dialog.ui(ui, locale), Dialog::default());
harness.get_by_role(egui::accesskit::Role::TextInput).focus();
compose(&mut harness, &KOREAN);
assert_eq!(harness.state().name, "한");
```

`compose` sends the preedit and commit events an input method sends (`KOREAN`, `JAPANESE`,
`PINYIN`, `TELEX`).

## Docking workflows

With the `testing` feature, `bevel_widgets::docking::testing` drives a docking surface
without a window: drag a tab, drop it on a zone, and check the resulting layout.

## Rendering needs a GPU adapter

Snapshots render with wgpu. On CI runners without a GPU, Mesa's software Vulkan driver
(`mesa-vulkan-drivers` on Ubuntu) provides one: bevel's own CI does this.
