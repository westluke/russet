use std::collections::{HashSet, HashMap};
use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use crate::pos::TermPos;
use crate::util::*;
use crate::Id;
use crate::bounds::Bounds;

use super::pre_sprite::{PreSprite};
use super::SpriteCell;
use super::dirt::Dirt;

use log::info;

// Does sprite need to store dirt?
// If so, does it need to be in refcell? Yes..

// Ugh. Ok, the problem is that, conceptually, every sprite needs a mutable reference, which isn't possible.

// For the problem of the trees, the conceptual issue, again, is that every tree in the sprite must be
// a mutable reference. Can replace this with reference to refcell. But then, who owns it? Can't be
// the vec inside the same struct. Would have to come from outside. But in THAT case, we can never
// really delete any sprites - the compiler's inferences aren't powerful enough for us to be able
// to tell it when we're done with a dumb reference. Need a smart pointer, i.e. Rc, even though
// there IS a single owner. Yeah, it has to be Rc<RefCell>

#[derive(Debug, Clone)]
pub struct Sprite {
    pre_sprite: PreSprite,
    dirt: Dirt,
}


impl Sprite {
    pub fn new(pre_sprite: PreSprite, dirt: Dirt) -> Self {
        // Actually, should Sprites always be initialized with maximal dirt? Lol wait this dirt is
        // external. So also it doesn't have to be Rc? Try it with reference first I guess.
        // Cuz multiple ownership of dirt is unnecessary here, the manager owns it.
        // sprites don't rly need to be wrapped in Rcs either, the manager vector will own them.
        // Modification methods could also pass out custom ref objects to sprites?
        // That or operate using IDs
        Self { pre_sprite, dirt }
    }

    pub fn redirt(&mut self, dirt: &Dirt) {
        self.dirt = dirt.clone()
    }

    pub fn reanchor(&mut self, anchor: TermPos) {
        self.pre_sprite.reanchor(anchor);
    }

    pub fn reorder(&mut self, z: i16) {
        self.pre_sprite.reorder(z);
    }

    pub fn destroy(&mut self) {
        self.dirty_all()
    }

    // TODO: should probably never dirty anything that's out of bounds? Nooo,, cuz maybe bounds
    // will expand. Just shouldn't WRITE anything that's out of bounds. That's somewhat trickier...

    pub fn dirty_all(&self) {
        info!("{:?}", self.pre_sprite.bounds());
        self.dirt.dirty_all(self.pre_sprite.bounds());
    }

    pub fn get_rel(&self, pos: TermPos) -> Result<SpriteCell> {
        self.pre_sprite.get_rel(pos)
    }

    pub fn get(&self, pos: TermPos) -> Result<SpriteCell> {
        self.pre_sprite.get(pos)
    }

    pub fn set(&mut self, pos: TermPos, cel: SpriteCell) -> Result<SpriteCell> {
        let result = self.pre_sprite.set(pos, cel);
        self.dirt.set_dirty(pos);
        result
    }

    pub fn set_rel(&mut self, pos: TermPos, cel: SpriteCell) -> Result<SpriteCell> {
        let result = self.pre_sprite.set_rel(pos, cel);
        self.dirt.set_dirty(pos);
        result
    }

    pub fn bounds(&self) -> Bounds<i16> {
        self.pre_sprite.bounds()
    }

    pub fn id(&self) -> Id<Sprite> {
        self.pre_sprite.id().into()
    }

    pub fn visible(&self) -> bool {
        self.pre_sprite.visible()
    }

    pub fn clickable(&self) -> bool {
        self.pre_sprite.clickable()
    }

    pub fn set_visible(&mut self, v: bool) {
        self.pre_sprite.set_visible(v)
    }

    pub fn set_clickable(&mut self, c: bool) {
        self.pre_sprite.set_clickable(c);
    }
}


