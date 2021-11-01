use super::internal::*;

/// Randomly pick one of the `num` best moves, provided they lie within `max_diff` from the best.
/// Useful when comparing/training strategy function that would otherwise be highly deterministic,
/// causing most games to be the same or very similar.
pub fn pick_randomized_within(rng: &mut StdRng, options: &[(Move, i32)], num: usize, max_diff: u32) -> Option<Move> {
	assert!(num < 10); // so we don't accidentally swap with max_diff

	let best = best_value(options)?;

	let top_options = options //
		.iter()
		.filter(|(_, value)| (i32::abs(value - best) as u32) < max_diff)
		.collect::<SmVec<_>>();

	let n = usize::min(top_options.len(), num);
	let random = rng.gen_range(0..n);
	top_options.get(random).map(|v| v.0)
}

/// randomly pick from all moves with best value
pub fn pick_best_with_tiebreak(rng: &mut StdRng, options: &[(Move, i32)]) -> Option<Move> {
	let equal = equally_best_options(options);
	let random = rng.gen_range(0..equal.len());
	equal.get(random).map(|v| v.0)
}

/// Select all (Move, value) pairs with value equal to the best (maximum) value.
pub fn equally_best_options(options: &[(Move, i32)]) -> SmVec<(Move, i32)> {
	let best = best_value(options).unwrap_or_default();

	options //
		.iter()
		.copied()
		.filter(|(_, value)| *value == best)
		.collect()
}

pub fn best_value(options: &[(Move, i32)]) -> Option<i32> {
	options.iter().map(|v| v.1).max()
}
