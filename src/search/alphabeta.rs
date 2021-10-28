use super::internal::*;

pub const INF: i32 = 999_999_999; // effectively infinite value

/// How good is board for player?
/// Good scores are always positive, bad scores always negative,
/// regardless of player color.
pub fn alphabeta<B, F>(board: &B, player: Color, leaf_eval: &F, depth: u32) -> i32
where
	B: Board,
	F: Fn(&B, Color) -> i32,
{
	alphabeta_(board, player, leaf_eval, -INF, INF, depth).1
}

pub fn alphabeta_<B, F>(board: &B, player: Color, leaf_eval: &F, alpha: i32, beta: i32, depth: u32) -> (Option<Move>, i32)
where
	B: Board,
	F: Fn(&B, Color) -> i32,
{
	// must stop iteration so that we would not trade a king for a king :-)
	if !board.has_king(player) {
		return (None, -INF);
	}
	if !board.has_king(player.opposite()) {
		return (None, INF);
	}

	if depth == 0 {
		return (None, leaf_eval(board, player));
	}

	let mut best_value = -INF;
	let mut best_move = None;
	for mv in board.all_moves(player) {
		// TODO: sort
		let board = board.with_move(mv);

		// TODO: filter out bad moves at board level.
		if board.is_check(player) {
			continue;
		}

		let (_, value) = alphabeta_(&board, player.opposite(), leaf_eval, -beta, -alpha, depth - 1);
		if -value >= best_value {
			best_value = -value;
			best_move = Some(mv);
		}

		let alpha = i32::max(alpha, value);
		if alpha >= beta {
			break;
		}
	}
	(best_move, best_value)
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_alphabeta_mate() {
		let board = board(
			r"
		. . . . R . . k
		. . . . R . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . K
		",
		);

		debug_assert_eq!(alphabeta(&board, White, &material, 0), 10);
		debug_assert_eq!(alphabeta(&board, Black, &material, 0), -10);

		debug_assert_eq!(alphabeta(&board, White, &material, 1), INF);
		debug_assert_eq!(alphabeta(&board, Black, &material, 2), -INF);
	}

	#[test]
	fn test_alpahbeta_1() {
		let board = board(
			r"
		. . . . . . . .
		. . . . . p . .
		. . . . p . . .
		. . . . . . . .
		. . . . . . . .
		. . . . Q . . .
		. . . . . . . .
		k . . . . . . K
		",
		);

		debug_assert_eq!(alphabeta(&board, White, &material, 0), 7);
		debug_assert_eq!(alphabeta(&board, Black, &material, 0), -7);

		debug_assert_eq!(alphabeta(&board, White, &material, 1), 8); // white greedily takes a pawn
		debug_assert_eq!(alphabeta(&board, Black, &material, 1), -7);

		debug_assert_eq!(alphabeta(&board, White, &material, 2), 7); // white sees that the pawn is protected
	}

	#[test]
	fn test_alphabeta_2() {
		let board = board(
			r"
		Q . . p p p . k
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . K
		",
		);

		debug_assert_eq!(alphabeta(&board, White, &material, 0), 6); // no moves
		debug_assert_eq!(alphabeta(&board, Black, &material, 0), -6); // no moves

		debug_assert_eq!(alphabeta(&board, White, &material, 1), 7); // white: Qd8xp
		debug_assert_eq!(alphabeta(&board, Black, &material, 1), -6); // black: no moves

		debug_assert_eq!(alphabeta(&board, White, &material, 2), 7); // white: Qd8xp,    black: no moves
		debug_assert_eq!(alphabeta(&board, Black, &material, 2), -7); // black: no moves, white: Qd8xp

		debug_assert_eq!(alphabeta(&board, White, &material, 3), 8); // white: Qd8xp,   black: no moves, white: Qe8xp
		debug_assert_eq!(alphabeta(&board, Black, &material, 3), -7); // black: no moves, white: Qd8xp,  black: no moves

		debug_assert_eq!(alphabeta(&board, White, &material, 4), 8); // white: Qd8xp,   black: no moves, white: Qe8xp,    black: no moves
		debug_assert_eq!(alphabeta(&board, Black, &material, 4), -8); // black: no moves, white: Qd8xp,  black: no moves, white: Qe8xp
	}

	#[test]
	fn test_alphabeta_3() {
		let board = board(
			r"
		. . . . . . . k
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		q . . P P P . K
		",
		);

		debug_assert_eq!(alphabeta(&board, Black, &material, 0), 6);
		debug_assert_eq!(alphabeta(&board, White, &material, 0), -6);

		debug_assert_eq!(alphabeta(&board, Black, &material, 1), 7);
		debug_assert_eq!(alphabeta(&board, White, &material, 1), -6);

		debug_assert_eq!(alphabeta(&board, Black, &material, 2), 7);
		debug_assert_eq!(alphabeta(&board, White, &material, 2), -7);

		debug_assert_eq!(alphabeta(&board, Black, &material, 3), 8);
		debug_assert_eq!(alphabeta(&board, White, &material, 3), -7);

		debug_assert_eq!(alphabeta(&board, Black, &material, 4), 8);
		debug_assert_eq!(alphabeta(&board, White, &material, 4), -8);
	}

	fn board(board: &str) -> BitBoard {
		BitBoard::from_str(board).unwrap()
	}
}
