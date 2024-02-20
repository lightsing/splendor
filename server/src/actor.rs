use crate::error::ClientError;
use async_trait::async_trait;
use futures_util::{SinkExt, StreamExt};
use splendor_core::{
    ActionRequest, ActionType, ActorError, DropTokensAction, GameSnapshot, PlayerAction,
    PlayerActor, SelectNoblesAction,
};
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite, WebSocketStream};

#[derive(Debug)]
pub struct WebSocketActor {
    stream: WebSocketStream<TcpStream>,
}

impl WebSocketActor {
    pub fn new(stream: WebSocketStream<TcpStream>) -> Self {
        Self { stream }
    }

    async fn get_result<T>(
        &mut self,
        ty: ActionType,
        snapshot: GameSnapshot,
    ) -> Result<T, ClientError>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let req = ActionRequest { ty, snapshot };
        let req = tungstenite::Message::Text(serde_json::to_string(&req)?);
        self.stream.send(req).await?;
        let res = self
            .stream
            .next()
            .await
            .ok_or(ClientError::UnexpectedEOF)??
            .into_text()?;
        let res: T = serde_json::from_str(&res)?;
        Ok(res)
    }
}

#[async_trait]
impl PlayerActor for WebSocketActor {
    async fn get_action(&mut self, snapshot: GameSnapshot) -> Result<PlayerAction, ActorError> {
        let res = self
            .get_result::<PlayerAction>(ActionType::GetAction, snapshot)
            .await?;
        Ok(res)
    }

    async fn drop_tokens(
        &mut self,
        snapshot: GameSnapshot,
    ) -> Result<DropTokensAction, ActorError> {
        let res = self
            .get_result::<DropTokensAction>(ActionType::DropTokens, snapshot)
            .await?;
        Ok(res)
    }

    async fn select_noble(
        &mut self,
        snapshot: GameSnapshot,
    ) -> Result<SelectNoblesAction, ActorError> {
        let res = self
            .get_result::<SelectNoblesAction>(ActionType::SelectNoble, snapshot)
            .await?;
        Ok(res)
    }
}
