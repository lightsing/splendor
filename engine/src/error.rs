use serde::Serialize;
use std::fmt::Display;

#[derive(Debug, Serialize, thiserror::Error)]
pub enum StepError {
    #[error("invalid action: {0}")]
    InvalidAction(#[from] InvalidActionError),
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
