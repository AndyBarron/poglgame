#![allow(dead_code)]
extern crate piston;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;

mod input;
use input::InputManager;

mod screen;
use screen::{Screen, UpdateResult};

mod testing;
use testing::TestScreen;

use piston::window::WindowSettings;
use piston::event::*;
use sdl2_window::Sdl2Window as PistonWindow;
use opengl_graphics::{ GlGraphics, OpenGL };

fn main() {
    launch(TestScreen::default(), "Test", 400, 300)
}

fn launch<S: Screen>(start: S, title: &str, w: u32, h: u32) {
    let gl = OpenGL::_3_2;
    let mut cur_set = vec![WindowSettings::new(title, [w, h])];
    let mut screen: Box<Screen> = Box::new(start);
    'game: loop {
        let window = PistonWindow::new(gl, cur_set.pop()
                .expect("ERROR: cur_set"));
        let mut gfx = GlGraphics::new(gl);
        let mut im = InputManager::new();
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
                }
                UpdateResult::Quit => {
                    break 'game;
                }
                // TODO how to restart/resize window?
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