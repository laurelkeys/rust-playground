//! # Art
//!
//! A library for modeling artistic concepts.

// @Note: by re-exporting items at the top level with `pub use`
// we can remove the internal organization from the public API.
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::kinds::RYB;
pub use self::utils::mix;

pub mod kinds {
    /// Either a primary or secondary color in the RYB color model.
    #[derive(Debug, PartialEq, Clone)]
    pub enum RYB {
        Primary(PrimaryColor),
        Secondary(SecondaryColor),
    }

    /// The primary colors according to the RYB color model.
    #[derive(Debug, PartialEq, Clone)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    #[derive(Debug, PartialEq, Clone)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two different primary colors in equal amounts
    /// to create a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> RYB {
        use PrimaryColor::*;
        use SecondaryColor::*;

        match c1 {
            Red => match c2 {
                Red => RYB::Primary(Red),
                Yellow => RYB::Secondary(Orange),
                Blue => RYB::Secondary(Purple),
            },

            Yellow => match c2 {
                Red => RYB::Secondary(Orange),
                Yellow => RYB::Primary(Yellow),
                Blue => RYB::Secondary(Green),
            },

            Blue => match c2 {
                Red => RYB::Secondary(Purple),
                Yellow => RYB::Secondary(Green),
                Blue => RYB::Primary(Blue),
            },
        }
    }
}

//
// Test functions.
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mix_different_colors() {
        use PrimaryColor::*;
        use SecondaryColor::*;

        let orange = RYB::Secondary(Orange);
        let green = RYB::Secondary(Green);
        let purple = RYB::Secondary(Purple);

        assert_eq!(mix(Red, Yellow), orange);
        assert_eq!(mix(Yellow, Red), orange);
        assert_eq!(mix(Red, Blue), purple);
        assert_eq!(mix(Blue, Red), purple);
        assert_eq!(mix(Yellow, Blue), green);
        assert_eq!(mix(Blue, Yellow), green);
    }

    #[test]
    fn mix_same_colors() {
        use PrimaryColor::*;

        assert_eq!(mix(Red, Red), RYB::Primary(Red));
        assert_eq!(mix(Yellow, Yellow), RYB::Primary(Yellow));
        assert_eq!(mix(Blue, Blue), RYB::Primary(Blue));
    }
}
