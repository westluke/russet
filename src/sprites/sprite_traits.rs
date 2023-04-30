use std::cell::RefCell;
use std::rc::Rc;

use crate::bounds::Bounds;
use crate::id::Id;
use crate::util::*;

use super::sprite::Sprite;

pub type SpriteTreeNode = Rc<RefCell<Sprite>>;
pub use SpriteTreeNode as Stn;

// hmm. These don't work great for spritetreeref, do they....
// only want modification functions, really. not making new shit functions.
// Mmmm well... part of the problem is that "Self" isn't really the right type for some of these.
// Or maybe... maybe SpriteManagerRef just shouldn't implement this?
// cuz its deref target already implements it.
// It should just have an inherent impl for, idk, insert/push methods?
// Should it even have any other methods? like tree_mut? Yeah no, that seems wrong.
// OK so it shouldn't even be deref. It should just explicitly override those few methods.
//
// If we really waant to get fancy with it, have it implement a separate trait, SpriteRefLike.
// That explicitly overrides those methods.

pub trait SpriteTreeLike where Self: Sized {
    // type SpriteRef: SpriteRefLike;

    fn mk(sp: Option<Stn>, children: Vec<Self>) -> Self;
    fn bounds(&self) -> Option<Bounds<i16>>;

    fn node(&self) -> Option<&Stn>; 
    fn set_node(&mut self, node: Option<Stn>);
    // fn node_mut(&mut self) -> &

    fn children(&self) -> &Vec<Self>;
    fn children_mut(&mut self) -> &mut Vec<Self>;

    fn id(&self) -> Id<Self>;

    fn new(sp: Option<Stn>) -> Self {
        Self::mk(sp, Default::default())
    }

    // need better name to distinguish from node()
    fn find_node(&self, id: Id<Sprite>) -> Option<&Stn> {
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

    fn find_tree(&self, id: Id<Self>) -> Option<&Self> {
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

    fn tree_mut(&mut self, id: Id<Self>) -> Option<&mut Self> {
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

    fn push_tree(&mut self, tr: Self) {
        self.insert_tree(tr, None);
    }

    // Should insert_tree be changed somehow? Like, should you really need to construct your own
    // tree to do this? Idk, feels off. It's fine for now though.

    fn insert_tree(&mut self, tr: Self, parent: Option<Id<Self>>) -> Result<()> {
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

    fn push_sprite(&mut self, sp: Stn) -> Id<Self> {
        self.insert_sprite(sp, None).unwrap()
    }

    fn insert_sprite(&mut self, sp: Stn, parent: Option<Id<Self>>) -> Option<Id<Self>> {
        let new_tr = Self::new(Some(sp));
        let new_id = new_tr.id();
        match self.insert_tree(new_tr, parent) {
            Ok(_) => Some(new_id),
            Err(_) => None
        }
    }

    fn all_sprites(&self) -> Vec<Stn> {
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

    // THIS ALSO NEEDS METHODS FOR REORDERING, REANCHORING, ETC!
    // otherwise those are inherent-specific, and that's bad.
}


// pub trait SpriteRefLike{}


