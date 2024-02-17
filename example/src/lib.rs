#![allow(improper_ctypes_definitions)]

use abi_stable::std_types::RNone;
use splendor_core::{
    Color, ColorVec, DropTokensAction, GameSnapshot, PlayerAction, ReserveCardAction,
    SelectNoblesAction, TakeTokenAction, Tier,
};
use splendor_ffi::{declare_module, PlayerActor};

#[derive(Debug)]
struct ExampleActor;

impl ExampleActor {
    fn new() -> Self {
        ExampleActor
    }
}

impl PlayerActor for ExampleActor {
    fn get_action(&self, snapshot: GameSnapshot) -> PlayerAction {
        let can_take_3: Vec<_> = snapshot
            .tokens
            .iter()
            .enumerate()
            .filter(|(_, x)| *x > 0)
            .collect();
        if can_take_3.len() > 3 {
            let mut takes = ColorVec::empty();
            for (i, _) in can_take_3.iter().take(3) {
                takes.set(Color::try_from(*i as u8).unwrap(), 1)
            }
            return PlayerAction::TakeTokens(TakeTokenAction::ThreeDifferent(takes));
        }
        let can_take_2: Vec<_> = snapshot
            .tokens
            .iter()
            .enumerate()
            .filter(|(_, x)| *x > 3)
            .collect();
        if !can_take_2.is_empty() {
            let mut takes = ColorVec::empty();
            takes.set(Color::try_from(can_take_2[0].0 as u8).unwrap(), 2);
            return PlayerAction::TakeTokens(TakeTokenAction::TwoSame(takes));
        }
        if snapshot.players[snapshot.current_player]
            .reserved_cards
            .len()
            < 3
        {
            let first = snapshot.card_pool.remaining.iter().position(|&x| x > 0);
            if let Some(first) = first {
                return PlayerAction::ReserveCard(ReserveCardAction {
                    tier: Tier::try_from(first as u8).unwrap(),
                    idx: RNone,
                });
            }
        }
        PlayerAction::NoOp
    }

    fn drop_tokens(&self, snapshot: GameSnapshot) -> DropTokensAction {
        let mut drops = snapshot.players[snapshot.current_player].tokens.total() - 10;
        let mut to_drop = ColorVec::empty();
        for (i, mut x) in snapshot.tokens.iter().enumerate() {
            while x > 0 && drops > 0 {
                to_drop.add(Color::try_from(i as u8).unwrap(), 1);
                x -= 1;
                drops -= 1;
            }
        }
        DropTokensAction(to_drop)
    }

    fn select_noble(&self, snapshot: GameSnapshot) -> SelectNoblesAction {
        let noble = snapshot
            .nobles
            .iter()
            .position(|n| {
                n.requires
                    < snapshot.players[snapshot.current_player]
                        .development_cards
                        .bonus
            })
            .unwrap();
        SelectNoblesAction(noble)
    }
}

declare_module!(ExampleActor);
