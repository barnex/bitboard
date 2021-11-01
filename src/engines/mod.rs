mod internal;

mod engine;
pub use engine::*;

mod e1_valid;
pub use e1_valid::*;

mod e2_greedy;
pub use e2_greedy::*;

mod e3_negamax;
pub use e3_negamax::*;

mod e4_alphabeta;
pub use e4_alphabeta::*;

mod e5_par_alphabeta;
pub use e5_par_alphabeta::*;

mod value_functions;
pub use value_functions::*;

mod pick_move;
pub use pick_move::*;

mod parse_engine;
pub use parse_engine::*;

/// Effectively infinity,
/// but don't use MAX_I32 to stay away from wrap-around.
pub const INF: i32 = 1_000_000_000;
