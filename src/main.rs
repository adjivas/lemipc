// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/lemipc
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use] extern crate clap;

extern crate screen;

mod option;

extern crate piston;
extern crate opengl_graphics;
extern crate graphics;
#[cfg(feature = "include_sdl2")]
extern crate sdl2_window;
#[cfg(feature = "include_glfw")]
extern crate glfw_window;
#[cfg(feature = "include_glutin")]
extern crate glutin_window;

#[cfg(feature = "include_sdl2")]
use sdl2_window::Sdl2Window as Window;
#[cfg(feature = "include_glfw")]
use glfw_window::GlfwWindow as Window;
#[cfg(feature = "include_glutin")]
use glutin_window::GlutinWindow as Window;

use graphics::Transformed;

use piston::input::*;
use piston::event_loop::*;

/// The `Draw` structure defines the board, the
/// font and the size.

struct Draw <'a> {
    board: screen::board::Board,
    font: &'a std::path::Path,
    width: u32,
    height: u32,
}

impl <'a> Draw <'a> {

    /// The `new` constructor function returns the
    /// value Draw interface.

    pub fn new (
        font: &'a std::path::Path,
        width: u32,
        height: u32,
    ) -> Self {
        Draw {
            board: Default::default(),
            font: font,
            width: width,
            height: height,
        }
    }

    pub fn get_diameter (
        &self,
    ) -> usize {
        let size: usize = self.board.get_size();

        size + size
    }

    /// The `set_coord` updates the window size.

    pub fn set_coord (
        &mut self,
        width: u32,
        height: u32,
    ) {
        self.width = width;
        self.height = height;
    }

    /// The `put_cell_color` draws cell by cell a
    /// colored line.

    fn put_cell_color <G> (
        &self,
        len: usize,
        y: usize,
        color: graphics::types::Color,
        context: &graphics::Context,
        g: &mut G,
    ) where G: graphics::Graphics {
        let y_size:f64 = self.height as f64 / self.get_diameter() as f64;
        let y_size_demi:f64 = y_size / 2.0;
        let x_move:f64 = {self.board.get_size() - len} as f64 * y_size_demi;
        let y_coord:f64 = y as f64 * y_size_demi;
        let margin:f64 = self.board.get_size() as f64 * y_size_demi / 2.0;

        for x in 0..len {
            let x_coord:f64 = x_move + x as f64 * y_size + y_size_demi;

            graphics::Rectangle::new (
                color,
            ).draw (
                [0.0, 0.0, y_size, y_size],
                &context.draw_state,
                context.transform.rot_deg (
                    45.0
                ).trans (
                    y_coord + x_coord + margin + margin,
                    y_coord - x_coord + margin - margin,
                ),
                g
            );
        }
    }

    /// The `put_cell_color` draws line by line
    /// the grid.

    fn put_cell <G> (
        &self,
        len: usize,
        y: usize,
        context: &graphics::Context,
        g: &mut G,
    ) where G: graphics::Graphics {
        if y % 2 == 0 {
            self.put_cell_color (
                len,
                y,
                graphics::color::hex("404557"),
                &context,
                g,
            )
        }
        else {
            self.put_cell_color (
                len,
                y,
                graphics::color::hex("c7d5ce"),
                &context,
                g,
            )
        }
    }

    /// The `put_grid` draws our grid.

    pub fn put_grid <G> (
        &self,
        context: &graphics::Context,
        g: &mut G,
    ) where G: graphics::Graphics {
        let size = self.board.get_size();
        let mut len = (1..size).chain((1..size + 1).rev());
        let mut y = (0..).take(size * 2);

        loop {
            match (len.next(), y.next()) {
                (Some(len), Some(y)) => self.put_cell(len, y, &context, g),
                (_, _) => break ,
            }
        }
    }
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let opts = option::Command::parse (
        &clap::App::from_yaml(yaml).get_matches()
    );
    let mut draw: Draw = Draw::new (
        std::path::Path::new (
            "/home/adjivas/Project/42/ipc/assets/FiraSans-Regular.ttf"
        ),
        opts.width,
        opts.height,
    );
    let opengl = opengl_graphics::OpenGL::V3_2;
    let window: Window = piston::window::WindowSettings::new("lemipc", [
        opts.width,
        opts.height,
    ]).exit_on_esc(true).opengl(opengl).build().expect("window_setting");
    let window = std::rc::Rc::new(std::cell::RefCell::new(window));
    let ref mut gl = opengl_graphics::GlGraphics::new(opengl);

    for event in window.clone().events() {

        if let Some((x, y)) = event.resize(|w, h| (w, h)) {
            draw.set_coord(x, y);
        }
        if let Some(args) = event.render_args() {
            gl.draw(args.viewport(), |context, g| {
                graphics::clear(graphics::color::hex("c0ede6"), g);
                draw.put_grid(&context, g);
 //               draw.put_info(&context, g);
                /*graphics::Text::new_color (
                    [0.0, 0.0, 0.0, 1.0], 32
                ).draw (
                    "Lemipc",
                    &mut opengl_graphics::glyph_cache::GlyphCache::new (
                       draw.font
                    ).expect("glyph_cache fail"),
                    &context.draw_state,
                    context.transform.trans(10.0, 100.0),
                    g
                );*/
            });
        }
        event.update(|_| {});
    }
}
