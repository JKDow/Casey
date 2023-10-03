use rand::seq::SliceRandom;

use crate::card::{PlayCard, Rank, Suit};

pub(crate) struct Deck {
    cards: Vec<PlayCard>,
}

impl Deck {
    pub(crate) fn new() -> Self {
        let mut cards = Vec::new();
        for suit_num in 0..4 {
            for _ in 0..2 {
                for rank_num in 1..14 {
                    let id = cards.len() as u8;
                    let card = PlayCard::new(id, suit_num.try_into().unwrap(), rank_num.into());
                    cards.push(card);
                }
            }
            let card = PlayCard::new(cards.len() as u8, suit_num.try_into().unwrap(), Rank::Joker);
            cards.push(card);
        }
        let mut rng = rand::thread_rng();
        cards.shuffle(&mut rng);
        Self { cards }
    }

    pub(crate) fn draw(&mut self) -> Option<PlayCard> {
        self.cards.pop()
    } 
}
