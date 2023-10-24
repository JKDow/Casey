use std::collections::BTreeMap;
use crate::{card::{PlayCard, Suit, Rank}, card_collections::meld::Meld, errors::internal_meld_error::InternalMeldError};

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

    /// attempts to meld a list of card IDs 
    ///
    /// If the card cannot be melded or the given ID is not in the players hand 
    /// then the whole operation will fail and an error will be returned 
    pub(crate) fn meld(&mut self, cards: Vec<u8>, rank: Rank) -> Result<(), InternalMeldError> {
        let mut to_meld: Vec<PlayCard> = Vec::new();
        for to_meld_id in &cards {
            let hand_index = match self.hand.iter().position(|c| c.id() == *to_meld_id) {
                Some(i) => i,
                None => {
                    self.hand.extend(to_meld);
                    return Err(InternalMeldError::InvalidCardId(*to_meld_id))
                }
            };
            let card = &self.hand[hand_index];
            if card.is_wild() { to_meld.push(self.hand.remove(hand_index)); }
            else if card.is_red_three() { 
                self.hand.extend(to_meld.drain(..));
                return Err(InternalMeldError::InvalidCardToMeld(*to_meld_id))
            }
            else if *card.rank() != rank {
                self.hand.extend(to_meld);
                return Err(InternalMeldError::IncorrectRank(*to_meld_id))
            }
            else { to_meld.push(self.hand.remove(hand_index)); }
        }
        self.temp_melds[Into::<usize>::into(&rank)].extend(to_meld);
        Ok(())
    }

    /// View a temp meld for a given rank 
    pub(crate) fn view_temp_meld(&self, rank: Rank) -> &[PlayCard] {
        &self.temp_melds[Into::<usize>::into(&rank)]
    }

    /// View a list of all temp melds 
    ///
    /// The index of the meld aligns with the cards rank
    pub(crate) fn view_all_temp(&self) -> [&[PlayCard]; 13] {
        let mut slices: [&[PlayCard]; 13] = Default::default();
        for (i, meld) in self.temp_melds.iter().enumerate() {
            slices[i] = meld;
        }
        slices
    }

    /// Remove cards of some id from the temp list 
    ///
    /// If any fail it will return an error containing the failed IDs
    pub(crate) fn remove_from_temp(&mut self, cards: Vec<u8>) -> Result<(), Vec<u8>> {
        let failed_remove: Vec<u8> = cards.iter().filter_map(|&id| {
            self.temp_melds.iter_mut()
                .find_map(|meld| meld.iter().position(|c| c.id() == id).map(|pos| meld.remove(pos)))
                .map_or(Some(id), |card| {
                    self.hand.push(card);
                    None
                })
        }).collect();
        failed_remove.is_empty().then(|| ()).ok_or(failed_remove)
    }

    pub(crate) fn clear_temp_meld(&mut self) { 
        self.temp_melds.iter_mut().for_each(|meld| {
            self.hand.extend(meld.drain(..))
        })
    }

    pub(crate) fn commit_meld(&mut self) -> Result<(), ()> {
         
        
        Ok(())
    }
}


