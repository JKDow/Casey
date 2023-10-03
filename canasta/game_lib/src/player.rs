use std::collections::BTreeMap;
use crate::{card::{PlayCard, Suit}, card_collections::meld::Meld};

pub(crate) struct Player {
    pub(crate) id: u8,
    pub(crate) hand: Vec<PlayCard>,
    pub(crate) melds: [Option<Meld>; 13],
}


