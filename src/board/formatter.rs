use super::internal::*;

pub fn format_board<I, T>(el: I) -> String
where
	I: Iterator<Item = T>,
	T: fmt::Display,
{
	let collect: Vec<_> = el.collect();

	let mut str = String::from("\n");
	for r in (0..8).rev() {
		str += &format!("{}", r + 1);
		for c in 0..8 {
			str.push(' ');
			str += &(collect[pos(r, c).index()]).to_string();
		}
		str.push('\n')
	}
	str + "  a b c d e f g h"
}

pub fn print_ansi(board: &impl Board, mark: &Set<Pos>) {
	let is_light = |p: Pos| (p.row() + p.col()) % 2 == 0;
	let color_of = |p: Pos| match (is_light(p), mark.contains(&p)) {
		(false, false) => DARK,
		(false, true) => MARK_DARK,
		(true, false) => LIGHT,
		(true, true) => MARK_LIGHT,
	};

	println!("{}{}{}{}", RESET, FG_DARK, "  a b c d e f g h", RESET);
	for r in (0..8).rev() {
		// row number
		print!("{}{}{}{}", RESET, FG_DARK, r + 1, RESET);

		for c in 0..8 {
			let pos = pos(r, c);
			let piece = board.at(pos).unicode();

			print!("{}", RESET);
			if pos.col() == 0 {
				print!(
					"{}{}{}{}{}{}{}{}",
					FG,
					color_of(pos),
					HALF_R,
					BG,
					color_of(pos),
					FG,
					BLACK,
					piece
				);
			} else {
				print!(
					"{}{}{}{}{}{}{}{}{}{}",
					BG,
					color_of(pos + delta(0, -1)),
					FG,
					color_of(pos),
					HALF_R,
					BG,
					color_of(pos),
					FG,
					BLACK,
					piece
				);
			}
		}

		// print final transition, end of row
		print!("{}{}{}{}", RESET, FG, color_of(pos(r, 7)), HALF_L);
		// row number, again
		println!("{}{}{}{}", RESET, FG_DARK, r + 1, RESET);
	}
	// print column numbers
	println!("{}{}{}{}", RESET, FG_DARK, "  a b c d e f g h", RESET);
}

const RESET: &str = "\x1b[39;49m";
const HALF_L: &str = "\u{258C}";
const HALF_R: &str = "\u{2590}";
const FG_DARK: &str = "\x1b[38;5;242m";
const BG: &str = "\x1b[48;5;";
const FG: &str = "\x1b[38;5;";
const DARK: &str = "252m";
const LIGHT: &str = "231m";
const BLACK: &str = "232m";
const MARK_DARK: &str = "49m";
const MARK_LIGHT: &str = "193m";
//const BG_LIGHT: &str = "\x1b[48;5;255m";
//const BG_DARK: &str = "\x1b[48;5;250m";
//const FG_LIGHT: &str = "\x1b[38;5;255m";
//const FG_BLACK: &str = "\x1b[38;5;232m";
