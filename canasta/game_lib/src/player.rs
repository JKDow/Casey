use std::collections::BTreeMap;
use crate::{card::{PlayCard, Suit}, card_collections::meld::Meld};

pub(crate) struct Player {
    id: u8,
    hand: Vec<PlayCard>,
    melds: [Option<Meld>; 13],
    red_threes: Vec<PlayCard>,
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
            red_threes: vec![],
        }
    }

    pub(crate) fn add_hand(&mut self, card: PlayCard) -> &PlayCard {
        self.hand.push(card);
        self.hand.last().unwrap()
    }
    
    /// Melds red threes in a players hand 
    /// Returns the number of cards melded 
    pub(crate) fn meld_red_threes(&mut self) -> u8 {
        let mut counter = 0;
        for i in (0..self.hand.len()).rev() {
            if self.hand[i].is_red_three() { 
                self.red_threes.push(self.hand.remove(i))
            }
        }
        counter 
    }

    pub(crate) fn get_hand(&self) -> &[PlayCard] {
        &self.hand
    }

    /// Throw the card of some ID
    ///
    /// If the card is found it is removed from the hand and returned 
    ///
    /// If it is not then `None` will be returned 
    pub(crate) fn discard(&mut self, card_id: u8) -> Option<PlayCard> {
        for i in 0..self.hand.len() {
            if self.hand[i].id() == card_id {
                return Some(self.hand.remove(i))
            }
        }
        None
    }
}


