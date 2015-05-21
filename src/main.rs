#![macro_use]
extern crate glium;
extern crate glutin;

use glium::DisplayBuild;
use glium::Surface;
use glium::Program;

mod board;
mod solver;
mod board_serialize;

use std::io::prelude::*;
use std::path::Path;
use std::fs;
use std::fs::File;
use std::error::Error;

use board::Board;
use board::{new_empty, new_with_entries};

use solver::solve_with_backtracing;


fn main() {
    /*
    let width:u32 = 1024;
    let height:u32 = 768;
    let display = glutin::WindowBuilder::new()
        .with_dimensions(width, height)
        .with_title(format!("Sudoku"))
        .build_glium()
        .unwrap();
    let (verts, idx) = square::geometry(&display);
    */

    let mut all_solved = false;
    // we'll enumerate our sample sudoku tests from the 'tests.txt' file,
    // one sudoku puzzle per line.
    let path = Path::new("./tests/tests.txt"); // starts in the main dir
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(E) => panic!("couldn't open file {}: {}", path.display(),
                        Error::description(&E)),
    };
    let mut string = String::new();
    file.read_to_string(&mut string);
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
