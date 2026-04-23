//! # Square, File, Rank
//!
//! `Square` is a newtype over `u8` in 0..=63, indexed as:
//!
//! ```text
//! 56 57 58 59 60 61 62 63   ← rank 8
//! 48 49 50 51 52 53 54 55   ← rank 7
//!  .  .  .  .  .  .  .  .
//!  8  9 10 11 12 13 14 15   ← rank 2
//!  0  1  2  3  4  5  6  7   ← rank 1
//!  a  b  c  d  e  f  g  h
//! ```
//!
//! All conversions are `const` so they can be used in compile-time table generation.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum File {
  A = 0,
  B = 1,
  C = 2,
  D = 3,
  E = 4,
  F = 5,
  G = 6,
  H = 7,
}

impl File {
  #[inline(always)]
  pub const fn from_u8(v: u8) -> Self {
    assert!(v < 8, "File index out of range");
    unsafe { std::mem::transmute(v) }
  }

  #[inline(always)]
  pub const fn index(self) -> u8 {
    self as u8
  }
}

impl fmt::Display for File {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", (b'a' + self.index()) as char)
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Rank {
  R1 = 0,
  R2 = 1,
  R3 = 2,
  R4 = 3,
  R5 = 4,
  R6 = 5,
  R7 = 6,
  R8 = 7,
}

impl Rank {
  #[inline(always)]
  pub const fn from_u8(v: u8) -> Self {
    assert!(v < 8, "Rank index out of range");
    unsafe { std::mem::transmute(v) }
  }

  #[inline(always)]
  pub const fn index(self) -> u8 {
    self as u8
  }
}

impl fmt::Display for Rank {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.index() + 1)
  }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Square(u8);

impl Square {
  pub const A1: Square = Square(0);
  pub const B1: Square = Square(1);
  pub const C1: Square = Square(2);
  pub const D1: Square = Square(3);
  pub const E1: Square = Square(4);
  pub const F1: Square = Square(5);
  pub const G1: Square = Square(6);
  pub const H1: Square = Square(7);

  pub const A2: Square = Square(8);
  pub const B2: Square = Square(9);
  pub const C2: Square = Square(10);
  pub const D2: Square = Square(11);
  pub const E2: Square = Square(12);
  pub const F2: Square = Square(13);
  pub const G2: Square = Square(14);
  pub const H2: Square = Square(15);

  pub const A3: Square = Square(16);
  pub const B3: Square = Square(17);
  pub const C3: Square = Square(18);
  pub const D3: Square = Square(19);
  pub const E3: Square = Square(20);
  pub const F3: Square = Square(21);
  pub const G3: Square = Square(22);
  pub const H3: Square = Square(23);

  pub const A4: Square = Square(24);
  pub const B4: Square = Square(25);
  pub const C4: Square = Square(26);
  pub const D4: Square = Square(27);
  pub const E4: Square = Square(28);
  pub const F4: Square = Square(29);
  pub const G4: Square = Square(30);
  pub const H4: Square = Square(31);

  pub const A5: Square = Square(32);
  pub const B5: Square = Square(33);
  pub const C5: Square = Square(34);
  pub const D5: Square = Square(35);
  pub const E5: Square = Square(36);
  pub const F5: Square = Square(37);
  pub const G5: Square = Square(38);
  pub const H5: Square = Square(39);

  pub const A6: Square = Square(40);
  pub const B6: Square = Square(41);
  pub const C6: Square = Square(42);
  pub const D6: Square = Square(43);
  pub const E6: Square = Square(44);
  pub const F6: Square = Square(45);
  pub const G6: Square = Square(46);
  pub const H6: Square = Square(47);

  pub const A7: Square = Square(48);
  pub const B7: Square = Square(49);
  pub const C7: Square = Square(50);
  pub const D7: Square = Square(51);
  pub const E7: Square = Square(52);
  pub const F7: Square = Square(53);
  pub const G7: Square = Square(54);
  pub const H7: Square = Square(55);

  pub const A8: Square = Square(56);
  pub const B8: Square = Square(57);
  pub const C8: Square = Square(58);
  pub const D8: Square = Square(59);
  pub const E8: Square = Square(60);
  pub const F8: Square = Square(61);
  pub const G8: Square = Square(62);
  pub const H8: Square = Square(63);

  #[inline(always)]
  pub const fn from_index(idx: u8) -> Self {
    assert!(idx < 64, "Square index out of range");
    Square(idx)
  }

  #[inline(always)]
  pub const fn from_file_rank(file: File, rank: Rank) -> Self {
    Square(rank.index() * 8 + file.index())
  }

  #[inline(always)]
  pub const fn index(self) -> u8 {
    self.0
  }

  #[inline(always)]
  pub const fn file(self) -> File {
    File::from_u8(self.0 % 8)
  }

  #[inline(always)]
  pub const fn rank(self) -> Rank {
    Rank::from_u8(self.0 / 8)
  }

  #[inline(always)]
  pub const fn bitboard(self) -> crate::bitboard::Bitboard {
    1u64 << self.0
  }

  pub fn chebyshev(self, other: Square) -> u8 {
    let file_dist =
      (self.file().index() as i8 - other.file().index() as i8).unsigned_abs();
    let rank_dist =
      (self.rank().index() as i8 - other.rank().index() as i8).unsigned_abs();
    file_dist.max(rank_dist)
  }

  pub fn manhattan(self, other: Square) -> u8 {
    let file_dist =
      (self.file().index() as i8 - other.file().index() as i8).unsigned_abs();
    let rank_dist =
      (self.rank().index() as i8 - other.rank().index() as i8).unsigned_abs();
    file_dist + rank_dist
  }

  #[inline(always)]
  pub const fn flip_rank(self) -> Self {
    Square(self.0 ^ 56)
  }
}

impl fmt::Debug for Square {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self)
  }
}

impl fmt::Display for Square {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}", self.file(), self.rank())
  }
}

impl std::str::FromStr for Square {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let s = s.as_bytes();
    if s.len() != 2 {
      return Err(format!("invalid square: expected 2 chars, got {}", s.len()));
    }
    let file = match s[0] {
      b'a'..=b'h' => File::from_u8(s[0] - b'a'),
      _ => return Err(format!("invalid file: '{}'", s[0] as char)),
    };
    let rank = match s[1] {
      b'1'..=b'8' => Rank::from_u8(s[1] - b'1'),
      _ => return Err(format!("invalid rank: '{}'", s[1] as char)),
    };
    Ok(Square::from_file_rank(file, rank))
  }
}

pub struct SquareIter(u8);

impl Iterator for SquareIter {
  type Item = Square;
  fn next(&mut self) -> Option<Self::Item> {
    if self.0 < 64 {
      let sq = Square(self.0);
      self.0 += 1;
      Some(sq)
    } else {
      None
    }
  }
}

impl Square {
  pub fn all() -> SquareIter {
    SquareIter(0)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::str::FromStr;

  #[test]
  fn e4_index() {
    let sq = Square::from_str("e4").unwrap();
    assert_eq!(sq.index(), 28);
    assert_eq!(sq.file(), File::E);
    assert_eq!(sq.rank(), Rank::R4);
  }

  #[test]
  fn display_roundtrip() {
    for i in 0u8..64 {
      let sq = Square::from_index(i);
      let back = Square::from_str(&sq.to_string()).unwrap();
      assert_eq!(sq, back);
    }
  }

  #[test]
  fn flip_rank_e1_is_e8() {
    assert_eq!(Square::E1.flip_rank(), Square::E8);
  }

  #[test]
  fn bitboard_is_single_bit() {
    let sq = Square::D4;
    assert_eq!(sq.bitboard().count_ones(), 1);
    assert_eq!(sq.bitboard().trailing_zeros(), sq.index() as u32);
  }

  #[test]
  fn all_iter_yields_64_squares() {
    assert_eq!(Square::all().count(), 64);
  }
}
