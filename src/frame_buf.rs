use std::io::{Write};
use std::collections::{HashSet, HashMap};
use std::ops::BitOr;

use crossterm::style::{self, Color, ContentStyle, StyledContent, PrintStyledContent};
use crossterm::{queue, execute, cursor};

use crate::pos::*;
use crate::term_char::*;
use crate::util::*;
use crate::Id;

use log::{info, warn, error};

mod grid;
mod line_update;
mod termable;
mod frame_tree;

use grid::Grid;
pub use frame_tree::FrameTree;
use line_update::LineUpdate;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayerCell {
    Opaque(TermChar),
    Transparent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirtyBit {
    Dirty,
    Clean
}

impl LayerCell {
    fn is_opaque(&self) -> bool {
        if let Transparent = self {
            false
        } else {
            true
        }
    }

    fn is_transparent(&self) -> bool {
        !self.is_opaque()
    }
}

use LayerCell::*;

impl Default for LayerCell {
    fn default() -> Self {
        Transparent
    }
}

pub struct FrameBuf<T: Write> {
    // The underlying Write object (should be a terminal, probably stdout)
    under: T,

    // Each layer is an independent "panel" that can be manipulated across the screen, i.e. a
    // playing card sliding around. Start of the vec is top of the stack
    frame_tree: FrameTree
}

impl<T: Write> FrameBuf<T> {
    pub fn new(under: T, frame_tree: FrameTree) -> Self {
        Self { under, frame_tree }
    }

    pub fn tree(&self) -> &FrameTree {
        &self.frame_tree
    }

    pub fn tree_mut(&mut self) -> &mut FrameTree {
        &mut self.frame_tree
    }


    // What's the right abstraction for frame tree here?
    // Am I SURE I just want to hand over mutable access to internals?
    // Yeah I think it's fine actually. Cuz everything is still mediated by method api anyways.
    //
    // Also, means all of these functions can be provided just by leaf nodes, pretty much.

    // // Pushes layer onto the TOP (most visible part) of the stack
    // pub fn push_layer(&mut self, id: String, lay: Layer) {
    //     self.layer_groups.insert(0, LayerGroup::new(id, vec![lay]));
    // }

    // // Slides layer under the BOTTOM (least visible part) of the stack
    // pub fn shup_layer(&mut self, id: String, lay: Layer) {
    //     self.layer_groups.push(LayerGroup::new(id, vec![lay]));
    // }

    // pub fn push_layer_group(&mut self, layg: LayerGroup) {
    //     self.layer_groups.insert(0, layg);
    // }

    // pub fn shup_layer_group(&mut self, layg: LayerGroup) {
    //     self.layer_groups.push(layg);
    // }

    // pub fn get_layer_mut(&mut self, gid: String, id: String) -> Option<&mut Layer> {
    //     for lay in &mut self.layer_groups {
    //         if lay.get_id() == gid {
    //             return lay.get_layer_mut(id);
    //         }
    //     }
    //     return None;
    // }

    // pub fn get_layer_group_mut(&mut self, gid: String) -> Option<&mut LayerGroup> {
    //     for lay in &mut self.layer_groups {
    //         if lay.get_id() == gid {
    //             return Some(lay);
    //         }
    //     }
    //     None
    // }

    // Hmm. theoretically, if this wasn't performing well enough,
    // I could use several work-stealing threads to parallelize it pretty easily.
    // That would be pretty cool if I ever split this out into a separate rendering engine.
    pub fn flush(&mut self)  {
        // self.frame_tree.propagate_dirt();
        
        let mut dirt: HashMap<i16, HashSet<i16>> = self.frame_tree.dirt();
        dirt.retain(|&k, mut v| {
            v.retain(|&e| e >= 0 && e < TS.width());
            k >= 0 && k < TS.height()
        });

        // info!("len: {}", self.frame_tree.len());

        // OH EXTEND IS WRONG LOL
        // SHOULDNT BE EXTEND< SHOULD BE COMBINING

        // info!("{:?}", dirt);
        // info!("height: {:?}", TS.height());

        for (row, cols) in dirt {
            // info!("{:?}", row);
            // if row < 0 || row >= TS.height() { continue; };
            let mut lnup = LineUpdate::new(TS.width());

            for col in cols {
                // if col < 0 || col >= TS.width() { continue; };
                let cel = self.frame_tree.cell((row, col).finto());
                // info!("{:?}", cel);
                lnup.set(col, cel);
            }

            queue!(self.under, cursor::MoveTo(0, row.finto()));

            for (col, styled) in lnup.finalize() {
                queue!(
                    self.under,
                    cursor::MoveToColumn(col.finto()),
                    PrintStyledContent(styled)
                );
            }
        }

        self.frame_tree.clean();
        self.under.flush();
    }

        // but i feel like im missing some possible benefits associated just with using this new
        // tree structure...
        //
        // should i be allowed to reach in and modify nodes directly????
        // I think so. Can I account for that in a nice way?
        //
        // Ok, I think I should be doing all of these checks at flushtime anyways.
        // Doesn't really make sense to have an active tracker of dirties.
        // Why?
        //
        // Well, how would you track that? It would require traversing the whole tree everytime you
        // just wanted to modify a node, and that kinda sucks. If you extract mut node references,
        // that makes more sense, but then you can't update dirties higher up the tree.
        //
        // So every node's dirty map is just the sum of the dirty maps of its children, if any.
        // Also, is there any point in caching an output result at every branch in the tree?
        // Not clear that it makes any sense... I suppose, if I really needed the performance,
        // each cell in the cache could be associated with the leaf it came from, and each dirty
        // marker could also be associated with a leaf, and then if I see the cell in the cache is
        // before the dirty leaf, just keep it.... But that's so complicated lmao.
        // Your existing implementation was fast enough. Just do the same thing but recursive,
        // and now you get the benefit of grouped translation! Stop being so fancy.
        //
        // first day of symptoms was thursday.
        // so allowed to come back tuesday
        // self.frame_tree.propagate_dirt();

        // let dirtied: HashMap<i16, <HashSet<i16>> = self.frame_tree.get_dirt();

        // for every dirty line:
            // start a new line update
            // for each dirty cell:
                // fill in the line update
                //

    // }
}
