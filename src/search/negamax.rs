
use super::internal::*;

const INF: i32 = 999_999_999; // effectively infinite value

/// How good is board for player?
/// Good scores are always positive, bad scores always negative,
/// regardless of player color.
pub fn negamax<B, F>(board: &B, player: Color, leaf_eval: &F, depth: u32) -> i32
where
	B: Board,
	F: Fn(&B, Color) -> i32,
{
	// must stop iteration so that we would not trade a king for a king :-)
	if !board.has_king(player) {
		return -INF;
	}
	if !board.has_king(player.opposite()) {
		return INF;
	}

	if depth == 0 {
		return leaf_eval(board, player);
	}

	let mut value = -INF;
	for mv in board.collect_moves(player) {
		let board = board.with_move(mv);

		// TODO: filter out bad moves at board level.
		if board.is_check(player) {
			continue;
		}

		value = i32::max(value, -negamax(&board, player.opposite(), leaf_eval, depth - 1));
	}
	value
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_negamax_mate() {
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

		debug_assert_eq!(negamax(&board, White, &material_value, 0), 10);
		debug_assert_eq!(negamax(&board, Black, &material_value, 0), -10);

		debug_assert_eq!(negamax(&board, White, &material_value, 1), INF);
		debug_assert_eq!(negamax(&board, Black, &material_value, 2), -INF);
	}

	#[test]
	fn test_negamax_1() {
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

		debug_assert_eq!(negamax(&board, White, &material_value, 0), 7);
		debug_assert_eq!(negamax(&board, Black, &material_value, 0), -7);

		debug_assert_eq!(negamax(&board, White, &material_value, 1), 8); // white greedily takes a pawn
		debug_assert_eq!(negamax(&board, Black, &material_value, 1), -7);

		debug_assert_eq!(negamax(&board, White, &material_value, 2), 7); // white sees that the pawn is protected
	}

	#[test]
	fn test_negamax_2() {
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

		debug_assert_eq!(negamax(&board, White, &material_value, 0), 6); // no moves
		debug_assert_eq!(negamax(&board, Black, &material_value, 0), -6); // no moves

		debug_assert_eq!(negamax(&board, White, &material_value, 1), 7); // white: Qd8xp
		debug_assert_eq!(negamax(&board, Black, &material_value, 1), -6); // black: no moves

		debug_assert_eq!(negamax(&board, White, &material_value, 2), 7); // white: Qd8xp,    black: no moves
		debug_assert_eq!(negamax(&board, Black, &material_value, 2), -7); // black: no moves, white: Qd8xp

		debug_assert_eq!(negamax(&board, White, &material_value, 3), 8); // white: Qd8xp,   black: no moves, white: Qe8xp
		debug_assert_eq!(negamax(&board, Black, &material_value, 3), -7); // black: no moves, white: Qd8xp,  black: no moves

		debug_assert_eq!(negamax(&board, White, &material_value, 4), 8); // white: Qd8xp,   black: no moves, white: Qe8xp,    black: no moves
		debug_assert_eq!(negamax(&board, Black, &material_value, 4), -8); // black: no moves, white: Qd8xp,  black: no moves, white: Qe8xp
	}

	#[test]
	fn test_negamax_3() {
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

		debug_assert_eq!(negamax(&board, Black, &material_value, 0), 6);
		debug_assert_eq!(negamax(&board, White, &material_value, 0), -6);

		debug_assert_eq!(negamax(&board, Black, &material_value, 1), 7);
		debug_assert_eq!(negamax(&board, White, &material_value, 1), -6);

		debug_assert_eq!(negamax(&board, Black, &material_value, 2), 7);
		debug_assert_eq!(negamax(&board, White, &material_value, 2), -7);

		debug_assert_eq!(negamax(&board, Black, &material_value, 3), 8);
		debug_assert_eq!(negamax(&board, White, &material_value, 3), -7);

		debug_assert_eq!(negamax(&board, Black, &material_value, 4), 8);
		debug_assert_eq!(negamax(&board, White, &material_value, 4), -8);
	}

	fn board(board: &str) -> BitBoard {
		BitBoard::from_str(board).unwrap()
	}
}
