use std::ops::{Index, IndexMut};
use std::collections::{HashMap, HashSet, hash_map::Keys};
use std::iter;
use iter::Copied;
use std::ops::BitOr;

use crate::deck::Card;
use crate::termchar::TermChar;
use crate::pos::TermPos;
use crate::util::*;

use log::{info};

use super::Grid;

use crossterm::style::Color;

pub use super::{LayerCell::{self, *}};

// pub struct TreeID {
//     ids: Vec<String>
// }

pub struct FrameTree {
    tk: FrameTreeKind,
    id: String,
    active: bool,
    anchor: TermPos,
    dirt: HashMap<i16, HashSet<i16>>
}

enum FrameTreeKind {
    Leaf {
        frame: Grid<LayerCell>,
    },

    Branch {
        children: Vec<FrameTree>,
        // dirty_cache: HashMap<i16, HashSet<i16>>,
        // cell_cache: Grid<LayerCell>,
    }
}

// impl Default for Tree {
// }

// How does this get flushed?
// how do cells get set?
// how do cells get read?
// Ok, for reading cells, we advance along the branches of the root until we reach one that's opaque at that cell,
// or until we fall through. Falling all the way through is expensive, though, which is where caching comes in I think.
// I mean, it's not much more expensive than it was before. So skip caching for now, but keep in mind that it's very possible here.

// impl FrameTreeKind {
// }

impl FrameTree {
    pub fn new_leaf((height, width): (i16, i16), fill: LayerCell, id: String, display: bool, anchor: TermPos) -> Self {
        debug_assert!(height >= 1 && width >= 1);
        let res = Self {
            tk: FrameTreeKind::Leaf {
                frame: Grid::new(height.finto(), width.finto(), fill)
            },
            id,
            display,
            anchor,
            dirt: HashMap::new()
        };

        // res.dirty_opaq();
        res
    }

    pub fn new_branch(children: Vec<FrameTree>, id: String, display: bool, anchor: TermPos) -> Self {
        let res = Self {
            tk: FrameTreeKind::Branch {
                children
            },
            id,
            display,
            anchor,
            dirt: HashMap::new()
        };

        // res.dirty_opaq();
        res
    }

    pub fn dirty_opaq;

    pub fn get_subtree(&self, mut ids: Vec<String>) -> Option<&Self> {
        if let Some(s) = ids.pop() {
            match &self.tk {
                FrameTreeKind::Branch {children} =>
                    for child in children.iter() {
                        if child.id == s {
                            return child.get_subtree(ids);
                        }
                    },
                _ => return None
            }
        };

        return Some(self)
    }

    pub fn get_subtree_mut(&mut self, mut ids: Vec<String>) -> Option<&mut Self> {
        if let Some(s) = ids.pop() {
            match self.tk {
                FrameTreeKind::Branch {ref mut children} =>
                    for child in children.iter_mut() {
                        if child.id == s {
                            return child.get_subtree_mut(ids);
                        }
                    },
                _ => return None
            }
        } else {
            return Some(self);
        };

        unreachable!()
    }

    pub fn propagate_dirt(&mut self) {
        match &mut self.tk {
            FrameTreeKind::Leaf {..} => (),
            FrameTreeKind::Branch {children, ..} => {
                for child in children {
                    child.propagate_dirt();
                    self.dirt.extend(std::mem::take(&mut child.dirt));
                };
            }
        };
    }

    pub fn get_dirt(&self) -> &HashMap<i16, HashSet<i16>> {
        &self.dirt
    }

    pub fn leaves(&self) -> Vec<&FrameTree> {
        match &self.tk {
            FrameTreeKind::Leaf {..} => vec![self],
            FrameTreeKind::Branch {children, ..} => {
                children.iter()
                    .flat_map(|c| c.leaves())
                    .collect()
            }
        }
    }

    pub fn leaves_mut(&mut self) -> Vec<&mut FrameTree> {
        match self.tk {
            FrameTreeKind::Leaf {..} => vec![self],
            FrameTreeKind::Branch {children: ref mut ch, ..} => {
                ch.iter_mut()
                    .flat_map(|c| c.leaves_mut())
                    .collect()
            }
        }
    }

    // does the whole cell recovery algorithm
    pub fn get_cell(&self, pos: TermPos) {
    }

    pub fn get_cell_rel(&self, pos: TermPos, ids: Vec<String>) {
    }

    // only operates on leaves
    pub fn set_cell(&mut self, pos: TermPos, cel: LayerCell) {
    }

    pub fn set_cell_rel(&mut self, pos: TermPos, cel: LayerCell) {
    }

    pub fn push_tree(&mut self, tr: FrameTree) {
    }

    pub fn shup_tree(&mut self, tr: FrameTree) {
    }

    pub fn leaf_to_branch(&mut self, tr: FrameTree) {
    }
}

// //     pub fn set_s_rel(&mut self, mut pos: TermPos, s: String, fg: Color, bg: Color) -> Result<()> {
// //         let start_x = pos.x();
// //         let chars: Vec<char> = s.chars().collect();

// //         // for every character in the string...
// //         for i in 0..chars.len() {
            
// //             // if it's a newline, we jump down one step, and move back to our original column index
// //             if chars[i] == '\n' {
// //                 pos = pos + (1, 0).finto();
// //                 pos = pos.set_x(start_x);

// //             // otherwise, we set the cell to this character and advance one step to the right.
// //             } else {
// //                 self.set_c_rel(pos, Opaque(TermChar::new(chars[i], fg, bg)))?;
// //                 pos = pos + (0, 1).finto();
// //             };
// //         };

// //         Ok(())
// //     }

// //     pub fn over(&self, other: &Self) -> Self {
// //         let mut corners = self.corners();
// //         corners.append(&mut other.corners());
// //         let (tl, br) = TermPos::bounding_box(corners);

// //         // result is just as wide as necessary to cover both input layers
// //         let mut lay = Self::new_by_bounds(String::new(), tl, br, Transparent);

// //         // For each position in this new layer...
// //         for pos in tl.range_to(br) {
// //             let ch: LayerCell = match (self.covers(pos), other.covers(pos)) {
// //                 // if self is opaque at this position, use the self cell. Otherwise, use other.
// //                 (true, true) =>
// //                     if let cel @ Opaque(_) = self.get_c(pos) {
// //                         cel
// //                     } else {
// //                         other.get_c(pos)
// //                     },
// //                 (true, false) => self.get_c(pos),
// //                 (false, true) => other.get_c(pos),
// //                 (false, false) => Transparent
// //             };

// //             lay.set_c(pos, ch);
// //         };

// //         lay
// //     }
