mod internal;

mod engine;
pub use engine::*;

mod random;
pub use random::*;

mod greedy;
pub use greedy::*;

mod greedy_with;
pub use greedy_with::*;

mod lookahead1;
pub use lookahead1::*;

use crate::types::*;

pub fn parse_engine(name: &str, seed: u64) -> Result<Box<dyn Engine>> {
	Ok(match name {
		"random" => Box::new(Random::new(seed)),
		"l0.mat" | "l0" | "greedy" => Box::new(Greedy::new(seed)),
		"l0.k_dist" | "greedy_k_dist" => Box::new(greedy_with_king_dist(seed)),
		"l1.mat" | "l1" => Box::new(l1_material(seed)),
		unknown => return Err(format_err!("unknown engine: {}", unknown)),
	})
}
