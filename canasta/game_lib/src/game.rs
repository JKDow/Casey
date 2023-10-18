use crate::{player::Player, card_collections::{deck::Deck, discard::Discard}, game_builder::GameBuilder};


pub struct CanastaGame {
    game_id: u32,
    players: Vec<Player>,
    deck: Deck,
    discard: Discard,
    full_game: bool,    
    canastas_go_out: u8,    
}

impl CanastaGame {
    pub(crate) fn new(players: u8, canstas: u8, full_game: bool) -> Self {
        let mut game = Self { 
            game_id: 0, 
            players: vec![],
            deck: Deck::new(), 
            discard: Discard::new(), 
            full_game,
            canastas_go_out: canstas,
        };

        for i in 0..players {
            game.players.push(Player::new(i))
        }

        let deal = 
            if players == 2 { 15 } 
            else if players == 3 { 13 }
            else { 11 };

        if deal * players >= game.deck.remaining() as u8 { panic!("Too many players") }

        for _ in 0..deal {
            for player in &mut game.players {
                player.add_hand(game.deck.draw().unwrap())
            }
        }

        let mut valid_turn = false;

        while !valid_turn {
            let card = game.deck.draw().unwrap();
            if !(card.is_wild() || card.is_red_three() || card.is_black_three()) {
                valid_turn = true;
            }
            game.discard.throw(card);
        }
        game
    }

    pub fn builder() -> GameBuilder {
        GameBuilder::new()
    }

    pub fn draw(&mut self) {
        todo!()
    }

    pub fn discard(&mut self) {
        todo!()
    }

    pub fn meld(&mut self) {
        todo!()
    }

    pub fn take_discard(&mut self) {
        todo!()
    }
}
