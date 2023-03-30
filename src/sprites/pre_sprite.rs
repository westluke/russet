use crate::pos::TermPos;
use crate::id::Id;
use crate::bounds::Bounds;
use crate::Result;
use crate::util::*;

use super::SpriteCell;
use super::img::Img;

#[derive(Debug, Clone)]
pub struct PreSprite {
    pub(super) img: Img,
    pub(super) anchor: TermPos,
    pub(super) id: Id,

    // Only in the case of ties do we advance to additional entries
    pub(super) zs: Vec<i16>
}

// There is no reason we should be making up default values when getters/setters fail! We can just
// report them, legitimately, as errors. Defaulting can always be done in the calling code.
impl PreSprite {
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            img: Img::rect(height, width, SpriteCell::default()),
            anchor: (0, 0).finto(),
            id: Id::default(),
            zs: Vec::<i16>::default()
        }
    }

    pub fn mk(img: Img, anchor: TermPos, id: Id, zs: Vec<i16>) -> Self {
        Self { img, anchor, id, zs }
    }

    pub fn bounds(&self) -> Bounds<i16> {
        return (
            self.anchor
            ..(self.anchor + (self.img.height()-1, self.img.width()-1).finto())
        ).finto()
    }

    // pub fn with_size(height: usize, width: usize)
    pub fn get(&self, pos: TermPos) -> Result<SpriteCell> {
        self.img.get(pos)
    }

    pub fn set(&mut self, pos: TermPos, cel: SpriteCell) -> Result<SpriteCell> {
        self.img.set(pos, cel)
    }
}

#[derive(Debug, Clone, Default)]
pub struct PreSpriteBuilder {
    anchor: TermPos,
    id: Id,
    zs: Vec<i16>
}

impl PreSpriteBuilder {
    pub fn anchor(&mut self, anchor: TermPos) -> &mut Self {
        self.anchor = anchor;
        self
    }

    pub fn id(&mut self, id: Id) -> &mut Self {
        self.id = id;
        self
    }

    pub fn zs(&mut self, zs: Vec<i16>) -> &mut Self {
        self.zs = zs;
        self
    }

    // Img doesn't have a sensible default, so we require img at the final step
    pub fn build(&self, img: Img) -> PreSprite {
        PreSprite::mk(img, self.anchor, self.id.clone(), self.zs.clone())
    }
}
