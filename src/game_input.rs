use std::collections::HashMap;
use std::collections::hash_map::Iter;
use graphics::math::Vec2d;
use piston::input::*;
use piston::event::*;

// TODO maybe make mouse stuff Option<..> for off-screen?
pub struct GameInput {
    // persistent data
    buttons_down: HashMap<Button, bool>,
    mouse_p: Vec2d,
    focus: bool,
    // data that resets per frame
    pub buttons_pressed: HashMap<Button, bool>,
    buttons_released: HashMap<Button, bool>,
    mouse_d: Vec2d,
    scroll_d: Vec2d,
}

pub struct ButtonIter<'a> {
    it: Iter<'a, Button, bool>,
}

impl<'a> Iterator for ButtonIter<'a> {
    type Item = Button;
    fn next(&mut self) -> Option<Button> {
        loop {
            match self.it.next() {
                Some( (btn, &true) ) => return Some(*btn),
                None => return None,
                _ => continue,
            }
        }
    }
}

impl Default for GameInput {
    fn default() -> Self {
        GameInput {
            buttons_down: Default::default(),
            mouse_p: [0., 0.,],
            focus: false,
            buttons_pressed: Default::default(),
            buttons_released: Default::default(),
            mouse_d: [0., 0.],
            scroll_d: [0., 0.],
        }
    }
}

impl GameInput {
    pub fn new() -> Self { Self::default() }
    pub fn update(&mut self, e: &Event) {
        match *e {
            Event::Input(ref i) => self.handle_input(i),
            _ => {}
        }
    }
    pub fn end_frame(&mut self) {
        // for-loop necessary b/c iter adaptors are lazy ... ugh
        for _ in self.buttons_pressed.iter_mut().map(|(_, v)| *v = false) {}
        for _ in self.buttons_released.iter_mut().map(|(_, v)| *v = false) {}
        self.mouse_d = [0., 0.];
        self.scroll_d = [0., 0.];
    }
    /* poll */
    pub fn is_down(&self, btn: &Button) -> bool {
        match self.buttons_down.get(btn) {
            Some(b) => *b,
            None => false,
        }
    }
    pub fn was_pressed(&self, btn: &Button) -> bool {
        match self.buttons_pressed.get(btn) {
            Some(b) => *b,
            None => false,
        }
    }
    pub fn was_released(&self, btn: &Button) -> bool {
        match self.buttons_released.get(btn) {
            Some(b) => *b,
            None => false,
        }
    }
    pub fn is_key_down(&self, key: &Key) -> bool {
        self.is_down(&Button::Keyboard(*key))
    }
    pub fn was_key_pressed(&self, key: &Key) -> bool {
        self.was_pressed(&Button::Keyboard(*key))
    }
    pub fn was_key_released(&self, key: &Key) -> bool {
        self.was_released(&Button::Keyboard(*key))
    }
    pub fn is_mouse_down(&self, mouse: &MouseButton) -> bool {
        self.is_down(&Button::Mouse(*mouse))
    }
    pub fn was_mouse_pressed(&self, mouse: &MouseButton) -> bool {
        self.was_pressed(&Button::Mouse(*mouse))
    }
    pub fn was_mouse_released(&self, mouse: &MouseButton) -> bool {
        self.was_released(&Button::Mouse(*mouse))
    }
    pub fn is_focused(&self) -> bool {
        self.focus // TODO broken
    }
    pub fn mouse_position(&self) -> Vec2d {
        self.mouse_p
    }
    pub fn mouse_delta(&self) -> Vec2d {
        self.mouse_d
    }
    pub fn scroll_delta(&self) -> Vec2d {
        self.scroll_d
    }
    /* iter */
    pub fn iter_down(&self) -> ButtonIter {
        ButtonIter { it: self.buttons_down.iter() }
    }
    pub fn iter_pressed(&self) -> ButtonIter {
        ButtonIter { it: self.buttons_pressed.iter() }
    }
    pub fn iter_released(&self) -> ButtonIter {
        ButtonIter { it: self.buttons_released.iter() }
    }
    /* private */
    fn handle_input(&mut self, e: &Input) {
        match *e {
            Input::Press(ref btn) => if !self.is_down(btn) {
                self.buttons_down.insert(*btn, true);
                self.buttons_pressed.insert(*btn, true);
            },
            Input::Release(ref btn) => if self.is_down(btn) {
                self.buttons_down.insert(*btn, false);
                self.buttons_released.insert(*btn, true);
            },
            Input::Move(ref motion) => match *motion {
                Motion::MouseCursor(x, y) => {
                    self.mouse_p = [x, y];
                }
                Motion::MouseScroll(x, y) => {
                    self.scroll_d = [x, y];
                }
                Motion::MouseRelative(x, y) => {
                    self.mouse_d = [x, y]; // TODO never called w/ Glutin
                }
                // _ => {}
            },
            Input::Focus(ref b) => {
                self.focus = *b // TODO doesn't work w/ Glutin
            }
            _ => {}
        }
    }
}