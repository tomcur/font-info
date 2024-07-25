use std::path::PathBuf;

use dwrote::{
    FontCollection, FontStretch as DWriteStretch, FontStyle as DWriteStyle,
    FontWeight as DWriteWeight,
};

use crate::{Error, Font, Stretch, Style, Weight};

impl Style {
    fn from_direct_write(style: DWriteStyle) -> Self {
        match style {
            DWriteStyle::Normal => Style::Normal,
            DWriteStyle::Italic => Style::Italic,
            DWriteStyle::Oblique => Style::Oblique(None),
        }
    }
}

impl Weight {
    fn from_direct_write(weight: DWriteWeight) -> Self {
        match weight {
            DWriteWeight::Thin => Weight::THIN,
            DWriteWeight::ExtraLight => Weight::EXTRA_LIGHT,
            DWriteWeight::Light => Weight::LIGHT,
            DWriteWeight::SemiLight => Weight::SEMI_LIGHT,
            DWriteWeight::Regular => Weight::NORMAL,
            DWriteWeight::Medium => Weight::MEDIUM,
            DWriteWeight::SemiBold => Weight::SEMI_BOLD,
            DWriteWeight::Bold => Weight::BOLD,
            DWriteWeight::ExtraBold => Weight::EXTRA_BOLD,
            DWriteWeight::Black => Weight::BLACK,
            DWriteWeight::ExtraBlack => Weight::EXTRA_BLACK,
            DWriteWeight::Unknown(val) => Weight::new(val as f32),
        }
    }
}

impl Stretch {
    fn from_direct_write(stretch: DWriteStretch) -> Self {
        match stretch {
            DWriteStretch::UltraCondensed => Stretch::ULTRA_CONDENSED,
            DWriteStretch::ExtraCondensed => Stretch::EXTRA_CONDENSED,
            DWriteStretch::Condensed => Stretch::CONDENSED,
            DWriteStretch::SemiCondensed => Stretch::SEMI_CONDENSED,
            DWriteStretch::Normal => Stretch::NORMAL,
            DWriteStretch::SemiExpanded => Stretch::SEMI_EXPANDED,
            DWriteStretch::Expanded => Stretch::EXPANDED,
            DWriteStretch::ExtraExpanded => Stretch::EXTRA_EXPANDED,
            DWriteStretch::UltraExpanded => Stretch::ULTRA_EXPANDED,

            // hmmm...
            DWriteStretch::Undefined => Stretch::NORMAL,
        }
    }
}

pub fn all_fonts() -> Result<Box<[Font]>, Error> {
    let collection = FontCollection::system();

    let mut fonts = Vec::new();

    for family in collection.families_iter() {
        for idx in 0..family.get_font_count() {
            let font = family.get_font(idx);
            let face = font.create_font_face();

            // check: are there ever multiple files in a face?
            let path = face.get_files()[0].get_font_file_path().unwrap();
            fonts.push(Font {
                family_name: font.family_name(),
                font_name: font.face_name(),
                path,
                style: Style::from_direct_write(font.style()),
                weight: Weight::from_direct_write(font.weight()),
                stretch: Stretch::from_direct_write(font.stretch()),
            })
        }
    }

    Ok(fonts.into_boxed_slice())
}
