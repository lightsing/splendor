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
use std::{array, env};
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
    drop(game);

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

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
        let dir = format!("{path}/player{idx}");
        tokio::fs::create_dir_all(&dir).await?;
        tokio::fs::write(format!("{dir}/secret"), secret).await?;
        info!("Player#{idx} secret written to: {dir}/secret={secret}");
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

    let n = secrets.len();
    let mut actors: SmallVec<Option<Box<dyn PlayerActor>>, MAX_PLAYERS> = SmallVec::new();
    for _ in 0..n {
        actors.push(None);
    }
    for _ in 0..n {
        let (idx, actor) = rx
            .recv()
            .await
            .ok_or_else(|| anyhow::anyhow!("actor channel closed"))?;
        actors[idx] = Some(actor);
    }
    server.abort();

    Ok(actors.into_iter().map(|actor| actor.unwrap()).collect())
}

async fn accept_connection(
    stream: tokio::net::TcpStream,
    secrets: &[String],
    tx: Sender<(usize, Box<dyn PlayerActor>)>,
) -> anyhow::Result<()> {
    let addr = stream.peer_addr()?;
    let mut ws_stream = tokio_tungstenite::accept_async(stream).await?;
    info!("New connection from: {addr}");

    let secret = ws_stream
        .next()
        .await
        .ok_or(ClientError::UnexpectedEOF)??
        .into_text()?;
    let player_id = secrets.iter().position(|s| s == &secret);
    if player_id.is_none() {
        warn!("Invalid secret from: {addr}, got {secret}");
        return Ok(());
    }
    let player_id = player_id.unwrap();
    info!("Player#{player_id} accepted from: {addr}");

    let actor = actor::WebSocketActor::new(ws_stream);
    tx.send((player_id, Box::new(actor))).await?;
    Ok(())
}
