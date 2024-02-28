use rand::prelude::*;
use splendor_core::naive_actors::RandomActor;
use splendor_sdk::WebSocketActorClient;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let rng = SmallRng::from_entropy();
    let actor = RandomActor::new(rng);
    let mut client = WebSocketActorClient::from_env(actor).await.unwrap();
    client.run().await.unwrap();
}
