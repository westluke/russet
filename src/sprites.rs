use std::cell::{RefCell, Ref, RefMut};
use std::rc::Rc;

pub use SpriteCell::*;

use crate::term_char::TermChar;
use crate::util::config::*;

use crossterm::style::Color;

mod termable;
mod dirt;
mod grid;

pub mod img;
pub mod sprite;

pub mod sprite_tree;
pub mod sprite_manager;

// Basically more descriptive names for Some/None
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpriteCell {
    Opaque(TermChar),
    Transparent,
}

impl Default for SpriteCell {
    fn default() -> Self {
        Opaque(TermChar::Bg(Color::Reset))
    }
}

pub type Stn = Rc<RefCell<sprite::Sprite>>;
pub fn new_stn(sp: sprite::Sprite) -> Stn {
    Rc::new(RefCell::new(sp))
}
pub fn borrow_stn(sp: &Stn) -> Ref<sprite::Sprite> {
    sp.borrow()
}
pub fn borrow_stn_mut(sp: &mut Stn) -> RefMut<sprite::Sprite> {
    sp.borrow_mut()
}

impl SpriteCell {
    pub fn is_opaque(&self) -> bool {
        return !(*self == Transparent)
    }

    pub fn is_transparent(&self) -> bool {
        return *self == Transparent
    }

    pub fn bg() -> Self {
        Self::Opaque(TermChar::new(' ', TERM_BG, TERM_BG))
    }
}
