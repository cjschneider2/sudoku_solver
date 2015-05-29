//! This module contains the structs and fn's to setup and run a genetic algorithm
//! search on the sudoku board
//!

use board::Board;

// The storage vector
struct GaState {
    state: Vec<u32>,
}

// The fitness function
impl GaState {
    fn fitness(&self) -> u32 {
        0
    }
}
