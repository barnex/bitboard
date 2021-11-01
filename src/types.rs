pub use std::fmt;
pub use std::ops::Index;
pub use std::str::FromStr;

#[allow(unused)]
pub type Result<T> = anyhow::Result<T>;

pub use anyhow::format_err;

pub type Set<T> = fnv::FnvHashSet<T>;

pub use rand::rngs::StdRng;
pub use rand::Rng;
pub use rand::SeedableRng;

#[allow(unused)]
pub type SmVec<T> = smallvec::SmallVec<[T; 32]>;

/*
pub struct MiniVec<T: Default + Copy, const N: usize> {
	el: [T; N],
	len: usize,
}

impl<T: Default + Copy, const N: usize> MiniVec<T, N> {
	pub fn new() -> Self {
		Self {
			el: [T::default(); N],
			len: 0,
		}
	}
	pub fn push(&mut self, el: T) {
		self.el[self.len] = el;
		self.len += 1;
	}
}
*/
