use std::io::{Write};
use std::collections::{HashMap};
use std::ops::{Index, IndexMut};
use std::cmp::{Ordering, min, max};

use crossterm::style::{self, Color, Colors, SetColors, Print};
use crossterm::{queue, cursor};

use crate::pos::*;
use crate::err::*;
use crate::deck::Card;
use crate::termchar::*;

use log::{info, warn, error};

mod grid;
mod stain;
mod layer;

use grid::Grid;
use stain::Stain;
use stain::StainSet;
use layer::Layer;

pub struct FrameBuf<T: Write> {
    height: i16,
    width: i16,

    // The underlying Write object (should be a terminal, probably stdout)
    under: T,

    // Each layer is an independent "panel" that can be manipulated across the screen, i.e. a
    // playing card sliding around
    layers: Vec<Layer>,
}

// impl<T: Write> Index<Option<Card>> for FrameBuf<T> {
//     type Output = Layer;

//     fn index(&self, id: Option<Card>) -> &Self::Output {
//         self.layers.iter().find(|x| x.id == id).unwrap()
//     }
// }

// impl<T: Write> IndexMut<Option<Card>> for FrameBuf<T> {
//     fn index_mut(&mut self, id: Option<Card>) -> &mut Self::Output {
//         self.layers.iter_mut().find(|x| x.id == id).unwrap()
//     }
// }

impl<T: Write> FrameBuf<T> {
    pub fn new(under: T, height: i16, width: i16) -> Self {
        Self {
            under,
            height, width,
            layers: Vec::new()
        }
    }

    // pub fn push_layer(&mut self, lay: Layer) {
    //     self.layers.push(lay);
    //     // self.change_flags.append(&mut lay.flags());
    //     // merge_flagss(self.change_flags);
    // }

    // fn char_at(&self, pos: TermPos) -> TermChar {
    //     for lay in self.layers.iter().rev() {
    //         if !lay.contains(pos) { continue; };
    //         if let Some(tc) = lay[pos] { return tc; }
    //     }
    //     TermChar::default()
    // }

    // fn queue_acc(&mut self, s: &str, foreground: Option<Color>, background: Option<Color>) {
    //     queue!(
    //         self.under,
    //         SetColors (
    //             Colors{ foreground, background }
    //         ),
    //         Print(s)
    //     );
    // }

    pub fn flush(&mut self)  {
        // check the change flags, for each pix in flags, go through layers from top to bottom
        // then for that flag, construct sequence of commands to queue, based on color changes.

        // need to go in order from highest precedence to lowest, and write only the portions
        // that changed? alternatively, write every change, from lowest precedence to highest.
        // Second one is simpler but potentially slower.
        //
        // Also note that we can be MUCH more conservative in issuing change flags if we know that
        // the panel is uniform fill.
        //
        // Or, wait, do we do an initial run through, merge all stains,
        // then take the intersection of that merger with each individual layer's stains?
        //
        // ugh. so complicated. but eagerly outputting changes any time a layer is changed
        // wastes so much.
        //
        // I think kwe want to take a merger of all stians, but for each stain, we don't
        // want to output wasted termchars, cuz that's so much more expensive.
        //
        // I'm also imagining you could make some much more complex structure, like a termchar
        // array but with a depth dimension, where  that depth dimension somehow indexes  into the
        // layers at  the appropriate places... That would be cool. But a question for another
        // time. Right now the merger/intersection operation should be good enough.
        //
        // What are my actual formal constraints here? Maybe figure that out later.
        // For now, here's what's good enough: printing EVERY layer, from bottom to top.
        //
        // Ugh now I'm starting to think about goign back to the central buffer updated every
        // change, but with annotations for layers and change dates...
        // No, that's still a bad idea, doesn't account well for things like moving around or
        // deletion.
        //
        // Stop premature optimizing!!!

        // map from lines to all the styledcontents that will be output.
        // So, could optimize by also storing the "stains so far" that we've covered, and then
        // adding to the list only if we're outside that range. But that's optimization.
        // For now, just add everything.
        
        let stys = HashMap::new();

        // for every layer...
        for lay_i in 0..self.layers.len() {
            let lay = self.layers.get(lay_i).unwrap();

            // and every *stained* row of that layer...
            for key in lay.stains.stains.keys() {

                // initialize a new set of changes if none are available
                let mut changes = stys.entry(key).or_insert(Vec::new());
                let mut nch = Termable::new();
                let (start, end) = lay.stains.stains.get(key).unwrap().start_end();

                // for every character that was stained...
                for tc_i in start..end {
                    let tc = lay.get(key, tc);
                    if let Some(mut x) = nch.push(tc) {
                        changes.push(nch.finalize());
                        nch = x;
                    }
                }
                if nch != Termable::Empty {
                    changes.push(nch.finalize());
                }
            }
        }

        for key in stys.keys() {
            for ch in stys.get(key) {
                execute!(key, ch);
            }
        }
                    
            
        // for i in 0..self.stains.len() {

        //     let f = self.stains[i];
        //     let (y, x): (i16, i16) = f.start().into();
        //     debug_assert!(f.len() != 0);
        //     let (mut fg0, mut bg0) = (None, None);
        //     let mut acc = String::new();

        //     for i in x..(x+f.len()) {
        //         let tc = self.char_at(TermPos::new(y, i));
        //         let (fg, bg) = tc.get_fg_bg();

        //         if fg0.is_none() { fg0 = fg; };
        //         if bg0.is_none() { bg0 = Some(bg); };

        //         if  (fg0.is_some() && fg.is_some() && fg0 != fg) ||
        //             (bg0.is_some() && bg0 != Some(bg)) {

        //             self.queue_acc(&acc, fg0, bg0);
        //             (fg0, bg0) = (None, None);
        //             acc = String::new();

        //         } else {
        //             acc.push(tc.get_c());
        //         }
        //     }

        //     if !acc.is_empty() {
        //         self.queue_acc(&acc, fg0, bg0);
        //     }
        // }

        // self.stains = Vec::new();
    }
}
