use std::collections::BTreeMap;
use crate::{card::{PlayCard, Suit}, card_collections::meld::Meld};

pub(crate) struct Player {
    id: u8,
    hand: Vec<PlayCard>,
    melds: [Option<Meld>; 13],
    temp_melds: [Vec<PlayCard>; 13],
    red_threes: Vec<PlayCard>,
}

impl Player {
    pub(crate) fn new(id: u8) -> Self {
        Self { 
            id,
            hand: vec![],
            melds: Default::default(),
            temp_melds: Default::default(),
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

    /// Melds non wild cards into the temp melds 
    pub(crate) fn meld_normal(&mut self, cards: Vec<u8>) -> Result<(), ()> {
        let mut to_meld: Vec<PlayCard> = Vec::new();
        for to_meld_id in &cards {
            match self.hand.iter().position(|c| c.id() == *to_meld_id) {
                Some(i) => {
                    to_meld.push(self.hand.remove(i));
                    if to_meld.last().unwrap().is_wild() {
                        self.hand.extend(to_meld.drain(..));
                        return Err(())
                    }
                }
                None => {
                    self.hand.extend(to_meld.drain(..));
                    return Err(())
                }
            }
        }
        for card in to_meld {
            self.temp_melds[Into::<u8>::into(card.rank()) as usize].push(card);
        }
        Ok(())
    }
}


