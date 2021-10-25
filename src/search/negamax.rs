use super::internal::*;

const INF: i32 = 1_000_000; // effectively infinite value

pub fn negamax<B, F>(board: &B, depth: u32, c: Color, mv: Move, val: &F) -> i32
where
	B: Board,
	F: Fn(&B) -> i32,
{
	debug_assert!(depth != 0);

	//if board[mv.to].mask(KIND_MASK) == KING{
	//	return - INF  * board[mv.to].color().map_or(0, Color::sign)
	//}

	let board = board.with_move(mv);
	if depth == 1 {
		return val(&board);
	}

	let mut value = INF;

	for mv in board.all_moves(c.opposite()) {
		value = i32::min(value, -negamax(&board, depth - 1, c.opposite(), mv, val))
	}

	value
}

/*
func (e *worf) negamax(n *Node, depth int, c Color, m Move) int {

	if dst := n.board.At(m.DstI()); dst == WK || dst == BK {
		return inf(-c * dst.Color())
	}

	if depth == 0 {
		return int(c) * Heuristic3(n, m)
	}

	value := inf(1)

	n2 := n.WithMove(m)
	n = nil
	for _, m := range AllMoves(&n2.board, -c) {
		v := e.negamax(n2, depth-1, -c, m) * -1
		value = min(value, v)
	}
	return value
}

*/
