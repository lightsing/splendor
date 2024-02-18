use crate::{ActionType, DropTokensAction, GameSnapshot, PlayerAction, SelectNoblesAction};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// An error type for actor errors.
#[derive(Debug)]
pub struct ActorError {
    error: Box<dyn std::error::Error + Send + Sync>,
}

impl<E> From<E> for ActorError
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn from(error: E) -> Self {
        Self {
            error: Box::new(error),
        }
    }
}

impl Display for ActorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.error.fmt(f)
    }
}

/// A struct to represent an action request.
///
/// Might be used to request an action from a player.
#[derive(Debug, Serialize, Deserialize)]
pub struct ActionRequest {
    /// The type of the action.
    #[serde(rename = "type")]
    pub ty: ActionType,
    /// The game snapshot.
    pub snapshot: GameSnapshot,
}

/// A player actor trait.
#[async_trait::async_trait]
pub trait PlayerActor: Send + Sync {
    /// It's the player's turn. Get the action to take.
    async fn get_action(&mut self, snapshot: GameSnapshot) -> Result<PlayerAction, ActorError>;

    /// The player has more than 10 tokens. Get the tokens to drop.
    async fn drop_tokens(&mut self, snapshot: GameSnapshot)
        -> Result<DropTokensAction, ActorError>;

    /// The player has more than 1 noble to visit. Select the noble to visit.
    async fn select_noble(
        &mut self,
        snapshot: GameSnapshot,
    ) -> Result<SelectNoblesAction, ActorError>;
}
