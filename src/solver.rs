//! This module contains the different solvers for the Sudoku puzzle
//!
//! Currently implemented:
//!     Backtracing solver (single threaded)

use board::Board;
use board::new_empty;

struct SolState {
    board: Board,
    possi: Vec<uint>, // the Vector of possible valid entries
    previous: box<SolState>
}

/// The back-tracing algorithm functions as follows:
/// Empty spaces will generate a vector of possible entries for
/// that position. The back-tracing algorithm then generates a new
/// solution state, popping the current value off of the list of possible
/// vectors, and continues calculating the open spaces recursively.
///
/// This process has two possible outcomes once it runs out of spaces, either
/// 1) All of the spaces are use and there are no left over possibilities
///    which means that this is the solution.
/// 2) All the spaces are used, yet there are still possibilities in the list.
///    This means that one of the initial guesses was wrong and will return a
///    None value. When this is encountered by the calling function, the next
///    available entry from the possibilities is chosen. If there are no more
///    possibilities, and yet still more spaces then the function will return a
///    None value and the process will continue again until there are no other
///    possibilities left.
fn solve_with_backtracing( in_state: SolState) -> box<Option<SolState>> {
    Some(out_state)
}
