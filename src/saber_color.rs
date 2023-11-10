use std::fmt;

#[derive(PartialEq, Debug)]
pub enum SaberColor {
    Red,
    Green,
    Blue,
    Purple,
}

#[derive(PartialEq, Debug)]
pub enum Trait {
    Power,
    Peace,
    Wisdom,
    Defense,
}

impl SaberColor {
    pub fn from_trait(trait_: &Trait) -> SaberColor {
        match trait_ {
            Trait::Power => SaberColor::Red,
            Trait::Peace => SaberColor::Green,
            Trait::Wisdom => SaberColor::Blue,
            Trait::Defense => SaberColor::Purple,
        }
    }
}

impl fmt::Display for SaberColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color_str = match self {
            SaberColor::Blue => "Blue",
            SaberColor::Green => "Green",
            SaberColor::Red => "Red",
            SaberColor::Purple => "Purple",
        };
        write!(f, "{}", color_str)
    }
}

impl Trait {
    pub fn from_character(character: &str) -> Trait {
        match character {
            "power" => Trait::Power,
            "peace" => Trait::Peace,
            "wisdom" => Trait::Wisdom,
            "defense" => Trait::Defense,
            _ => panic!("invalid character"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_correct_color_for_trait() {
        assert_eq!(SaberColor::from_trait(&Trait::Power), SaberColor::Red);
        assert_eq!(SaberColor::from_trait(&Trait::Peace), SaberColor::Green);
        assert_eq!(SaberColor::from_trait(&Trait::Wisdom), SaberColor::Blue);
        assert_eq!(SaberColor::from_trait(&Trait::Defense), SaberColor::Purple);
    }

    fn returns_correct_trait_for_character() {
        assert_eq!(Trait::from_character("power"), Trait::Power);
        assert_eq!(Trait::from_character("peace"), Trait::Peace);
        assert_eq!(Trait::from_character("wisdom"), Trait::Wisdom);
        assert_eq!(Trait::from_character("defense"), Trait::Defense);
    }
}
