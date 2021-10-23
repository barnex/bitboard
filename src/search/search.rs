use super::internal::*;
use Color::*;

pub fn search(board: &Mailbox, player: Color, depth: u32) -> Move {
	let sign = match player {
		White => 1,
		Black => -1,
	};
	board
		.all_moves(player.mask())
		.iter()
		.map(|&mv| (sign * negamax(board, depth, player, mv), mv))
		.max()
		.expect(">= 1 valid move")
		.1
}
