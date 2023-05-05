use crate::id::Id;
use crate::bounds::Bounds;
use crate::Result;
use crate::util::err::*;
use crate::pos::TermPos;

use super::sprite::Sprite;
use super::dirt::Dirt;
use super::Stn;

#[derive(Default, Debug, Clone)]
pub struct SpriteTree {
    node: Stn,
    children: Vec<Self>,
    id: Id<Self>
}

// These need to hand out SpriteRefs ANYWAYS, so that changes made to the SpriteRef
// propagate down the entire tree... Or wait, I could just manually implement those lol.

impl SpriteTree {
    pub fn mk(sp: Stn, children: Vec<Self>) -> Self {
        Self { node: sp, children, id: Id::default()}
    }

    pub fn node(&self) -> &Stn {
        &self.node
    }

    pub fn set_node(&mut self, node: Stn) {
        self.node = node;
    }

    pub fn children(&self) -> &Vec<Self> {
        &self.children
    }

    pub fn children_mut(&mut self) -> &mut Vec<Self> {
        &mut self.children
    }

    pub fn id(&self) -> Id<Self> {
        self.id.clone()
    }
    
    pub fn bounds(&self) -> Bounds<i16> {
        let mut bounds = self.node.borrow().bounds();
        for child in &self.children {
            bounds = child.bounds().merge(bounds);
        }
        bounds
    }

    pub fn new(sp: Stn) -> Self {
        Self::mk(sp, Default::default())
    }

    pub fn find_node(&self, id: Id<Sprite>) -> Option<&Stn> {
        if self.node.borrow().id() == id {
            Some(&self.node)
        } else {
            for child in &self.children {
                if let Some(sp) = child.find_node(id) {
                    return Some(sp);
                }
            };
            None
        }
    }

    pub fn find_tree(&self, id: Id<Self>) -> Option<&Self> {
        if self.id == id {
            Some(self)
        } else {
            for child in &self.children {
                if let Some(tr) = child.find_tree(id) {
                    return Some(tr)
                }
            }
            None
        }
    }

    pub fn tree_mut(&mut self, id: Id<Self>) -> Option<&mut Self> {
        if self.id == id {
            Some(self)
        } else {
            for child in &mut self.children {
                if let Some(tr) = child.tree_mut(id) {
                    return Some(tr)
                }
            }
            None
        }
    }

    // Hmmmmmm. just looking at the parent above might not give us enough information to adequately give the child inheritance. Could either say we're looking at the whole tree very time (kinda gross) or else force every node to be inhabited? But maybe by like a, fake, placeholder sprite?
    // I think it's time to make SpriteTreeNode an actual thing, containing either a sprite or a sprite placeholder.
    // mmmmm, should I?
    // I feel like... just empty sprite?
    // Just empty sprite is by far the simplest option, and honestly it does make sense.
    // Just need Img to support emptiness, and need to optimize by not including them in
    // sprite vec.a
    pub fn inherit(&mut self, other: &Self) {
        let n = other.node.borrow();

        // Conceptually, we can think of self as having one parent, all of whose fields are zeroed.
        // In that case, when its parent becomes other, all of the field values for other become
        // the deltas for shifting, since we're comparing to the zeroed parent.
        self.shift(n.anchor());
        self.shift_order(n.order());
        self.set_visible(n.visible());
        self.set_clickable(n.clickable());
    }

    pub fn push_tree(&mut self, tr: Self) {
        self.insert_tree(tr, None);
    }

    // Should insert_tree be changed somehow? Like, should you really need to construct your own
    // tree to do this? Idk, feels off. It's fine for now though.

    pub fn insert_tree(&mut self, mut tr: Self, parent: Option<Id<Self>>) -> Result<()> {
        if let Some(id) = parent {
            let parent_tr_opt = self.tree_mut(id);
            if let Some(parent_tr) = parent_tr_opt {
                tr.inherit(parent_tr);
                parent_tr.children.push(tr);
                Ok(())
            } else {
                Err(SetError::new(SetErrorKind::IdNotFound, &format!("No tree found with id {}", id)))
            }
        } else {
            tr.inherit(self);
            self.children.push(tr);
            Ok(())
        }
    }

    pub fn push_sprite(&mut self, sp: Stn) -> Id<Self> {
        self.insert_sprite(sp, None).unwrap()
    }


    pub fn insert_sprite(&mut self, sp: Stn, parent: Option<Id<Self>>) -> Option<Id<Self>> {
        let new_tr = Self::new(sp);
        let new_id = new_tr.id();
        match self.insert_tree(new_tr, parent) {
            Ok(_) => Some(new_id),
            Err(_) => None
        }
    }

    pub fn all_sprites(&self) -> Vec<Stn> {
        let mut ret = vec![self.node.clone()];
        for tr in &self.children {
            ret.append(&mut tr.all_sprites());
        }

        // NOTE: could this cause panic, double borrow? Would only happen if x = y, can that
        // happen? Probably not.
        ret.dedup_by(|x, y| {
            let xid = x.borrow().id();
            let yid = y.borrow().id();
            xid == yid
        });
        ret
    }

    pub fn register_dirt(&mut self, dirt: Option<&Dirt>) {
        for tr in &mut self.children {
            tr.register_dirt(dirt.clone());
        }

        let dirt: Option<Dirt> = dirt.map(Clone::clone);

        let mut sp = self.node.borrow_mut();
        sp.set_dirt(dirt);
        sp.dirty_all();
    }

    // Hierarchical sprite manipulation

    pub fn shift(&mut self, delta: TermPos) {
        let mut n = self.node.borrow_mut();

        // Confusion on mut semantics - difference between stacked borrows and just ignoring borrow
        // rules completely? Need to nail down difference between borrow no longer existing, and
        // borrow being deactivated. Actual idea of "stacking"

        let anchor = n.anchor();
        n.reanchor(anchor + delta);
        for child in &mut self.children {
            child.shift(delta);
        }
    }

    pub fn reanchor(&mut self, pos: TermPos) {
        let delta;
        {   
            // Block is necessary so that compiler knows n is dropped early - not sure why NLL
            // didn't catch this? Maybe ask Jack
            let n = self.node.borrow();
            delta = pos - n.anchor();
        };
        self.shift(delta);
    }

    pub fn shift_order(&mut self, delta: i16) {
        let mut n = self.node.borrow_mut();
        let order = n.order();
        n.reorder(order + delta);
        for child in &mut self.children {
            child.shift_order(delta);
        }
    }

    pub fn reorder(&mut self, order: i16) {
        let delta;
        {
            let n = self.node.borrow();
            delta = order - n.order();
        };
        self.shift_order(delta);
    }

    pub fn set_visible(&mut self, v: bool) {
        self.node.borrow_mut().set_visible(v);
        for child in &mut self.children {
            child.set_visible(v);
        }
    }

    pub fn set_clickable(&mut self, c: bool) {
        self.node.borrow_mut().set_clickable(c);
        for child in &mut self.children {
            child.set_clickable(c);
        }
    }
}
