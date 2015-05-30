extern crate piston;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate poglgame;

use piston::event::*;
use piston::window::WindowSettings;
use piston::input::{Key, MouseButton};
use opengl_graphics::GlGraphics;
use graphics::types::Color;
use graphics::Context;

use poglgame::launch;
use poglgame::screen::*;
use poglgame::input::*;

#[test]
fn launch_test_screen() {
    launch(TestScreen::default(), "Test Screen", 400, 300)
}

const QUIT_AFTER: f64 = 3.0;

const COLORS: [Color; 3] = [
    [0.5, 0.0, 0.0, 1.0],
    [0.0, 0.5, 0.0, 1.0],
    [0.0, 0.0, 0.5, 1.0],
];

pub struct TestScreen {
    color_idx: usize,
    rotation: f64, // radians
    age: f64, // seconds
}

impl Default for TestScreen {
    fn default() -> Self {
        Self::new(0, 0.)
    }
}

impl TestScreen {
    fn new(idx: usize, rot: f64) -> Self {
        TestScreen {
            color_idx: idx,
            rotation: rot,
            age: 0.,
        }
    }
}

impl Screen for TestScreen {
    fn update(&mut self, args: &UpdateArgs, im: &GameInput)
        -> UpdateResult
    {
        if self.age > QUIT_AFTER || im.was_key_pressed(&Key::Escape) { 
            UpdateResult::Quit
        } else if im.was_key_pressed(&Key::R) {
            UpdateResult::ReloadWindow(WindowSettings::new(
                "Resized",
                [300,300]
            ))
        } else if im.was_mouse_pressed(&MouseButton::Left) {
            let i = (self.color_idx + 1) % COLORS.len();
            UpdateResult::ChangeScreen(
                    Box::new(TestScreen::new(i, self.rotation)))
        } else {
            self.age += args.dt;
            self.rotation += 2.0 * args.dt;
            UpdateResult::Done
        }
    }
    fn draw(&self, args: &RenderArgs, c: Context, gl: &mut GlGraphics) {
        use graphics::*;
        clear(COLORS[self.color_idx], gl);
        let square = rectangle::square(0.0, 0.0, 50.0);
        let x = args.width as f64 / 2.;
        let y = args.height as f64/ 2.;
        let col = COLORS[(self.color_idx + 1) % COLORS.len()];
        let transform = c.transform.trans(x, y)
                .rot_rad(self.rotation)
                .trans(-25.0, -25.0);
        rectangle(col, square, transform, gl);
    }
}