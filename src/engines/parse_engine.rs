use super::internal::*;

pub fn parse_engine(name: &str) -> Result<Box<dyn Engine>> {
	Ok(match name {
		"valid" => Box::new(Valid()),
		"greedy-zero" => Box::new(Greedy(zero)),
		"greedy-material" => Box::new(Greedy(material)),

		"negamax0-material" => Box::new(NegaMax::new(0, material)),
		"negamax1-material" => Box::new(NegaMax::new(1, material)),
		"negamax2-material" => Box::new(NegaMax::new(2, material)),
		"negamax3-material" => Box::new(NegaMax::new(3, material)),

		"alphabeta0-material" => Box::new(AlphaBeta::new(0, material)),
		"alphabeta1-material" => Box::new(AlphaBeta::new(1, material)),
		"alphabeta2-material" => Box::new(AlphaBeta::new(2, material)),
		"alphabeta3-material" => Box::new(AlphaBeta::new(3, material)),
		"alphabeta4-material" => Box::new(AlphaBeta::new(4, material)),

		"palphabeta0-material" => Box::new(ParAlphaBeta::new(0, material)),
		"palphabeta1-material" => Box::new(ParAlphaBeta::new(1, material)),
		"palphabeta2-material" => Box::new(ParAlphaBeta::new(2, material)),
		"palphabeta3-material" => Box::new(ParAlphaBeta::new(3, material)),
		"palphabeta4-material" => Box::new(ParAlphaBeta::new(4, material)),

		"palphabeta0-strat1" => Box::new(ParAlphaBeta::new(0, heuristic1)),
		"palphabeta1-strat1" => Box::new(ParAlphaBeta::new(1, heuristic1)),
		"palphabeta2-strat1" => Box::new(ParAlphaBeta::new(2, heuristic1)),
		"palphabeta3-strat1" => Box::new(ParAlphaBeta::new(3, heuristic1)),
		"palphabeta4-strat1" => Box::new(ParAlphaBeta::new(4, heuristic1)),

		"palphabeta0-strat2" => Box::new(ParAlphaBeta::new(0, heuristic2)),
		"palphabeta1-strat2" => Box::new(ParAlphaBeta::new(1, heuristic2)),
		"palphabeta2-strat2" => Box::new(ParAlphaBeta::new(2, heuristic2)),
		"palphabeta3-strat2" => Box::new(ParAlphaBeta::new(3, heuristic2)),
		"palphabeta4-strat2" => Box::new(ParAlphaBeta::new(4, heuristic2)),

		unknown => return Err(format_err!("unknown engine: {}", unknown)),
	})
}
