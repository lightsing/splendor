use serde::Serialize;
use std::fmt::Display;

#[derive(Debug, Serialize, thiserror::Error)]
pub enum StepError {
    #[error("invalid action: {0}")]
    InvalidAction(#[from] InvalidActionError),
    #[error("actor error: {0}")]
    ActorError(ActorError),
}

#[derive(Debug, Serialize, thiserror::Error)]
pub struct InvalidActionError {
    pub player: usize,
    pub reason: &'static str,
}

impl Display for InvalidActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "player {}: {}", self.player, self.reason)
    }
}

#[derive(Debug, Serialize, thiserror::Error)]
pub struct ActorError {
    pub msg: String,
}

impl From<splendor_core::ActorError> for StepError {
    fn from(error: splendor_core::ActorError) -> Self {
        StepError::ActorError(ActorError {
            msg: error.to_string(),
        })
    }
}

impl Display for ActorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}
