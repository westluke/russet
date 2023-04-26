use std::cell::RefCell;

use crate::id::Id;
use crate::bounds::Bounds;

use super::sprite::Sprite;
use super::sprite_traits::{SpriteTreeLike, SpriteTreeNode as STN};

pub struct SpriteTree {
    node: Option<STN>,
    children: Vec<Self>,
    id: Id<Self>
}

impl SpriteTree {
    // pub fn shift(&self, shift: TermPos) {
    //     // shift node
    //     // recurse through children
    // }
}

impl SpriteTreeLike for SpriteTree {
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
