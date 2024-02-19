use rand::prelude::SliceRandom;
use rand::RngCore;
use smallvec::SmallVec;
use splendor_core::{ColorVec, Noble, MAX_PLAYERS};

/// A struct to represent the noble pool.
#[derive(Debug)]
pub(crate) struct Nobles(pub SmallVec<Noble, { MAX_PLAYERS + 1 }>);

impl Nobles {
    /// Create a new noble pool with a given random number generator.
    ///
    /// This can be used to create a noble pool with a specific seed for reproducibility.
    pub fn with_rng<R: RngCore>(rng: &mut R, n: usize) -> Self {
        let nobles = NOBLES.choose_multiple(rng, n).copied().collect();
        Nobles(nobles)
    }

    /// Get the length of the pool.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Peek a noble from the pool.
    pub fn get(&self, idx: usize) -> Noble {
        self.0[idx]
    }

    /// Remove a noble from the pool.
    pub fn remove(&mut self, idx: usize) -> Noble {
        self.0.remove(idx)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Noble> {
        self.0.iter()
    }
}

const NOBLES: [Noble; 10] = [
    Noble::new(ColorVec::new(0, 0, 4, 4, 0, 0)),
    Noble::new(ColorVec::new(0, 4, 0, 0, 4, 0)),
    Noble::new(ColorVec::new(4, 0, 0, 0, 4, 0)),
    Noble::new(ColorVec::new(0, 4, 4, 0, 0, 0)),
    Noble::new(ColorVec::new(4, 0, 0, 4, 0, 0)),
    Noble::new(ColorVec::new(3, 0, 0, 3, 3, 0)),
    Noble::new(ColorVec::new(3, 3, 0, 0, 3, 0)),
    Noble::new(ColorVec::new(0, 3, 3, 3, 0, 0)),
    Noble::new(ColorVec::new(0, 3, 3, 0, 3, 0)),
    Noble::new(ColorVec::new(3, 0, 3, 3, 0, 0)),
];
