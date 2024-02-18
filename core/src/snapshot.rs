use crate::{Card, CardView, ColorVec, DevelopmentCards, Noble, MAX_PLAYERS};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

/// A struct to represent the game snapshot.
#[derive(Debug, Serialize, Deserialize)]
pub struct GameSnapshot {
    /// Is the game in the last round.
    pub last_round: bool,
    /// The current round.
    pub current_round: usize,
    /// The current player.
    pub current_player: usize,

    /// The tokens available in the game.
    pub tokens: ColorVec,
    /// The card pool snapshot.
    pub card_pool: CardPoolSnapshot,
    /// The nobles available in the game.
    pub nobles: SmallVec<Noble, { MAX_PLAYERS + 1 }>,

    /// The players' snapshot.
    pub players: SmallVec<PlayerSnapshot, MAX_PLAYERS>,
}

/// A struct to represent the card pool snapshot.
#[derive(Debug, Serialize, Deserialize)]
pub struct CardPoolSnapshot {
    /// The remaining cards in the pool.
    pub remaining: [usize; 3],
    /// The revealed cards in the pool.
    pub revealed: [SmallVec<Card, 4>; 3],
}

/// A struct to represent the player snapshot.
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerSnapshot {
    /// The index of the player.
    pub idx: usize,
    /// The number of points the player has.
    pub points: u8,
    /// The tokens the player has.
    pub tokens: ColorVec,
    /// The development cards the player has.
    pub development_cards: DevelopmentCards,
    /// The reserved cards the player has.
    pub reserved_cards: SmallVec<CardView, 3>,
    /// The nobles the player has visited.
    pub nobles: SmallVec<Noble, { MAX_PLAYERS + 1 }>,
}
