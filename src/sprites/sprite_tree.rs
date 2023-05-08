use crate::id::Id;
use crate::bounds::Bounds;
use crate::Result;
// use crate::util::err::*;
use crate::util::*;
use crate::pos::TermPos;

use super::sprite::Sprite;
use super::dirt::Dirt;
use super::Stn;
use super::*;


#[derive(Default, Debug, Clone)]
pub struct SpriteTree {
    node: Stn,
    children: Vec<Self>,
    id: Id<Self>
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub enum InheritanceType {
    #[default]
    Dont,
    Children
}
use InheritanceType::*;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Inheritances {
    pub anchor: InheritanceType,
    pub order: InheritanceType,
    pub visibility: InheritanceType,
    pub clickability: InheritanceType
}

pub const INHERIT_NONE: Inheritances = Inheritances {
    anchor: Dont,
    order: Dont,
    visibility: Dont,
    clickability: Dont,
};

pub const INHERIT_ALL: Inheritances = Inheritances {
    anchor: Children,
    order: Children,
    visibility: Children,
    clickability: Children,
};

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
    
    pub fn new(sp: Stn) -> Self {
        Self::mk(sp, Default::default())
    }

    pub fn bounds(&self) -> Bounds<i16> {
        let mut bounds = self.node.borrow().bounds();
        for child in &self.children {
            bounds = child.bounds().merge(bounds);
        }
        bounds
    }

    // Returns a vec of all the SpriteTrees reporting a collision?
    // Or all the sprites reporting a collision?
    // But then need to go from there, back to card representaiton. How to do that?
    // Suggests either a bidirectional hash, or tags on sprites/trees
    // bidirectional hash makes the most sense, conceptually and practically.
    // I guess I could add the capability for SpriteTrees to also store arbitrary data?
    // as tags, notes, etc. But that seems like too much. maybe later.
    // Until then, assuming a pure spritetree, makes the most sense to get back some Ids that we
    // then look up in a table
    pub fn collide(&self, p: TermPos) -> Vec<Id<Sprite>> {
        let mut ret = vec![];
        let rf = self.node.borrow();
        if rf.clickable() == Clickability::Clickable && rf.bounds().contains(p.finto()) {
            ret.push(rf.id());
        }

        for child in &self.children {
            ret.append(&mut child.collide(p));
        }

        ret
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
    pub fn inherit(&mut self, other: &Self, ins: Inheritances) {
        let n = other.node.borrow();

        // Conceptually, we can think of self as having one parent, all of whose fields are zeroed.
        // In that case, when its parent becomes other, all of the field values for other become
        // the deltas for shifting, since we're comparing to the zeroed parent.
        if ins.anchor == Children { self.shift(n.anchor(), Children); };
        if ins.order == Children { self.shift_order(n.order(), Children); };
        if ins.visibility == Children { self.set_visible(n.visible(), Children); };
        if ins.clickability == Children { self.set_clickable(n.clickable(), Children); };
    }

    pub fn push_tree(&mut self, tr: Self, ins: Inheritances) {
        self.insert_tree(tr, None, ins);
    }

    // Should insert_tree be changed somehow? Like, should you really need to construct your own
    // tree to do this? Idk, feels off. It's fine for now though.

    pub fn insert_tree(&mut self, mut tr: Self, parent: Option<Id<Self>>, ins: Inheritances) -> Result<()> {
        if let Some(id) = parent {
            let parent_tr_opt = self.tree_mut(id);
            if let Some(parent_tr) = parent_tr_opt {
                tr.inherit(parent_tr, ins);
                parent_tr.children.push(tr);
                Ok(())
            } else {
                Err(SetError::new(SetErrorKind::IdNotFound, &format!("No tree found with id {}", id)))
            }
        } else {
            tr.inherit(self, ins);
            self.children.push(tr);
            Ok(())
        }
    }

    pub fn push_sprite(&mut self, sp: Stn, ins: Inheritances) -> Id<Self> {
        self.insert_sprite(sp, None, ins).unwrap()
    }


    pub fn insert_sprite(&mut self, sp: Stn, parent: Option<Id<Self>>, ins: Inheritances) -> Option<Id<Self>> {
        let new_tr = Self::new(sp);
        let new_id = new_tr.id();
        match self.insert_tree(new_tr, parent, ins) {
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

    pub fn shift(&mut self, delta: TermPos, ins: InheritanceType) {
        let mut n = self.node.borrow_mut();

        // Confusion on mut semantics - difference between stacked borrows and just ignoring borrow
        // rules completely? Need to nail down difference between borrow no longer existing, and
        // borrow being deactivated. Actual idea of "stacking"

        let anchor = n.anchor();
        n.reanchor(anchor + delta);

        if ins == Children {
            for child in &mut self.children {
                child.shift(delta, Children);
            }
        }
    }

    pub fn reanchor(&mut self, pos: TermPos, ins: InheritanceType) {
        let delta;
        {   
            // Block is necessary so that compiler knows n is dropped early - not sure why NLL
            // didn't catch this? Maybe ask Jack
            let n = self.node.borrow();
            delta = pos - n.anchor();
        };
        self.shift(delta, ins);
    }

    pub fn shift_order(&mut self, delta: i16, ins: InheritanceType) {
        let mut n = self.node.borrow_mut();
        let order = n.order();
        n.reorder(order + delta);

        if ins == Children {
            for child in &mut self.children {
                child.shift_order(delta, Children);
            }
        };
    }

    pub fn reorder(&mut self, order: i16, ins: InheritanceType) {
        let delta;
        {
            let n = self.node.borrow();
            delta = order - n.order();
        };
        self.shift_order(delta, ins);
    }

    pub fn set_visible(&mut self, v: Visibility, ins: InheritanceType) {
        self.node.borrow_mut().set_visible(v);
        if ins == Children {
            for child in &mut self.children {
                child.set_visible(v, Children);
            }
        };
    }

    pub fn set_clickable(&mut self, c: Clickability, ins: InheritanceType) {
        self.node.borrow_mut().set_clickable(c);
        if ins == Children {
            for child in &mut self.children {
                child.set_clickable(c, Children);
            }
        };
    }
}
