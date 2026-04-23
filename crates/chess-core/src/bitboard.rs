//! # Bitboard
//!
//! A `Bitboard` is a `u64` where each bit represents one of the 64 squares
//! of the chess board, indexed as:
//!
//! ```text
//! bit 0  = A1,  bit 7  = H1  (rank 1, white's back rank)
//! bit 56 = A8,  bit 63 = H8  (rank 8, black's back rank)
//! ```
//!
//! This layout matches the Little-Endian Rank-File (LERF) convention used by
//! most open-source engines (Stockfish, Crafty, etc.) and makes rank/file
//! arithmetic very natural.
//!
//! ## Why bitboards?
//!
//! Modern CPUs can operate on all 64 squares *simultaneously* with a single
//! bitwise instruction.  Counting pieces (`count_ones`), finding the lowest
//! set bit (`trailing_zeros`), and computing attacks all reduce to one or two
//! CPU instructions — far faster than looping over square arrays.

pub type Bitboard = u64;

pub const EMPTY: Bitboard = 0;

pub const ALL: Bitboard = u64::MAX;

pub const RANK_1: Bitboard = 0x0000_0000_0000_00FF;
pub const RANK_2: Bitboard = 0x0000_0000_0000_FF00;
pub const RANK_3: Bitboard = 0x0000_0000_00FF_0000;
pub const RANK_4: Bitboard = 0x0000_0000_FF00_0000;
pub const RANK_5: Bitboard = 0x0000_00FF_0000_0000;
pub const RANK_6: Bitboard = 0x0000_FF00_0000_0000;
pub const RANK_7: Bitboard = 0x00FF_0000_0000_0000;
pub const RANK_8: Bitboard = 0xFF00_0000_0000_0000;

pub const FILE_A: Bitboard = 0x0101_0101_0101_0101;
pub const FILE_B: Bitboard = 0x0202_0202_0202_0202;
pub const FILE_C: Bitboard = 0x0404_0404_0404_0404;
pub const FILE_D: Bitboard = 0x0808_0808_0808_0808;
pub const FILE_E: Bitboard = 0x1010_1010_1010_1010;
pub const FILE_F: Bitboard = 0x2020_2020_2020_2020;
pub const FILE_G: Bitboard = 0x4040_4040_4040_4040;
pub const FILE_H: Bitboard = 0x8080_8080_8080_8080;

pub const NOT_FILE_A: Bitboard = !FILE_A;
pub const NOT_FILE_H: Bitboard = !FILE_H;
pub const NOT_FILE_AB: Bitboard = !FILE_A & !FILE_B;
pub const NOT_FILE_GH: Bitboard = !FILE_G & !FILE_H;

#[inline(always)]
pub const fn lsb(bb: Bitboard) -> u32 {
  bb.trailing_zeros()
}

#[inline(always)]
pub const fn msb(bb: Bitboard) -> u32 {
  63 - bb.leading_zeros()
}

#[inline(always)]
pub fn pop_lsb(bb: &mut Bitboard) -> u32 {
  let idx = bb.trailing_zeros();
  *bb &= *bb - 1;
  idx
}

#[inline(always)]
pub const fn pop_count(bb: Bitboard) -> u32 {
  bb.count_ones()
}

#[inline(always)]
pub const fn test_bit(bb: Bitboard, sq: u32) -> bool {
  (bb >> sq) & 1 == 1
}

#[inline(always)]
pub const fn set_bit(bb: Bitboard, sq: u32) -> Bitboard {
  bb | (1u64 << sq)
}

#[inline(always)]
pub const fn clear_bit(bb: Bitboard, sq: u32) -> Bitboard {
  bb & !(1u64 << sq)
}

#[inline(always)]
pub const fn toggle_bit(bb: Bitboard, sq: u32) -> Bitboard {
  bb ^ (1u64 << sq)
}

#[inline(always)]
pub const fn north(bb: Bitboard) -> Bitboard {
  bb << 8
}

#[inline(always)]
pub const fn south(bb: Bitboard) -> Bitboard {
  bb >> 8
}

#[inline(always)]
pub const fn east(bb: Bitboard) -> Bitboard {
  (bb & NOT_FILE_H) << 1
}

#[inline(always)]
pub const fn west(bb: Bitboard) -> Bitboard {
  (bb & NOT_FILE_A) >> 1
}

pub const fn north_east(bb: Bitboard) -> Bitboard {
  (bb & NOT_FILE_H) << 9
}
pub const fn north_west(bb: Bitboard) -> Bitboard {
  (bb & NOT_FILE_A) << 7
}
pub const fn south_east(bb: Bitboard) -> Bitboard {
  (bb & NOT_FILE_H) >> 7
}
pub const fn south_west(bb: Bitboard) -> Bitboard {
  (bb & NOT_FILE_A) >> 9
}

pub fn pretty_print(bb: Bitboard) -> String {
  let mut s = String::with_capacity(128);
  for rank in (0..8).rev() {
    s.push_str(&format!("  {} │", rank + 1));
    for file in 0..8 {
      let sq = rank * 8 + file;
      s.push(if test_bit(bb, sq) { ' ' } else { '·' });
      s.push(if test_bit(bb, sq) { '■' } else { ' ' });
    }
    s.push('\n');
  }
  s.push_str("    └────────────────\n");
  s.push_str("      a b c d e f g h\n");
  s
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn pop_count_rank1() {
    assert_eq!(pop_count(RANK_1), 8);
  }

  #[test]
  fn lsb_rank1() {
    assert_eq!(lsb(RANK_1), 0);
  }

  #[test]
  fn pop_lsb_iterates_all_bits() {
    let mut bb = RANK_1;
    let mut squares = Vec::new();
    while bb != 0 {
      squares.push(pop_lsb(&mut bb));
    }
    assert_eq!(squares, vec![0, 1, 2, 3, 4, 5, 6, 7]);
  }

  #[test]
  fn set_clear_toggle() {
    let bb = set_bit(0, 27);
    assert!(test_bit(bb, 27));
    let bb = clear_bit(bb, 27);
    assert!(!test_bit(bb, 27));
    let bb = toggle_bit(bb, 27);
    assert!(test_bit(bb, 27));
  }

  #[test]
  fn shifts_do_not_wrap() {
    assert_eq!(east(FILE_H), 0);
    assert_eq!(west(FILE_A), 0);
  }

  #[test]
  fn north_south_inverses() {
    assert_eq!(south(north(RANK_4)), RANK_4);
  }
}
