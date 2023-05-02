use crate::id::Id;
use crate::bounds::Bounds;
use crate::Result;
use crate::util::err::*;

use super::sprite::Sprite;
use super::Stn;

#[derive(Default, Debug, Clone)]
pub struct SpriteTree {
    node: Option<Stn>,
    children: Vec<Self>,
    id: Id<Self>
}

impl SpriteTree {
    pub fn mk(sp: Option<Stn>, children: Vec<Self>) -> Self {
        Self { node: sp, children, id: Id::default()}
    }

    pub fn node(&self) -> Option<&Stn> {
        self.node.as_ref()
    }

    pub fn set_node(&mut self, node: Option<Stn>) {
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
    
    pub fn bounds(&self) -> Option<Bounds<i16>> {
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

    pub fn new(sp: Option<Stn>) -> Self {
        Self::mk(sp, Default::default())
    }

    // need better name to distinguish from node()
    pub fn find_node(&self, id: Id<Sprite>) -> Option<&Stn> {
        if let Some(node) = self.node() {
            if node.borrow().id() == id {
                return self.node();
            }
        };

        for child in self.children() {
            if let Some(sp) = child.find_node(id) {
                return child.find_node(id);
            }
        };

        None
    }

    pub fn find_tree(&self, id: Id<Self>) -> Option<&Self> {
        if self.id() == id {
            return Some(self)
        }

        for child in self.children() {
            if let Some(tr) = child.find_tree(id) {
                return Some(tr)
            }
        }

        None
    }

    pub fn tree_mut(&mut self, id: Id<Self>) -> Option<&mut Self> {
        if self.id() == id {
            return Some(self)
        }

        for child in self.children_mut() {
            if let Some(tr) = child.tree_mut(id) {
                return Some(tr)
            }
        }

        None
    }

    pub fn push_tree(&mut self, tr: Self) {
        self.insert_tree(tr, None);
    }

    // Should insert_tree be changed somehow? Like, should you really need to construct your own
    // tree to do this? Idk, feels off. It's fine for now though.

    pub fn insert_tree(&mut self, tr: Self, parent: Option<Id<Self>>) -> Result<()> {
        if let Some(id) = parent {
            let parent_tr_opt = self.tree_mut(id);
            if let Some(parent_tr) = parent_tr_opt {
                parent_tr.children_mut().push(tr);
                Ok(())
            } else {
                Err(SetError::new(SetErrorKind::IdNotFound, &format!("No tree found with id {}", id)))
            }
        } else {
            self.children_mut().push(tr);
            Ok(())
        }
    }

    pub fn push_sprite(&mut self, sp: Stn) -> Id<Self> {
        self.insert_sprite(sp, None).unwrap()
    }


    // Note: this is wrong, it should inherit position, visibility, clickability, and order.
    pub fn insert_sprite(&mut self, sp: Stn, parent: Option<Id<Self>>) -> Option<Id<Self>> {
        let new_tr = Self::new(Some(sp));
        let new_id = new_tr.id();
        match self.insert_tree(new_tr, parent) {
            Ok(_) => Some(new_id),
            Err(_) => None
        }
    }

    pub fn all_sprites(&self) -> Vec<Stn> {
        let ret = Vec::new();
        if let Some(stn) = self.node() {
            ret.push(stn.clone())
        }
        for tr in self.children() {
            ret.append(&mut tr.all_sprites());
        }
        // NOTE: could this cause panic, double borrow?
        ret.dedup_by(|x, y| x.borrow().id() == y.borrow().id());
        ret
    }

}
