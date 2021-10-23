use super::internal::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct Move {
	pub from: Pos,
	pub to: Pos,
}

impl Move {
	#[inline]
	pub fn new(from: Pos, to: Pos) -> Self {
		debug_assert!(from != to);
		Self { from, to }
	}

	pub fn is_valid(self) -> bool {
		self.from.is_valid() && self.to.is_valid() && self.from != self.to
	}
}

//impl From<(Pos, Pos)> for Move {
//	fn from((from, to): (Pos, Pos)) -> Self {
//		Self::new(from, to)
//	}
//}

impl fmt::Display for Move {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}{}", self.from, self.to)
	}
}

pub struct Moves {
	inner: SmVec<Move>,
}

impl Moves {
	pub fn push(&mut self, mv: Move) {
		self.inner.push(mv)
	}

	pub fn to_set(self) -> Set<Move> {
		self.inner.into_iter().collect()
	}
}
