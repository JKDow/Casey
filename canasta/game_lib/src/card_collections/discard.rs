use crate::card::PlayCard;


pub(crate) struct Discard {
    cards: Vec<PlayCard>,
    frozen: bool,
}

impl Discard {
    pub(crate) fn new() -> Self {
        Self { cards: vec![], frozen: false }
    }

    pub(crate) fn top(&self) -> Option<&PlayCard> {
        self.cards.last() 
    }

    pub(crate) fn throw(&mut self, card: PlayCard) {
        if card.is_wild() { self.frozen = true }
        self.cards.push(card)
    }

    pub(crate) fn take(&mut self) -> Vec<PlayCard> {
        std::mem::take(&mut self.cards)   
    }

    pub(crate) fn is_frozen(&self) -> bool {
        self.frozen
    }
}
