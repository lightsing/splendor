//! # Splendor SDK
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

#[macro_use]
extern crate log;

use futures_util::{SinkExt, StreamExt};
use splendor_core::{ActionRequest, ActionType, ActorError, PlayerActor};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{client::IntoClientRequest, protocol::frame::coding::CloseCode, Message},
    MaybeTlsStream, WebSocketStream,
};

/// An error that can occur when using the `WebSocketActorClient`.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An error occurred while using the WebSocket.
    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),
    /// An error occurred while (de)serializing JSON.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    /// An error occurred while reading the secret file.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    /// An error occurred while calling the actor.
    #[error("Actor error: {0}")]
    Actor(splendor_core::ActorError),
}

impl From<ActorError> for Error {
    fn from(error: ActorError) -> Self {
        Self::Actor(error)
    }
}

/// A WebSocket client for a `PlayerActor`.
#[derive(Debug)]
pub struct WebSocketActorClient<A> {
    ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    actor: A,
}

impl<A: PlayerActor> WebSocketActorClient<A> {
    /// Create a new `WebSocketActorClient` from the environment.
    ///
    /// This method reads the `RPC_URL` and `CLIENT_SECRET` environment variables to connect to the
    /// server.
    pub async fn from_env(actor: A) -> Result<Self, Error> {
        let rpc_url = std::env::var("RPC_URL").unwrap();
        let secret_path = std::env::var("CLIENT_SECRET").unwrap();
        let secret = tokio::fs::read_to_string(secret_path).await?;
        Self::new(rpc_url, secret, actor).await
    }

    /// Create a new `WebSocketActorClient` from a url and a token.
    pub async fn new<R, T>(request: R, token: T, actor: A) -> Result<Self, Error>
    where
        R: IntoClientRequest + Unpin,
        T: ToString,
    {
        let request = request.into_client_request()?;
        info!("Connecting to {}", request.uri());
        let (mut ws_stream, _) = connect_async(request).await?;
        ws_stream.send(Message::Text(token.to_string())).await?;
        Ok(Self { ws_stream, actor })
    }

    /// Run the client.
    pub async fn run(&mut self) -> Result<(), Error> {
        while let Some(msg) = self.ws_stream.next().await {
            let msg = msg?;
            if let Message::Close(frame) = msg {
                let frame = frame.expect("CloseFrame should not be empty");
                match frame.code {
                    CloseCode::Normal => {
                        info!("Game finished");
                    }
                    _ => error!("Connection closed due to: {:?}", frame),
                }
                break;
            }
            let ActionRequest { ty, snapshot }: ActionRequest =
                serde_json::from_str(msg.to_text()?)?;
            info!("Received action request: {:?}", ty);
            let action = match ty {
                ActionType::GetAction => {
                    let action = self.actor.get_action(snapshot).await?;
                    info!("Took action: {:?}", action);
                    serde_json::to_string(&action)
                        .expect("PlayerAction serialization should not fail")
                }
                ActionType::DropTokens => {
                    let action = self.actor.drop_tokens(snapshot).await?;
                    info!("Dropped tokens: {:?}", action);
                    serde_json::to_string(&action)
                        .expect("DropTokensAction serialization should not fail")
                }
                ActionType::SelectNoble => {
                    let action = self.actor.select_noble(snapshot).await?;
                    info!("Selected noble: {:?}", action);
                    serde_json::to_string(&action)
                        .expect("SelectNoblesAction serialization should not fail")
                }
            };
            self.ws_stream.send(Message::Text(action)).await?;
        }
        Ok(())
    }
}
