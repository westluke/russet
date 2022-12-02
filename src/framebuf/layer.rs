use std::ops::{Index, IndexMut};
use std::collections::{HashMap, HashSet, hash_map::Keys};
use std::iter::Copied;

use crate::deck::Card;
use crate::termchar::TermChar;
use crate::pos::TermPos;
use crate::util::*;

use log::{info};

use super::Grid;

use crossterm::style::Color;

pub use super::LayerCell;

#[derive(Clone, Debug)]
pub struct Layer {
    
    // so calling code can pull out specific layers
    card: Option<Card>,

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
    pub fn new(card: Option<Card>, height: i16, width: i16, anchor: TermPos, fill: LayerCell) -> Self {
        let mut dirtied = HashMap::new();

        if fill.is_some() {
            let (y, x): (i16, i16) = anchor.finto();
            for row in 0..height {
                dirtied.insert(row + y, HashSet::from_iter(x..(x+width)));
            };
        };

        Self {
            card,
            display: true,
            dirtied,
            panel: Grid::new(height.try_into().unwrap(), width.try_into().unwrap(), fill),
            anchor,
        }
    }

    pub fn get_anchor(&self) -> TermPos {
        self.anchor
    }

    pub fn dirty_opaq(&mut self) {
        let (tl, br) = TermPos::bounding_box(self.corners());
        for pos in tl.range_to(br) {
            let cel = self.get_c(pos);
            if cel.is_some() {
                self.dirtied
                    .entry(pos.y())
                    .or_insert_with(|| HashSet::new())
                    .insert(pos.x());
            };
        };
    }

    pub fn dirty_all(&mut self) {
        let (tl, br) = TermPos::bounding_box(self.corners());
        for y in tl.y()..br.y() {
            self.dirtied
                .entry(y)
                .or_insert(HashSet::new())
                .extend(tl.x()..br.x());
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

    pub fn get_dirty_lines(&self) -> Copied<Keys<i16, HashSet<i16>>> {
        self.dirtied.keys().copied()
    }

    pub fn get_c(&self, pos: TermPos) -> Option<LayerCell>{
        self.panel.get(pos - self.anchor)
    }

    // pos is relative to the top left of this layer.
    // Maybe there should also be an absolute-positioning version of this method?
    pub fn set_c(&mut self, pos: TermPos, cel: LayerCell) -> Result<()> {
        self.panel.set(pos - self.anchor, cel)?;
        self.dirtied
            .entry(pos.y())
            .or_insert(HashSet::new())
            .insert(pos.x());
        Ok(())
    }

    // Spaces are treated as OPAQUE in this function.
    // pos is relative to the top left of this layer.
    pub fn set_s(&mut self, mut pos: TermPos, s: String, fg: Color, bg: Color) -> Result<()> {
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
                self.set_c(pos, Some(TermChar::new(chars[i], fg, bg)))?;
                pos = pos + (0, 1).finto();
            };
        };

        Ok(())
    }

    // Spaces are treated as CLEAR in this function.
    pub fn set_s_clear(&mut self, mut pos: TermPos, s: String, fg: Color, bg: Color) -> Result<()> {
        let start_x = pos.x();
        let chars: Vec<char> = s.chars().collect();

        for i in 0..chars.len() {
            info!("char: {:?}", chars[i]);
            if chars[i] == '\n' {
                pos = pos + (1, 0).finto();
                pos = pos.set_x(start_x);
            } else if chars[i] == ' ' {

                // if space, no change to cell underneath
                pos = pos + (0, 1).finto();
            } else {
                self.set_c(pos, Some(TermChar::new(chars[i], fg, bg)))?;
                // self.set_c(pos, Some(TermChar::new('┏', fg, bg)))?;
                // self.set_c(pos, Some(TermChar::new('┗', fg, bg)))?;
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
    pub fn over(&self, other: &Self) -> Self {
        let mut corners = self.corners();
        corners.append(&mut other.corners());
        let (tl, br) = TermPos::bounding_box(corners);

        // result is just as wide as necessary to cover both input layers
        let mut lay = Self::new_by_bounds(None, tl, br, None);

        // For each position in this new layer...
        for pos in tl.range_to(br) {
            let ch = match (self.covers(pos), other.covers(pos)) {

                // if self is opaque at this position, use the self cell. Otherwise, use other.
                (true, true) =>
                    if let Some(x) = self.panel[pos] {
                        Some(x)
                    } else {
                        other.panel[pos]
                    },
                (true, false) => self.panel[pos],
                (false, true) => other.panel[pos],
                (false, false) => None
            };

            lay.set_c(pos, ch);
        };

        lay
    }

    pub fn beneath(&self, other: &Self) -> Self {
        other.over(self)
    }

    pub fn new_by_bounds(id: Option<Card>, tl: TermPos, br: TermPos, cel: Option<TermChar>) -> Self {
        Self::new(id, br.y() - tl.y(), br.x() - tl.x(), tl, cel)
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

// impl Index<TermPos> for Layer {
//     type Output = LayerCell;

//     fn index(&self, pos: TermPos) -> &Self::Output {
//         &self.panel[pos]
//     }
// }

// impl IndexMut<TermPos> for Layer {
//     fn index_mut(&mut self, pos: TermPos) -> &mut Self::Output {
//         &mut self.panel[pos]
//     }
// }

// impl Index<(i16, i16)> for Layer {
//     type Output = LayerCell;

//     fn index(&self, pos: (i16, i16)) -> &Self::Output {
//         let y = usize::try_from(pos.0).unwrap();
//         let x = usize::try_from(pos.1).unwrap();
//         &self.panel[(y, x)]
//     }
// }
