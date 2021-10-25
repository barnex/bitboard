use std::hash::Hasher;

pub fn rand(seed: u64, i: usize) -> u64 {
	let mut h = fnv::FnvHasher::with_key(seed);
	h.write_usize(i);
	h.finish()
}


