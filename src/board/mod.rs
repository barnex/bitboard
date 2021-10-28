mod internal;

mod bitboard;
mod board;
mod color;
mod formatter;
mod mailbox;
mod moves;
mod parser;
mod pos;
mod square;

pub use board::*;
pub use color::*;
pub use formatter::*;
pub use mailbox::*;
pub use moves::*;
pub use square::*;

pub use Color::*;
pub use Square::*;

#[cfg(test)]
mod bitboard_stresstest;
#[cfg(test)]
mod bitboard_test;
#[cfg(test)]
mod mailbox_test;
