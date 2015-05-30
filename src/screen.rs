use piston::window::{Window, WindowSettings};
use piston::event::{Event, UpdateArgs, RenderArgs};
use super::input::InputManager;
use opengl_graphics::GlGraphics;
use graphics::{ Context, Graphics };

pub enum UpdateResult {
    Done,
    Quit,
    ChangeScreen(Box<Screen>),
    ReloadWindow(WindowSettings),
}

// TODO how to manage screen resizing, video modes?
pub trait Screen {
    /* Required */
    fn update(&mut self, args: &UpdateArgs, im: &InputManager)
            -> UpdateResult;
    fn draw(&self, args: &RenderArgs, c: Context, gfx: &mut GlGraphics);
    /* Optional */
    fn on_exit(&self) {}
}