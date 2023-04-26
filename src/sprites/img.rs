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

#[derive(Debug, Clone)]
pub struct Img(Grid<SpriteCell>);

impl Img {
    pub fn horiz(len: i16, px: SpriteCell) -> Self {
        Self (Grid::new(1, len.finto(), px))
    }

    pub fn vert(len: i16, px: SpriteCell) -> Self {
        Self (Grid::new(len.finto(), 1, px))
    }

    pub fn square(side_len: i16, px: SpriteCell) -> Self {
        Self (Grid::new(side_len.finto(), side_len.finto(), px))
    }

    pub fn rect(height: i16, width: i16, px: SpriteCell) -> Self {
        Self (Grid::new(height.finto(), width.finto(), px))
    }

    pub fn get(&self, pos: TermPos) -> Result<SpriteCell> {
        self.0.get(pos.finto())
    }

    pub fn set(&mut self, pos: TermPos, cel: SpriteCell) -> Result<SpriteCell> {
        self.0.set(pos.finto(), cel)
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
