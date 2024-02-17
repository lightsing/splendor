use crate::{ColorVec, Tier};
use abi_stable::std_types::ROption;
use abi_stable::StableAbi;

#[repr(C)]
#[derive(Debug, Copy, Clone, StableAbi)]
pub struct DropTokensAction(pub ColorVec);

#[repr(C)]
#[derive(Debug, Copy, Clone, StableAbi)]
pub struct SelectNoblesAction(pub usize);

/// An enum to represent the actions a player can take.
#[repr(C)]
#[derive(Debug, Copy, Clone, StableAbi)]
pub enum PlayerAction {
    /// Take tokens.
    TakeTokens(TakeTokenAction),
    /// Buy a card.
    BuyCard(BuyCardAction),
    /// Reserve a card.
    ReserveCard(ReserveCardAction),
    /// Do nothing.
    NoOp,
}

/// An enum to represent the take tokens action.
#[repr(C)]
#[derive(Debug, Copy, Clone, StableAbi)]
pub enum TakeTokenAction {
    /// Take up to 3 different color tokens.
    ThreeDifferent(ColorVec),
    /// Take 2 tokens of the same color.
    TwoSame(ColorVec),
}

/// A struct to represent the buy card action.
#[repr(C)]
#[derive(Debug, Copy, Clone, StableAbi)]
pub struct BuyCardAction {
    pub tier: Tier,
    pub idx: usize,
    pub uses: ColorVec,
}

/// A struct to represent the reserve card action.
#[repr(C)]
#[derive(Debug, Copy, Clone, StableAbi)]
pub struct ReserveCardAction {
    pub tier: Tier,
    /// The index of the reserved card. None if the card is from the pool.
    pub idx: ROption<usize>,
}

impl TakeTokenAction {
    pub fn tokens(&self) -> &ColorVec {
        match self {
            TakeTokenAction::ThreeDifferent(tokens) => tokens,
            TakeTokenAction::TwoSame(tokens) => tokens,
        }
    }
}
