use super::grid::*;
use super::SpriteCell;
use crate::pos::TermPos;
use crate::Result;
use crate::util::FInto as _;

// Img is just a Grid, but specialized to use SpriteCell, and i16 rather than usize
// Should it specialize to i16? If out of range, do I return error or abort?
// Well yes, because it should specialize to TermPos.
// Then, error or panic? Panic, I think. I shouldn't ever be trying out-of-range stuff
// on this.
//
// Become electrician?
//
// Ugh no, cuz i16 can make sense in a TermPos, it can't make sense here.

#[derive(Debug, Clone, Default)]
pub struct Img(Grid<SpriteCell>);

impl Img {
    pub fn horiz(len: usize, px: SpriteCell) -> Self {
        Self (Grid::new(1, len, px))
    }

    pub fn vert(len: usize, px: SpriteCell) -> Self {
        Self (Grid::new(len, 1, px))
    }

    pub fn square(side_len: usize, px: SpriteCell) -> Self {
        Self (Grid::new(side_len, side_len, px))
    }

    pub fn rect(height: usize, width: usize, px: SpriteCell) -> Self {
        Self (Grid::new(height, width, px))
    }

    pub fn get(&self, pos: (usize, usize)) -> Result<SpriteCell> {
        self.0.get(pos)
    }

    pub fn set(&mut self, pos: (usize, usize), cel: SpriteCell) -> Result<SpriteCell> {
        self.0.set(pos, cel)
    }

    // pub fn set_string(&mut self, 

    pub fn enumerate(&self) -> GridEnumerator<SpriteCell> {
        self.0.enumerate()
    }

    pub fn width(&self) -> usize {
        self.0.width()
    }

    pub fn height(&self) -> usize {
        self.0.height()
    }

    pub fn top_left(&self) -> TermPos {
        (0, 0).finto()
    }

    pub fn top_right(&self) -> TermPos {
        (0, self.0.width()-1).finto()
    }

    pub fn bottom_left(&self) -> TermPos {
        (self.0.height()-1, 0).finto()
    }

    pub fn bottom_right(&self) -> TermPos {
        (self.0.height()-1, self.0.width()-1).finto()
    }
}
