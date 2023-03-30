pub use SpriteCell::*;

use crate::term_char::TermChar;
use crate::util::*;

mod grid;
mod sprite_anchor_tree;
mod sprite_order_tree;
mod sprite_onto_tree;
mod sprite_tree;
mod sprite_traits;

pub mod img;
pub mod pre_sprite;
pub mod sprite;
pub mod sprite_manager;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpriteCell {
    Opaque(TermChar),
    #[default]
    Transparent,
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
