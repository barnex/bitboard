/*
use std::hash::Hasher;

pub fn rand(seed: u64, i: usize) -> u64 {
	let mut h = fnv::FnvHasher::with_key(seed);
	h.write_usize(i);
	h.finish()
}


pub fn randn(seed: u64, i: usize, n: u64) -> u64 {
	rand(seed, i) % n
}
*/