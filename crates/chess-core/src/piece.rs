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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PieceType {
  Pawn = 0,
  Knight = 1,
  Bishop = 2,
  Rook = 3,
  Queen = 4,
  King = 5,
}

impl PieceType {
  #[inline(always)]
  pub const fn index(self) -> usize {
    self as usize
  }

  #[inline(always)]
  pub const fn bb_index(self, color: Color) -> usize {
    self.index() + color.index() * 6
  }

  pub const fn fen_char(self) -> char {
    match self {
      PieceType::Pawn => 'P',
      PieceType::Knight => 'N',
      PieceType::Bishop => 'B',
      PieceType::Rook => 'R',
      PieceType::Queen => 'Q',
      PieceType::King => 'K',
    }
  }

  pub const fn value(self) -> i32 {
    match self {
      PieceType::Pawn => 100,
      PieceType::Knight => 320,
      PieceType::Bishop => 330,
      PieceType::Rook => 500,
      PieceType::Queen => 900,
      PieceType::King => 20000,
    }
  }

  pub fn from_fen_char(c: char) -> Option<Self> {
    match c.to_ascii_uppercase() {
      'P' => Some(PieceType::Pawn),
      'N' => Some(PieceType::Knight),
      'B' => Some(PieceType::Bishop),
      'R' => Some(PieceType::Rook),
      'Q' => Some(PieceType::Queen),
      'K' => Some(PieceType::King),
      _ => None,
    }
  }

  pub const ALL: [PieceType; 6] = [
    PieceType::Pawn,
    PieceType::Knight,
    PieceType::Bishop,
    PieceType::Rook,
    PieceType::Queen,
    PieceType::King,
  ];
}

impl fmt::Display for PieceType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.fen_char())
  }
}
