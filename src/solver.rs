//! This module contains the different solvers for the Sudoku puzzle
//!
//! Currently implemented:
//!     Backtracing solver (single threaded)

use board::Board;

#[derive(Clone, Debug)]
pub struct SolState {
    pub board: Board,
    pub possi: Vec<usize>,       // the Vector of possible valid entries
    pub entry: (usize, usize),    // the entry position that was changed
}

impl SolState {
    pub fn next_state(&self) -> Option<SolState> {
        let mut ret_state = self.clone();
        // Find the next empty X and Y coordinates
        let (mut e_x, mut e_y, mut empty) = (0,0,false);
        match ret_state.board.next_empty() {
            Some((x,y)) => { e_x = x; e_y = y; },
            None => {empty = true;},
        };
        println!("empty at {},{}", e_x, e_y);
        // more possibilities
        if ret_state.possi.len() > 0 {
            if !empty {
                println!("vectors, has space");
                let (x, y) = ret_state.entry;
                ret_state.board.insert(ret_state.possi.pop().unwrap(), x, y);
                return Some(ret_state);
            } else {
                println!("vectors, no space");
                return None;
            }
        } else {
            if !empty {
                println!("no vectors, space");
                ret_state.possi = ret_state.board.get_valid_pos(e_x,e_y);
                ret_state.entry = (e_x,e_y);
                return Some(ret_state);
            } else {
                println!("no vectors, no space");
                return None;
            }
        }
    }
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
pub fn solve_with_backtracing(in_state: Box<SolState>) -> Option<Box<SolState>> {
    // Create/initialize a new solution state
    let new_state = Box::new(in_state.next_state());
    println!("{:?}",new_state);
    match *new_state {
        Some(state) => {
            let (_, solved) = state.board.is_valid_solution();
            if solved {
                return Some(Box::new(state));
            } else {
                return solve_with_backtracing(Box::new(state));
            }
        },
        None => { return None; },
    }
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
        let init_state = Box::new(super::SolState {
            board: a_board,
            possi: vec!(),
            entry: (1,1),
        });
        let out_state = super::solve_with_backtracing(init_state);
        println!("{:?}", out_state);
        let (valid, solved) = out_state.unwrap().board.is_valid_solution();
        assert!(valid && solved);
    }
}
