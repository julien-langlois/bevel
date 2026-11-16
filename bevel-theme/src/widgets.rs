//! Adapters from the theme tokens to bevel-widgets styles.

use bevel_widgets::buttons::{ButtonVisuals, IconButtonStyle};
use egui::{Color32, CornerRadius, Stroke};

use crate::Theme;

impl Theme {
    /// A borderless icon button: rail items, toolbar icons, window buttons.
    /// Selected uses the rail's active background and the accent.
    pub fn icon_button(&self) -> IconButtonStyle {
        let p = &self.palette;
        let normal = ButtonVisuals { fill: Color32::TRANSPARENT, stroke: Stroke::NONE, icon: p.text_secondary };
        IconButtonStyle {
            normal,
            hovered: ButtonVisuals { fill: p.raised, icon: p.text, ..normal },
            pressed: None,
            selected: ButtonVisuals { fill: p.rail_active, icon: p.accent, ..normal },
            corner_radius: CornerRadius::same(self.radii.medium),
            focus_stroke: Stroke::new(1.0, p.accent),
        }
    }

    /// An outlined icon button, as next to a search field.
    pub fn outlined_icon_button(&self) -> IconButtonStyle {
        let p = &self.palette;
        let normal = ButtonVisuals { fill: Color32::TRANSPARENT, stroke: Stroke::new(1.0, p.border), icon: p.text_secondary };
        IconButtonStyle {
            normal,
            hovered: ButtonVisuals { fill: p.raised, icon: p.text, ..normal },
            pressed: None,
            selected: ButtonVisuals { fill: p.selection, stroke: Stroke::new(1.0, p.accent), icon: p.accent },
            corner_radius: CornerRadius::same(self.radii.medium),
            focus_stroke: Stroke::new(1.0, p.accent),
        }
    }
}
