use piston::window::WindowSettings;
use piston::event::{UpdateArgs, RenderArgs};
use super::input::InputManager;
use opengl_graphics::GlGraphics;
use graphics::Context;

pub enum UpdateResult {
    Done,
    Quit,
    ChangeScreen(Box<Screen>),
    ReloadWindow(WindowSettings),
}

pub trait Screen {
    /* Required */
    fn update(&mut self, args: &UpdateArgs, im: &InputManager)
            -> UpdateResult;
    fn draw(&self, args: &RenderArgs, c: Context, gfx: &mut GlGraphics);
    /* Optional */
    fn on_exit(&self) {}
}