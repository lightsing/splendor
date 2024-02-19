//! A Rust implementation of the board game Splendor.
#![deny(missing_docs)]

#[macro_use]
extern crate log;

mod action;
mod cards;
mod error;
mod game;
mod nobles;
mod player;
#[cfg(all(feature = "test", test))]
mod tests;

pub use game::GameContext;
