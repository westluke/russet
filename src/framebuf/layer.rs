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

pub struct Tree {
    tk: TreeKind,
    id: String,
    display: bool,
    anchor: TermPos
}

enum TreeKind {
    Leaf {
        panel: Grid<LayerCell>,
        dirtied: HashMap<i16, HashSet<i16>>,
    },

    Branch {
        children: Vec<Tree>,
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

impl TreeKind {
}

impl Tree {
    pub fn new_leaf((height, width): (i16, i16), fill: LayerCell, id: String, display: bool, anchor: TermPos) -> Self {
        debug_assert!(height >= 1 && width >= 1);
        let res = Self {
            tk: TreeKind::Leaf {
                panel: Grid::new(height.finto(), width.finto(), fill),
                dirtied: HashMap::new()
            },
            id,
            display,
            anchor
        };

        // res.dirty_opaq();
        res
    }

    pub fn leaves(&self) -> Vec<&Tree> {
        match self.tk {
            TreeKind::Leaf {..} => vec![self],
            TreeKind::Branch {children, ..} => {
                children.iter()
                    .flat_map(|c| c.leaves())
                    .collect()
            }
        }
    }

    pub fn leaves_mut(&mut self) -> Vec<&mut Tree> {
        match self.tk {
            TreeKind::Leaf {..} => vec![self],
            TreeKind::Branch {children, ..} => {
                children.iter()
                    .flat_map(|c| c.leaves_mut())
                    .collect()
            }
        }
    }

    pub fn new_branch(children: Vec<Tree>, id: String, display: bool, anchor: TermPos) -> Self {
        let res = Self {
            tk: TreeKind::Branch {
                children
            },
            id,
            display,
            anchor
        };

        // res.dirty_opaq();
        res
    }

    // pub fn set_cell(
    // pub fn get_cell

    // set_cell_rel
    // get_cell_rel
    
}

pub struct LayerGroup {
    id: String,
    layers: Vec<(Layer>
}

impl LayerGroup {
    pub fn new(id: String, layers: Vec<Layer>) -> Self {
        Self { id, layers }
    }

    pub fn get_id(&self) -> String {
        self.id
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn push_layer(&mut self, lay: Layer) {
        self.layers.insert(0, lay);
    }

    pub fn shup_layer(&mut self, lay: Layer) {
        self.layers.push(lay);
    }

    pub fn get_layer_mut(&mut self, id: String) -> Option<&mut Layer> {
        for lay in &mut self.layers {
            if lay.get_id() == id {
                return Some(lay);
            }
        }
        return None;
    }

    pub fn get_dirty_lines(&self) -> HashSet<i16> {
        let mut res = HashSet::new();
        for lay in self.layers {
            res = res.bitor(&lay.get_dirty_lines());
        }
        res
    }

    pub fn clean(&mut self) {
        for lay in self.layers {
            lay.clean();
        }
    }
    
    // should layergroups have anchors?? not yet

    // could optimize this further by caching results, checking dirty lines to see if cache
    // invalidated. But that's something for later - maybe with tree-structured layers.
    pub fn get_c(&self, pos: TermPos) -> (LayerCell, bool) {
        let mut res = LayerCell::default();
        let mut change = false;

        for lay_i in 0..self.layers.len() {
            let lay = self.layers.get(lay_i).unwrap();
            let cel = lay.get_c(pos);
            let dirt = lay.is_dirty(pos);
            change = change || dirt;

            match cel {
                // If we hit an opaque cell, we're done -- we won't see changes past this
                Opaque(_) => return (cel, change),
                _ => (),
            };
        };

        (res, change)
    }
}

#[derive(Clone, Debug)]
pub struct Layer {
    
    // so calling code can pull out specific layers
    id: String,

    // analogous to css display property: if false, this layer is invisible.
    display: bool,

    // for each line number (in the entire screen!) we store all column indices
    // that we've changed.
    dirtied: HashMap<i16, HashSet<i16>>,

    // a None indicates transparency - lower layers should be printed instead
    panel: Grid<LayerCell>,

    // location of top-left corner of this panel
    anchor: TermPos,
}

impl Layer {
    pub fn new(id: String, height: i16, width: i16, anchor: TermPos, fill: LayerCell) -> Self {
        debug_assert!(height >= 0 && width >= 0);
        let dirtied = HashMap::new();

        let mut res = Self {
            id,
            display: true,
            dirtied,
            panel: Grid::new(height.try_into().unwrap(), width.try_into().unwrap(), fill),
            anchor,
        };

        res.dirty_opaq();
        res
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }
    
    pub fn get_id(&mut self) -> String {
        self.id
    }

    pub fn activate(&mut self) {
        self.display = true;
        self.dirty_opaq();
    }

    pub fn deactivate(&mut self) {
        self.dirty_opaq();
        self.display = false;
    }

    pub fn get_anchor(&self) -> TermPos {
        self.anchor
    }

    pub fn dirty_opaq(&mut self) {
        let (tl, br) = TermPos::bounding_box(self.corners());
        for pos in tl.range_to(br).filter(|p| p.onscreen()) {
            let cel = self.get_c(pos);
            if cel.is_opaque() {
                self.dirtied
                    .entry(pos.y())
                    .or_insert_with(|| HashSet::new())
                    .insert(pos.x());
            };
        };
    }

    pub fn dirty_all(&mut self) {
        let (tl, br) = TermPos::bounding_box(self.corners());
        for y in (tl.y()..br.y()).filter(|y| *y >= 0) {
            self.dirtied
                .entry(y)
                .or_insert(HashSet::new())
                .extend(
                    (tl.x()..br.x())
                        .filter(|x| *x >= 0)
                );
        };
    }

    pub fn set_anchor(&mut self, anchor: TermPos) {
        self.dirty_opaq();
        self.anchor = anchor;
        self.dirty_opaq();
    }

    pub fn is_dirty(&self, pos: TermPos) -> bool {
        if let Some(x) = self.dirtied.get(&pos.y()) {
            x.contains(&pos.x())
        } else { false }
    }

    pub fn clean(&mut self) {
        self.dirtied.clear();
    }

    pub fn get_dirty_lines(&self) -> HashSet<i16> {
        self.dirtied.keys().copied().collect()
    }

    // All methods use ABSOLUTE positions unless otherwise stated
    pub fn get_c(&self, pos: TermPos) -> LayerCell {
        self.get_c_rel(pos - self.anchor)
    }

    pub fn get_c_rel(&self, pos: TermPos) -> LayerCell {
        if self.display {
            self.panel.get(pos).unwrap_or_default()
        } else {
            Transparent
        }
    }

    pub fn set_c(&mut self, pos: TermPos, cel: LayerCell) -> Result<()> {
        self.set_c_rel(pos - self.anchor, cel)
    }

    pub fn set_c_rel(&mut self, pos: TermPos, cel: LayerCell) -> Result<()> {
        self.panel.set(pos, cel)?;
        self.dirtied
            .entry(pos.y() + self.anchor.y())
            .or_insert(HashSet::new())
            .insert(pos.x() + self.anchor.x());
        Ok(())
    }

    // Spaces are treated as OPAQUE in this function.
    // Again, pos is absolute unless otherwise stated.
    pub fn set_s(&mut self, mut pos: TermPos, s: String, fg: Color, bg: Color) -> Result<()> {
        self.set_s_rel(pos - self.anchor, s, fg, bg)
    }

    pub fn set_s_rel(&mut self, mut pos: TermPos, s: String, fg: Color, bg: Color) -> Result<()> {
        let start_x = pos.x();
        let chars: Vec<char> = s.chars().collect();

        // for every character in the string...
        for i in 0..chars.len() {
            
            // if it's a newline, we jump down one step, and move back to our original column index
            if chars[i] == '\n' {
                pos = pos + (1, 0).finto();
                pos = pos.set_x(start_x);

            // otherwise, we set the cell to this character and advance one step to the right.
            } else {
                self.set_c_rel(pos, Opaque(TermChar::new(chars[i], fg, bg)))?;
                pos = pos + (0, 1).finto();
            };
        };

        Ok(())
    }

    pub fn set_s_clear(&mut self, mut pos: TermPos, s: String, fg: Color, bg: Color) -> Result<()> {
        self.set_s_clear_rel(pos - self.anchor, s, fg, bg)
    }

    // Spaces are treated as CLEAR in this function.
    pub fn set_s_clear_rel(&mut self, mut pos: TermPos, s: String, fg: Color, bg: Color) -> Result<()> {
        let start_x = pos.x();
        let chars: Vec<char> = s.chars().collect();

        for i in 0..chars.len() {
            if chars[i] == '\n' {
                pos = pos + (1, 0).finto();
                pos = pos.set_x(start_x);
            } else if chars[i] == ' ' {
                // if space, no change to cell underneath
                pos = pos + (0, 1).finto();
            } else {
                self.set_c_rel(pos, Opaque(TermChar::new(chars[i], fg, bg)))?;
                pos = pos + (0, 1).finto();
            };
        };

        Ok(())
    }

    // Does this panel cover location pnt?
    pub fn covers(&self, pnt: TermPos) -> bool {
        let (tl, br) = TermPos::bounding_box(self.corners());
        return 
            (tl.y() <= pnt.y() && tl.x() <= pnt.x()) &&
            (br.y() >= pnt.y() && br.x() >= pnt.x())
    }

    // Produce a new panel that is the result of overlaying self on top of other
    // its cuz im mixing up absolute and relative positioning again.... god damn.
    pub fn over(&self, other: &Self) -> Self {
        let mut corners = self.corners();
        corners.append(&mut other.corners());
        let (tl, br) = TermPos::bounding_box(corners);

        // result is just as wide as necessary to cover both input layers
        let mut lay = Self::new_by_bounds(String::new(), tl, br, Transparent);

        // For each position in this new layer...
        for pos in tl.range_to(br) {
            let ch: LayerCell = match (self.covers(pos), other.covers(pos)) {
                // if self is opaque at this position, use the self cell. Otherwise, use other.
                (true, true) =>
                    if let cel @ Opaque(_) = self.get_c(pos) {
                        cel
                    } else {
                        other.get_c(pos)
                    },
                (true, false) => self.get_c(pos),
                (false, true) => other.get_c(pos),
                (false, false) => Transparent
            };

            lay.set_c(pos, ch);
        };

        lay
    }

    pub fn beneath(&self, other: &Self) -> Self {
        other.over(self)
    }

    pub fn new_by_bounds(id: String, tl: TermPos, br: TermPos, cel: LayerCell) -> Self {
        Self::new(id, (br.y() - tl.y())+1, (br.x() - tl.x())+1, tl, cel)
    }

    pub fn height(&self) -> i16 {
        self.panel.height().finto()
    }

    pub fn width(&self) -> i16 {
        self.panel.width().finto()
    }

    // Returns the four corner-points of this layer (watch out for off-by-one errors,
    // these corners are INCLUSIVE cuz exclusivity doesn't make as much sense here.)
    pub fn corners(&self) -> Vec<TermPos> {
        vec![
            self.anchor, 
            self.anchor + (self.height()-1, 0).finto(),
            self.anchor + (0, self.width()-1).finto(),
            self.anchor + (self.height()-1, self.width()-1).finto()]
    }
}

    // pub fn set_c(&mut self, tp: TermPos, tc: Option<TermChar>) {
    //     self.panel[tp] = tc;
    // }

    // pub fn set_anch(&mut self, tp: TermPos) {
    //     self.anchor = tp;
    // }
    

    // pub(self) fn stains(&self) -> Vec<Stain> {
    //     let mut res = Vec::new();

    //     for i in 0..self.panel.height() {
    //         res.push((
    //             self.anchor + (i.try_into().unwrap(), 0_i16),
    //             i16::try_from(self.panel.width()).unwrap()
    //         ));
    //     }

    //     Vec::new()
    // }

    // pub fn contains(&self, p: TermPos) -> bool  {
    //     let (y, x) = <(i16, i16)>::from(self.anchor);
    //     let (py, px) = <(i16, i16)>::from(p);

    //     (y <= py) && (py < y + i16::try_from(self.panel.height()).unwrap()) &&
    //     (x <= px) && (px < x + i16::try_from(self.panel.width()).unwrap())
    // }

    // pub fn write(&mut self, start: TermPos, buf: &str, fg: Color, bg: Color) {
    //     let mut res = Vec::new();

    //     for (y, ln) in buf.lines().enumerate() {

    //         res.push((
    //             self.anchor + start + (y, 0),
    //             i16::try_from(ln.len()).unwrap()
    //         ));

    //         for (x, ch) in buf.chars().enumerate() {
    //             let np = start + (y, x);
    //             self.panel[np] = Some(TermChar::new(ch, fg, bg));
    //         }
    //     };
    // }

    // pub fn resize(&mut self, height: usize, width: usize) {
    //     self.panel.resize(height, width, Some(TermChar::default()));
    // }
