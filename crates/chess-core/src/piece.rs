//! # Pieces
//!
//! Defines the `Color`, `PieceType`, and `Piece` enums, plus the compact
//! index scheme that maps each (color, piece-type) pair to a slot 0..11 in
//! the `Board::pieces` bitboard array.
//!
//! ## Bitboard array layout
//!
//! ```text
//! index  0 = White Pawns        index  6 = Black Pawns
//! index  1 = White Knights      index  7 = Black Knights
//! index  2 = White Bishops      index  8 = Black Bishops
//! index  3 = White Rooks        index  9 = Black Rooks
//! index  4 = White Queens       index 10 = Black Queens
//! index  5 = White Kings        index 11 = Black Kings
//! ```

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Color {
  White = 0,
  Black = 1,
}

impl Color {
  #[inline(always)]
  pub const fn flip(self) -> Color {
    match self {
      Color::White => Color::Black,
      Color::Black => Color::White,
    }
  }

  #[inline(always)]
  pub const fn index(self) -> usize {
    self as usize
  }
}

impl fmt::Display for Color {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Color::White => write!(f, "white"),
      Color::Black => write!(f, "black"),
    }
  }
}
