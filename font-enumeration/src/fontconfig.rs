use std::path::PathBuf;

use fontconfig::{Fontconfig, ObjectSet, Pattern};

use crate::{Error, OwnedFont};

pub fn all_fonts() -> Result<Box<[OwnedFont]>, Error> {
    let fc = Fontconfig::new().ok_or(Error::SystemCollection)?;

    let pattern = Pattern::new(&fc);
    let mut objects = ObjectSet::new(&fc);
    objects.add(fontconfig::FC_FAMILY);
    objects.add(fontconfig::FC_FULLNAME);
    objects.add(fontconfig::FC_FILE);
    let fonts = fontconfig::list_fonts(&pattern, Some(&objects));

    let fonts = fonts
        .iter()
        .filter_map(|font| {
            let family = font.get_string(fontconfig::FC_FAMILY)?;
            let name = font.get_string(fontconfig::FC_FULLNAME).unwrap_or("");
            let path = font.get_string(fontconfig::FC_FILE)?;

            Some(OwnedFont {
                family_name: family.to_owned(),
                font_name: name.to_owned(),
                path: PathBuf::from(path),
            })
        })
        .collect();

    Ok(fonts)
}
