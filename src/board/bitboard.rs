use super::internal::*;
use Square::*;

#[derive(Clone)]
pub struct BitBoard {
	sets: [u64; 13],
}

impl Board for BitBoard {
	fn new() -> Self {
		let mut pieces = [0; 13];
		pieces[Empty.index()] = !0;
		Self { sets: pieces }
	}

	fn at(&self, pos: Pos) -> Square {
		let mask = 1 << pos.index();
		for sq in Square::ALL_SQUARES {
			if self.sets[sq.index()] & mask != 0 {
				return sq;
			}
		}
		unreachable!()
	}

	fn set(&mut self, pos: Pos, sq: Square) {
		debug_assert!(pos.is_valid());
		let pos = pos.index() as u8;
		self.clear(pos);
		self.sets[sq.index()] |= 1 << pos;
	}

	fn all_moves(&self, player: Color) -> SmVec<Move> {
		SmVec::new()
	}

	fn with_move(&self, mv: Move) -> Self {
		let mut b = self.clone();
		b.set(mv.to, b.at(mv.from));
		b.set(mv.from, Square::Empty);
		b
	}
}

fn fmt_bits(bits: u64) -> String {
	let mut str = String::with_capacity(2 * 64 + 8);
	for r in (0..8).rev() {
		for c in 0..8 {
			let mask = 1 << pos(r, c).index();
			let bit = (bits & mask) != 0;

			str.push(if bit { '1' } else { '0' });
			str.push(' ');
		}
		str.push('\n');
	}
	str
}

impl fmt::Debug for BitBoard {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for sq in Square::ALL_SQUARES {
			write!(f, "{:?}:\n{}\n", sq, fmt_bits(self.sets[sq.index()]))?;
		}
		Ok(())
	}
}

impl BitBoard {
	fn w_pawn_pushes(&self) -> u64 {
		//let pawns = self.pieces[W_PAWN_I];
		//let push1 = (pawns << 8 & self.pieces[EMPTY_I]);

		//push1
		0
	}

	fn clear(&mut self, pos: u8) {
		let mask = !(1 << pos);
		for sq in Square::ALL_SQUARES {
			self.sets[sq.index()] &= mask;
		}
	}
}
