use std::ops::Sub;

use super::internal::*;
use rand::prelude::*;
use rand::SeedableRng;
use Color::*;
use Square::*;

/* COPY-PASTE ZONE
#[test]
fn _() {
	test_moves(
		BitBoard::f,
		r"
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		",
		r"
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		",
	);
}

#[test]
fn _() {
	test_bits(
		BitBoard::f,
		r"
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		",
		r"
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		",
	);
}
*/

#[test]
fn rook_moves() {
	test_bits(
		BitBoard::b_rook_moves,
		r"
		. . . . . . . .
		. p . . r . . P
		. . . . . . . .
		. . . . . . . .
		. P . . R . . .
		. . . . . . . .
		. . . . P . . .
		. . . . . . . .
		",
		r"
		. . . . x . . .
		. p x x r x x x
		. . . . x . . .
		. . . . x . . .
		. P . . x . . .
		. . . . . . . .
		. . . . P . . .
		. . . . . . . .
		",
	);
	test_bits(
		BitBoard::w_rook_moves,
		r"
		. . . . . . . .
		. p . . r . . P
		. . . . . . . .
		. . . . . . . .
		. P . . R . . .
		. . . . . . . .
		. . . . P . . .
		. . . . . . . .
		",
		r"
		. . . . . . . .
		. p . . x . . P
		. . . . x . . .
		. . . . x . . .
		. P x x R x x x
		. . . . x . . .
		. . . . P . . .
		. . . . . . . .
		",
	);
}

#[test]
fn rook_reach() {
	test_bits(
		|b| b.rook_reach(b.bits(BRook)),
		r"
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		r . . . . . . .
		",
		r"
		x . . . . . . .
		x . . . . . . .
		x . . . . . . .
		x . . . . . . .
		x . . . . . . .
		x . . . . . . .
		x . . . . . . .
		r x x x x x x x
		",
	);
	test_bits(
		|b| b.rook_reach(b.bits(WRook)),
		r"
		. . . . . . R .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . p . R . p .
		p . . . . . . .
		. . . . p . . .
		R p . P . P R .
		",
		r"
		x x x x x x R x
		. . . . x . x .
		. . . . x . x .
		. . . . x . x .
		. . x x R x x .
		x . . . x . x .
		x . . . x . x .
		R x . . . x R x
		",
	);
}

#[test]
fn slide_w() {
	test_bits(
		|b| b.slide(b.bits(WRook), sh_w),
		r"
		. . . . . . . R
		. . . . . . . .
		R . . . . . . .
		. . . . . . . .
		. . p . R . p .
		. . . . . . . .
		. . . . . . . .
		R R . P . . R .
		",
		r"
		x x x x x x x R
		. . . . . . . .
		R . . . . . . .
		. . . . . . . .
		. . x x R . . .
		. . . . . . . .
		. . . . . . . .
		x R . x x x R .
		",
	);
}

#[test]
fn slide_e() {
	test_bits(
		|b| b.slide(b.bits(WRook), sh_e),
		r"
		. . . . . . . R
		. . . . . . . .
		R . . . . . . .
		. . . . . . . .
		. . . . R . p .
		. . . . . . . .
		. . . . . . . .
		R . . P . . R .
		",
		r"
		. . . . . . . R
		. . . . . . . .
		R x x x x x x x
		. . . . . . . .
		. . . . R x x .
		. . . . . . . .
		. . . . . . . .
		R x x x . . R x
		",
	);
}

#[test]
fn all_b_pawn_moves() {
	test_moves(
		Black,
		r"
		. . . . . . . .
		. . . p . . . .
		. . . . R . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . p
		. . . . . . R R
		. . . . . . . .
		",
		&["pd7d6", "pd7d5", "pd7e6", "pg3g2"],
	);
}

#[test]
fn b_pawn_move() {
	test_bits(
		BitBoard::b_pawn_move,
		r"
		. . . . . . . .
		. . . p . . . .
		p . . . . . . .
		. R . . . . p .
		. . p . . R . R
		. R . R . . . .
		. . . . . . . .
		. . . . . p . .
		",
		r"
		. . . . . . . .
		. . . p . . . .
		p . . x . . . .
		x x . x . . p .
		. . p . . x x x
		. x x x . . . .
		. . . . . . . .
		. . . . . p . .
		",
	);
}

#[test]
fn b_pawn_capture() {
	test_bits(
		BitBoard::b_pawn_capture,
		r"
		. . . . . . . .
		. . . p . . . .
		p . . . . . . .
		. R . . . . p .
		. . p . . R . R
		. R . R . . . .
		. . . . . . . .
		. . . . . p . .
		",
		r"
		. . . . . . . .
		. . . p . . . .
		p . . . . . . .
		. x . . . . p .
		. . p . . x . x
		. x . x . . . .
		. . . . . . . .
		. . . . . p . .
		",
	);
}

#[test]
fn b_pawn_capture_we() {
	test_bits(
		BitBoard::b_pawn_capture_sw,
		r"
		. . . . . . . .
		. . . p . . . .
		p . . . . . . .
		. R . . . . p .
		. . p . . R . R
		. R . R . . . .
		. . . . . . . .
		. . . . . p . .
		",
		r"
		. . . . . . . .
		. . . p . . . .
		p . . . . . . .
		. R . . . . p .
		. . p . . x . R
		. x . R . . . .
		. . . . . . . .
		. . . . . p . .
		",
	);
}

#[test]
fn b_pawn_capture_se() {
	test_bits(
		BitBoard::b_pawn_capture_se,
		r"
		. . . . . . . .
		. . . p . . . .
		p . . . . . . .
		. R . . . . p .
		. . p . . R . R
		. R . R . . . .
		. . . . . . . .
		. . . . . p . .
		",
		r"
		. . . . . . . .
		. . . p . . . .
		p . . . . . . .
		. x . . . . p .
		. . p . . R . x
		. R . x . . . .
		. . . . . . . .
		. . . . . p . .
		",
	);
}

#[test]
fn b_pawn_push() {
	test_bits(
		BitBoard::b_pawn_push,
		r"
		. . . . . . . .
		p . p . . . p p
		r . . . . . R .
		. . . . . . . .
		. . p p p . . .
		. . . R . . . .
		. . . . . . . .
		p . . . . . . p
		",
		r"
		. . . . . . . .
		p . p . . . p p
		r . x . . . R x
		. . x . . . . x
		. . p p p . . .
		. . x R x . . .
		. . . . . . . .
		p . . . . . . p
		",
	);
}

#[test]
fn w_pawn_move() {
	test_bits(
		BitBoard::w_pawn_move,
		r"
		P . . . . . . P
		. . . . . . . .
		. . . . . . . .
		. r . . r . . .
		. . P P . . . .
		r P . . r P r .
		P . . . . P . P
		. . . . . . . .
		",
		r"
		P . . . . . . P
		. . . . . . . .
		. . . . . . . .
		. x x x x . . .
		. x P P . x . x
		r P . . x P x x
		P . . . . P . P
		. . . . . . . .
		",
	);
}

#[test]
fn w_pawn_capture() {
	test_bits(
		BitBoard::w_pawn_capture,
		r"
		P . . . . . . P
		. . . . . . . .
		. . . . . . . .
		. r . . r . . .
		. . P P . . . .
		r . . . r . r .
		P . . . . P . .
		. . . . . . . P
		",
		r"
		P . . . . . . P
		. . . . . . . .
		. . . . . . . .
		. x . . x . . .
		. . P P . . . .
		r . . . x . x .
		P . . . . P . .
		. . . . . . . P
		",
	);
}

#[test]
fn w_pawn_capture_nw() {
	test_bits(
		BitBoard::w_pawn_capture_nw,
		r"
		P . . . . . . P
		. . . . . . . .
		. . . . . . . .
		. r . . r . . .
		. . P P . . . .
		r . . . r . r .
		P . . . . P . .
		. . . . . . . P
		",
		r"
		P . . . . . . P
		. . . . . . . .
		. . . . . . . .
		. x . . r . . .
		. . P P . . . .
		r . . . x . . .
		P . . . . P . .
		. . . . . . . P
		",
	);
}

#[test]
fn w_pawn_capture_ne() {
	test_bits(
		BitBoard::w_pawn_capture_ne,
		r"
		P . . . . . . P
		. . . . . . . .
		. . . . . . . .
		. . . . r . . .
		. . P P . . . .
		r . . . r . r .
		P . . . . P . .
		. . . . . . . P
		",
		r"
		P . . . . . . P
		. . . . . . . .
		. . . . . . . .
		. . . . x . . .
		. . P P . . . .
		r . . . r . x .
		P . . . . P . .
		. . . . . . . P
		",
	);
}

#[test]
fn w_pawn_push() {
	test_bits(
		BitBoard::w_pawn_push,
		r"
		. . . . . . P .
		. . . . P . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . r . .
		. R . P . . . .
		P P . . . P . P
		. . . . . . . .
		",
		r"
		. . . . x . P .
		. . . . P . . .
		. . . . . . . .
		. . . . . . . .
		x . . x . R . x
		x R . P . x . x
		P P . . . P . P
		. . . . . . . .
		",
	);
}

#[test]
fn white_black() {
	test_bits(
		BitBoard::white,
		r"
		r n b k q . . .
		p . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		P . . . . . . .
		R N B K Q . . .
		",
		r"
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		x . . . . . . .
		x x x x x . . .
		",
	);
	test_bits(
		BitBoard::black,
		r"
		r n b k q . . .
		p . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		P . . . . . . .
		R N B K Q . . .
		",
		r"
		x x x x x . . .
		x . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		",
	);
}

fn test_moves(player: Color, board: &str, want: &[&str]) {
	let board = BitBoard::from_str(board).unwrap();
	let have = board.all_moves(player).iter().copied().collect::<Set<_>>();
	let want = want
		.iter()
		.map(|s| Move::from_str(s).expect("move: syntax error"))
		.collect::<Set<_>>();
	if have != want {
		let have = have.iter().map(|mv| mv.to).collect();
		let want = want.iter().map(|mv| mv.to).collect();
		println!("have:");
		print_ansi(&board, &have);
		println!("want:");
		print_ansi(&board, &want);
		println!("diff: +{:?}, -{:?}", have.sub(&want), want.sub(&have));
		assert_eq!(have, want);
	}
}

fn test_bits<F: Fn(&BitBoard) -> u64>(f: F, board: &str, want: &str) {
	let board = BitBoard::from_str(board).unwrap();
	let have = as_set(f(&board));
	let want = parse_positions(want);
	if have != want {
		println!("have:");
		print_ansi(&board, &have);
		println!("want:");
		print_ansi(&board, &want);
		println!("diff: +{:?}, -{:?}", have.sub(&want), want.sub(&have));
		assert_eq!(have, want);
	}
}

#[test]
fn all_moves() {
	// use Color::*;

	// for board in &random_boards(1000) {
	// 	let mut bb = BitBoard::new();
	// 	for (pos, sq) in board.iter() {
	// 		bb.set(pos, sq);
	// 	}

	// 	for player in [White, Black] {
	// 		let have = bb.all_moves(player);
	// 		let want = board.all_moves(player);

	// 		if have != want {
	// 			print_ansi(board, &Set::default());
	// 			assert_eq!(have, want);
	// 		}
	// 	}
	// }
}

#[test]
fn set_get() {
	for board in &random_boards(100) {
		let mut bb = BitBoard::new();
		for (pos, sq) in board.iter() {
			bb.set(pos, sq);
		}

		let mut mb = Mailbox::new();
		for r in 0..8 {
			for c in 0..8 {
				let pos = pos(r, c);
				mb.set(pos, bb.at(pos))
			}
		}

		if &mb != board {
			println!("have:");
			print_ansi(&mb, &Set::default());
			println!("want:");
			print_ansi(board, &Set::default());
			panic!("test failed");
		}
	}
}

fn as_set(bits: u64) -> Set<Pos> {
	let mut set = Set::default();
	for r in 0..8 {
		for c in 0..8 {
			let pos = pos(r, c);
			if bit_at(bits, pos) {
				set.insert(pos);
			}
		}
	}
	set
}

fn random_boards(n: usize) -> Vec<Mailbox> {
	let seed = 12345;
	let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
	let mut boards = Vec::with_capacity(n);

	for _ in 0..n {
		let mut board = Mailbox::starting_position();

		// remove some random pieces (not kings)
		for _ in 0..rng.gen_range(10..100) {
			let r = rng.gen_range(0..8) as u8;
			let c = rng.gen_range(0..8) as u8;
			let pos = pos(r, c);
			if !board.at(pos).is_king() {
				board.set(pos, Square::Empty)
			}
		}

		for mv in 0..rng.gen_range(0..100) {
			let player = if mv % 2 == 0 { Color::White } else { Color::Black };
			let moves = board.all_moves(player);
			if moves.len() != 0 {
				let mv = moves[rng.gen_range(0..moves.len())];
				if !board.at(mv.to).is_king() {
					board.do_move(mv)
				}
			}
		}

		boards.push(board);
	}

	boards
}
