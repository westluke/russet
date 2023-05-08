use std::rc::Rc;
use std::cell::RefCell;
use std::io::Write;
use std::cmp::Ordering;

use super::{sprite::Sprite};
use super::sprite_tree::SpriteTree;
use super::dirt::Dirt;
use super::{SpriteCell::*, Stn};
use super::termable::Termable;

use crate::pos::TermPos;

use crossterm::queue;
use crossterm::cursor;
use crossterm::style::{PrintStyledContent, Stylize, StyledContent, ContentStyle};

use log::info;

use crate::term_char::TermChar;
use crate::util::*;

// This is a VERY leaky abstraction, not worth the effort of making it automatic.

#[derive(Default, Clone, Debug)]
pub struct SpriteManager {
    pub tree: SpriteTree,

    // Resorted after every order manipulation
    pub sprites: Vec<Stn>,

    // Inserted into sprites so they can dirty their backgrounds when they manipulate themselves.
    pub dirt: Dirt
}

// impl From<PreSprite> for SpriteManager {
//     fn from(pre: PreSprite) -> Self {
//         let mut ret = Self::default();
//         let post = ret.attach(pre);
//         ret.sprites.push(Rc::new(RefCell::new(post)));
//         ret
//     }
// }

impl SpriteManager {

    // pub fn set_tree(&mut self, tree: SpriteTree) {
    //     self.tree = tree
    // }

    // pub fn get_tree_mut(&mut self) -> &mut SpriteTree {
    //     &mut self.tree
    // }
        
    pub fn refresh_sprites(&mut self) {
        self.sprites = self.tree.all_sprites();
    }

    pub fn sort(&mut self) {
        self.sprites.sort_by(|x, y| {
            let xo = x.borrow().order();
            let yo = y.borrow().order();
            if xo == yo {
                Ordering::Equal
            } else if xo < yo {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
    }

    // This just naively writes to writer, but for maximum performance that writer SHOULD be an
    // object communicating writes to a separate thread, which ACTUALLY writes them to terminal.
    pub fn write(&mut self, writer: &mut impl Write) {

        {   
            let rf = self.dirt.borrow();

            for (&y, line) in rf.iter() {
                if y < 0 || TS.height() <= y { continue; };
                write_line(writer, &self.sprites, y, line);
            }
        };

        self.dirt.clear();
        writer.flush();
    }
}

fn queue_term(writer: &mut impl Write, term: Termable, y: u16, x: u16) {
    queue!(
        writer,
        cursor::MoveTo(x, y),
        PrintStyledContent(term.finalize())
    );
}

fn write_line(writer: &mut impl Write, sprites: &Vec<Stn>, y: i16, line: &Vec<i16>) {

    let mut term = Termable::default();
    let mut start: i16 = 0;
    let mut last: i16 = -1;
    let mut at_least_one = false;

    for &x in line {
        if x < 0 { continue; };
        if TS.width() <= x { break; };

        at_least_one = true;

        // We write the default background color if we don't hit anything opaque
        let mut char_to_write = TermChar::default();

        for sprite in sprites {
            let cel = sprite.borrow_mut()
                .get((y, x).finto())
                .unwrap_or(Transparent);

            if let Opaque(tc) = cel {
                char_to_write = tc;
                break;
            }
        }

        // If there was a jump, or if pushing onto the current termable fails,
        // then write the old termable and start a new one. In either case, termable is
        // non-empty by end.
        
        if (x != last + 1) || !term.push(char_to_write) {
            queue_term(writer, term, y.finto(), start.finto());
            term = Termable::default();
            term.push(char_to_write);
            start = x;
        }

        last = x;
    }

    // Push the last term, if non-empty.
    if at_least_one { queue_term(writer, term, y.finto(), start.finto()); };
}
