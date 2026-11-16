# Upstream

Part of bevel is a fork of [craft-libs](https://github.com/storytold/craft-libs)
(MIT OR Apache-2.0), taken once and then maintained here. Changes from upstream are
picked by hand, not merged wholesale.

Forked at commit `bd97cb760c36c4398def923817702a7e7f3096af`.

| bevel crate | From | Changes |
| --- | --- | --- |
| `bevel-widgets` | `crates/ui` (`craft-ui`) | Renamed. Removed the `ime`, `printing` and `capture` modules and the docking IME hooks, which need the craft-libs fork of egui. |
| `bevel-window` | `crates/window` (`craft-window`) | Renamed. |
| `bevel-i18n` | `crates/i18n` (`craft-i18n`) | Renamed. |
| `bevel-fonts-build` | `crates/fonts-build` (`craft-fonts-build`) | Renamed. |
| `bevel-theme` | — | bevel's own. |

## Differences that matter

- **egui comes from crates.io.** craft-libs patches egui and winit onto its own fork;
  bevel does not, so it can be published. The upstream code already falls back when
  the fork's extra APIs are missing (font baseline tweaks, IME context); only the IME
  composer could not, hence its removal.
- **Names.** "Craft" and the app names are storytold's; bevel uses none of them in
  crate names. Environment variables and the names suggested to apps take the `BEVEL_`
  prefix where upstream has `CRAFT_` (`BEVEL_FONTS_DIR`, `BEVEL_FONTS_REQUIRED`,
  `BEVEL_TITLEBAR`, `BEVEL_UI_DIRECTION`, …): rename them when picking an upstream change.
  In bevel-widgets' `fonts`, upstream's `CraftFace`, `UiFonts::craft_faces` and
  `SystemFonts::{BeforeCraft, AfterCraft}` are `BundledFace`, `UiFonts::bundled_faces` and
  `SystemFonts::{BeforeBundled, AfterBundled}`. The craft-fonts and craft-i18n data
  repositories keep their names, as does the `craft_fonts.rs` file the examples generate.

## Picking an upstream change

1. Read the upstream commit; skip it if it touches only removed modules or the egui fork.
2. Apply it with the crate names above, run `cargo test --workspace --features bevel-window/egui`.
3. Record the upstream commit in the changelog of the crate it lands in.
