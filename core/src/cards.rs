use crate::colors::{Color, ColorVec};
use num_enum::{TryFromPrimitive, TryFromPrimitiveError};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use strum::EnumIter;

/// An enum to represent the card tiers.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, EnumIter, TryFromPrimitive)]
pub enum Tier {
    /// The first tier.
    I = 0,
    /// The second tier.
    II,
    /// The third tier.
    III,
}

impl Tier {
    /// Get the emoji representation of the tier.
    #[inline(always)]
    pub fn emoji(&self) -> &'static str {
        match self {
            Tier::I => "1️⃣",
            Tier::II => "2️⃣",
            Tier::III => "3️⃣",
        }
    }
}

impl Serialize for Tier {
    #[inline(always)]
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u8(*self as u8)
    }
}

impl<'de> Deserialize<'de> for Tier {
    #[inline(always)]
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Tier::try_from(u8::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

impl TryFrom<usize> for Tier {
    type Error = TryFromPrimitiveError<Self>;

    #[inline(always)]
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Tier::try_from(value as u8)
    }
}

/// A struct to represent a card.

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DevelopmentCards {
    /// The total points of the development cards.
    pub points: u8,
    /// The total bonus of the development cards.
    pub bonus: ColorVec,
    /// The cards in the development cards, grouped by bonus color.
    pub inner: [SmallVec<Card, 7>; 5],
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "view")]
#[serde(rename_all = "snake_case")]
pub enum CardView {
    /// The card is invisible.
    Invisible(Tier),
    /// The card is visible.
    Visible(Card),
}

impl CardView {
    /// Construct a new visible card view.
    #[inline(always)]
    pub const fn visible(card: Card) -> Self {
        CardView::Visible(card)
    }

    /// unwrap the card view.
    #[inline(always)]
    pub fn unwrap(&self) -> &Card {
        match self {
            CardView::Visible(ref card) => card,
            _ => panic!("cannot unwrap invisible card"),
        }
    }
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
