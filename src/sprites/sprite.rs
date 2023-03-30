use std::collections::{HashSet, HashMap};
use std::cell::{RefCell, RefMut};

use crate::pos::TermPos;
use crate::util::*;
use crate::Id;
use crate::bounds::Bounds;

use super::pre_sprite::{PreSprite};
use super::SpriteCell;

#[derive(Debug, Clone)]
pub struct Sprite<'a> {
    pre_sprite: PreSprite,
    dirt: &'a RefCell<HashMap<i16, HashSet<i16>>>,
}

fn set_dirty(mut dirt: RefMut<HashMap<i16, HashSet<i16>>>, p: TermPos) {
    dirt.entry(p.y())
        .or_insert(HashSet::new())
        .insert(p.x());
}

fn dirty_all(mut dirt: RefMut<HashMap<i16, HashSet<i16>>>, bnd: Bounds<i16>) {
    for y in bnd.y_range() {
        dirt.entry(y)
            .or_insert(HashSet::new())
            .extend(bnd.x_range())
    }
}

impl<'a> Sprite<'a> {
    pub fn new(pre_sprite: PreSprite, dirt: &'a RefCell<HashMap<i16, HashSet<i16>>>, zs: Vec<i16>) -> Self {
        // Actually, should Sprites always be initialized with maximal dirt? Lol wait this dirt is
        // external. So also it doesn't have to be Rc? Try it with reference first I guess.
        // Cuz multiple ownership of dirt is unnecessary here, the manager owns it.
        // sprites don't rly need to be wrapped in Rcs either, the manager vector will own them.
        // Modification methods could also pass out custom ref objects to sprites?
        // That or operate using IDs
        Self { pre_sprite, dirt }
    }

    pub fn reanchor(&mut self, anchor: TermPos) {
        self.pre_sprite.anchor = anchor
    }

    pub fn reorder(&mut self, zs: Vec<i16>) {
        self.pre_sprite.zs = zs
    }

    pub fn destroy(&mut self) {
        self.dirty_all()
    }

    pub fn dirty_all(&self) {
        dirty_all(self.dirt.borrow_mut(), self.pre_sprite.bounds())
    }

    pub fn get(&self, pos: TermPos) -> Result<SpriteCell> {
        self.pre_sprite.img.get(pos)
    }

    pub fn set(&mut self, pos: TermPos, cel: SpriteCell) -> Result<SpriteCell> {
        let result = self.pre_sprite.img.set(pos, cel);
        set_dirty(self.dirt.borrow_mut(), pos);
        result
    }

    pub fn bounds(&self) -> Bounds<i16> {
        self.pre_sprite.bounds()
    }

    pub fn id(&self) -> Id {
        self.pre_sprite.id.clone()
    }
}


