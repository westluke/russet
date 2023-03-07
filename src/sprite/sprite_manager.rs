use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::io::Write;

use super::Sprite;
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
    pub fn new(
        anchors: SAT<'a>, onto: SOnT<'a>, order: SOrT<'a>,
        sprites: Vec<RefCell<Sprite<'a>>>,
        dirt: RefCell<HashMap<i16, HashSet<i16>>>
    ) -> Self {
        Self { anchors, onto, order, sprites, dirt }
    }
        
    // pub fn merge(
    //     mut sm0: SpriteManager, mut sm1: SpriteManager,
    //     anchor_parent: Id, onto_parent: Id, order_parent: Id
    // ) -> SpriteManager {
    // }

    // pub fn absorb(
    //     &mut self,
    //     sp: PreSprite,
    //     anchor_parent: Id, onto_parent: Id, order_parent: Id
    // ){
    // }
    
    fn get_cell(&self, p: TermPos) -> TermChar {
        if self.sprites.is_empty() {
            Default::default()
        } else {
            let x = &self.sprites[0];
            
            Default::default()
        }
    }

    // This just naively writes to writer, but for maximum performance that writer SHOULD be an
    // object communicating writes to a separate thread, which ACTUALLY writes them to terminal.
    pub fn write(&self, writer: &mut impl Write) {
        queue!(writer, cursor::MoveTo(0, 0), PrintStyledContent("heyo".stylize()));
        writer.flush();
    }
}
