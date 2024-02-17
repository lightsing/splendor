use rand::prelude::SliceRandom;
use rand::RngCore;
use smallvec::SmallVec;
use splendor_core::{Card, Tier};
use strum::IntoEnumIterator;

mod defs;
use defs::*;

/// A struct to represent the card pool.
#[derive(Debug, Clone, Default)]
pub(crate) struct CardPool {
    pool: [SmallVec<Card, 40>; 3],
    pub revealed: [SmallVec<Card, 4>; 3],
}

impl CardPool {
    pub fn with_rng<R: RngCore>(rng: &mut R) -> Self {
        let mut tier1 = SmallVec::from(TIRE1_CARDS.as_slice());
        let mut tier2 = SmallVec::from(TIRE2_CARDS.as_slice());
        let mut tier3 = SmallVec::from(TIRE3_CARDS.as_slice());

        tier1.shuffle(rng);
        tier2.shuffle(rng);
        tier3.shuffle(rng);

        let mut this = CardPool {
            pool: [tier1, tier2, tier3],
            ..Default::default()
        };

        for tier in Tier::iter() {
            for _ in 0..4 {
                this.reveal(tier);
            }
        }

        this
    }

    /// Reveal a card from the given tier.
    ///
    /// Returns true if a new card is revealed, false otherwise.
    #[inline(always)]
    fn reveal(&mut self, tier: Tier) -> bool {
        let revealed = &mut self.revealed[tier as usize];
        let cards = &mut self.pool[tier as usize];
        if revealed.len() < 4 {
            if let Some(card) = cards.pop() {
                revealed.push(card);
                return true;
            }
        }
        false
    }

    /// Get the remaining cards in the tiers.
    #[inline(always)]
    pub fn remaining(&self) -> [usize; 3] {
        self.pool
            .iter()
            .map(|x| x.len())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    /// Get the amount of cards in the revealed cards.
    #[inline(always)]
    pub fn revealed(&self) -> [usize; 3] {
        self.revealed
            .iter()
            .map(|x| x.len())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    /// Peek a card from the revealed cards.
    pub fn peek(&self, tier: Tier, index: usize) -> Option<&Card> {
        self.revealed[tier as usize].get(index)
    }

    /// Take a card from the revealed cards and reveal a new card.
    #[inline(always)]
    pub fn take(&mut self, tier: Tier, index: usize) -> Card {
        assert!(index < self.revealed[tier as usize].len());
        let card = self.revealed[tier as usize].remove(index);
        self.reveal(tier);
        card
    }

    /// Take a card from top of the pool.
    #[inline(always)]
    pub fn take_from_pool(&mut self, tier: Tier) -> Card {
        self.pool[tier as usize].pop().unwrap()
    }
}
