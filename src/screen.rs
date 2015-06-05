use piston::window::WindowSettings;
use piston::event::{UpdateArgs, RenderArgs};
use graphics::Context;
use opengl_graphics::GlGraphics;
use super::game_input::GameInput;

pub enum UpdateResult {
    Done,
    Quit,
    ChangeScreen(Box<Screen>),
    ReloadWindow(WindowSettings), // TODO do this better
}

pub trait Screen {
    /* Required */
    fn update(&mut self, args: &UpdateArgs, im: &GameInput)
            -> UpdateResult;
    fn draw(&mut self, args: &RenderArgs, c: Context, gfx: &mut GlGraphics);
    /* Optional */
    fn on_exit(&self) {}
}