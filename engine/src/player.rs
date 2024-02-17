use abi_stable::std_types::RVec;
use splendor_core::{ColorVec, DevelopmentCards, Noble, ReservedCard};

#[derive(Debug, Clone, Default)]
pub(crate) struct PlayerContext {
    /// The index of the player.
    pub idx: usize,
    /// The tokens in the player's hand.
    pub tokens: ColorVec,
    /// The development cards in the player's hand.
    pub development_cards: DevelopmentCards,
    /// The reserved cards in the player's hand.
    pub reserved_cards: RVec<ReservedCard>,
    /// The nobles the player has visited.
    pub nobles: RVec<Noble>,
}

impl PlayerContext {
    /// Create a new player with empty cards.
    #[inline(always)]
    pub const fn new(idx: usize) -> Self {
        PlayerContext {
            idx,
            tokens: ColorVec::new(0, 0, 0, 0, 0, 0),
            development_cards: DevelopmentCards::new(),
            reserved_cards: RVec::new(),
            nobles: RVec::new(),
        }
    }

    pub fn points(&self) -> u8 {
        self.development_cards.points + self.nobles.iter().map(|n| n.points).sum::<u8>()
    }
}
