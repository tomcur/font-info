use std::path::PathBuf;

use dwrote::FontCollection;

use crate::{Error, OwnedFont};

pub fn all_fonts() -> Result<Box<[OwnedFont]>, Error> {
    let collection = FontCollection::system();

    let mut fonts = Vec::new();

    for family in collection.families_iter() {
        for idx in 0..family.get_font_count() {
            let font = family.get_font(idx);
            let face = font.create_font_face();

            // check: are there ever multiple files in a face?
            let path = face.get_files()[0].get_font_file_path().unwrap();
            fonts.push(OwnedFont {
                family_name: font.family_name(),
                font_name: font.face_name(),
                path,
            })
        }
    }

    Ok(fonts.into_boxed_slice())
}
