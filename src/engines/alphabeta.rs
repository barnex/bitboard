use super::internal::*;

pub struct AlphaBeta<F>
where
	F: Fn(&BitBoard, Color) -> i32 + Sync,
{
	leaf_value: F,
	depth: u32,
}

impl<F> AlphaBeta<F>
where
	F: Fn(&BitBoard, Color) -> i32 +Sync,
{
	pub fn new(leaf_value: F, depth: u32) -> Self {
		Self { leaf_value, depth }
	}

	fn alphabeta(&self, board: &BitBoard, player: Color, alpha: i32, beta: i32, depth: u32) -> i32 {
		// must stop iteration so that we would not trade a king for a king :-)
		if !board.has_king(player) {
			return -INF + (depth as i32);
		}
		if !board.has_king(player.opposite()) {
			return INF - (depth as i32); // tiny offset to push for mate in minimum moves
		}

		if depth == 0 {
			return (self.leaf_value)(board, player);
		}

		let mut mv_boards = board //
			.collect_moves(player)
			.iter()
			.map(|&mv| (mv, board.with_move(mv)))
			.collect::<Vec<_>>();

		// sorting moves most promising first
		// results in massively better alpha-beta pruning
		// but is only worth the cost at least two levels above leaf.
		if depth > 1 {
			let mut mv_board_value = mv_boards
				.into_iter()
				.map(|(mv, board)| {
					let value = (self.leaf_value)(&board, player);
					(mv, board, value)
				})
				.collect::<SmVec<_>>();
			mv_board_value.sort_by_key(|(_, _, v)| -*v);
			mv_boards = mv_board_value.into_iter().map(|(mv, board, _)| (mv, board)).collect();
		}

		let mut best_value = -INF + (depth as i32);
		let mut alpha = alpha;
		for (mv, board) in mv_boards {
			// TODO: filter out bad moves at board level.
			if board.is_check(player) {
				continue;
			}

			let value = -self.alphabeta(&board, player.opposite(), -beta, -alpha, depth - 1);
			if value >= best_value {
				best_value = value;
			}

			alpha = i32::max(alpha, value);
			if alpha >= beta {
				break;
			}
		}
		best_value
	}
}

impl<F> Engine for AlphaBeta<F>
where
	F: Fn(&BitBoard, Color) -> i32 + Sync,
{
	fn do_move(&self, rng: &mut StdRng, board: &BitBoard, player: Color) -> Option<Move> {
		psearch_with_tiebreak(rng, board, player, |board, player| {
			-self.alphabeta(board, player.opposite(), -INF, INF, self.depth)
		})
	}
}
