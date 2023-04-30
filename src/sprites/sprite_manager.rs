use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::ops::{Deref, DerefMut};
use crate::Result;

use super::{sprite::Sprite, pre_sprite::PreSprite};
use super::sprite_tree::SpriteTree;
use super::sprite_traits::{*, Stn};
use super::dirt::Dirt;
// use super::line_update::LineUpdateBuilder;
use super::SpriteCell::{self, *};
use super::termable::Termable;

use crossterm::queue;
use crossterm::cursor;
use crossterm::style::{PrintStyledContent, Stylize, StyledContent, ContentStyle};

use log::info;

use crate::Id;
use crate::pos::TermPos;
use crate::term_char::TermChar;
use crate::util::FInto;

// Ok, this idea of triple tree is surprisingly complicated. Maybe unsurprisingly.
// I need to think about invariants, and how to preserve them.
// Stuff like, can the same sprite appear twice in the same tree? Should each tree
// have the same contents, as a set, as every other tree? and the sprites vec?

// Should users be able to manipulate trees on their own, or only through SpriteForests?
// If only through SpriteForests, maintaining invariants might not be too hard. Otherwise...
// all bets are off, really.

// And do we manipulate forests susing the IDs on Sprites, or the ids on trees?

// I think we only use TreeIds, but we provide a method to conveniently transform sprite ids into tree ids.
//
// Also, it would be REALLY NICE to be able to use this as if its only a single tree, for when I
// don't want fancy hierarchical stuff. How can I do that?
//
// Maybe I could have it implement SpriteTreeLike?
// Ach, no, cuz conceptually its al ot more than just a tree, and it's not recursive.
// I COULD split out SpriteForest into its own type, make this SpriteManager again,
// and have SpriteForest implement SpriteTreeLike? Although, no, again that doesn't work.
// because the recursion causes issues. Wait no, that DOES work, as long as you don't use
// the splitting features. Just have to internally keep track of which trees are "the same".
// And then, kinda funky, need to make it so that once its split, it stays split.
//
// Really those should just be different types, huh. Except no, they shouldn't be different
// types, cuz that's a pain and then I'm duplicating work.
//
// Ok, so this new type dynamically switches between the two? clones internally when it switches to complex
// version, to avoid 3x space consumption. Could make alternate version later.

//
//
// Yeahhhhh, things should be done through a ref. Cuz we don't want to be messing with the top
// level object, that's not what we're interested in! We want to be modifying the tree.
// And messing wiht the top-level object feels like too much indirection.
//
// But can ignore that for now.

#[derive(Default, Clone, Debug)]
pub struct SpriteManager {
    pub tree: SpriteTree,

    // Resorted after every order manipulation
    pub sprites: Vec<Stn>,

    // Inserted into sprites so they can dirty their backgrounds when they manipulate themselves.
    // I could also just have them return hashmaps to be merged into the main one?
    // This is easier for now tho.
    pub dirt: Dirt
}

// impl Display for SpriteForest {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> }
//         write!(f, "SpriteForest {
//     anchors: {},
//     onto: {},
//     order: {},
//     sprites: {},
//     dirt: OMITTED
// }", anchors, onto, order, sprites);
//     }
// }


impl From<PreSprite> for SpriteManager {
    fn from(pre: PreSprite) -> Self {
        let mut ret = Self::default();
        let post = ret.attach(pre);
        ret.sprites.push(Rc::new(RefCell::new(post)));
        ret
    }
}


// Here's the big question, I think. How should users interact with these trees?
// In most cases, the trees are identical. So there should be an option to add a sprite,
// with the same parent in all trees. That suggests all interactions should go through the
// SpriteManager? On the other hand, the fact that other parenting options are exposed suggests
// that users need to know about and understand the other trees anyways.
// Suggests that both should be options if possible/reasonable
// But mostly should be done through SpriteManager


impl SpriteManager {
        
    // pub fn merge(
    //     mut sm0: SpriteManager, mut sm1: SpriteManager,
    //     anchor_parent: Id, onto_parent: Id, order_parent: Id
    // ) -> SpriteManager {
    // }

    // This is actually somewhat delicate... cuz all the sprites in other have to have their dirts replaced to point at this one.
    // pub fn naive_merge(&mut self, other: Self) {
    //     let Self { anchors, onto, order, mut sprites, dirt } = other;
    //     for sprite in &mut sprites {
    //         let mut sp = sprite.borrow_mut();
    //         sp.redirt(&self.dirt);
    //         sp.dirty_all();
    //     }
    //     self.anchors.add_tree(anchors, None);
    //     self.onto.add_tree(onto, None);
    //     self.order.add_tree(order, None);
    //     self.sprites.append(&mut sprites);
    //     self.dirt.merge(dirt);
    // }
    
    pub fn attach(&mut self, sp: PreSprite) -> Sprite {
        let ret = Sprite::new(sp, self.dirt.clone());
        ret.dirty_all();
        ret
    }

    // Adds sprite to the top level
    pub fn push_sprite(&mut self, sp: Sprite) -> Option<Id<SpriteTree>> {
        let stn = Rc::new(RefCell::new(sp));
        self.sprites.push(stn.clone());
        return self.tree.push(stn.clone());
    }

    // Should we have these modification functions? Or should it hand out smart pointer to modify
    // SpriteTreeLike directly?

    pub fn insert_sprite(sp: Sprite, id: Option<Id<SpriteTree>>) -> crate::Result<()> {
    }

    // // Adds sprite as child of the same node in every tree
    // pub fn naive_insert_sprite(){}

    // // Like naive_insert_sprite, but added sprite can have different parents in different trees.
    // pub fn insert_sprite(){}

    // pub fn get_tree(){}
    
    // pub fn get_tree_mut(){}

    // pub fn clean(){}

    // // pub fn add_sprite(
    // //     &mut self, sp: PreSprite,
    // //     anchor_parent: Option<Id>,
    // //     onto_parent: Option<Id>,
    // //     order_parent: Option<Id>
    // // ){
    // // }
    
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

        let rf = self.dirt.borrow();

        for (&y, line) in rf.iter() {

            // Don't abstract until you need to! It was no longer clear that I really needed
            // LineUpdate.

            let mut term = Termable::default();
            let mut start: i16 = 0;
            let mut last: i16 = -1;


            // TODO: check to make sure that the relevant pixels are in bounds!

            for &x in line {
                info!("y: {}, x: {}", y, x);

                // We write the default background color if we don't hit anything opaque
                let mut char_to_write = TermChar::default();

                for sprite in &self.sprites {
                    let cel = sprite.borrow_mut()
                        .get((y, x).finto())
                        .unwrap_or(Transparent);
                    info!("cel: {:?}", cel);

                    if let Opaque(tc) = cel {
                        char_to_write = tc;
                        break;
                    }
                }

                // info!("char: {:?}", char_to_write);
                
                // If there was a jump, then don't push onto current, push onto new term.
                // If there wasn't a jump, push onto current term - if that fails, push onto new
                // term. In either case, current term is non-empty by end of if.
                if (x != last + 1) || !term.push(char_to_write) {
                    // info!("term: {:?}", term);
                    // info!("printing at: {}, {}", y, start);
                    // info!("term displayed: |{}|", term);
                    queue!(
                        writer,
                        cursor::MoveTo(start.finto(), y.finto()),
                        PrintStyledContent(term.finalize())
                    );
                    term = Termable::default();
                    term.push(char_to_write);
                    start = x;
                }

                last = x;
            }

            info!("term: {:?}", term);

            queue!(
                writer,
                cursor::MoveTo(start.finto(), y.finto()),
                PrintStyledContent(term.finalize())
            );

            // queue!(
            //     writer,
            //     cursor::MoveTo(0, y.finto()),
            //     PrintStyledContent(StyledContent::new(
            //         ContentStyle::new(),
            //         "wtfdood"
            //     ))
            // );
        }

        // // queue!(writer, cursor::MoveTo(0, 0), PrintStyledContent("heyo".stylize()));
        writer.flush();
    }
}

// pub struct TreeRefMut<'a> (&'a mut SpriteManager);
// pub struct SpriteManagerRef<'a> (&'a SpriteManager);

// Ok, so here's the tricky thing. As-is, all of the below functions would be accessible
// to users. But they shouldn't be. children_mut, for example, would let users add subtrees
// arbitrarily, breaking invariants. I could use this deref opportunity to make those
// functions panic, but that's.... not a great option, to say the least. I would like to
// instead make them compilation errors. But they are part of the signature of SpriteTreeLike,
// which is the deref target, so I can't just make them disappear.  So... I need a private traits
// pattern. What are my options for that again?
//
// private supertrait seems to be the best way.
//
//
// Fuck all this, see sprite_traits notes. Just do inherent impl.
//
// Mmmmm, actually I COULD make functions like tree_mut by wrapping the tree in SpriteManagerRef
// again. And pairing with the original SpriteManager. Should I do that? What exactly do I need?
//
// No this doesn't work either, cuz it would require storing 
//
// Ok, it DOES make sense to have a treeref, but it means either separating out the vec and tree,
// or having the tree in a refcell instead.
//
// Alternatively, the simpler option: the sprite manager doesn't do that much automation for us,
// we have to tell it when to resort the vec for example, its more just like a struct.
// Let's do the second one for now.


// impl<'a> TreeRefMut<'a> {
//     pub fn insert_tree(&mut self, tr: SpriteTree, parent: Option<Id<SpriteTree>>) -> Result<()> {
//         self.0.insert_tree(tr, parent)
//     }
//     // pub fn push_tree(&mut self, tr: Self) {
//     // pub fn insert_sprite(&mut self, sp: Stn, parent: Option<Id<Self>>) -> Option<Id<Self>> {
//     // pub fn push_sprite(&mut self, sp: Stn) -> Id<Self> {
// }
