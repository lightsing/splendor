use super::SupervisorError;
use splendor_proto::supervisor::{
    game_ends_message::EndReason, supervisor_client::SupervisorClient, PreparePlayerChangeMessage,
};
use std::env;
use tokio::net::UnixStream;
use tonic::transport::{Endpoint, Uri};
use tower::service_fn;
use uuid::Uuid;

#[derive(Debug)]
pub struct Supervisor {
    uuid: Uuid,
    client: SupervisorClient<tonic::transport::Channel>,
}

impl Supervisor {
    pub async fn new(uuid: Uuid) -> Result<Self, SupervisorError> {
        let channel = Endpoint::try_from("http://[::]:50051")
            .expect("unreachable")
            .connect_with_connector(service_fn(|_: Uri| {
                let path = env::var("SUPERVISOR_SOCKET").expect("SUPERVISOR_SOCKET must be set");

                // Connect to a Uds socket
                UnixStream::connect(path)
            }))
            .await?;

        Ok(Self {
            uuid,
            client: SupervisorClient::new(channel),
        })
    }

    pub async fn report_game_ends(
        &mut self,
        winners: &[usize],
        timeout: bool,
        error: bool,
    ) -> Result<(), SupervisorError> {
        let reason = if timeout {
            EndReason::Timeout
        } else if error {
            EndReason::StepError
        } else if winners.is_empty() {
            EndReason::Draw
        } else {
            EndReason::Normal
        };
        self.client
            .report_game_ends(GameEndsMessage {
                game_id: self.uuid.to_string(),
                winners: winners.into_iter().map(|i| i as i32).collect(),
                reason: reason as i32,
            })
            .await
            .map(|_| ())
            .map_err(|_| SupervisorError::ChannelError)
    }

    pub async fn prepare_player_change(
        &mut self,
        next_player: usize,
    ) -> Result<(), SupervisorError> {
        self.client
            .prepare_player_change(PreparePlayerChangeMessage {
                game_id: self.uuid.to_string(),
                next_player: next_player as i32,
            })
            .await
            .map(|_| ())
            .map_err(|_| SupervisorError::ChannelError)
    }
}
