use std::collections::HashMap;
use std::collections::hash_map::Iter;
use piston::input::{Input, Button, Motion};
use piston::event::Event;
use graphics::math::{self, Vec2d};
use std::iter::Filter;

// TODO maybe make mouse stuff Option<..> for off-screen?
pub struct InputManager {
    // persistent data
    buttons_down: HashMap<Button, bool>,
    mouse_p: Vec2d,
    focus: bool,
    // data that resets per frame
    buttons_pressed: HashMap<Button, bool>,
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

impl Default for InputManager {
    fn default() -> Self {
        InputManager {
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

impl InputManager {
    pub fn new() -> Self { Self::default() }
    pub fn update(&mut self, e: &Event) {
        match *e {
            Event::Input(ref i) => self.handle_input(i),
            _ => {}
        }
    }
    pub fn end_frame(&mut self) {
        self.buttons_pressed.clear();
        self.buttons_released.clear();
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
    // pub fn is_focused(&self) -> bool {
    //     self.focus // TODO make this work
    // }
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
                    let current = [x, y];
                    self.mouse_d = math::sub(current, self.mouse_p);
                    self.mouse_p = current;
                }
                Motion::MouseScroll(x, y) => {
                    self.scroll_d = [x, y];
                }
                // Motion::MouseRelative(x, y) => {
                //     self.mouse_d = [x, y]; // TODO never called
                // }
                _ => {}
            },
            // Input::Focus(ref b) => {
            //     self.focus = *b // TODO doesn't work
            // }
            _ => {}
        }
    }
}