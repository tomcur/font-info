use std::path::{Path, PathBuf};

use thiserror::Error;

mod utils;

#[cfg(not(any(target_os = "macos", windows)))]
#[path = "./fontconfig.rs"]
mod system;

#[cfg(target_os = "macos")]
#[path = "./core_text.rs"]
mod system;

#[cfg(windows)]
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
            style: font.style,
            weight: font.weight,
            stretch: font.stretch,
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

/// Style of a font.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Style {
    /// Upright. Also known as "Roman".
    Normal,
    /// Italic style. Usually visually distinct from the normal style, rather than simply angled.
    Italic,
    /// Angle of the font in degrees
    Oblique(Option<f32>),
}

/// Weight class of a font, usually from 1 to 1000.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Weight(f32);

impl Weight {
    /// Weight corresponding to a CSS value of 100.
    pub const THIN: Self = Weight(100.);

    /// Weight corresponding to a CSS value of 200.
    pub const EXTRA_LIGHT: Self = Weight(200.);

    /// Weight corresponding to a CSS value of 300.
    pub const LIGHT: Self = Weight(300.);

    /// Weight corresponding to a CSS value of 350.
    pub const SEMI_LIGHT: Self = Weight(350.);

    /// Weight corresponding to a CSS value of 400.
    pub const NORMAL: Self = Weight(400.);

    /// Weight corresponding to a CSS value of 500.
    pub const MEDIUM: Self = Weight(500.);

    /// Weight corresponding to a CSS value of 600.
    pub const SEMI_BOLD: Self = Weight(600.);

    /// Weight corresponding to a CSS value of 700.
    pub const BOLD: Self = Weight(700.);

    /// Weight corresponding to a CSS value of 800.
    pub const EXTRA_BOLD: Self = Weight(800.);

    /// Weight corresponding to a CSS value of 900.
    pub const BLACK: Self = Weight(900.);

    /// Weight corresponding to a CSS value of 950.
    pub const EXTRA_BLACK: Self = Weight(950.);

    // Create the weight corresponding to the given CSS value.
    pub const fn new(weight: f32) -> Self {
        Weight(weight)
    }

    /// Get the corresponding CSS value of this weight.
    pub const fn value(self) -> f32 {
        self.0
    }
}

/// Stretch of a font.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Stretch(f32);

impl Stretch {
    /// Character width 50% of normal.
    pub const ULTRA_CONDENSED: Self = Stretch(0.5);

    /// Character width 62.5% of normal.
    pub const EXTRA_CONDENSED: Self = Stretch(0.625);

    /// Character width 75% of normal.
    pub const CONDENSED: Self = Stretch(0.75);

    /// Character width 87.5% of normal.
    pub const SEMI_CONDENSED: Self = Stretch(0.875);

    /// Character width 100% of normal.
    pub const NORMAL: Self = Stretch(1.0);

    /// Character width 112.5% of normal.
    pub const SEMI_EXPANDED: Self = Stretch(1.125);

    /// Character width 125% of normal.
    pub const EXPANDED: Self = Stretch(1.25);

    /// Character width 150% of normal.
    pub const EXTRA_EXPANDED: Self = Stretch(1.5);

    /// Character width 200% of normal.
    pub const ULTRA_EXPANDED: Self = Stretch(2.);

    /// Create the specified stretch as a factor of normal. 1.0 is normal, less than 1.0 is
    /// condensed, more than 1.0 is expanded.
    pub const fn new(stretch: f32) -> Self {
        Stretch(stretch)
    }

    /// Get the stretch value as a factor of normal. 1.0 is normal, less than 1.0 is condensed,
    /// more than 1.0 is expanded.
    pub const fn value(self) -> f32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Font<'c> {
    pub family_name: &'c str,
    pub font_name: &'c str,
    pub path: &'c Path,
    style: Style,
    weight: Weight,
    stretch: Stretch,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OwnedFont {
    family_name: String,
    font_name: String,
    path: PathBuf,
    style: Style,
    weight: Weight,
    stretch: Stretch,
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
