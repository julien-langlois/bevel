# Fonts for every script

egui draws a character only if one of its installed faces has it. An app with only a Latin
face shows boxes for a Japanese file name, an Arabic layer name, or the whole interface in
Thai. Covering every script takes several faces, and the CJK ones are large (5–17 MB each).
bevel splits the problem in two:

- **at build time**, `bevel-fonts-build` decides which faces the app carries, and cuts the
  large ones down to the characters the app's translations use;
- **at run time**, `bevel_widgets::fonts::UiFonts` orders those faces for the UI language and
  installs them on the egui context.

## The stack, in order

For each family (proportional, monospace, and any named family such as a semibold one),
`UiFonts` builds the same fallback order in every app:

1. the app's own faces for the family (`app_face`), or egui's default faces;
2. the bundled faces for the UI language;
3. the bundled faces for every other script the app ships, so a Japanese interface still draws
   Arabic file names;
4. installed system fonts (feature `system_fonts`), at regular weight;
5. emoji faces, then egui's default faces.

When the app's faces don't cover the UI language's main script (Arabic, Thai, Georgian), that
script's face goes first, so it sets the row height and its marks are not clipped. Han,
kana and Hangul fit a Latin row, so Chinese, Japanese and Korean interfaces keep the Latin
face first. Han characters take the face of the language's region: Traditional for `zh-TW`,
Hong Kong forms for `zh-HK`, Japanese for `ja`, Korean for `ko`.

```rust,ignore
use bevel_widgets::fonts::{BundledFace, I18nScripts, UiFonts};
use egui::{FontData, FontFamily};

// Written by the app's build.rs (below).
static BUNDLED: &[BundledFace] = include!(concat!(env!("OUT_DIR"), "/craft_fonts.rs"));

fn install_fonts(ctx: &egui::Context, ui_language: &str) {
    UiFonts::new(&I18nScripts)
        .app_face("ui", FontData::from_static(UI_FONT), &[FontFamily::Proportional])
        .bundled_faces(BUNDLED)
        .languages(OFFERED_LANGUAGES)
        .ui_language(ui_language)
        .install(ctx);
}
```

Call `install` again when the UI language changes. `UiFonts::stack` returns the stack before
installing it, for an app that wants to edit a chain.

## Choosing the faces at build time

The faces come from a checkout of [craft-fonts](https://github.com/storytold/craft-fonts), a
collection of open-licensed faces with a manifest (family, style, scripts, licence, role).
The app never lists it in `Cargo.toml`: its `build.rs` reads a local checkout named by an
environment variable, so ordinary builds stay fast and offline.

| Environment | Result |
| --- | --- |
| `BEVEL_FONTS_DIR` unset or empty | An empty table: the app builds and runs with its own faces only |
| `BEVEL_FONTS_DIR` = a checkout | Every selected face, embedded with `include_bytes!` |
| `BEVEL_FONTS_REQUIRED=1` | A missing checkout or face is a build error (for release builds) |

```rust,ignore
// build.rs, with bevel-fonts-build in [build-dependencies] and its `subset` feature.
fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    let written = bevel_fonts_build::Embed::new()
        // CJK faces only as interface faces, cut to the characters the catalogs use.
        .subset_to(catalogs())
        .write("craft_fonts.rs");
    if let Err(e) = written {
        println!("cargo::error={e}");
    }
}
```

`subset_to` takes the paths of the app's translation catalogs: each CJK interface face keeps
only the characters they use (about 3 MB for five CJK faces with a real app's catalogs, where the
full faces take 5–17 MB each). The cut is reproducible: the same inputs give the same bytes. The build warns, per
catalog, about characters no embedded face covers. `scripts`, `filter` and
`Embed::documents` narrow the selection further.

## Web builds

A web build serves the faces next to the wasm instead of embedding them, so the wasm stays
small and an English interface never downloads Chinese. `Embed::serve(dir, url_prefix)` copies
the faces and writes a table of `ServedFace`s (URL, size, Subresource Integrity hash, and the
characters each face maps). `UiFonts::served_faces` takes that table instead of
`bundled_faces`; on the web, `Stack::install` then fetches:

- the UI language's faces at once;
- for a character no installed face has, the one face that draws it;
- an emoji face the first time text needs an emoji.

A failed download is retried with a growing delay. `fonts::loading(ctx)` is true while the UI
language's faces are on their way, for an app that prefers to draw nothing over Latin
fallbacks in its first frames.

## Checking coverage

`fonts::assert_covers` checks every shipped translation against the stack without system
fonts, and fails naming the language, message key and character that has no glyph: a test the
app runs with `BEVEL_FONTS_DIR` set, to see what a release embeds.

The full details (manifest format, interface faces, document fonts) are in the
[bevel-fonts-build README](https://github.com/julien-langlois/bevel/tree/main/bevel-fonts-build)
and the [bevel-widgets README](https://github.com/julien-langlois/bevel/tree/main/bevel-widgets#ui-fonts).

[API: `bevel_widgets::fonts`](api/bevel_widgets/fonts/index.html) ·
[`bevel_fonts_build`](api/bevel_fonts_build/index.html)
