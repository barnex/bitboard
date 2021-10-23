use super::internal::*;

pub fn negamax(board: &Mailbox, depth: u32, c: Color, mv: Move) -> i32 {
	0
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
