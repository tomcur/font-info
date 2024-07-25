use std::path::PathBuf;

use core_text::{
    font_collection,
    font_descriptor::{CTFontTraits, SymbolicTraitAccessors, TraitAccessors},
};

use crate::{Error, Font, Stretch, Style, Weight};

fn roughly_eq(a: f32, b: f32) -> bool {
    const EPSILON: f32 = 0.00001;

    (a - b).abs() <= EPSILON
}

impl Style {
    fn from_core_text(traits: &CTFontTraits) -> Self {
        let symbolic = traits.symbolic_traits();

        if symbolic.is_vertical() {
            return Style::Normal;
        }

        // TODO: check if this true if and only if the font is italic, and not, e.g., when it is
        // oblique
        if symbolic.is_italic() {
            return Style::Italic;
        }

        let angle_degrees = traits.normalized_slant() / 30. * 360.;
        Style::Oblique(Some(angle_degrees as f32))
    }
}

impl Weight {
    fn from_core_text(weight: f64) -> Self {
        const CT_ULTRA_LIGHT: f32 = -0.8;
        const CT_THIN: f32 = -0.6;
        const CT_LIGHT: f32 = -0.4;
        const CT_REGULAR: f32 = 0.;
        const CT_MEDIUM: f32 = 0.23;
        const CT_SEMI_BOLD: f32 = 0.3;
        const CT_BOLD: f32 = 0.4;
        const CT_HEAVY: f32 = 0.56;
        const CT_BLACK: f32 = 0.62;

        const MAPPING: &[(f32, Weight)] = &[
            (CT_ULTRA_LIGHT, Weight::new(50.)),
            (CT_THIN, Weight::THIN),
            (CT_LIGHT, Weight::LIGHT),
            (CT_REGULAR, Weight::NORMAL),
            (CT_MEDIUM, Weight::MEDIUM),
            (CT_SEMI_BOLD, Weight::SEMI_BOLD),
            (CT_BOLD, Weight::BOLD),
            (CT_HEAVY, Weight::EXTRA_BOLD),
            (CT_BLACK, Weight::BLACK),
        ];

        let ct_weight = weight as f32;

        if ct_weight <= CT_ULTRA_LIGHT {
            // TODO: perhaps interpolate below Weight::THIN up to a min of 1.?
            return Weight::new(50.);
        }
        for idx in 1..MAPPING.len() {
            let (ct_weight_b, ot_weight_b) = MAPPING[idx];

            if roughly_eq(ct_weight, ct_weight_b) {
                return ot_weight_b;
            }

            if ct_weight < ct_weight_b {
                let (ct_weight_a, ot_weight_a) = MAPPING[idx - 1];
                let ot_weight_a = ot_weight_a.0;
                let ot_weight_b = ot_weight_b.0;

                let ot_weight = ot_weight_a
                    + (ct_weight - ct_weight_a) / (ct_weight_a - ct_weight_b)
                        * (ot_weight_a - ot_weight_b);
                return Weight::new(ot_weight);
            }
        }

        // TODO: perhaps interpolate above Weight::BLACK up to a max of 1000.?
        Weight::BLACK
    }
}

impl Stretch {
    fn from_core_text(width: f64) -> Self {
        let stretch = (width + 1.0) * 4.0;

        // TODO: perhaps clamp this by rough equality checking to conventional stretch values.
        Stretch::new(stretch as f32)
    }
}

pub fn all_fonts() -> Result<Box<[Font]>, Error> {
    let collection = font_collection::create_for_all_families();

    let fonts = collection
        .get_descriptors()
        .ok_or(Error::SystemCollection)?
        .iter()
        .filter_map(|font| {
            let traits = font.traits();
            Some(Font {
                family_name: font.family_name(),
                font_name: font.font_name(),
                path: font.font_path()?,
                style: Style::from_core_text(&traits),
                weight: Weight::from_core_text(traits.normalized_weight()),
                stretch: Stretch::from_core_text(traits.normalized_width()),
            })
        })
        .collect();

    Ok(fonts)
}
