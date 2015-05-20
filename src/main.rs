#![macro_use]
extern crate glium;
extern crate glutin;

use glium::DisplayBuild;
use glium::Surface;
use glium::Program;

mod board;
mod solver;

fn main() {
    let width:u32 = 1024;
    let height:u32 = 768;
    let display = glutin::WindowBuilder::new()
        .with_dimensions(width, height)
        .with_title(format!("Sudoku"))
        .build_glium()
        .unwrap();

    let (verts, idx) = square::geometry(&display);
}
