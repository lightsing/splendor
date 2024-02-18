//! Websocket bridge for the PlayerActor trait.
use tokio_tungstenite::connect_async;

pub struct WebsocketPlayerActor {}

impl WebsocketPlayerActor {
    pub fn new() -> Self {
        WebsocketPlayerActor {}
    }
}
