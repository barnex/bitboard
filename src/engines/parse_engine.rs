use super::internal::*;

pub fn parse_engine(name: &str) -> Result<Box<dyn Engine>> {
	Ok(match name {
		"random" => Box::new(Random()),
		"greedy" => Box::new(Greedy()),
		"l1.mat" => Box::new(Lookahead1(material)),
		"l2.mat" => Box::new(Lookahead2(material)),
		"l3.mat" => Box::new(Lookahead::new(material, 3)),
		//"l0.only_k_dist" => Box::new(Lookahead0::with(seed, king_distance)),
		//"l0.mat+k_dist" => Box::new(Lookahead0::with(seed, material_and(king_distance))),
		unknown => return Err(format_err!("unknown engine: {}", unknown)),
	})
}
