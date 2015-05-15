#![cfg(test)]

extern crate sudoku_solver;

use sudoku_solver::board;
use sudoku_solver::solver;

#[test]
fn display_empty_board() {
    let a_board = sudoku_solver::board::new_empty();
    a_board.display();
}

#[bench]
fn test_solution () {
    let mut a_board = Box::new(board::new_with_entries(
        [[ 4,5,3, 9,2,7, 1,8,6],
         [ 6,8,0, 4,3,5, 7,9,2],
         [ 7,9,2, 6,8,0, 3,4,5],

         [ 5,1,4, 7,9,3, 6,2,8],
         [ 3,2,6, 5,0,0, 4,0,9],
         [ 8,0,9, 2,6,4, 5,1,3],

         [ 9,3,7, 8,4,6, 2,5,1],
         [ 1,4,8, 3,5,2, 9,0,7],
         [ 0,6,5, 1,7,9, 8,3,4]]));
    let sol_board = board::new_with_entries(
        [[ 4,5,3, 9,2,7, 1,8,6],
         [ 6,8,1, 4,3,5, 7,9,2],
         [ 7,9,2, 6,8,1, 3,4,5],

         [ 5,1,4, 7,9,3, 6,2,8],
         [ 3,2,6, 5,1,8, 4,7,9],
         [ 8,7,9, 2,6,4, 5,1,3],

         [ 9,3,7, 8,4,6, 2,5,1],
         [ 1,4,8, 3,5,2, 9,6,7],
         [ 2,6,5, 1,7,9, 8,3,4]]);
    solver::solve_with_backtracing(&mut a_board);
    assert!(*a_board == sol_board);
}
