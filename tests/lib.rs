#![cfg(test)]

extern crate sudoku_solver;

use sudoku_solver::board::new_empty;

#[test]
fn display_empty_board() {
    let a_board = sudoku_solver::board::new_empty();
    a_board.display();
}
