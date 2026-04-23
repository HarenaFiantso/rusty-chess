//! # chess-core
//!
//! The foundation of the RustyChess engine.
//! This crate is dependency-free and contains everything needed to represent,
//! parse, and hash a chess position:

#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
pub mod bitboard;

pub use bitboard::Bitboard;
