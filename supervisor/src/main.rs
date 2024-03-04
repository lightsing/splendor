#[macro_use]
extern crate log;

use crate::instance::GameInstance;
use bollard::Docker;
use futures_util::{stream::FuturesUnordered, StreamExt};
use splendor_proto::{
    controller::{
        controller_server::{Controller, ControllerServer},
        CreateGameRequest, CreateGameResponse, StartGameRequest,
    },
    supervisor::{
        game_ends_message::EndReason,
        supervisor_server::{Supervisor, SupervisorServer},
        GameEndsMessage, PreparePlayerChangeMessage,
    },
};
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use tokio::net::UnixListener;
use tokio::sync::Mutex;
use tokio_stream::wrappers::UnixListenerStream;
use tonic::transport::Server;
use tonic::{Request, Response, Status};
use uuid::Uuid;

mod instance;

#[derive(Clone)]
struct GameController {
    docker: Docker,
    games: Arc<Mutex<HashMap<Uuid, GameInstance>>>,
}

impl GameController {
    pub fn new() -> Result<Self, bollard::errors::Error> {
        let docker = Docker::connect_with_local_defaults()?;
        Ok(Self {
            docker,
            games: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    pub async fn cleanup(&self) {
        let mut guard = self.games.lock().await;
        let mut futures = FuturesUnordered::new();
        for (_, game) in guard.drain() {
            futures.push(game.cleanup());
        }
        while let Some(ret) = futures.next().await {
            if let Err(e) = ret {
                error!("Failed to cleanup game: {}", e);
            }
        }
    }
}

#[tonic::async_trait]
impl Controller for GameController {
    async fn create_game(
        &self,
        request: Request<CreateGameRequest>,
    ) -> Result<Response<CreateGameResponse>, Status> {
        let CreateGameRequest {
            server_image,
            player_images,
            seed,
            step_timeout,
        } = request.into_inner();
        if player_images.len() != 3 && player_images.len() != 4 {
            return Err(Status::invalid_argument("Invalid number of players"));
        }

        let game = GameInstance::new(
            self.docker.clone(),
            &server_image,
            &player_images,
            seed,
            step_timeout.unwrap_or(60 * 5), // 5 minutes
        )
        .await
        .map_err(|e| {
            error!("Failed to create game: {}", e);
            Status::internal(format!("Failed to create game: {}", e))
        })?;
        let game_id = game.id;
        self.games.lock().await.insert(game_id, game);
        Ok(Response::new(CreateGameResponse {
            game_id: game_id.to_string(),
        }))
    }

    async fn start_game(&self, request: Request<StartGameRequest>) -> Result<Response<()>, Status> {
        let game_id = request.into_inner().game_id.parse::<Uuid>().map_err(|_| {
            error!("Received invalid UUID while handling start_game");
            Status::invalid_argument("Invalid UUID")
        })?;
        let guard = self.games.lock().await;
        let game = guard.get(&game_id).ok_or_else(|| {
            error!("Received unknown game ID while handling start_game");
            Status::not_found("Unknown game ID")
        })?;
        game.start().await.map_err(|e| {
            error!("Failed to start game: {}", e);
            Status::internal(format!("Failed to start game: {}", e))
        })?;
        Ok(Response::new(()))
    }
}

#[tonic::async_trait]
impl Supervisor for GameController {
    async fn report_game_ends(
        &self,
        request: Request<GameEndsMessage>,
    ) -> Result<Response<()>, Status> {
        let GameEndsMessage {
            game_id,
            winners,
            reason,
        } = request.into_inner();
        let game_id = game_id.parse::<Uuid>().map_err(|_| {
            error!("Received invalid UUID while handling report_game_ends");
            Status::invalid_argument("Invalid UUID")
        })?;
        let reason = EndReason::try_from(reason).map_err(|_| {
            error!(
                "Received invalid EndReasom while handling report_game_ends, reason: {}",
                reason
            );
            Status::invalid_argument("Invalid EndReasom")
        })?;
        info!("Game#{game_id} Ends ({reason:?}), winners: {winners:?}");
        let game = self.games.lock().await.remove(&game_id).unwrap();
        if let Err(e) = game.cleanup().await {
            error!("Failed to cleanup game: {}", e);
        }
        Ok(Response::new(()))
    }

    async fn prepare_player_change(
        &self,
        request: Request<PreparePlayerChangeMessage>,
    ) -> Result<Response<()>, Status> {
        let req = request.into_inner();
        let game_id = Uuid::from_str(&req.game_id).map_err(|_| {
            error!(
                "Received invalid UUID while handling prepare_player_change: {}",
                req.game_id
            );
            Status::invalid_argument("Invalid UUID")
        })?;
        let guard = self.games.lock().await;
        let game = guard.get(&game_id).ok_or_else(|| {
            error!(
                "Received unknown game ID while handling prepare_player_change: {}",
                req.game_id
            );
            Status::not_found("Unknown game ID")
        })?;
        info!(
            "Prepare player change for game#{game_id}, next player: {}",
            req.next_player
        );
        game.prepare_player_change(req.next_player as usize)
            .await
            .map_err(|e| {
                error!(
                    "Failed to prepare player change for game ID: {}, cause: {e}",
                    req.game_id
                );
                Status::internal(format!("Failed to prepare player change: {}", e))
            })?;
        Ok(Response::new(()))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let game_supervisor = GameController::new()?;
    let shared_volume_path = PathBuf::from(
        std::env::var_os("SHARED_VOLUME_PATH")
            .expect("SHARED_VOLUME_PATH must be set to the path of the UDS socket"),
    );
    let socket_path = shared_volume_path.join("supervisor.sock");

    let supervisor_server = {
        let game_supervisor = game_supervisor.clone();
        info!("Starting supervisor server at: {socket_path:?}");
        let uds = UnixListener::bind(&socket_path)?;
        let uds_stream = UnixListenerStream::new(uds);
        tokio::spawn(
            Server::builder()
                .add_service(SupervisorServer::new(game_supervisor))
                .serve_with_incoming(uds_stream),
        )
    };

    let controller_server = {
        let game_supervisor = game_supervisor.clone();
        let addr = std::env::var("CONTROLLER_ADDR")
            .expect("CONTROLLER_ADDR must be set to a socket address");
        info!("Starting controller server at: {addr:?}");
        tokio::spawn(
            Server::builder()
                .add_service(ControllerServer::new(game_supervisor))
                .serve(addr.parse().unwrap()),
        )
    };

    tokio::select! {
        _ = supervisor_server => {
            error!("Supervisor server terminated unexpectedly");
        }
        _ = controller_server => {
            error!("Controller server terminated unexpectedly");
        }
        _ = tokio::signal::ctrl_c() => {
            info!("Shutting down");
            tokio::fs::remove_file(&socket_path).await?;
            game_supervisor.cleanup().await;
        }
    }

    Ok(())
}
