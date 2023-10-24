use crate::card::{PlayCard, Rank};

pub(crate) struct Meld {
    rank: Rank,
    cards: Vec<PlayCard>,
    wilds: Vec<PlayCard>,
}

impl Meld {
    pub(crate) fn new(rank: Rank) -> Self {
        Meld { rank, cards: vec![], wilds: vec![] }
    }

    pub(crate) fn len(&self) -> usize {
        self.cards.len() + self.wilds.len()
    }

    pub(crate) fn normal_count(&self) -> usize {
        self.cards.len()
    }

    pub(crate) fn wild_count(&self) -> usize {
        self.wilds.len()
    }

    pub(crate) fn is_natural(&self) -> bool {
        self.wilds.is_empty()
    }

    pub(crate) fn can_add(&self, cards: &Vec<PlayCard>) -> Result<(), ()> {
        let mut wild_count = 0;
        for card in cards {
            if *card.rank() != self.rank && !card.is_wild() {
                return Err(());
            }
            if card.is_wild() { wild_count += 1; }
        }
        if wild_count + self.wilds.len() >= self.cards.len() + cards.len() - wild_count {
            // too many wilds 
            return Err(())
        }
        todo!("Check this covers all logic");
        Ok(())
    }
}
