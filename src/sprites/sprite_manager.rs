use std::rc::Rc;
use std::cell::RefCell;
use std::io::Write;
use std::cmp::Ordering;

use super::{sprite::Sprite, pre_sprite::PreSprite};
use super::sprite_tree::SpriteTree;
use super::dirt::Dirt;
use super::{SpriteCell::*, Stn};
use super::termable::Termable;

use crossterm::queue;
use crossterm::cursor;
use crossterm::style::{PrintStyledContent, Stylize, StyledContent, ContentStyle};

use log::info;

use crate::term_char::TermChar;
use crate::util::FInto;

// This is a VERY leaky abstraction, not worth the effort of making it automatic.

#[derive(Default, Clone, Debug)]
pub struct SpriteManager {
    pub tree: SpriteTree,

    // Resorted after every order manipulation
    pub sprites: Vec<Stn>,

    // Inserted into sprites so they can dirty their backgrounds when they manipulate themselves.
    pub dirt: Dirt
}

impl From<PreSprite> for SpriteManager {
    fn from(pre: PreSprite) -> Self {
        let mut ret = Self::default();
        let post = ret.attach(pre);
        ret.sprites.push(Rc::new(RefCell::new(post)));
        ret
    }
}

impl SpriteManager {
        
    pub fn attach(&mut self, sp: PreSprite) -> Sprite {
        let ret = Sprite::new(sp, self.dirt.clone());
        ret.dirty_all();
        ret
    }

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
