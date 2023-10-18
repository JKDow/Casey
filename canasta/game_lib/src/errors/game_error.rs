use thiserror::Error;

#[derive(Error, Debug)]
pub enum GameError {
    #[error("Invalid player number")]
    InvalidPlayer,
}
