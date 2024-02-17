use crate::colors::{Color, ColorVec};
use num_enum::TryFromPrimitive;
use serde::Serialize;
use smallvec::SmallVec;
use strum::EnumIter;

/// An enum to represent the card tiers.
#[repr(u8)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, EnumIter, TryFromPrimitive, Serialize,
)]
pub enum Tier {
    /// The first tier.
    I = 0,
    /// The second tier.
    II,
    /// The third tier.
    III,
}

/// A struct to represent a card.

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize)]
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
    /// Define a new card.
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

#[derive(Debug, Default, Clone, Serialize)]
pub struct DevelopmentCards {
    /// The total points of the development cards.
    pub points: u8,
    /// The total bonus of the development cards.
    pub bonus: ColorVec,
    /// The cards in the development cards, grouped by bonus color.
    inner: [SmallVec<Card, 7>; 5],
}

impl DevelopmentCards {
    /// Create a new development cards with empty cards.
    #[inline(always)]
    pub const fn new() -> Self {
        DevelopmentCards {
            points: 0,
            bonus: ColorVec::empty(),
            inner: [
                SmallVec::new(),
                SmallVec::new(),
                SmallVec::new(),
                SmallVec::new(),
                SmallVec::new(),
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
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ReservedCard {
    /// The card.
    pub card: Card,
    /// Is the card invisible.
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
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
