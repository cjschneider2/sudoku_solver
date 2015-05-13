#![cfg(test)]

extern crate sudoku_solver;

use sudoku_solver::board::new_empty;
use sudoku_solver::board::new_with_entries;

#[test]
fn display_empty_board() {
    let a_board = sudoku_solver::board::new_empty();
    a_board.display();
}

#[test]
fn empty_not_complete_solution() {
    let empty_board = sudoku_solver::board::new_empty();
    let ( _ , complete) = empty_board.is_valid_solution();
    assert_eq!( complete, false );
}

#[test]
fn test_a_valid_solution() {
    let a_board = sudoku_solver::board::new_with_entries(
        [[ 4,5,3, 9,2,7, 1,8,6],
         [ 6,8,1, 4,3,5, 7,9,2],
         [ 7,9,2, 6,8,1, 3,4,5],

         [ 5,1,4, 7,9,3, 6,2,8],
         [ 3,2,6, 5,1,8, 4,7,9],
         [ 8,7,9, 2,6,4, 5,1,3],

         [ 9,3,7, 8,4,6, 2,5,1],
         [ 1,4,8, 3,5,2, 9,6,7],
         [ 2,6,5, 1,7,9, 8,3,4]]);
    let (valid, _ ) = a_board.is_valid_solution();
    assert_eq!( valid, true );
}

#[test]
fn test_a_valid_incomplete_solution() {
    let a_board = sudoku_solver::board::new_with_entries(
        [[ 4,5,3, 9,2,7, 1,8,6],
         [ 6,8,0, 4,3,5, 7,9,2],
         [ 7,9,2, 6,8,0, 3,4,5],

         [ 5,1,4, 7,9,3, 6,2,8],
         [ 3,2,6, 5,1,0, 4,0,9],
         [ 8,0,9, 2,6,4, 5,1,3],

         [ 9,3,7, 8,4,6, 2,5,1],
         [ 1,4,8, 3,5,2, 9,0,7],
         [ 0,6,5, 1,7,9, 8,3,4]]);
    let (valid, complete) = a_board.is_valid_solution();
    assert_eq!((valid, complete), (true, false));
}

#[test]
fn test_valid_coordinates() {
    let a_board = sudoku_solver::board::new_with_entries(
        [[ 4,5,3, 9,2,7, 1,8,6],
         [ 6,8,0, 4,3,5, 7,9,2],
         [ 7,9,2, 6,8,0, 3,4,5],

         [ 5,1,4, 7,9,3, 6,2,8],
         [ 3,2,6, 5,0,0, 4,0,9],
         [ 8,0,9, 2,6,4, 5,1,3],

         [ 9,3,7, 8,4,6, 2,5,1],
         [ 1,4,8, 3,5,2, 9,0,7],
         [ 0,6,5, 1,7,9, 8,3,4]]);
    let valid_pos_1 = a_board.get_valid_pos(1,2);
    assert_eq!( valid_pos_1, vec!(1) );
    let valid_pos_2 = a_board.get_valid_pos(4,5);
    assert_eq!( valid_pos_2, vec!(1,8) );
}
