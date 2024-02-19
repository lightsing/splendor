use crate::GameContext;
use rand::prelude::*;
use smallvec::SmallVec;
use splendor_core::naive_actors::RandomActor;
use splendor_core::*;
use std::array;

#[ctor::ctor]
fn init_logger() {
    pretty_env_logger::init();
}

fn create_game(seed: Option<u64>) -> GameContext {
    let actors: [Box<dyn PlayerActor>; MAX_PLAYERS] = array::from_fn(|_| match seed {
        Some(seed) => Box::new(RandomActor::new(SmallRng::seed_from_u64(seed))) as _,
        None => Box::new(RandomActor::new(SmallRng::from_entropy())) as _,
    });
    match seed {
        Some(seed) => {
            let mut rng = SmallRng::seed_from_u64(seed);
            GameContext::with_rng(&mut rng, SmallVec::from_buf(actors))
        }
        None => GameContext::random(SmallVec::from_buf(actors)),
    }
}

#[tokio::test]
async fn test_game() {
    for _ in 0..100000 {
        let mut game = create_game(None);
        while !game.game_end() {
            game.step().await.unwrap();
        }
    }
}

#[tokio::test]
async fn test_serialize() {
    let mut game = create_game(Some(42));
    while !game.game_end() {
        game.step().await.unwrap();
    }

    let snapshot = game.snapshot();
    let serialized = serde_json::to_string(&snapshot).unwrap();

    assert_eq!(
        serialized,
        r#"{"last_round":true,"current_round":34,"current_player":3,"tokens":[0,1,1,2,1,5],"card_pool":{"remaining":[0,15,8],"revealed":[[],[{"tier":1,"bonus":"white","points":2,"requires":[3,0,0,5,0,0]},{"tier":1,"bonus":"red","points":1,"requires":[3,3,0,2,0,0]},{"tier":1,"bonus":"blue","points":1,"requires":[3,2,3,0,0,0]},{"tier":1,"bonus":"white","points":2,"requires":[0,0,0,5,0,0]}],[{"tier":2,"bonus":"black","points":5,"requires":[3,0,0,7,0,0]},{"tier":2,"bonus":"white","points":3,"requires":[3,3,3,5,0,0]},{"tier":2,"bonus":"white","points":4,"requires":[7,0,0,0,0,0]},{"tier":2,"bonus":"blue","points":4,"requires":[0,0,0,0,7,0]}]]},"nobles":[{"requires":[4,0,0,4,0,0]}],"players":[{"idx":0,"points":5,"tokens":[1,0,4,2,3,0],"development_cards":{"points":5,"bonus":[1,2,3,4,2,0],"inner":[[{"tier":0,"bonus":"black","points":0,"requires":[0,1,1,1,1,0]}],[{"tier":0,"bonus":"blue","points":0,"requires":[3,0,0,0,0,0]},{"tier":0,"bonus":"blue","points":0,"requires":[0,0,2,2,1,0]}],[{"tier":0,"bonus":"green","points":0,"requires":[0,1,0,0,2,0]},{"tier":0,"bonus":"green","points":0,"requires":[1,1,0,1,1,0]},{"tier":0,"bonus":"green","points":0,"requires":[0,0,0,3,0,0]}],[{"tier":0,"bonus":"red","points":0,"requires":[0,2,1,0,0,0]},{"tier":0,"bonus":"red","points":0,"requires":[0,0,0,2,2,0]},{"tier":0,"bonus":"red","points":1,"requires":[0,0,0,0,4,0]},{"tier":1,"bonus":"red","points":3,"requires":[0,0,0,6,0,0]}],[{"tier":0,"bonus":"white","points":1,"requires":[0,0,4,0,0,0]},{"tier":0,"bonus":"white","points":0,"requires":[2,2,0,0,0,0]}]]},"reserved_cards":[{"type":"visible","view":{"tier":2,"bonus":"white","points":5,"requires":[7,0,0,0,3,0]}},{"type":"visible","view":{"tier":0,"bonus":"white","points":0,"requires":[1,1,0,0,3,0]}},{"type":"invisible","view":2}],"nobles":[]},{"idx":1,"points":5,"tokens":[0,3,1,3,1,0],"development_cards":{"points":5,"bonus":[2,3,1,2,1,0],"inner":[[{"tier":0,"bonus":"black","points":1,"requires":[0,4,0,0,0,0]},{"tier":0,"bonus":"black","points":0,"requires":[1,0,1,3,0,0]}],[{"tier":0,"bonus":"blue","points":0,"requires":[2,0,0,0,1,0]},{"tier":0,"bonus":"blue","points":0,"requires":[1,0,1,2,1,0]},{"tier":1,"bonus":"blue","points":3,"requires":[0,6,0,0,0,0]}],[{"tier":0,"bonus":"green","points":0,"requires":[2,1,0,2,0,0]}],[{"tier":0,"bonus":"red","points":0,"requires":[3,0,0,1,1,0]},{"tier":0,"bonus":"red","points":0,"requires":[2,0,1,0,2,0]}],[{"tier":1,"bonus":"white","points":1,"requires":[2,0,3,2,0,0]}]]},"reserved_cards":[{"type":"visible","view":{"tier":1,"bonus":"black","points":2,"requires":[0,0,0,0,5,0]}},{"type":"visible","view":{"tier":1,"bonus":"red","points":2,"requires":[5,0,0,0,0,0]}},{"type":"visible","view":{"tier":2,"bonus":"blue","points":3,"requires":[5,0,3,3,3,0]}}],"nobles":[]},{"idx":2,"points":11,"tokens":[4,1,1,0,1,0],"development_cards":{"points":8,"bonus":[3,3,3,2,4,0],"inner":[[{"tier":0,"bonus":"black","points":0,"requires":[0,0,3,0,0,0]},{"tier":0,"bonus":"black","points":0,"requires":[0,0,2,1,0,0]},{"tier":0,"bonus":"black","points":0,"requires":[0,0,2,0,2,0]}],[{"tier":0,"bonus":"blue","points":0,"requires":[1,0,1,1,1,0]},{"tier":1,"bonus":"blue","points":2,"requires":[0,3,0,0,5,0]},{"tier":0,"bonus":"blue","points":1,"requires":[0,0,0,4,0,0]}],[{"tier":1,"bonus":"green","points":1,"requires":[2,3,0,0,2,0]},{"tier":1,"bonus":"green","points":2,"requires":[0,5,3,0,0,0]},{"tier":1,"bonus":"green","points":2,"requires":[1,2,0,0,4,0]}],[{"tier":0,"bonus":"red","points":0,"requires":[0,0,0,0,3,0]},{"tier":0,"bonus":"red","points":0,"requires":[1,1,1,0,2,0]}],[{"tier":0,"bonus":"white","points":0,"requires":[0,3,0,0,0,0]},{"tier":0,"bonus":"white","points":0,"requires":[1,1,2,1,0,0]},{"tier":0,"bonus":"white","points":0,"requires":[1,0,0,2,0,0]},{"tier":0,"bonus":"white","points":0,"requires":[1,1,1,1,0,0]}]]},"reserved_cards":[{"type":"invisible","view":1},{"type":"visible","view":{"tier":2,"bonus":"red","points":5,"requires":[0,0,7,3,0,0]}},{"type":"visible","view":{"tier":2,"bonus":"blue","points":4,"requires":[3,3,0,0,6,0]}}],"nobles":[{"requires":[3,0,0,3,3,0]}]},{"idx":3,"points":15,"tokens":[2,2,0,0,1,0],"development_cards":{"points":6,"bonus":[3,2,4,1,2,0],"inner":[[{"tier":0,"bonus":"black","points":0,"requires":[0,2,0,1,2,0]},{"tier":0,"bonus":"black","points":0,"requires":[0,2,1,1,1,0]},{"tier":2,"bonus":"black","points":3,"requires":[0,3,5,3,3,0]}],[{"tier":0,"bonus":"blue","points":0,"requires":[0,1,3,1,0,0]},{"tier":0,"bonus":"blue","points":0,"requires":[2,0,2,0,0,0]}],[{"tier":0,"bonus":"green","points":1,"requires":[4,0,0,0,0,0]},{"tier":0,"bonus":"green","points":0,"requires":[0,3,1,0,1,0]},{"tier":0,"bonus":"green","points":0,"requires":[0,2,0,2,0,0]},{"tier":0,"bonus":"green","points":0,"requires":[2,1,0,1,1,0]}],[{"tier":0,"bonus":"red","points":0,"requires":[1,1,1,0,1,0]}],[{"tier":1,"bonus":"white","points":2,"requires":[2,0,1,4,0,0]},{"tier":0,"bonus":"white","points":0,"requires":[1,2,2,0,0,0]}]]},"reserved_cards":[{"type":"visible","view":{"tier":2,"bonus":"green","points":4,"requires":[0,7,0,0,0,0]}},{"type":"visible","view":{"tier":2,"bonus":"green","points":3,"requires":[3,3,0,3,5,0]}}],"nobles":[{"requires":[0,4,0,0,4,0]},{"requires":[0,4,4,0,0,0]},{"requires":[0,3,3,3,0,0]}]}]}"#
    );

    game.records
}
