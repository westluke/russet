pub use SpriteCell::*;

use crate::term_char::TermChar;
use crate::util::*;

use crossterm::style::Color;

mod grid;
pub mod sprite_anchor_tree;
pub mod sprite_order_tree;
pub mod sprite_onto_tree;
pub mod sprite_tree;
pub mod sprite_manager;
mod sprite_traits;
mod dirt;

// mod line_update;
mod termable;

pub mod img;
pub mod pre_sprite;
pub mod sprite;
// pub mod sprite_forest;

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
