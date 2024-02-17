//! A Rust implementation of the board game Splendor.
#![deny(missing_docs)]

mod action;
mod cards;
mod error;
mod game;
mod nobles;
mod player;

pub use game::GameContext;
