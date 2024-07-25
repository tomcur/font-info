use std::path::PathBuf;

use fontconfig::{Fontconfig, ObjectSet, Pattern};

use crate::{Error, Font, Stretch, Style, Weight};

impl Stretch {
    fn from_fc(width: i32) -> Self {
        match width {
            // probably 62.5%
            63 => Self::EXTRA_CONDENSED,
            // probably 86.5%
            87 => Self::SEMI_CONDENSED,
            // probably 112.5%
            113 => Self::SEMI_EXPANDED,
            _ => Self(width as f32 / 100.0),
        }
    }
}

impl Style {
    fn from_fc(slant: i32) -> Self {
        match slant {
            100 => Self::Italic,
            110 => Self::Oblique(None),
            _ => Self::Normal,
        }
    }
}

impl Weight {
    fn from_fc(weight: i32) -> Self {
        use fontconfig as fc;

        const MAPPING: &[(i32, Weight)] = &[
            (fc::FC_WEIGHT_THIN, Weight::THIN),
            (fc::FC_WEIGHT_THIN, Weight::THIN),
            (fc::FC_WEIGHT_EXTRALIGHT, Weight::EXTRA_LIGHT),
            (fc::FC_WEIGHT_LIGHT, Weight::LIGHT),
            (fc::FC_WEIGHT_BOOK, Weight::SEMI_LIGHT),
            (fc::FC_WEIGHT_REGULAR, Weight::NORMAL),
            (fc::FC_WEIGHT_MEDIUM, Weight::MEDIUM),
            (fc::FC_WEIGHT_DEMIBOLD, Weight::SEMI_BOLD),
            (fc::FC_WEIGHT_BOLD, Weight::BOLD),
            (fc::FC_WEIGHT_EXTRABOLD, Weight::EXTRA_BOLD),
            (fc::FC_WEIGHT_BLACK, Weight::BLACK),
            (fc::FC_WEIGHT_EXTRABLACK, Weight::EXTRA_BLACK),
        ];

        for idx in 1..MAPPING.len() {
            let (fc_weight_b, ot_weight_b) = MAPPING[idx];

            if weight == fc_weight_b {
                return ot_weight_b;
            }

            if weight < fc_weight_b {
                let (fc_weight_a, ot_weight_a) = MAPPING[idx - 1];
                let fc_weight_a = fc_weight_a as f32;
                let fc_weight_b = fc_weight_b as f32;
                let ot_weight_a = ot_weight_a.0;
                let ot_weight_b = ot_weight_b.0;

                let weight = weight as f32;

                let ot_weight = ot_weight_a
                    + (weight - fc_weight_a) / (fc_weight_a - fc_weight_b)
                        * (ot_weight_a - ot_weight_b);
                return Weight::new(ot_weight);
            }
        }

        // if weight is more than FC_WEIGHT_EXTRABLACK, default to Weight::EXTRA_BLACK
        Weight::EXTRA_BLACK
    }
}

pub fn all_fonts() -> Result<Box<[Font]>, Error> {
    let fc = Fontconfig::new().ok_or(Error::SystemCollection)?;

    let pattern = Pattern::new(&fc);
    let mut objects = ObjectSet::new(&fc);
    objects.add(fontconfig::FC_FAMILY);
    objects.add(fontconfig::FC_FULLNAME);
    objects.add(fontconfig::FC_FILE);
    objects.add(fontconfig::FC_SLANT);
    objects.add(fontconfig::FC_WEIGHT);
    objects.add(fontconfig::FC_WIDTH);
    let fonts = fontconfig::list_fonts(&pattern, Some(&objects));

    let fonts = fonts
        .iter()
        .filter_map(|font| {
            let family = font.get_string(fontconfig::FC_FAMILY)?;
            let name = font.get_string(fontconfig::FC_FULLNAME).unwrap_or("");
            let path = font.get_string(fontconfig::FC_FILE)?;

            // is it ok to assume these defaults when the value is missing?
            let slant = font.slant().unwrap_or(fontconfig::FC_SLANT_ROMAN);
            let weight = font.weight().unwrap_or(fontconfig::FC_WEIGHT_REGULAR);
            let width = font.width().unwrap_or(fontconfig::FC_WIDTH_NORMAL);

            Some(Font {
                family_name: family.to_owned(),
                font_name: name.to_owned(),
                path: PathBuf::from(path),
                style: Style::from_fc(slant),
                weight: Weight::from_fc(weight),
                stretch: Stretch::from_fc(width),
            })
        })
        .collect();

    Ok(fonts)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_weight_conversion() {
        use fontconfig as fc;

        use crate::Weight;

        assert_eq!(Weight::from_fc(fc::FC_WEIGHT_THIN), Weight::THIN);
        assert_eq!(Weight::from_fc(1), Weight::new(102.5));
        assert_eq!(Weight::from_fc(70), Weight::new(340.));
        assert_eq!(
            Weight::from_fc(fc::FC_WEIGHT_EXTRABLACK - 1),
            Weight::new(940.)
        );
        assert_eq!(
            Weight::from_fc(fc::FC_WEIGHT_EXTRABLACK + 1),
            Weight::EXTRA_BLACK
        );
        assert_eq!(Weight::from_fc(50000), Weight::EXTRA_BLACK);
    }
}
