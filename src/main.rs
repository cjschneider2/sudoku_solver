#![allow(dead_code, unused_imports)]
extern crate graphics;
extern crate freetype;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate piston;

use std::path::Path;
use piston::window::WindowSettings;
use piston::event::*;
use piston::input::*;
use sdl2_window::Sdl2Window as Window;
//use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, Texture, OpenGL };
use graphics::math::Matrix2d;

mod board;
mod solver;
mod board_serialize;
mod compositor;

pub struct App {
    gl: GlGraphics,
    rotation: f64,
    is_busy: bool,
}

impl App {
    /// Renders the current state:
    /// 'RenderArgs' contains:
    ///     ext_dt: f64,  <- Extrapolated time in seconds (for animations),
    ///     width: u32,   <- Window Width,
    ///     height: u32,  <- Window Height
    fn render(&mut self, args: &RenderArgs, t_face: &mut freetype::Face) {
        // Imports
        use graphics::*;
        // setup
        let rotation = self.rotation;
        let is_busy = self.is_busy;
        // render code
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear screen
            clear([0.85, 0.85, 0.9, 1.0], gl);
            // Busy icon...
            if is_busy {
                let side = 15.0;
                let shape = rectangle::square(0.0, 0.0, side);
                let color = [0.0, 0.0, 0.9, 1.0];
                let y_off  = 20.0;
                let x_off  = (args.width - y_off as u32) as f64;
                let x_form = c.transform.trans(x_off, y_off)
                              .rot_rad(rotation)
                              .trans(-side/2.0, -side/2.0);
                rectangle( color, shape, x_form, gl);
            }
            // The board

            // Render text
            let x_form = c.transform.trans(0.0, 16.0);
            render_text(t_face, gl, x_form, "Sudoku!");
        });
    }

    /// updates the state variables:
    /// 'UpdateArgs' contains:
    ///     dt: f64 <- the delta time (in seconds)
    fn update(&mut self, args: &UpdateArgs) {
        // rotate at 2-rad per sec
        self.rotation += 2.0 * args.dt;
    }

    /// Handels the input
    /// pub enum Input {
    ///     press(button),
    ///     release(button),
    ///     move(motion),
    ///     text(string),
    ///     resize(u32, u32),
    ///     focus(bool),
    fn input(&mut self, args: &Button) {
        use piston::input::Button::Keyboard;
        use piston::input::keyboard::Key;
        match *args {
            Keyboard(Key::Space) => println!("Space Pressed"),
            Keyboard(Key::Up) => println!("Up pressed"),
            Keyboard(Key::Down) => println!("Down pressed"),
            Keyboard(Key::Left) => println!("Left pressed"),
            Keyboard(Key::Right) => println!("Right pressed"),
            _ => println!("Unhandeled input Event: {:?}", args),
        }
    }
}

fn render_text(face: &mut freetype::Face, gl: &mut GlGraphics,
               t: Matrix2d, text: &str) {
    use graphics::*;
    let mut x = 10;
    let mut y = 0;
    for ch in text.chars() {
        face.load_char(ch as usize, freetype::face::RENDER).unwrap();
        let g = face.glyph();
        let bitmap = g.bitmap();
        let texture = Texture::from_memory_alpha(bitmap.buffer(),
                                                 bitmap.width() as u32,
                                                 bitmap.rows() as u32).unwrap();
        Image::new_colored(color::BLACK).draw(
            &texture,
            default_draw_state(),
            t.trans((x + g.bitmap_left()) as f64, (y - g.bitmap_top()) as f64),
            gl
        );
        x += (g.advance().x >> 6) as i32;
        y += (g.advance().y >> 6) as i32;
    }
}

fn main() {
    let opengl = OpenGL::_3_2;
    // Window setup
    let window = Window::new(
        opengl,
        WindowSettings::new("Sudoku!", [512,512])
        .exit_on_esc(true)
    );

    // Font setup
    let font = Path::new("./assets/FiraSans-Regular.ttf");
    let font_library = freetype::Library::init().unwrap();
    let mut type_face = font_library.new_face(&font, 0).unwrap();
    type_face.set_pixel_sizes(0, 16).unwrap();

    // Setup the App state
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        is_busy: true,
    };

    // Main event loop
    for event in window.events() {
        if let Some(input) = event.press_args(){
            app.input(&input);
        }
        if let Some(update) = event.update_args() {
            app.update(&update);
        }
        if let Some(render) = event.render_args() {
            app.render(&render, &mut type_face);
        }
    }
}

