use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl TryFrom<u8> for Suit {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Suit::Hearts),
            1 => Ok(Suit::Diamonds),
            2 => Ok(Suit::Clubs),
            3 => Ok(Suit::Spades),
            _ => Err("Invalid suit value"),
        }
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let suit = match self {
            Suit::Hearts => "Hearts",
            Suit::Diamonds => "Diamonds",
            Suit::Clubs => "Clubs",
            Suit::Spades => "Spades",
        };
        write!(f, "{}", suit)
    }
}
