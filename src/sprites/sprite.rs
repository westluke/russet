use std::collections::{HashSet, HashMap};
use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use crate::pos::TermPos;
use crate::util::{*, SetError as SE, SetErrorKind as SEK};
use crate::Id;
use crate::bounds::Bounds;

use super::*;
use super::dirt::Dirt;
use super::img::Img;

use log::info;

#[derive(Debug, Default)]
pub struct Sprite {
    img: Img,
    anchor: TermPos,
    id: Id<Self>,
    order: i16,

    visible: Visibility,
    clickable: Clickability,
    dirt: Option<Dirt>,
}

impl Clone for Sprite {
    fn clone(&self) -> Self {
        Self {
            img: self.img.clone(),
            anchor: self.anchor,
            id: Id::default(),
            order: self.order,
            visible: self.visible,
            clickable: self.clickable,
            dirt: self.dirt.clone()
        }
    }
}

impl From<Img> for Sprite {
    fn from(img: Img) -> Self {
        Self {
            img,
            anchor: Default::default(),
            id: Default::default(),
            order: Default::default(),
            visible: Visible,
            clickable: Clickable,
            dirt: None
        }
    }
}

impl Sprite {
    pub fn new(height: i16, width: i16) -> Self {
        Self {
            img: Img::rect(height.finto(), width.finto(), SpriteCell::default()),
            anchor: (0, 0).finto(),
            id: Id::default(),
            order: Default::default(),
            visible: Visible,
            clickable: Clickable,
            dirt: None
        }
    }

    pub fn mk(img: Img, anchor: TermPos, order: i16, visible: Visibility, clickable: Clickability, dirt: Option<Dirt>) -> Self {
        Self { img, anchor, id: Id::default(), order, visible, clickable, dirt}
    }

    pub fn bounds(&self) -> Bounds<i16> {
        Bounds::mk(
            self.anchor.finto(),
            (self.anchor + (self.img.height(), self.img.width()).finto()).finto()
        )
    }

    // Dirt management

    pub fn set_dirt(&mut self, dirt: Option<Dirt>) {
        self.dirt = dirt
    }

    pub fn dirty_all(&self) {
        // info!("{:?}", self.bounds());
        if let Some(ref dirt) = self.dirt {
            dirt.dirty_all(self.bounds());
        }
    }

    // Pixel accesses

    pub fn get_rel(&self, pos: TermPos) -> Result<SpriteCell> {
        if pos.pos() {
            if self.visible == Visible { self.img.get(pos.finto()) }
            else { Ok(Transparent) }
            // self.img.get(pos.finto())
        } else {
            Err(SE::new(SEK::OutOfBounds, "negative coordinates for sprite pixel access"))
        }
    }

    pub fn set_rel(&mut self, pos: TermPos, cel: SpriteCell) -> Result<SpriteCell> {
        if pos.pos() {
            self.img.set(pos.finto(), cel)
        } else {
            Err(SE::new(SEK::OutOfBounds, "negative coordinates for sprite pixel access"))
        }
    }

    // Ah. bounds checking has to happen here?
    // What sorts of errors should be here? Should subzero errors be different from out of img
    // bounds errors?
    pub fn get(&self, pos: TermPos) -> Result<SpriteCell> {
        self.get_rel(pos - self.anchor)
    }

    pub fn set(&mut self, pos: TermPos, cel: SpriteCell) -> Result<SpriteCell> {
        self.set_rel(pos - self.anchor, cel)
    }

    // Ordinary field accesses
    
    pub fn anchor(&self) -> TermPos {
        self.anchor
    }

    pub fn reanchor(&mut self, pos: TermPos) {
        self.anchor = pos
    }

    pub fn order(&self) -> i16 {
        self.order
    }

    pub fn reorder(&mut self, order: i16) {
        self.order = order
    }

    pub fn id(&self) -> Id<Self> {
        self.id.clone()
    }

    pub fn visible(&self) -> Visibility {
        self.visible
    }

    pub fn set_visible(&mut self, v: Visibility) {
        self.visible = v
    }

    pub fn clickable(&self) -> Clickability {
        self.clickable
    }

    pub fn set_clickable(&mut self, c: Clickability) {
        self.clickable = c
    }
}
