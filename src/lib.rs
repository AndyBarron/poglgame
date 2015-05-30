extern crate piston as libpiston;
extern crate graphics as libgraphics;
extern crate opengl_graphics as libopengl_graphics;
extern crate sdl2_window;

pub use libpiston as piston;
pub use libgraphics as graphics;
pub use libopengl_graphics as opengl_graphics;

pub mod game_input;
pub mod screen;

pub use game_input::{GameInput, ButtonIter};
pub use screen::Screen;

use piston::window::WindowSettings;
use piston::event::{Events, UpdateEvent, RenderEvent, UpdateArgs};
use sdl2_window::Sdl2Window as PistonWindow;
use opengl_graphics::{ GlGraphics, OpenGL };
use screen::UpdateResult;

pub fn launch<S: Screen>(start: S, title: &str, w: u32, h: u32) {
    let gl = OpenGL::_3_2;
    let mut cur_set = vec![WindowSettings::new(title, [w, h])];
    let mut screen: Box<Screen> = Box::new(start);
    'game: loop {
        let window = PistonWindow::new(gl, cur_set.pop()
                .expect("ERROR: cur_set"));
        let mut gfx = GlGraphics::new(gl);
        let mut im = GameInput::new();
        'events: for e in window.events() {
            im.update(&e);
            let mut result = None;

            if let Some(args) = e.update_args() {
                result = Some(screen.update(&args, &im));
                im.end_frame();
            }
            if let Some(args) = e.render_args() {
                gfx.draw(args.viewport(), |c, gfx| {
                    screen.draw(&args, c, gfx);
                });
            }
            if result.is_none() { continue; }
            match result.unwrap() {
                UpdateResult::Done => {}
                UpdateResult::ChangeScreen(boxed) => {
                    screen = boxed;
                    screen.update(&UpdateArgs{dt: 0.}, &im);
                }
                UpdateResult::Quit => {
                    break 'game;
                }
                // TODO how to properly restart/resize window?
                UpdateResult::ReloadWindow(new_set) => {
                    cur_set.push(new_set);
                    continue 'game;
                }
            }
        }
        // if 'events ends, the window is closed
        break 'game;
    }

    screen.on_exit();
}