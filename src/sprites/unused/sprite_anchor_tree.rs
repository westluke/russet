use std::cell::{RefCell};
use std::rc::{Rc};

use crate::id::Id;
use crate::bounds::Bounds;
use crate::pos::TermPos;

use super::sprite::Sprite;
use super::sprite_traits::{*, SpriteTreeNode as STN};

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

// Hmm. starting to think this isn't the right abstraction. Cuz a SpriteAnchorTree, conceptually,
// contains more SpriteAnchorTrees. It doesn't just contain SpriteTrees.
// That suggests a few possibilities. One, there is no SpriteTree type, just the trait,
// and each variation contains basically the same fields. Yeah, I think that's the way to go.
//
// The other option would be to just have one type, like "DistributiveSpriteTree", but that
// wouldn't feel right really cuz then it has the reanchor, and delete, and everything?

// Stands for SpriteAnchorTree

#[derive(Default, Clone, Debug)]
pub struct SpriteAnchorTree {
    node: Option<Rc<RefCell<Sprite>>>,
    children: Vec<Self>,
    id: Id<Self>
}


pub use SpriteAnchorTree as SanTree;

// pub struct SpriteAnchorRef {
//     // sprite: &'a RefCell<Sprite<'a>>,
//     // tree: &'a SpriteAnchorTree<'a>
// }

// impl SpriteAnchorRef {
//     fn reanchor(&self, anchor: TermPos) {
//     }

//     fn shift(&self, shift: TermPos) {
//     }
// }

// impl SpriteRefLike for SpriteAnchorRef {}

// How should reanchoring actually work? Whole tree? just one sprite? Through ref?
// SHIFT through whole tree, or through ref. REANCHOR only through ref.
impl SanTree {
    pub fn shift(&self, shift: TermPos) {
        // shift node
        // recurse through children
    }
}

impl SpriteTreeLike for SanTree {
    // type SpriteRef = SpriteAnchorRef;

    fn mk(sp: Option<STN>, children: Vec<Self>) -> Self {
        Self { node: sp, children, id: Id::default()}
    }

    fn node(&self) -> Option<&STN> {
        self.node.as_ref()
    }

    fn set_node(&mut self, node: Option<STN>) {
        self.node = node;
    }

    fn children(&self) -> &Vec<Self> {
        &self.children
    }

    fn children_mut(&mut self) -> &mut Vec<Self> {
        &mut self.children
    }

    fn id(&self) -> Id<Self> {
        self.id.clone()
    }
    
    fn bounds(&self) -> Option<Bounds<i16>> {
        if let Some(ref rc) = self.node {
            let mut bounds = rc.borrow().bounds();
            for child in &self.children {
                let child_bounds = child.bounds();
                bounds = child_bounds.map_or(bounds, |b| b.merge(bounds));
            }
            Some(bounds)
        } else {
            None
        }
    }
}
