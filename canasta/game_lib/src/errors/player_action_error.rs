
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PlayerActionError {
    #[error("Is currently player {0}s turn")]
    NotPlayerTurn(u8),
    #[error("Incorrect turn phase")]
    IncorrectTurnPhase,
    #[error("Game has finished")]
    GameOver,
    #[error("Given card is not a valid ID")]
    InvalidCard,
}
