use std::ops::{Index, IndexMut};
use std::collections::{HashMap, HashSet, hash_map::Keys};
use std::iter;
use iter::Copied;
use std::ops::BitOr;
use std::cmp::{min, max};

use crate::deck::Card;
use crate::term_char::TermChar;
use crate::pos::TermPos;
use crate::util::*;

use log::{info};

use super::Grid;

use crossterm::style::Color;

pub use super::LayerCell::{self, *};
pub use super::DirtyBit::{self, *};

// pub struct TreeID {
//     ids: Vec<String>
// }


// STILL CONSIDER CHANGING THIS API SO THAT ALL ACCESS IS MEDIATED THROUGH THE PARENT TREE.
// COULD CACHE STRING QUERIES. THAT WAY BOUNDS DONT NEED TO BE CACHED

#[derive(Debug, Clone)]
pub struct FrameTree {
    kind: FrameTreeKind,
    id: String,
    active: bool,
    anchor: TermPos,
    tl: TermPos,
    br: TermPos,
    dirt: HashMap<i16, HashSet<i16>>
}

impl Default for FrameTree {
    fn default() -> Self {
        Self::new_leaf((1, 1), Transparent, "default".into(), true, (0, 0).finto())
    }
}

#[derive(Debug, Clone)]
enum FrameTreeKind {
    Leaf {
        frame: Grid<LayerCell>,
    },

    Branch {
        children: Vec<FrameTree>,
    }
}

impl Default for FrameTreeKind {
    fn default() -> Self {
        Self::Leaf { frame: Grid::default() }
    }
}


// How does this get flushed?
// how do cells get set?
// how do cells get read?
// Ok, for reading cells, we advance along the branches of the root until we reach one that's opaque at that cell,
// or until we fall through. Falling all the way through is expensive, though, which is where caching comes in I think.
// I mean, it's not much more expensive than it was before. So skip caching for now, but keep in mind that it's very possible here.

// impl FrameTreeKind {
// }

impl FrameTree {
    pub fn new_leaf((height, width): (i16, i16), fill: LayerCell, id: String, active: bool, anchor: TermPos) -> Self {
        debug_assert!(height >= 1 && width >= 1);
        let res = Self {
            kind: FrameTreeKind::Leaf {
                frame: Grid::new(height.finto(), width.finto(), fill)
            },
            id,
            active,
            anchor,
            tl: anchor,
            br: anchor + (height, width).finto(),
            dirt: HashMap::new()
        };

        // res.dirty_opaq();
        res
    }

    pub fn new_branch(children: Vec<FrameTree>, id: String, active: bool, anchor: TermPos) -> Self {
        let mut res = Self {
            kind: FrameTreeKind::Branch {
                children
            },
            id,
            active,
            anchor,
            tl: anchor,
            br: anchor,
            dirt: HashMap::new()
        };

        res.propagate_bounds();

        // res.dirty_opaq();
        res
    }

    pub fn propagate_bounds(&mut self) {
        match self.kind {
            FrameTreeKind::Leaf{ref frame} => {
                self.tl = self.anchor;
                self.br = self.anchor + (frame.height(), frame.width()).finto();
            }
            FrameTreeKind::Branch{ref mut children} => {
                let (mut top, mut left, mut bot, mut right) = 
                    (i16::MIN, i16::MIN, i16::MAX, i16::MAX);
                for ref mut child in children {
                    child.propagate_bounds();
                    let (t, l) = child.tl.finto();
                    let (b, r) = child.br.finto();
                    top = min(top, t);
                    left = min(left, l);
                    bot = max(bot, b);
                    right = max(right, r);
                }
                self.tl = (top, left).finto();
                self.br = (bot, right).finto();
            }
        }
    }

    pub fn anchor(&self) -> TermPos {
        self.anchor
    }

    pub fn set_anchor(&mut self, anchor: TermPos) {
        self.anchor = anchor
    }

    pub fn dirt(&self) -> &HashMap<i16, HashSet<i16>> {
        &self.dirt
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    // TermPos component of return value represents the origin point of tree (includes
    // contribution from the anchor of the tree itself.)
    pub fn subtree(&self, mut ids: Vec<String>) -> Option<(TermPos, &Self)> {
        if let Some(s) = ids.pop() {
            if let FrameTreeKind::Branch {children} = &self.kind {
                for child in children.iter() {
                    if child.id == s {
                        if let Some((p, tree)) = child.subtree(ids) {
                            return Some((p + self.anchor, tree));
                        } else {
                            return None;
                        }
                    };
                };
            };
            return None;
        };
        return Some((self.anchor, self));
    }

    pub fn subtree_mut(&mut self, mut ids: Vec<String>) -> Option<(TermPos, &mut Self)> {
        if let Some(s) = ids.pop() {
            if let FrameTreeKind::Branch {children} = &mut self.kind {
                for child in children.iter_mut() {
                    if child.id == s {
                        if let Some((p, tree)) = child.subtree_mut(ids) {
                            return Some((p + self.anchor, tree));
                        } else {
                            return None;
                        }
                    };
                };
            };
            return None;
        };
        return Some((self.anchor, self));
    }

    pub fn propagate_dirt(&mut self) {
        match &mut self.kind {
            FrameTreeKind::Leaf {..} => (),
            FrameTreeKind::Branch {children, ..} => {
                for child in children {
                    child.propagate_dirt();
                    self.dirt.extend(std::mem::take(&mut child.dirt));
                };
            }
        };
    }

    pub fn leaves(&self) -> Vec<(TermPos, &Self)> {
        match self.kind {
            FrameTreeKind::Leaf {..} => vec![(self.anchor, self)],
            FrameTreeKind::Branch {ref children, ..} => {
                children.iter()
                    .flat_map(|c| c.leaves())
                    .map(|(p, tree)| (p + self.anchor, tree))
                    .collect()
            }
        }
    }

    pub fn leaves_mut(&mut self) -> Vec<(TermPos, &mut Self)> {
        match self.kind {
            FrameTreeKind::Leaf {..} => vec![(self.anchor, self)],
            FrameTreeKind::Branch {ref mut children, ..} => {
                children.iter_mut()
                    .flat_map(|c| c.leaves_mut())
                    .map(|(p, tree)| (p + self.anchor, tree))
                    .collect()
            }
        }
    }

    pub fn is_dirty(&self, pos: TermPos) -> DirtyBit {
        match self.dirt.get(&pos.y()) {
            Some(set) => if set.contains(&pos.x()) { Dirty } else { Clean },
            None => Clean
        }
    }

    // does the whole cell recovery algorithm
    // if dirt propagates with std::mem::take, then can't get dirty status of cells as we go, cuz
    // that requires per-leaf dirt. So maybe I should just accept that, yeah, I think that's the move.
    pub fn cell(&self, pos: TermPos) -> LayerCell {
        for (off, leaf) in self.leaves() {
            match &leaf.kind {
                FrameTreeKind::Leaf {frame} =>
                    match frame.get(pos - off) {
                        Some(c @ Opaque(_)) => return c,
                        _ => ()
                    },
                _ => unreachable!()
            };
        };
        LayerCell::default()
    }

    // only operates on leaves, ignores anchor?
    pub fn set_cell(&mut self, pos: TermPos, cel: LayerCell) {
        match self.kind {
            FrameTreeKind::Leaf {ref mut frame} => {frame.set(pos, cel);},
            FrameTreeKind::Branch {..} => panic!("set_cell should only be called on leaves")
        }
    }

    pub fn push_tree(&mut self, tr: FrameTree) {
        match self.kind {
            FrameTreeKind::Branch {ref mut children} => children.insert(0, tr),
            FrameTreeKind::Leaf {..} => {
                self.to_branch();
                self.push_tree(tr);
            }
        };
    }

    pub fn shup_tree(&mut self, tr: FrameTree) {
        match self.kind {
            FrameTreeKind::Branch {ref mut children} => children.push(tr),
            FrameTreeKind::Leaf {..} => {
                self.to_branch();
                self.shup_tree(tr);
            }
        };
    }

    pub fn to_branch(&mut self) {
        match self.kind {
            FrameTreeKind::Leaf {..} => {
                let kind = std::mem::take(&mut self.kind);
                let newleaf = FrameTree {
                    kind,
                    id: "from to_branch".into(),
                    active: true,
                    anchor: (0, 0).finto(),
                    tl: self.tl,
                    br: self.br,
                    dirt: HashMap::new()
                };
                self.kind = FrameTreeKind::Branch {children: vec![newleaf]};
            }
            _ => ()
        }
    }

    pub fn over(&mut self, other: &mut Self) -> Self {
        self.propagate_bounds();
        other.propagate_bounds();
        let mut corners = vec![self.tl, self.br];
        corners.append(&mut vec![other.tl, other.br]);
        let (tl, br) = TermPos::bounding_box(corners);

        let mut tree = Self::new_leaf((br.y() - tl.y(), br.x() - tl.x()), Transparent, "from over".into(), true, tl);
        for pos in tl.range_to(br) {
            match (self.cell(pos), other.cell(pos)) {
                (c @ Opaque(_), _) => tree.set_cell(pos, c),
                (_, c) => tree.set_cell(pos, c)
            };
        };

        tree
    }
}

