#[macro_use]
extern crate log;

use crate::error::ClientError;
use futures_util::StreamExt;
use rand::distributions::{Alphanumeric, DistString};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use smallvec::SmallVec;
use splendor_core::{PlayerActor, MAX_PLAYERS};
use splendor_engine::GameContext;
use std::env;
use tokio::sync::mpsc::Sender;

mod actor;
mod error;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let n_players = env::var("N_PLAYERS")?.parse::<usize>()?;
    // random_seed use to deterministically reproduce the game.
    let random_seed = env::var("RANDOM_SEED")
        .ok()
        .map(|s| s.parse::<u64>().ok())
        .flatten();

    let secrets = gen_secrets(n_players);
    write_secrets(secrets).await?;
    let actors = start_server(secrets).await?;

    let mut game = match random_seed {
        None => GameContext::random(actors),
        Some(seed) => {
            let mut rng = ChaCha20Rng::seed_from_u64(seed);
            GameContext::with_rng(&mut rng, actors)
        }
    };

    while !game.game_end() {
        game.step().await?;
    }

    Ok(())
}

fn gen_secrets(n: usize) -> &'static [String] {
    let mut rng = rand::thread_rng();
    let mut secrets = Box::new(Vec::new());
    for _ in 0..n {
        secrets.push(Alphanumeric.sample_string(&mut rng, 32));
    }
    Box::leak(secrets)
}

async fn write_secrets(secrets: &[String]) -> anyhow::Result<()> {
    let path = env::var("SECRETS_PATH")?;
    for (idx, secret) in secrets.iter().enumerate() {
        tokio::fs::write(format!("{}/player{}", path, idx), secret).await?;
    }
    Ok(())
}

async fn start_server(
    secrets: &'static [String],
) -> anyhow::Result<SmallVec<Box<dyn PlayerActor>, MAX_PLAYERS>> {
    let (tx, mut rx) = tokio::sync::mpsc::channel(secrets.len());
    let addr = env::var("SERVER_ADDR")?;
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("Listening on: {}", addr);
    let server = tokio::spawn(async move {
        loop {
            if let Ok((stream, _)) = listener.accept().await {
                tokio::spawn(accept_connection(stream, secrets, tx.clone()));
            }
        }
    });

    let mut actors = SmallVec::new();
    for _ in 0..secrets.len() {
        let actor = rx
            .recv()
            .await
            .ok_or_else(|| anyhow::anyhow!("actor channel closed"))?;
        actors.push(actor);
    }
    server.abort();

    Ok(actors)
}

async fn accept_connection(
    stream: tokio::net::TcpStream,
    secrets: &[String],
    tx: Sender<Box<dyn PlayerActor>>,
) -> anyhow::Result<()> {
    let addr = stream.peer_addr()?;
    let mut ws_stream = tokio_tungstenite::accept_async(stream).await?;
    info!("New connection from: {}", addr);

    let secret = ws_stream
        .next()
        .await
        .ok_or(ClientError::UnexpectedEOF)??
        .into_text()?;
    let player_id = secrets.iter().position(|s| s == &secret);
    if player_id.is_none() {
        warn!("Invalid secret from: {}", addr);
        return Ok(());
    }
    let player_id = player_id.unwrap();
    info!("Player#{} accepted from: {}", player_id, addr);

    let actor = actor::WebSocketActor::new(ws_stream);
    tx.send(Box::new(actor)).await?;
    Ok(())
}
