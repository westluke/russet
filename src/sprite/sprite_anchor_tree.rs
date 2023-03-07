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

use SpriteAnchorTree as SAT;

impl<'a> SAT<'a> {
    // distributive
    pub fn reanchor() {}
}

