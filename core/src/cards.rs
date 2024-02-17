use crate::colors::{Color, ColorVec};
use abi_stable::std_types::RVec;
use abi_stable::StableAbi;
use num_enum::TryFromPrimitive;
use strum::EnumIter;

/// An enum to represent the card tiers.
#[repr(u8)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, EnumIter, TryFromPrimitive, StableAbi,
)]
pub enum Tier {
    I = 0,
    II,
    III,
}

/// A struct to represent a card.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, StableAbi)]
pub struct Card {
    /// The tier of the card.
    pub tier: Tier,
    /// The bonus color of the card.
    pub bonus: Color,
    /// The points of the card.
    pub points: u8,
    /// The color requirements of the card.
    pub requires: ColorVec,
}

impl Card {
    pub const fn new(tier: Tier, bonus: Color, points: u8, requires: ColorVec) -> Self {
        Card {
            tier,
            bonus,
            points,
            requires,
        }
    }
}

/// A struct to represent the development cards in player's hand.
#[repr(C)]
#[derive(Debug, Default, Clone, StableAbi)]
pub struct DevelopmentCards {
    pub points: u8,
    pub bonus: ColorVec,
    inner: [RVec<Card>; 5],
}

impl DevelopmentCards {
    /// Create a new development cards with empty cards.
    #[inline(always)]
    pub const fn new() -> Self {
        DevelopmentCards {
            points: 0,
            bonus: ColorVec::empty(),
            inner: [
                RVec::new(),
                RVec::new(),
                RVec::new(),
                RVec::new(),
                RVec::new(),
            ],
        }
    }

    /// Add a card to the development cards.
    #[inline(always)]
    pub fn add(&mut self, card: Card) {
        self.inner[card.bonus as usize].push(card);
        self.points += card.points;
        self.bonus.add(card.bonus, 1);
    }

    /// Iterate over the development cards.
    #[inline(always)]
    pub fn iter(&self) -> impl Iterator<Item = &Card> {
        self.inner.iter().flatten()
    }
}

/// A struct to represent a reserved card.
///
/// Card can be invisible if it's reserved from the pool rather than the revealed cards.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, StableAbi)]
pub struct ReservedCard {
    pub card: Card,
    pub invisible: bool,
}

impl ReservedCard {
    /// Create a new reserved card.
    #[inline(always)]
    pub const fn new(card: Card, invisible: bool) -> Self {
        ReservedCard { card, invisible }
    }
}

impl From<Card> for ReservedCard {
    #[inline(always)]
    fn from(card: Card) -> Self {
        ReservedCard::new(card, false)
    }
}

impl From<ReservedCard> for Card {
    #[inline(always)]
    fn from(card: ReservedCard) -> Self {
        card.card
    }
}

/// A struct to represent the view of other players' reserved cards.
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, StableAbi)]
pub enum CardView {
    /// The card is invisible.
    Invisible(Tier),
    /// The card is visible.
    Visible(Card),
}

impl From<ReservedCard> for CardView {
    #[inline(always)]
    fn from(card: ReservedCard) -> Self {
        if card.invisible {
            CardView::Invisible(card.card.tier)
        } else {
            CardView::Visible(card.card)
        }
    }
}
