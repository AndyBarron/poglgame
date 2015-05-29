use piston::window::Window;
use piston::event::{Event, UpdateArgs, RenderArgs};
use super::input::InputManager;
use opengl_graphics::GlGraphics;

pub enum UpdateResult {
    Done,
    ChangeScreen(Box<Screen>),
    Quit,
}

// TODO how to manage screen resizing, video modes?
pub trait Screen {
    fn update(&mut self, args: &UpdateArgs, im: &InputManager)
            -> UpdateResult;
    fn draw(&self, args: &RenderArgs, gl: &mut GlGraphics);
    #[allow(unused_variables)]
    fn on_event(&mut self, e: &Event) {}
}