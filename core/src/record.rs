//! History Record

use crate::{DropTokensAction, Noble, PlayerAction};

/// Wraps an action with the player who took it.
#[derive(Debug, Clone)]
pub struct ActionRecord<T> {
    /// The player who took the action.
    pub player: usize,
    /// The action taken.
    pub action: T,
}

impl<T> ActionRecord<T> {
    /// Create a new action record.
    pub fn new(player: usize, action: T) -> Self {
        ActionRecord { player, action }
    }
}

/// A record of a game event.
#[derive(Debug, Clone)]
pub enum Record {
    /// A player has taken an action.
    PlayerAction(ActionRecord<PlayerAction>),
    /// A player has dropped tokens due to having more than 10 tokens.
    DropTokens(ActionRecord<DropTokensAction>),
    /// A player has visited a noble.
    VisitNoble(ActionRecord<Noble>),
}
