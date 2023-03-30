use std::cell::RefCell;
use std::rc::Rc;

use crate::bounds::Bounds;
use crate::id::Id;
use crate::util::*;

use super::sprite::Sprite;

type SpriteTreeNode<'a> = Option<Rc<RefCell<Sprite<'a>>>>;
use SpriteTreeNode as STN;

pub trait SpriteTreeLike<'a> where Self: Sized {
    // type SpriteRef: SpriteRefLike;

    fn mk(sp: STN<'a>, children: Vec<Self>, id: Id) -> Self;
    fn bounds(&self) -> Option<Bounds<i16>>;

    fn node(&self) -> STN<'a>; 
    fn set_node(&mut self, node: STN<'a>);
    // fn node_mut(&mut self) -> &

    fn children(&self) -> &Vec<Self>;
    fn children_mut(&mut self) -> &mut Vec<Self>;

    // Hmm. Why does adding 'a lifetime annotation break this? Think about it.
    // Oh, kinda makes sense. Forces reference to live extra long.
    fn id(&self) -> &Id;

    fn new(sp: STN<'a>) -> Self {
        Self::mk(sp, Default::default(), Default::default())
    }

    fn sprite(&self, id: &Id) -> STN<'a> {
        if let Some(node) = self.node() {
            if node.borrow().id() == *id {
                return Some(node)
            }
        };

        for child in self.children() {
            if let Some(sp) = child.sprite(id) {
                return Some(sp)
            }
        };

        None
    }

    fn tree(&self, id: &Id) -> Option<&Self> {
        if *self.id() == *id {
            return Some(self)
        }

        for child in self.children() {
            if let Some(tr) = child.tree(id) {
                return Some(tr)
            }
        }

        None
    }

    fn tree_mut(&mut self, id: &Id) -> Option<&mut Self> {
        if *self.id() == *id {
            return Some(self)
        }

        for child in self.children_mut() {
            if let Some(tr) = child.tree_mut(id) {
                return Some(tr)
            }
        }

        None
    }

    fn add_tree(&mut self, tr: Self, parent: Option<&Id>) -> Result<()> {
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

    fn push_sprite(&mut self, sp: Rc<RefCell<Sprite<'a>>>) {
        self.children_mut().push(
            Self::new(Some(sp))
        );
    }

    // fn insert_sprite();
}


pub trait SpriteRefLike{}


