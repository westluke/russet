use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::io::Write;

use super::{Sprite, PreSprite};
use super::sprite_anchor_tree::SpriteAnchorTree as SAT;
use super::sprite_onto_tree::SpriteOntoTree as SOnT;
use super::sprite_order_tree::SpriteOrderTree as SOrT;

use crossterm::queue;
use crossterm::cursor;
use crossterm::style::{PrintStyledContent, Stylize};

use crate::Id;
use crate::pos::TermPos;
use crate::term_char::TermChar;

pub struct SpriteManager<'a> {
    anchors: SAT<'a>,
    onto: SOnT<'a>,
    order: SOrT<'a>,

    // Resorted after every order manipulation
    sprites: Vec<RefCell<Sprite<'a>>>,

    // Inserted into sprites so they can dirty their backgrounds when they manipulate themselves.
    // I could also just have them return hashmaps to be merged into the main one?
    // This is easier for now tho.
    dirt: RefCell<HashMap<i16, HashSet<i16>>>
}


// Here's the big question, I think. How should users interact with these trees?
// In most cases, the trees are identical. So there should be an option to add a sprite,
// with the same parent in all trees. That suggests all interactions should go through the
// SpriteManager? On the other hand, the fact that other parenting options are exposed suggests
// that users need to know about and understand the other trees anyways.
// Suggests that both should be options if possible/reasonable
// But mostly should be done through SpriteManager


NEED TO PUT SOME OF THIS SHIT IN TRAITS, TOO MUCH DUPLICATION
How do I do that?

impl<'a> Default for SpriteManager<'a> {
    fn default() -> Self {
        Self {
            anchors: Default::default(),
            onto: Default::default(),
            order: Default::default(),
            sprites: Default::default(),
            dirt: RefCell::new(Default::default())
        }
    }
}

impl<'a> SpriteManager<'a> {
        
    // pub fn merge(
    //     mut sm0: SpriteManager, mut sm1: SpriteManager,
    //     anchor_parent: Id, onto_parent: Id, order_parent: Id
    // ) -> SpriteManager {
    // }

    // Adds sprite to the top level of every tree
    pub fn push_sprite(&mut self, sp: RefCell<Sprite<'a>>){
        self.anchors.push_sprite(&sp);
        self.sprites.push(sp);
    }

    // Adds sprite as child of the same node in every tree
    pub fn naive_insert_sprite(){}

    // Like naive_insert_sprite, but added sprite can have different parents in different trees.
    pub fn insert_sprite(){}

    pub fn get_tree(){}
    
    pub fn get_tree_mut(){}

    pub fn clean(){}

    // pub fn add_sprite(
    //     &mut self, sp: PreSprite,
    //     anchor_parent: Option<Id>,
    //     onto_parent: Option<Id>,
    //     order_parent: Option<Id>
    // ){
    // }
    
    // fn get_cell(&self, p: TermPos) -> TermChar {
    //     if self.sprites.is_empty() {
    //         Default::default()
    //     } else {
    //         let x = &self.sprites[0];
    //         x.
            
    //         Default::default()
    //     }
    // }

    // This just naively writes to writer, but for maximum performance that writer SHOULD be an
    // object communicating writes to a separate thread, which ACTUALLY writes them to terminal.
    pub fn write(&mut self, writer: &mut impl Write) {
        // let bounds = self.anchor_tree.bounds();
        // for pos in bounds {
        //     if let Entry(e) = dirt.entry(y) {
        //         if e.contains(x) {
        //             for sprite in &self.sprites {
        //                 if sprite.get(pos) != 
        //             }
        //         }
        //     }
        // }
        queue!(writer, cursor::MoveTo(0, 0), PrintStyledContent("heyo".stylize()));
        writer.flush();
    }
}

    // pub fn new(
    //     anchors: SAT<'a>, onto: SOnT<'a>, order: SOrT<'a>,
    //     sprites: Vec<RefCell<Sprite<'a>>>,
    //     dirt: RefCell<HashMap<i16, HashSet<i16>>>
    // ) -> Self {
    //     Self { anchors, onto, order, sprites, dirt }
    // }
