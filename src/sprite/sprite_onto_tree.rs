use super::SpriteTree;

#[derive(Default)]
pub struct SpriteOntoTree<'a> (SpriteTree<'a>);

use SpriteOntoTree as SOT;

impl<'a> SOT<'a> {
    // distributive
    pub fn destroy() {}
}

