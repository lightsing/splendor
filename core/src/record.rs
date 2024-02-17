//! History Record

use crate::{DropTokensAction, Noble, PlayerAction};
use abi_stable::StableAbi;

#[repr(C)]
#[derive(Debug, Clone, StableAbi)]
pub struct ActionRecord<T> {
    pub player: usize,
    pub action: T,
}

impl<T> ActionRecord<T> {
    pub fn new(player: usize, action: T) -> Self {
        ActionRecord { player, action }
    }
}

#[repr(C)]
#[derive(Debug, Clone, StableAbi)]
pub enum Record {
    /// A player has taken an action.
    PlayerAction(ActionRecord<PlayerAction>),
    /// A player has dropped tokens due to having more than 10 tokens.
    DropTokens(ActionRecord<DropTokensAction>),
    /// A player has visited a noble.
    VisitNoble(ActionRecord<Noble>),
}
