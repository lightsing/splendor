use crate::{Card, CardView, ColorVec, DevelopmentCards, Noble};
use abi_stable::std_types::RVec;
use abi_stable::StableAbi;

#[repr(C)]
#[derive(StableAbi)]
pub struct GameSnapshot {
    pub last_round: bool,
    pub current_round: usize,
    pub current_player: usize,

    pub tokens: ColorVec,
    pub card_pool: CardPoolSnapshot,
    pub nobles: RVec<Noble>,

    pub players: RVec<PlayerSnapshot>,
}

#[repr(C)]
#[derive(StableAbi)]
pub struct CardPoolSnapshot {
    pub remaining: [usize; 3],
    pub revealed: [RVec<Card>; 3],
}

#[repr(C)]
#[derive(StableAbi)]
pub struct PlayerSnapshot {
    pub idx: usize,
    pub points: u8,
    pub tokens: ColorVec,
    pub development_cards: DevelopmentCards,
    pub reserved_cards: RVec<CardView>,
    pub nobles: RVec<Noble>,
}
