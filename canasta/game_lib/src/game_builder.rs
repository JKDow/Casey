use crate::game::CanastaGame;


pub struct GameBuilder {
    num_players: Option<u8>,
    num_canastas: Option<u8>,
    full_game: Option<bool>,
}

impl GameBuilder {
    pub(crate) fn new() -> Self {
        Self {
            num_players: None,
            num_canastas: None,
            full_game: None,
        }
    }
    
    pub fn players(mut self, players: u8) -> Self {
        self.num_players = Some(players);
        self
    }

    pub fn canastas(mut self, canastas: u8) -> Self {
        self.num_canastas = Some(canastas);
        self
    }

    pub fn full_game(mut self) -> Self {
        self.full_game = Some(true);
        self
    }

    pub fn hand(mut self) -> Self {
        self.full_game = Some(false);
        self
    }

    pub fn build(&mut self) -> Option<CanastaGame> {
        let players = self.num_players?;
        let canastas = self.num_canastas?;
        let full_game = self.full_game?;

        Some(CanastaGame::new(players, canastas, full_game))
    }
}
