//! This module contains the different solvers for the Sudoku puzzle
//!
//! Currently implemented:
//!     Backtracing solver (single threaded)

use board::Board;

#[derive(Clone, Debug)]
pub struct SolState {
    pub board: Board,
    pub entry: (usize, usize),    // the entry position that was changed
}

/// The backtracking algorithm functions as follows:
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
pub fn solve_with_backtracing(state: Box<Board>) -> bool {
    let mut row = 0;
    let mut col = 0;

    match state.next_empty() {
        Some((x,y)) => { row = x; col = y; }
        None => { return true; } // the board is full
    }

    for num in 1..10 {
        // check for conflicts and insert a new number if okay
        if {
            state.insert(num, row, col);
            // if solved then return true
            if solve_with_backtracing(state) { return true; }
            // else we'll undo our changes
            state.insert(0, row, col);
        }
    }
    return false; // initate backtracking
}

#[cfg(test)]
mod tests {
    use board::new_with_entries;
    #[test]
    fn test_next_state_next_vector() {
        let a_board = new_with_entries(
            [[ 0,5,3, 9,2,7, 1,8,6],
             [ 6,0,1, 4,3,5, 7,9,2],
             [ 7,9,2, 6,8,1, 3,4,5],

             [ 5,1,4, 7,9,3, 6,2,8],
             [ 3,2,6, 5,1,8, 4,7,9],
             [ 8,7,9, 2,6,4, 5,1,3],

             [ 9,3,7, 8,4,6, 2,5,1],
             [ 1,4,8, 3,5,2, 9,6,7],
             [ 2,6,5, 1,7,9, 8,3,4]]);
        let out_state = super::solve_with_backtracing(a_board);
        println!("{:?}", out_state);
        let (valid, solved) = out_state.is_valid_solution();
        assert!(valid && solved);
    }
}
