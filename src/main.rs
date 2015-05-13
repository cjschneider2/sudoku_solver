#![allow(dead_code)]

mod board;

fn main() {
    let game_board = board::new_empty();
    game_board.display();
}
