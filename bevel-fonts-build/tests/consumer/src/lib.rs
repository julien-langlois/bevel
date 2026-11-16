#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

//! The fixture faces as an app's crate sees them; read by `bevel-fonts-build`'s tests.

use bevel_fonts_build::{EmbeddedFace, ServedFace};

pub static BEVEL_FONTS: &[EmbeddedFace] = include!(concat!(env!("OUT_DIR"), "/craft_fonts.rs"));

pub static BEVEL_FONTS_SERVED: &[ServedFace] = include!(concat!(env!("OUT_DIR"), "/craft_fonts_served.rs"));
