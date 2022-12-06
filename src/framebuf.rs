use std::io::{Write};
use std::borrow::Borrow;
use std::collections::{HashSet};
use std::ops::BitOr;

use crossterm::style::{self, Color, ContentStyle, StyledContent, PrintStyledContent};
use crossterm::{queue, cursor};

use crate::pos::*;
use crate::termchar::*;
use crate::util::*;

use log::{info, warn, error};

mod grid;
mod line_update;
mod termable;
pub mod frame_tree;

use grid::Grid;
use frame_tree::FrameTree;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayerCell {
    Opaque(TermChar),
    Transparent,
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

    pub fn get_tree(&self) -> &FrameTree {
        &self.frame_tree
    }

    pub fn get_tree_mut(&mut self) -> &mut FrameTree {
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

    // pub fn flush(&mut self)  {

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

    // Writes all new changes out to the underlying buffer
    // pub fn flush(&mut self)  {

    //     // can optimize by pre-fetching dirtied line numbers
    //     let mut dirty_lines = HashSet::new();

    //     for lay_i in 0..self.layer_groups.len() {
    //         let keys: HashSet<i16> = self.layer_groups[lay_i].get_dirty_lines();
    //         dirty_lines = dirty_lines.bitor(&keys);
    //     }

    //     // for every dirty line...
    //     for row_i in dirty_lines {

    //         // start a new line update
    //         let mut lnup = LineUpdate::new(TS.width());

            
    //         // note: i don't actually need to check every cell, can optimize this out
    //         // for every cell in this line...
    //         for col_i in 0..TS.width() {

    //             // for every layer...
    //             for layg_i in 0..self.layer_groups.len() {
    //                 let layg = self.layer_groups.get(layg_i).unwrap();
    //                 let pos = TermPos::ffrom((row_i, col_i)).chk();
    //                 let (cel, change) = layg.get_c(pos);

    //                 match (change, cel) {
    //                 // how doew this algorithm extend to layergroups?
    //                     // If we hit an opaque cell, we're done -- we won't see changes past this
    //                     // stupid and wrong, not actuall optimizing
    //                     (_, cel @ Opaque(_)) => {
    //                         lnup.set(col_i, cel);
    //                         break;
    //                     },

    //                     // If we hit a newly transparent cell, we have to keep going,
    //                     // waiting for the next opaque cell. But in case we fall ALL the way
    //                     // through, we put in the default value (terminal background)
    //                     (true, Transparent) => lnup.set(col_i, Opaque(Default::default())),

    //                     // If we hit an old transparent cell, we just fall through
    //                     (false, Transparent) => (),
    //                 };
    //             };
    //         };

    //         queue!(
    //             self.under,
    //             cursor::MoveToRow(u16::try_from(row_i).unwrap())
    //         );
            
    //         for (col_i, cont) in lnup.finalize() {
    //             queue!(
    //                 self.under, 
    //                 cursor::MoveToColumn(
    //                     u16::try_from(col_i).unwrap()),
    //                 PrintStyledContent(cont)
    //             );
    //         };
    //     };

    //     // clear all layers
    //     for lay in &mut self.layer_groups {
    //         lay.clean();
    //     }

    //     self.under.flush();
    // }
}
