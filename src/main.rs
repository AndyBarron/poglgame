extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use std::iter::Iterator;

mod input;
use input::InputManager;

mod screen;
use screen::{Screen, UpdateResult};

use piston::window::{Window, WindowSettings, AdvancedWindow};
use piston::event::*;
use piston::input::{Key, MouseButton};
use glutin_window::GlutinWindow;
use opengl_graphics::{ GlGraphics, OpenGL };
use graphics::types::Color;

fn main() {
    // launch(TestScreen, 400, 300)
    old_main()
}

fn easy_launch<S: Screen>(start: S, title: &str, w: u32, h: u32) {
    launch( start, WindowSettings::new(title, [w, h]) )
}

fn launch<S: Screen>(start: S, settings: WindowSettings) {
    let gl = OpenGL::_3_2;
    let mut gfx = GlGraphics::new(gl);
    let mut screen: Box<Screen> = Box::new(start);
    let mut cur_set = vec![settings];

    'game: loop {
        let mut window = GlutinWindow::new(gl, cur_set.pop()
                .expect("ERROR: cur_set"));
        let mut im = InputManager::new();
        'events: for e in window.events() {
            im.update(&e);
            let mut result = None;
            if let Some(args) = e.update_args() {
                result = Some(screen.update(&args, &im));
            }
            if let Some(args) = e.render_args() {
                gfx.draw(args.viewport(), |c, gfx| {
                    screen.draw(&args, c, gfx);
                });
            }
            im.end_frame();
            if result.is_none() { continue; }
            match result.unwrap() {
                UpdateResult::Done => {}
                UpdateResult::ChangeScreen(boxed) => {
                    screen = boxed;
                }
                UpdateResult::Quit => {
                    break 'game;
                }
                UpdateResult::ReloadWindow(new_set) => {
                    cur_set.push(new_set);
                    break 'events;
                }
            }
        }
    }

    screen.on_exit();
}

// pub struct TestScreen;

// impl Screen for TestScreen {
//     fn update(&mut self, args: &UpdateArgs, im: &InputManager)
//             -> UpdateResult
//     {
//         UpdateResult::Done
//     }
//     fn draw(&self, args: &RenderArgs, gl: &mut GlGraphics) {

//     }
// }

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

fn old_main() {
    let opengl = OpenGL::_3_2;

    // Create an Glutin window.
    let window = GlutinWindow::new(
        opengl,
        WindowSettings::new(
            "",
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