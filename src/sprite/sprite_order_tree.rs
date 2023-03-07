use super::SpriteTree;

#[derive(Default)]
pub struct SpriteOrderTree<'a> (SpriteTree<'a>);

use SpriteOrderTree as SOT;

impl<'a> SOT<'a> {
    // distributive
    pub fn reorder() {}
}

