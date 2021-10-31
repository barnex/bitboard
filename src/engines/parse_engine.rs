use super::internal::*;

pub fn parse_engine(name: &str, seed: u64) -> Result<Box<dyn Engine>> {
	Ok(match name {
		"random" => Box::new(Random::new(seed)),
		"greedy" => Box::new(Greedy::new(seed)),
		"l0.mat" => Box::new(Lookahead0::with(seed, material)),
		"l0.only_k_dist" => Box::new(Lookahead0::with(seed, king_distance)),
		"l0.mat+k_dist" => Box::new(Lookahead0::with(seed, material_and(king_distance))),
		unknown => return Err(format_err!("unknown engine: {}", unknown)),
	})
}
