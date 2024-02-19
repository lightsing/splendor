//! Core game structures implementation.
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

#[macro_use]
extern crate log;

mod action;
mod actor;
mod cards;
mod colors;
mod nobles;
mod record;
mod snapshot;

pub use action::*;
pub use actor::*;
pub use cards::*;
pub use colors::*;
pub use nobles::*;
pub use record::*;
pub use snapshot::*;

/// The maximum number of players in a game.
pub const MAX_PLAYERS: usize = 4;
