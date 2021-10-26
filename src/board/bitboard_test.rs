use super::internal::*;
use rand::prelude::*;
use rand::SeedableRng;
use Square::*;

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
