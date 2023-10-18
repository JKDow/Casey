use std::collections::BTreeMap;
use crate::{card::{PlayCard, Suit}, card_collections::meld::Meld};

pub(crate) struct Player {
    pub(crate) id: u8,
    pub(crate) hand: Vec<PlayCard>,
    pub(crate) melds: [Option<Meld>; 13],
}

impl Player {
    pub(crate) fn new(id: u8) -> Self {
        Self { 
            id,
            hand: vec![],
            melds: [
                None, 
                None, 
                None, 
                None, 
                None, 
                None, 
                None, 
                None, 
                None, 
                None, 
                None, 
                None, 
                None
            ],
        }
    }

    pub(crate) fn add_hand(&mut self, card: PlayCard) {
        self.hand.push(card)
    }
}


