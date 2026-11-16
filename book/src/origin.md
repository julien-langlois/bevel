# Origin and licenses

## Origin

Most of `bevel-widgets`, `bevel-window`, `bevel-i18n` and `bevel-fonts-build` is forked from
[craft-libs](https://github.com/storytold/craft-libs), the shared libraries of the Craft
suite of creative apps. bevel differs in two ways:

- it builds on **egui from crates.io**, where craft-libs patches egui and winit onto its own
  fork, so it can be published; the few features that needed the fork (an IME composer, some
  font baseline tweaks) were removed or fall back;
- it uses its own names (`bevel-*` crates, `BEVEL_*` environment variables, `BundledFace`).

[UPSTREAM.md](https://github.com/julien-langlois/bevel/blob/main/UPSTREAM.md) lists every
change and how upstream fixes are picked. `bevel-theme` is bevel's own.

## Licenses

The code is licensed under either of the
[Apache License, Version 2.0](https://github.com/julien-langlois/bevel/blob/main/LICENSE-APACHE)
or the [MIT license](https://github.com/julien-langlois/bevel/blob/main/LICENSE-MIT), at your
option. The [NOTICE](https://github.com/julien-langlois/bevel/blob/main/NOTICE) file carries
the attributions of the forked code.

Two crates also ship material under its own license:

- `bevel-theme`: the Inter and JetBrains Mono fonts, under the SIL Open Font License 1.1;
- `bevel-i18n`: number, plural and collation data generated from CLDR, under the
  [Unicode License v3](https://www.unicode.org/license.txt).
