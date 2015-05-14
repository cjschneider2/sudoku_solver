#![allow(dead_code)]

mod board;
mod solver;

fn main() {
    /*
    let a_board = board::new_with_entries(
        [[ 0,5,3, 9,2,7, 1,8,6],
         [ 6,0,1, 4,3,5, 7,9,2],
         [ 7,9,2, 6,8,1, 3,4,5],

         [ 5,1,4, 7,9,3, 6,2,8],
         [ 3,2,6, 5,1,8, 4,7,9],
         [ 8,7,9, 2,6,4, 5,1,3],

         [ 9,3,7, 8,4,6, 2,5,1],
         [ 1,4,8, 3,5,2, 9,6,7],
         [ 2,6,5, 1,7,9, 8,3,4]]);
    */
    let a_board = board::new_empty();
    let init_state = Box::new(solver::SolState {
        board: a_board,
        possi: vec!(),
        entry: (0,0),
    });
    let game_board = solver::solve_with_backtracing(init_state).unwrap();
    println!("{:?}", game_board);
}
