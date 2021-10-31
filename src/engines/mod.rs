mod internal;

mod engine;
pub use engine::*;

mod random;
pub use random::*;

mod greedy;
pub use greedy::*;

mod lookahead0;
pub use lookahead0::*;

mod lookahead1;
pub use lookahead1::*;

mod value_functions;
pub use value_functions::*;

use crate::types::*;

pub fn parse_engine(name: &str, seed: u64) -> Result<Box<dyn Engine>> {
	Ok(match name {
		"random" => Box::new(Random::new(seed)),
		"greedy" => Box::new(Greedy::new(seed)),
		"l0.mat" => Box::new(Lookahead0::with(seed, material)),
		"l0.only_k_dist" => Box::new(Lookahead0::with(seed, king_distance)),
		"l0.mat+k_dist" => Box::new(Lookahead0::with(seed, material_and(king_distance))),
		//"l0.k_dist" => Box::new(greedy_with_king_dist(seed)),
		//"l1.mat" => Box::new(l1_material(seed)),
		//"l1.k_dist" => Box::new(l1_material(seed)),
		unknown => return Err(format_err!("unknown engine: {}", unknown)),
	})
}
