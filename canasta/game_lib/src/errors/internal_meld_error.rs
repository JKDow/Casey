
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum InternalMeldError {
    #[error("Card ID is not in hand")]
    InvalidCardId(u8),
    #[error("Selected card cannot me part of meld")]
    InvalidCardToMeld(u8),
    #[error("Selected card is not of the selected rank")]
    IncorrectRank(u8),
}
