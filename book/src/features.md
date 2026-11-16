# Features, platforms and MSRV

## Cargo features

| Crate | Feature | Default | Adds |
| --- | --- | --- | --- |
| `bevel-theme` | `widgets` | yes | Styles for bevel-widgets' icon buttons |
| | `fonts` | | Inter and JetBrains Mono, embedded (about 1.5 MB) |
| `bevel-widgets` | `serde` | | Saving and loading docking layouts |
| | `native-windows` | | Floating panels in their own OS windows (egui viewports) |
| | `system_fonts` | | Installed fonts for characters the app's faces lack (desktop) |
| | `color_fonts` | | Colour glyphs (COLR, sbix, CBDT), e.g. colour emoji |
| | `testing` | | `docking::testing`, a headless driver for docking tests |
| | `screen-testing` | | `testing::Screens`: screenshot and input-method tests per language (desktop) |
| `bevel-window` | `egui` | | `decorate`, the title bar, edge resizing and `Viewports` |
| `bevel-i18n` | `collate` | | Locale sorting, with collation data embedded (about 1 MB) |
| `bevel-fonts-build` | `subset` | | Cutting faces to the app's characters (build-dependency only) |

Every feature is checked on its own in CI.

## Platforms

| | Linux (X11, Wayland) | macOS | Windows | Web (wasm32) |
| --- | --- | --- | --- | --- |
| `bevel-theme` | ✓ | ✓ | ✓ | ✓ |
| `bevel-widgets` | ✓ | ✓ | ✓ | ✓ (served fonts; no system fonts) |
| `bevel-window` | ✓ (Wayland probe) | ✓ (`sync_movable`) | ✓ | builds |
| `bevel-i18n` | ✓ | ✓ (Apple locale) | ✓ (registry) | ✓ (browser locale) |
| `bevel-fonts-build` | build scripts, on any host | | | |

## Versions

- **egui:** bevel 0.1 uses egui 0.37 from crates.io, and follows egui's releases.
- **Rust:** 1.95 or later, checked in CI. Raising it is a minor version bump while bevel
  is at 0.x.
- **Semver:** the crates are released together with one version; at 0.x, a breaking change
  bumps the minor version. CI compares the public API with the last release
  (cargo-semver-checks).
