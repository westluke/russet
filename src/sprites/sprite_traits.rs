use std::cell::RefCell;
use std::rc::Rc;

use crate::bounds::Bounds;
use crate::id::Id;
use crate::util::*;

use super::sprite::Sprite;

pub type SpriteTreeNode = Rc<RefCell<Sprite>>;
pub use SpriteTreeNode as Stn;

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
    fn find_node(&self, id: &Id<Sprite>) -> Option<&Stn> {
        if let Some(node) = self.node() {
            if node.borrow().id() == *id {
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

    fn find_tree(&self, id: &Id<Self>) -> Option<&Self> {
        if self.id() == *id {
            return Some(self)
        }

        for child in self.children() {
            if let Some(tr) = child.find_tree(id) {
                return Some(tr)
            }
        }

        None
    }

    fn tree_mut(&mut self, id: &Id<Self>) -> Option<&mut Self> {
        if self.id() == *id {
            return Some(self)
        }

        for child in self.children_mut() {
            if let Some(tr) = child.tree_mut(id) {
                return Some(tr)
            }
        }

        None
    }

    fn add_tree(&mut self, tr: Self, parent: Option<&Id<Self>>) -> Result<()> {
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

    // Should this take sprite or presprite?
    // Mmm, might as well separate. So it should be sprite, and spritemanager gives back completed
    // sprites. Wait, so if sprites are intrinsically tied to spritemanager, should they be called
    // something else? Eh, can do that later.
    fn push_sprite(&mut self, sp: Rc<RefCell<Sprite>>) -> Id<Self> {
        let new = Self::new(Some(sp));
        let id = new.id();
        self.children_mut().push(new);
        id
    }

    // fn insert_sprite();
}


pub trait SpriteRefLike{}


