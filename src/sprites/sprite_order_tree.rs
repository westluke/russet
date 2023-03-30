use std::cell::RefCell;

use crate::id::Id;
use crate::bounds::Bounds;
use crate::pos::TermPos;

use super::sprite::Sprite;
use super::sprite_traits::*;
use super::sprite_anchor_tree::SpriteAnchorTree as SAT;

pub type SpriteOrderTree<'a> = SAT<'a>;

