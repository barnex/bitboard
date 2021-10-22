/*
use std::str::FromStr;

use super::internal::*;

/// https://en.wikipedia.org/wiki/Bitboard.
#[derive(PartialEq, Eq, Default)]
pub struct BitBoard {
	/// Bitfields indexed by Piece index.
	/// (A `1` indicates the presence of a piece).
	fields: [u64; 12],
}

impl BitBoard {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn from_mask(piece: Piece, mask: Mask) -> Self {
		let mut masks = [Mask(0); 12];
		masks[piece.index()] = mask;
		Self { fields: masks }
	}

	#[inline]
	pub fn set_i(&mut self, index: u8, piece: Piece) {
		self.clear(index);
		self.mask_mut(piece).set(index);
	}

	#[inline]
	pub fn set_rc(&mut self, pos: (u8, u8), piece: Piece) {
		let index = pos.0 << 3 | pos.1;
		self.clear(index);
		self.mask_mut(piece).set(index);
	}

	#[inline]
	fn clear(&mut self, index: u8) {
		let clear = !(1 << index);
		for i in 0..self.fields.len() {
			self.fields[i].0 &= clear;
		}
	}

	#[inline]
	pub fn mask(&self, piece: Piece) -> Mask {
		self.fields[piece.index()]
	}

	#[inline]
	pub fn mask_mut(&mut self, piece: Piece) -> &mut Mask {
		&mut self.fields[piece.index()]
	}

	pub fn mailbox(&self) -> [[Option<Piece>; 8]; 8] {
		let mut decoded = [[None; 8]; 8];
		for piece in Piece::ALL {
			let mask = self.mask(piece);
			for row in 0..8 {
				for col in 0..8 {
					let i = row << 3 | col;
					if mask.bit(i) {
						decoded[col as usize][row as usize] = Some(piece);
					}
				}
			}
		}
		decoded
	}
}

impl ToString for BitBoard {
	fn to_string(&self) -> String {
		let decoded = self.mailbox();

		let mut str = String::from("\n");
		for r in (0..8).rev() {
			str += &format!("{}", r + 1);
			for c in 0..8 {
				str.push(' ');
				str.push(Piece::ascii(decoded[r][c]));
			}
			str.push('\n')
		}
		str += "  a b c d e f g h";
		str
	}
}

impl fmt::Debug for BitBoard {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.to_string())
	}
}

impl FromStr for BitBoard {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self> {
		Ok(Self::new())
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn from_mask() {
		use Piece::*;
		let a = BitBoard::from_mask(WPawn, Mask::from_bit()
		let b = BitBoard::new();
		assert_eq!(a, b);
	}

	#[test]
	fn test_set() {
		use Piece::*;
		let a = BitBoard::new();
		let mut b = BitBoard::new();
		b.set_rc((1, 2), WPawn);
		assert_eq!(a, b);
	}
}
*/
