use std::rc::Rc;
use std::cell::RefCell;
use uuid::Uuid;
use super::Sprite;
use super::SpriteTree;

// Do branches need names?
// Well, otherwise, what's the point of this thing?
// Need to refer to branches to shift entire subtrees.
//
// How do I initialize/control tree layout as a user?
// I think I should just go through the normal operations, not
// initialize with full trees. So even when setting up cards, we start with an empty
// SpriteManager, and just build it up through ID operations.
//
// The tricky bit there is that the cards are all sepaarate at first. Do i need a way to
// combine SpriteManagers? I guess I could do that by picking an embedding id for each of the
// component trees... But what if there is ID repeat? Just make sure there isn't.

#[derive(Default)]
pub struct SpriteAnchorTree<'a> (SpriteTree<'a>);

// pub struct SpriteAnchorRef<'a> {
//     sprite: &'a RefCell<Sprite<'a>>,
//     tree: &'a SpriteAnchorTree<'a>
// }

use SpriteAnchorTree as SAT;
// use SpriteAnchorRef as SAR;

impl<'a> SAT<'a> {
    // distributive
    pub fn reanchor() {}
    pub fn push_sprite(&mut self, sp: &'a RefCell<Sprite<'a>>) {
        self.0.push_sprite(sp)
    }
}

