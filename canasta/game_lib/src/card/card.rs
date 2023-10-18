use super::{suit::Suit, rank::Rank};


pub struct PlayCard {
    id: u8,
    suit: Suit,
    rank: Rank,
    value: u8,
}

impl PlayCard {
    pub(crate) fn new(id: u8, suit: Suit, rank: Rank) -> Self {
        let value = calculate_card_value(&suit, &rank);
        Self { id, suit, rank, value }
    }

    pub(crate) fn is_red_three(&self) -> bool {
        (self.suit == Suit::Diamonds || self.suit == Suit::Hearts) && self.rank == Rank::Three
    }

    pub(crate) fn is_wild(&self) -> bool {
        self.rank == Rank::Two || self.rank == Rank::Joker
    }

    pub(crate) fn is_black_three(&self) -> bool {
        (self.suit == Suit::Clubs || self.suit == Suit::Spades) && self.rank == Rank::Three 
    }

    pub fn get_id(&self) -> u8 {
        self.id
    }

    pub fn suit(&self) -> &Suit {
        &self.suit
    }
    
    pub fn rank(&self) -> &Rank {
        &self.rank  
    }
}

fn calculate_card_value(suit: &Suit, rank: &Rank) -> u8 {
    let num: u8 = rank.into(); 
    if num <= 2 { return 20 }
    else if num == 3 {
        match suit {
            Suit::Spades | Suit::Clubs => return 5,
            Suit::Hearts | Suit::Diamonds => return 100,
        }
    }
    else if num <= 7 { return 5 }
    else if num <= 13 { return 10 }
    else { return 50 }
}


