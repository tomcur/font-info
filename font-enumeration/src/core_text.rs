use std::path::PathBuf;

use core_text::font_collection;

use crate::{Error, OwnedFont};

pub fn all_fonts() -> Result<Box<[OwnedFont]>, Error> {
    let collection = font_collection::create_for_all_families();

    let fonts = collection
        .get_descriptors()
        .ok_or(Error::SystemCollection)?
        .iter()
        .filter_map(|font| {
            Some(OwnedFont {
                family: font.family_name(),
                name: font.font_name(),
                path: font.font_path()?,
            })
        })
        .collect();

    Ok(fonts)
}
