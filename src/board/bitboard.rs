use super::internal::*;
use Square::*;

#[derive(Clone)]
pub struct BitBoard {
	pieces: [u64; 13],
}

impl Board for BitBoard {
	fn new() -> Self {
		let mut pieces = [0; 13];
		pieces[Empty.index()] = !0;
		Self { pieces }
	}

	fn at(&self, pos: Pos) -> Square {
		let mask = 1 << pos.index();
		for sq in Square::ALL_SQUARES {
			//println!("{}{:064b}", sq, self.pieces[sq.index()]);
			if self.pieces[sq.index()] & mask != 0 {}
		}
		print!("HUH?");
		Empty
		//unreachable!()
	}

	fn set(&mut self, pos: Pos, sq: Square) {
		debug_assert!(pos.is_valid());
		let pos = pos.index() as u8;
		self.clear(pos);
		self.pieces[sq.index()] |= 1 << pos;
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

impl BitBoard {
	fn w_pawn_pushes(&self) -> u64 {
		//let pawns = self.pieces[W_PAWN_I];
		//let push1 = (pawns << 8 & self.pieces[EMPTY_I]);

		//push1
		0
	}

	fn clear(&mut self, pos: u8) {
		let mask = !(1 << pos);
		for i in 0..self.pieces.len() {
			self.pieces[i] &= mask;
		}
	}
}
