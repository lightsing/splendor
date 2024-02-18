use serde::Serialize;
use tokio_tungstenite::tungstenite;

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("unexpected EOF while reading message")]
    UnexpectedEOF,
    #[error("invalid message")]
    InvalidMessage(#[from] tungstenite::Error),
    #[error("invalid json")]
    InvalidJson(#[from] serde_json::Error),
}

impl Serialize for ClientError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}
