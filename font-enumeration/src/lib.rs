use std::path::{Path, PathBuf};

use thiserror::Error;

mod utils;

#[cfg(not(any(target_os = "macos", windows)))]
#[path = "./fontconfig.rs"]
mod system;

#[cfg(target_os = "macos")]
#[path = "./core_text.rs"]
mod system;

#[cfg(target_os = "macos")]
#[path = "./direct_write.rs"]
mod system;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Could not initialize system collection")]
    SystemCollection,
}

pub struct Collection {
    // Using a boxed slice rather than Vec saves [Collection] from having to store a capacity
    all_fonts: Box<[OwnedFont]>,
}

impl Collection {
    pub fn new() -> Result<Self, Error> {
        let all_fonts = system::all_fonts()?;

        Ok(Self { all_fonts })
    }

    pub fn all<'c>(&'c self) -> impl Iterator<Item = Font<'c>> {
        self.all_fonts.iter().map(move |font| Font {
            family_name: &font.family_name,
            font_name: &font.font_name,
            path: &font.path,
        })
    }

    pub fn by_family<'c, 'f>(&'c self, family_name: &'f str) -> impl Iterator<Item = Font<'c>> + 'f
    where
        'c: 'f,
    {
        self.all()
            .filter(|font| utils::case_insensitive_match(font.family_name, family_name))
    }

    pub fn take(self) -> Vec<OwnedFont> {
        self.all_fonts.into_vec()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Font<'c> {
    pub family_name: &'c str,
    pub font_name: &'c str,
    pub path: &'c Path,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct OwnedFont {
    family_name: String,
    font_name: String,
    path: PathBuf,
}

#[cfg(test)]
mod test {
    use super::Collection;

    #[test]
    fn has_fonts() {
        let collection = Collection::new().unwrap();

        // is this a reasonable assumption?
        assert!(!collection.take().is_empty());
    }
}
