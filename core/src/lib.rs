//! Core game structures implementation.
#![deny(missing_docs)]

mod action;
mod cards;
mod colors;
mod nobles;
mod record;
mod snapshot;

pub use action::*;
pub use cards::*;
pub use colors::*;
pub use nobles::*;
pub use record::*;
pub use snapshot::*;

/// The maximum number of players in a game.
pub const MAX_PLAYERS: usize = 4;

/// A player actor trait.
pub trait PlayerActor {
    /// It's the player's turn. Get the action to take.
    fn get_action(&self, snapshot: GameSnapshot) -> PlayerAction;

    /// The player has more than 10 tokens. Get the tokens to drop.
    fn drop_tokens(&self, snapshot: GameSnapshot) -> DropTokensAction;

    /// The player has more than 1 noble to visit. Select the noble to visit.
    fn select_noble(&self, snapshot: GameSnapshot) -> SelectNoblesAction;
}
