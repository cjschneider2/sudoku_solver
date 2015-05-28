//! This module contains the different solvers for the Sudoku puzzle
//!
//! Currently implemented:
//!     Backtracing solver (single threaded)

use board::Board;

/// The recursive backtracking algorithm functions as follows:
/// Each time it's run it'll look for an empty space and then try to fill this
/// space with a valid entry, and then continue. If we run into a problem, this
/// entry will be set back to zero and the process will continue again; until it
/// completes or fails.
pub fn solve_with_backtracing(state:&mut Box<Board>) -> bool {
    let mut _row = 0; // the underscore silences a silly warning about the
    let mut _col = 0; // variables being (possibly) unused.

    match state.next_empty() {
        Some((x,y)) => { _row = x; _col = y; }
        None => { return true; } // the board is full
    }

    /*
     * To be honest I'm not sure the method of creating a vector with
     * valid empty positions will work.
     * We create a vector, and then iterator over those values
     *
    let valid_pos = state.get_valid_pos(_row, _col);
    if valid_pos.is_empty() { return false; }
    */

    for num in 1..10 {
        // check for conflicts and insert a new number if okay
        let (valid, _ ) = state.is_valid_solution();
        if valid {
            state.insert(num, _row, _col);
            // if solved then return true
            if solve_with_backtracing(state) { return true; }
            // else we'll undo our changes
            state.insert(0, _row, _col);
        }
    }
    false // initate backtracking
}

#[cfg(test)]
mod tests {
    use std::io::prelude::*;
    use std::path::Path;
    use std::fs;
    use std::fs::File;
    use std::error::Error;
    use board_serialize;
    use board::Board;
    use board::{new_empty, new_with_entries};
    use solver::solve_with_backtracing;

    #[test]
    fn test_next_state_next_vector() {
        let mut a_board = Box::new(new_with_entries(
            [[ 0,5,3, 9,2,7, 1,8,6],
             [ 6,0,1, 4,3,5, 7,9,2],
             [ 7,9,2, 6,8,1, 3,4,5],

             [ 5,1,4, 7,9,3, 6,2,8],
             [ 3,2,6, 5,1,8, 4,7,9],
             [ 8,7,9, 2,6,4, 5,1,3],

             [ 9,3,7, 8,4,6, 2,5,1],
             [ 1,4,8, 3,5,2, 9,6,7],
             [ 2,6,5, 1,7,9, 8,3,4]]));
        let out_state = super::solve_with_backtracing(&mut a_board);
        println!("{:?}", a_board);
        assert!(out_state);
    }
    //#[test]
    fn solve_all() {
        let mut all_solved = false;
        // we'll enumerate our sample sudoku tests from the 'tests.txt' file,
        // one sudoku puzzle per line.
        let path = Path::new("./tests/tests.txt"); // starts in the main dir
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(error) => panic!("couldn't open file {}: {}", path.display(),
                            Error::description(&error)),
        };
        let mut string = String::new();
        let _ = file.read_to_string(&mut string);
        let entries: Vec<&str> = string.split_terminator("\r\n").collect();

        let mut solved_cnt = 0;
        let mut cnt = 0;
        for ent in entries{
            let mut input_board = Box::new(board_serialize::deserialize(&ent));
            println!("Solving board:\n{:?}", input_board );
            let solved = solve_with_backtracing(&mut input_board);
            println!("{}", solved);
            if solved { solved_cnt += 1; }
            cnt += 1;
        }
        if cnt == solved_cnt { all_solved = true; }
        assert!(all_solved);
    }
}
