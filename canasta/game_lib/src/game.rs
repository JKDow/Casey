use crate::{player::Player, card_collections::{deck::Deck, discard::Discard}, game_builder::GameBuilder, card::PlayCard, errors::{player_action_error::PlayerActionError, game_error::GameError}};

/// Reflects the current phase of the game 
/// # Phases 
/// - `Draw` - The current player either needs to draw a card or take the pack
/// - `Meld` - The current player can meld cards and discard
/// - `TurnOver` - The turn is over, switch to next player
/// - `GameOver` - The game has ended 
pub enum TurnPhase {
    Draw, 
    Meld,
    TurnOver,
    GameOver,
}

pub struct CanastaGame {
    game_id: u32,
    players: Vec<Player>,
    deck: Deck,
    discard: Discard,
    full_game: bool,    
    canastas_go_out: u8,    
    current_player: u8,
    turn_phase: TurnPhase,
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
            current_player: players-1,
            turn_phase: TurnPhase::TurnOver,
        };

        for i in 0..players {
            game.players.push(Player::new(i))
        }

        let deal = 
            if players == 2 { 15 } 
            else if players == 3 { 13 }
            else { 11 };

        if deal * players >= game.deck.remaining() as u8 { todo!("Too many players") }

        for _ in 0..deal {
            for player in &mut game.players {
                player.add_hand(game.deck.draw().unwrap());
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

    pub(crate) fn end_game(&mut self) {
        todo!()
    }

    pub fn builder() -> GameBuilder {
        GameBuilder::new()
    }
    
    pub fn quick_hand() -> Self {
        CanastaGame::new(2, 1, false)
    }

    /// Returns which players turn it currently is 
    /// # Overview 
    /// Will return the player number for the curret player.
    ///
    /// This will start at player 0 and will change evert time a player 
    /// discards.
    ///
    /// Once the final player discards this will return to player 0.
    ///
    /// This is the number that should be entered into any player actions.
    /// # Returns 
    /// - `u8` - The player number for whos turn it is 
    pub fn get_current_player(&self) -> u8 {
        self.current_player
    }

    /// Attempt to draw a card for a player
    /// # Overview 
    /// For an entered player number attempt to draw a card.
    ///
    /// If it is the entered players turn this will draw a card from the deck
    /// and place it into their hand. 
    ///
    /// This will transition the phase of their turn into the `Meld` phase
    ///
    /// If a red three is drawn this will be placed into the player's red three
    /// pile and a new card will be drawn.
    ///
    /// If a draw is successful a reference to the newly drawn card will be returned.
    /// # Parameters 
    /// - `player` - the player number for the player drawing. 
    /// # Returns 
    /// - `Ok(&PlayCard)` - A successful draw has occured. 
    /// - `Err(PlayerActionError::NotPlayerTurn(u8)` - Was not the entered players turn, 
    /// error contains current player number as a u8.
    /// - `Err(PlayerActionError::IncorrectTurnPhase)` - Is the entered players turn but
    /// they have already drawn.
    /// # Example
    /// ```
    /// // Create the game
    /// let game = CanastaGame::quick_hand();
    /// // Find which players turn it is 
    /// let player = game.get_current_player();
    /// // Draw a card will work
    /// assert!(game.draw(player).is_ok());
    /// // Drawing again will fail as the player has already drawn
    /// assert!(game.draw(player).is_err());
    /// ```
    pub fn draw(&mut self, player: u8) -> Result<&PlayCard, PlayerActionError> {
        // Check that current game state is valid for request 
        if player != self.current_player { return Err(PlayerActionError::NotPlayerTurn(self.current_player)) }
        match self.turn_phase {
            TurnPhase::Draw => {}
            TurnPhase::Meld | TurnPhase::TurnOver => return Err(PlayerActionError::IncorrectTurnPhase),
            TurnPhase::GameOver => return Err(PlayerActionError::GameOver)
        }
        // get a card off the deck
        let card: PlayCard = loop {
            let card = match self.deck.draw() {
                Some(card) => card,
                None => {
                    self.end_game();
                    return Err(PlayerActionError::GameOver)
                }
            };
            // if the card is a red three then add to player and redraw
            if !card.is_red_three() {
                break card
            } else {
                let player = &mut self.players[player as usize];
                player.add_hand(card);
                player.meld_red_threes();
            }
        };
        self.turn_phase = TurnPhase::Meld;
        let player = &mut self.players[player as usize];
        Ok(player.add_hand(card))
    }

    /// Returns a reference to a players hand 
    /// # Overview 
    /// For the given player a reference will be returned to a slice of thier hand 
    ///
    /// If the player given is out of range an error will be returned 
    /// # Returns 
    /// - `Ok(&[PlayCard]) - Player is valid and a reference to their hand is given
    /// - `Err(GameError::InvalidPlayer)` - The player given was not a valid player number,
    /// likely out of bounds 
    /// # Examples 
    /// ```rust 
    /// // create quick game with 2 players 
    /// let game = CanastaGame::quick_hand();
    /// // check player hands 
    /// // 0 and 1 are valid as it's a 2 player hand
    /// assert!(game.get_hand(1).is_ok());
    /// // there are 2 players so 4 is invalid 
    /// assert!(game.get_hand(4).is_err());
    /// ```
    pub fn get_hand(&self, player: u8) -> Result<&[PlayCard], GameError> {
        match self.players.get(player as usize) {
            Some(player) => Ok(player.get_hand()),
            None => Err(GameError::InvalidPlayer),
        }
    }

    /// Discard a card of a given ID for a player
    /// # Overview 
    ///
    pub fn discard(&mut self, player: u8, card_id: u8) -> Result<&PlayCard, PlayerActionError> {
        // Check that current game state is valid for request 
        if player != self.current_player { return Err(PlayerActionError::NotPlayerTurn(self.current_player)) }
        match self.turn_phase {
            TurnPhase::Meld => {}
            TurnPhase::Draw | TurnPhase::TurnOver => return Err(PlayerActionError::IncorrectTurnPhase),
            TurnPhase::GameOver => return Err(PlayerActionError::GameOver)
        };
        let player = &mut self.players[player as usize];
        let card = match player.discard(card_id) {
            Some(card) => card,
            None => return Err(PlayerActionError::InvalidCard)
        };
        Ok(self.discard.throw(card))
    }

    pub fn meld(&mut self) {
        todo!()
    }

    pub fn take_discard(&mut self) {
        todo!()
    }
}



