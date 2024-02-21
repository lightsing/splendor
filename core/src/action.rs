use crate::{ColorVec, Tier};
use serde::{Deserialize, Serialize};

/// An enum to represent the actions a player can take.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionType {
    /// Get the action to take.
    GetAction,
    /// Get the tokens to drop.
    DropTokens,
    /// Select the noble to visit.
    SelectNoble,
}

/// A struct to represent the drop tokens action.
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DropTokensAction(pub ColorVec);

/// A struct to represent the select nobles action.
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SelectNoblesAction(pub usize);

/// An enum to represent the actions a player can take.
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", content = "action")]
#[serde(rename_all = "snake_case")]
pub enum PlayerAction {
    /// Take tokens.
    TakeTokens(TakeTokenAction),
    /// Buy a card.
    BuyCard(BuyCardAction),
    /// Reserve a card.
    ReserveCard(ReserveCardAction),
    /// Do nothing.
    Nop,
}

impl PlayerAction {
    /// Is nop action.
    pub fn is_nop(&self) -> bool {
        matches!(self, PlayerAction::Nop)
    }
}

/// An enum to represent the take tokens action.
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", content = "tokens")]
#[serde(rename_all = "snake_case")]
pub enum TakeTokenAction {
    /// Take up to 3 different color tokens.
    ThreeDifferent(ColorVec),
    /// Take 2 tokens of the same color.
    TwoSame(ColorVec),
}

/// A struct to represent the buy card action.
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BuyCardAction {
    /// Source of the card
    pub source: BuyCardSource,
    /// The color of the joker token used to buy the card.
    pub uses: ColorVec,
}

/// An enum to represent the source of the card to buy.
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", content = "location")]
#[serde(rename_all = "snake_case")]
pub enum BuyCardSource {
    /// The card is from the revealed cards.
    Revealed {
        /// The tier of the bought card.
        tier: Tier,
        /// The index of the bought card.
        idx: usize,
    },
    /// The card is from the reserved cards.
    Reserved(usize),
}

/// A struct to represent the reserve card action.
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde_player_action() {
        let action = PlayerAction::TakeTokens(TakeTokenAction::ThreeDifferent(ColorVec::empty()));
        let serialized = serde_json::to_string(&action).unwrap();
        assert_eq!(
            serialized,
            r#"{"type":"take_tokens","action":{"type":"three_different","tokens":[0,0,0,0,0,0]}}"#
        );
        let deserialized: PlayerAction = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, action);

        let action = PlayerAction::TakeTokens(TakeTokenAction::TwoSame(ColorVec::empty()));
        let serialized = serde_json::to_string(&action).unwrap();
        assert_eq!(
            serialized,
            r#"{"type":"take_tokens","action":{"type":"two_same","tokens":[0,0,0,0,0,0]}}"#
        );
        let deserialized: PlayerAction = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, action);

        let action = PlayerAction::BuyCard(BuyCardAction {
            source: BuyCardSource::Revealed {
                tier: Tier::I,
                idx: 0,
            },
            uses: ColorVec::empty(),
        });
        let serialized = serde_json::to_string(&action).unwrap();
        assert_eq!(
            serialized,
            r#"{"type":"buy_card","action":{"source":{"type":"revealed","location":{"tier":0,"idx":0}},"uses":[0,0,0,0,0,0]}}"#
        );
        let deserialized: PlayerAction = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, action);

        let action = PlayerAction::BuyCard(BuyCardAction {
            source: BuyCardSource::Reserved(0),
            uses: ColorVec::empty(),
        });

        let serialized = serde_json::to_string(&action).unwrap();
        assert_eq!(
            serialized,
            r#"{"type":"buy_card","action":{"source":{"type":"reserved","location":0},"uses":[0,0,0,0,0,0]}}"#
        );
        let deserialized: PlayerAction = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, action);

        let action = PlayerAction::ReserveCard(ReserveCardAction {
            tier: Tier::I,
            idx: None,
        });
        let serialized = serde_json::to_string(&action).unwrap();
        assert_eq!(
            serialized,
            r#"{"type":"reserve_card","action":{"tier":0,"idx":null}}"#
        );

        let action = PlayerAction::Nop;
        let serialized = serde_json::to_string(&action).unwrap();
        assert_eq!(serialized, r#"{"type":"nop"}"#);
        let deserialized: PlayerAction = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, action);
    }

    #[test]
    fn test_serde_drop_tokens_action() {
        let action = DropTokensAction(ColorVec::empty());
        let serialized = serde_json::to_string(&action).unwrap();
        assert_eq!(serialized, r#"[0,0,0,0,0,0]"#);
        let deserialized: DropTokensAction = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, action);
    }

    #[test]
    fn test_serde_select_nobles_action() {
        let action = SelectNoblesAction(0);
        let serialized = serde_json::to_string(&action).unwrap();
        assert_eq!(serialized, r#"0"#);
        let deserialized: SelectNoblesAction = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, action);
    }
}
