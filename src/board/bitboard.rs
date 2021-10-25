use super::internal::*;

#[derive(Clone, Default)]
pub struct BitBoard {
	sets: [u64; 12],
	mailbox: Mailbox,
}

impl Board for BitBoard {
	fn at(&self, pos: Pos) -> Square {
		self.mailbox.at(pos)
	}

	fn set(&mut self, pos: Pos, sq: Square) {
		debug_assert!(pos.is_valid());

		self.mailbox.set(pos, sq)

		//let pos = pos.index64();
		//self.clear(pos);

		//let idx = Self::set_idx(sq);
		//if idx != 255{
		//    self.sets[i]
		//}
	}

	// fn set_i(&mut self, pos: u8, : Square) {
	// 	//let idx = Self::set_idx(sq);
	// 	//if idx != 255{
	// 	//    self.sets[i]
	// 	//}
	// }

	fn all_moves(&self, player: Color) -> SmVec<Move> {
		self.mailbox.all_moves(player)
	}

	fn with_move(&self, mv: Move) -> Self {
		let mut b = self.clone();
		b.set(mv.to, b.at(mv.from));
		b.set(mv.from, Square::Empty);
		b
	}
}

impl BitBoard {
	fn clear(&mut self, pos: u8) {
		let mask = !(1 << pos);
		for i in 0..self.sets.len() {
			self.sets[i] &= mask;
		}
	}

	fn set_idx(sq: Square) -> u8 {
		use Square::*;
		match sq {
			WPawn => 0,
			WRook => 1,
			WKnight => 2,
			WBisshop => 3,
			WQueen => 4,
			WKing => 5,
			BPawn => 6,
			BRook => 7,
			BKnight => 8,
			BBisshop => 9,
			BQueen => 10,
			BKing => 11,
			_ => 255,
		}
	}
}
