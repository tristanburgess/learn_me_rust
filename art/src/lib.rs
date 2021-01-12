//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Purple,
        Green,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> Option<SecondaryColor> {
        match c1 {
            PrimaryColor::Red => match c2 {
                PrimaryColor::Red => None,
                PrimaryColor::Yellow => Some(SecondaryColor::Orange),
                PrimaryColor::Blue => Some(SecondaryColor::Purple),
            },
            PrimaryColor::Yellow => match c2 {
                PrimaryColor::Red => Some(SecondaryColor::Orange),
                PrimaryColor::Yellow => None,
                PrimaryColor::Blue => Some(SecondaryColor::Green),
            },
            PrimaryColor::Blue => match c2 {
                PrimaryColor::Red => Some(SecondaryColor::Purple),
                PrimaryColor::Yellow => Some(SecondaryColor::Green),
                PrimaryColor::Blue => None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
