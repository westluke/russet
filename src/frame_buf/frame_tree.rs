use std::ops::{Index, IndexMut};
use std::collections::{HashMap, HashSet, hash_map::Keys};
use std::iter;
use iter::Copied;
use std::ops::BitOr;
use std::cmp::{min, max};
use std::fmt::{Display, Formatter};

use crate::deck::Card;
use crate::term_char::TermChar;
use crate::pos::TermPos;
use crate::util::*;
use crate::Id;

use log::{info};

use super::Grid;

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
    id: Id,
    active: bool,
    anchor: TermPos,
    dirt: HashMap<i16, HashSet<i16>>,
    zmark: i32
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

impl Default for FrameTree {
    fn default() -> Self {
        Self::new_leaf((1, 1), Transparent, "default".into(), true, (0, 0).finto(), 0)
    }
}

impl Default for FrameTreeKind {
    fn default() -> Self {
        Self::Leaf { frame: Grid::default() }
    }
}

impl Display for FrameTree {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        self._fmt(fmt, 0)
    }
}

impl FrameTree {

    // To make Display trait work properly.
    fn _fmt(&self, fmt: &mut Formatter<'_>, indent_level: usize) -> std::result::Result<(), std::fmt::Error> {
        match self.kind {
            FrameTreeKind::Leaf {..} => {
                write!(fmt, "{}", "    ".repeat(indent_level))?;
                write!(fmt, "Leaf (id= {}, active= {}, anchor= {})\n", self.id, self.active, self.anchor)?;
            }
            FrameTreeKind::Branch {ref children} => {
                write!(fmt, "{}", "    ".repeat(indent_level))?;
                write!(fmt, "Branch (id= {}, active= {}, anchor= {}) {{\n", self.id, self.active, self.anchor)?;
                for child in children {
                    child._fmt(fmt, indent_level + 1)?;
                };
                write!(fmt, "{}}}\n", "    ".repeat(indent_level))?;
            }
        }
        Ok(())
    }

    pub fn new_leaf(
        (height, width): (i16, i16),
        fill: LayerCell,
        id: Id,
        active: bool,
        anchor: TermPos,
        zmark: i32,
    ) -> Self {
        debug_assert!(height >= 1 && width >= 1);
        let mut res = Self {
            kind: FrameTreeKind::Leaf {
                frame: Grid::new(height.finto(), width.finto(), fill)
            },
            id,
            active,
            anchor,
            dirt: HashMap::new(),
            zmark
        };

        res.dirty_opaq();
        res
    }

    pub fn new_branch(
        children: Vec<FrameTree>,
        id: Id,
        active: bool,
        anchor: TermPos,
        zmark: i32,
    ) -> Self {
        let mut res = Self {
            kind: FrameTreeKind::Branch {
                children
            },
            id,
            active,
            anchor,
            dirt: HashMap::new(),
            zmark
        };

        res.dirty_opaq();
        res
    }

    pub fn set_dirty(&mut self, p: TermPos) {
        self.dirt
            .entry(p.y())
            .or_insert(HashSet::new())
            .insert(p.x());
    }

    pub fn dirty_opaq(&mut self) {
        match self.kind {
            FrameTreeKind::Leaf {..} => {
                let (tl, br) = self.bounds();
                // info!("bounds: {:?}", (tl, br));
                for p in tl.range_to(br) {
                    // info!("{:?}", p);
                    if let Some(Opaque(_)) = self._cell(p, true) { self.set_dirty(p + self.anchor); }
                }
            }
            FrameTreeKind::Branch {ref mut children} => {
                for child in children {
                    child.dirty_opaq();
                }
            }
        };
    }

    pub fn bounds(&self) -> (TermPos, TermPos) {
        match self.kind {
            FrameTreeKind::Leaf{ref frame} =>
                (self.anchor, self.anchor + (frame.height() - 1, frame.width() - 1).finto()),
            FrameTreeKind::Branch{ref children} => {
                let (mut top, mut left, mut bot, mut right) = 
                    (i16::MIN, i16::MIN, i16::MAX, i16::MAX);
                for (tl, br) in children.iter().map(|c| c.bounds()) {
                    let (t, l) = tl.finto();
                    let (b, r) = br.finto();
                    top = min(top, t);
                    left = min(left, l);
                    bot = max(bot, b);
                    right = max(right, r);
                };
                ((top, left).finto(), (bot, right).finto())
            }
        }
    }


    pub fn dirt(&self) -> HashMap<i16, HashSet<i16>> {
        match self.kind {
            FrameTreeKind::Leaf{..} => self.dirt.clone(),
            FrameTreeKind::Branch{ref children} => {
                let mut result = self.dirt.clone();

                for child in children {
                    for (k, v) in child.dirt() {
                        result.entry(k + self.anchor.y())
                            .or_insert(HashSet::new())
                            .extend(
                                v.iter().map(|col| col + self.anchor.x())
                            );
                    }
                };

                result
            }
        }
    }

    pub fn clean(&mut self) {
        match self.kind {
            FrameTreeKind::Leaf {..} => self.dirt.clear(),
            FrameTreeKind::Branch {ref mut children} =>
                for child in children {
                    child.clean()
                }
        }
    }

    // pub fn propagate_bounds(&mut self) {
    //     match self.kind {
    //         FrameTreeKind::Leaf{ref frame} => {
    //             self.tl = self.anchor;
    //             self.br = self.anchor + (frame.height(), frame.width()).finto();
    //         }
    //         FrameTreeKind::Branch{ref mut children} => {
    //             let (mut top, mut left, mut bot, mut right) = 
    //                 (i16::MIN, i16::MIN, i16::MAX, i16::MAX);
    //             for ref mut child in children {
    //                 child.propagate_bounds();
    //                 let (t, l) = child.tl.finto();
    //                 let (b, r) = child.br.finto();
    //                 top = min(top, t);
    //                 left = min(left, l);
    //                 bot = max(bot, b);
    //                 right = max(right, r);
    //             }
    //             self.tl = (top, left).finto();
    //             self.br = (bot, right).finto();
    //         }
    //     }
    // }
    //
    // pub fn propagate_dirt(&mut self) {
    //     match &mut self.kind {
    //         FrameTreeKind::Leaf {..} => (),
    //         FrameTreeKind::Branch {children, ..} => {
    //             for child in children {
    //                 child.propagate_dirt();
    //                 self.dirt.extend(std::mem::take(&mut child.dirt));
    //             };
    //         }
    //     };
    // }

    pub fn anchor(&self) -> TermPos {
        self.anchor
    }

    pub fn set_anchor(&mut self, anchor: TermPos) {
        self.dirty_opaq();
        self.anchor = anchor;
        self.dirty_opaq();
    }

    // pub fn dirt(&self) -> &HashMap<i16, HashSet<i16>> {
    //     &self.dirt
    // }
    //
    // pub fn clean(&mut self) {
    //     self.dirt.clear();
    // }

    pub fn id(&self) -> Id {
        self.id.clone()
    }

    pub fn set_id(&mut self, id: Id) {
        self.id = id;
    }

    pub fn activate(&mut self) {
        self.dirty_opaq();
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.dirty_opaq();
        self.active = false;
    }

    pub fn find(&self, id: &Id) -> Option<&Self> {
        match self.kind {
            FrameTreeKind::Leaf {..} => 
                if self.id == *id {
                    Some(self)
                } else { None }
            FrameTreeKind::Branch {ref children} => {
                for child in children {
                    let res = child.find(id);
                    if res.is_some() { return res } 
                }; 
                return None
            }
        }
    }

    pub fn find_mut(&mut self, id: &Id) -> Option<&mut Self> {
        if self.id == *id { return Some(self); };
        match self.kind {
            FrameTreeKind::Leaf {..} => None,
            FrameTreeKind::Branch {ref mut children} => {
                for child in children {
                    let res = child.find_mut(id);
                    if res.is_some() { return res } 
                }; 
                return None;
            }
        }
    }

    // TermPos component of return value represents the origin point of tree (includes
    // contribution from the anchor of the tree itself.)
    pub fn exact(&self, mut ids: Vec<Id>) -> Option<(TermPos, &Self)> {
        if let Some(s) = ids.pop() {
            if let FrameTreeKind::Branch {children} = &self.kind {
                for child in children.iter() {
                    if child.id == s {
                        if let Some((p, tree)) = child.exact(ids) {
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

    pub fn exact_mut(&mut self, mut ids: Vec<Id>) -> Option<(TermPos, &mut Self)> {
        if let Some(s) = ids.pop() {
            if let FrameTreeKind::Branch {children} = &mut self.kind {
                for child in children.iter_mut() {
                    if child.id == s {
                        if let Some((p, tree)) = child.exact_mut(ids) {
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


    // TermPos represents anchor of leaf's frame, bool is whether this leaf is active or not
    // (could be deactivated by ancestral inactive tree)
    pub fn leaves(&self) -> Vec<(TermPos, bool, &Self)> {
        match self.kind {
            FrameTreeKind::Leaf {..} => vec![(self.anchor, self.active, self)],
            FrameTreeKind::Branch {ref children, ..} => {
                children.iter()
                    .flat_map(|c| c.leaves())
                    .map(|(p, act, tree)| (p + self.anchor, act && self.active, tree))
                    .collect()
            }
        }
    }

    pub fn leaves_mut(&mut self) -> Vec<(TermPos, bool, &mut Self)> {
        match self.kind {
            FrameTreeKind::Leaf {..} => vec![(self.anchor, self.active, self)],
            FrameTreeKind::Branch {ref mut children, ..} => {
                children.iter_mut()
                    .flat_map(|c| c.leaves_mut())
                    .map(|(p, act, tree)| (p + self.anchor, act && self.active, tree))
                    .collect()
            }
        }
    }

    // pub fn is_dirty(&self, pos: TermPos) -> DirtyBit {
    //     match self.dirt().get(&pos.y()) {
    //         Some(set) => if set.contains(&pos.x()) { Dirty } else { Clean },
    //         None => Clean
    //     }
    // }

    // does the whole cell recovery algorithm
    // if dirt propagates with std::mem::take, then can't get dirty status of cells as we go, cuz
    // that requires per-leaf dirt. So maybe I should just accept that, yeah, I think that's the move.
    // pub fn cell(&self, pos: TermPos) -> LayerCell {
    //     for (off, act, leaf) in self.leaves() {
    //         match &leaf.kind {
    //             FrameTreeKind::Leaf {frame} =>
    //                 match frame.get(pos - off) {
    //                     Some(c @ Opaque(_)) => return c,
    //                     _ => ()
    //                 },
    //             _ => unreachable!()
    //         };
    //     };
    //     LayerCell::default()
    // }

    pub fn cell(&self, pos: TermPos) -> LayerCell {
        if let Some(c) = self._cell(pos, false) { c } else { LayerCell::bg() }
    }

    fn _cell(&self, pos: TermPos, ignore_active_status: bool) -> Option<LayerCell> {
        if !self.active && !ignore_active_status { return None; };
        match self.kind {
            FrameTreeKind::Leaf {ref frame} => 
                match frame.get(pos - self.anchor) {
                    None | Some(Transparent) => return None,
                    c => return c
                },
            FrameTreeKind::Branch {ref children} => {
                for child in children {
                    match child._cell(pos - self.anchor, ignore_active_status) {
                        None | Some(Transparent) => continue,
                        c => return c
                    };
                }
                return None;
            }
        }
    }

    pub fn collide(&self, pos: TermPos) -> Option<Vec<Id>> {
        if !self.active { return None; };

        let pos = pos - self.anchor;

        match self.kind {
            FrameTreeKind::Leaf {ref frame} =>
                if 0 <= pos.x()
                && pos.x() < frame.width().finto()
                && 0 <= pos.y()
                && pos.y() < frame.height().finto() {
                    return Some(vec![self.id.clone()]);
                } else {
                    return None;
                },
            FrameTreeKind::Branch {ref children} => {
                for child in children {
                    if let Some(mut v) = child.collide(pos) {
                        v.push(self.id.clone());
                        return Some(v);
                    };
                };
                return None;
            }
        };
    }

    // only operates on leaves, ignores anchor?
    // really, if this can panic like this, it should not even be provided as a method...
    pub fn set_cell(&mut self, pos: TermPos, cel: LayerCell) {
        match self.kind {
            FrameTreeKind::Leaf {ref mut frame} => {
                let res = frame.set(pos - self.anchor, cel).unwrap();
                // info!("pos: {:?}", pos);
                // info!("height: {:?}", frame.height());
                // info!("width: {:?}", frame.width());
                // info!("anchor: {:?}", self.anchor);
                // if let Err(x) = res {
                //     info!("ERROR ERROR ERROR ERROR");
                //     info!("err: {:?}", x);
                // }
                self.set_dirty(pos);
            },
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
                    // tl: self.tl,
                    // br: self.br,
                    dirt: HashMap::new(),
                    zmark: 0
                };
                self.kind = FrameTreeKind::Branch {children: vec![newleaf]};
            }
            _ => ()
        }
    }

    pub fn over(&mut self, other: &mut Self) -> Self {
        // self.propagate_bounds();
        // other.propagate_bounds();

        let (tl0, br0) = self.bounds();
        let (tl1, br1) = other.bounds();
        let corners = vec![tl0, br0, tl1, br1];
        let (tl, br) = TermPos::bounding_box(corners);


        let mut tree = Self::new_leaf(
            (1 + br.y() - tl.y(), 1 + br.x() - tl.x()),
            Transparent, "from over".into(), true, tl, 0
        );

        for pos in tl.range_to(br) {
            match (self.cell(pos), other.cell(pos)) {
                (c @ Opaque(_), _) => tree.set_cell(pos, c),
                (_, c) => tree.set_cell(pos, c)
            };
        };

        tree
    }
}

