use crate::{ColorVec, Tier};
use serde::{Deserialize, Serialize};

/// An enum to represent the actions a player can take.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum ActionType {
    /// Get the action to take.
    GetAction,
    /// Get the tokens to drop.
    DropTokens,
    /// Select the noble to visit.
    SelectNoble,
}

/// A struct to represent the drop tokens action.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct DropTokensAction(pub ColorVec);

/// A struct to represent the select nobles action.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct SelectNoblesAction(pub usize);

/// An enum to represent the actions a player can take.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum TakeTokenAction {
    /// Take up to 3 different color tokens.
    ThreeDifferent(ColorVec),
    /// Take 2 tokens of the same color.
    TwoSame(ColorVec),
}

/// A struct to represent the buy card action.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct BuyCardAction {
    /// The tier of the bought card.
    pub tier: Tier,
    /// The index of the bought card.
    pub idx: usize,
    /// The color of the joker token used to buy the card.
    pub uses: ColorVec,
}

/// A struct to represent the reserve card action.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct ReserveCardAction {
    /// The source tier of the reserved card.
    pub tier: Tier,
    /// The index of the reserved card. None if the card is from the pool.
    pub idx: Option<usize>,
}

impl TakeTokenAction {
    /// Get the tokens of the action.
    pub fn tokens(&self) -> &ColorVec {
        match self {
            TakeTokenAction::ThreeDifferent(tokens) => tokens,
            TakeTokenAction::TwoSame(tokens) => tokens,
        }
    }
}
