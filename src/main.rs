extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use std::iter::Iterator;

mod input;
use input::InputManager;

mod screen;
use screen::Screen;

use piston::window::WindowSettings;
use piston::event::*;
use piston::input::{Key, MouseButton};
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64   // Rotation for the square.
}

fn ax(c: graphics::Context, gl: opengl_graphics::GlGraphics) {

}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c.transform.trans(x, y)
                                       .rot_rad(rotation)
                                       .trans(-25.0, -25.0);

            // ax(c, gl);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}

fn main() {
    let opengl = OpenGL::_3_2;

    // Create an Glutin window.
    let window = Window::new(
        opengl,
        WindowSettings::new(
            "spinning-square",
            [200, 200]
        )
    );

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0
    };

    let mut im = InputManager::new();

    for e in window.events() {
        im.update(&e);
        let changed = im.iter_pressed().count() > 0 ||
                im.iter_released().count() > 0;
        if changed {
            println!("{:?}", im.iter_down().collect::<Vec<_>>());
        }
        if im.scroll_delta() != [0., 0.] {
            println!("{:?}", im.scroll_delta());
        }
        if let Some(u) = e.update_args() {
            app.update(&u);
        }
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
        im.end_frame();
    }
}