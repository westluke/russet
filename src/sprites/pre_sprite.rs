use crate::pos::TermPos;
use crate::id::Id;
use crate::bounds::Bounds;
use crate::Result;
use crate::util::*;

use super::SpriteCell;
use super::img::Img;

#[derive(Debug)]
pub struct PreSprite {
    img: Img,
    anchor: TermPos,
    id: Id<Self>,
    z: i16,

    visible: bool,
    clickable: bool
}

impl Clone for PreSprite {
    fn clone(&self) -> Self {
        Self {
            img: self.img.clone(),
            anchor: self.anchor,
            id: Id::default(),
            z: self.z,
            visible: true,
            clickable: true
        }
    }
}

impl From<Img> for PreSprite {
    fn from(img: Img) -> Self {
        Self {
            img,
            anchor: Default::default(),
            id: Default::default(),
            z: Default::default(),
            visible: true,
            clickable: true
        }
    }
}

// There is no reason we should be making up default values when getters/setters fail! We can just
// report them, legitimately, as errors. Defaulting can always be done in the calling code.
impl PreSprite {
    pub fn new(height: i16, width: i16) -> Self {
        Self {
            img: Img::rect(height, width, SpriteCell::default()),
            anchor: (0, 0).finto(),
            id: Id::default(),
            z: Default::default(),
            visible: true,
            clickable: true
        }
    }

    pub fn mk(img: Img, anchor: TermPos, z: i16, visible: bool, clickable: bool) -> Self {
        Self { img, anchor, id: Id::default(), z, visible, clickable}
    }

    pub fn bounds(&self) -> Bounds<i16> {
        Bounds::mk(
            self.anchor.finto(),
            (self.anchor + (self.img.height()-1, self.img.width()-1).finto()).finto()
        )
    }

    // pub fn with_size(height: usize, width: usize)
    pub fn get_rel(&self, pos: TermPos) -> Result<SpriteCell> {
        self.img.get(pos)
    }

    pub fn set_rel(&mut self, pos: TermPos, cel: SpriteCell) -> Result<SpriteCell> {
        self.img.set(pos, cel)
    }

    pub fn get(&self, pos: TermPos) -> Result<SpriteCell> {
        self.get_rel(pos - self.anchor)
    }

    pub fn set(&mut self, pos: TermPos, cel: SpriteCell) -> Result<SpriteCell> {
        self.set_rel(pos - self.anchor, cel)
    }

    pub fn reanchor(&mut self, pos: TermPos) {
        self.anchor = pos
    }

    pub fn reorder(&mut self, z: i16) {
        self.z = z
    }

    pub fn id(&self) -> Id<Self> {
        self.id.clone()
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn clickable(&self) -> bool {
        self.clickable
    }

    pub fn set_visible(&mut self, v: bool) {
        self.visible = v
    }

    pub fn set_clickable(&mut self, c: bool) {
        self.clickable = c
    }
}
