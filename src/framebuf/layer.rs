use std::ops::{Index, IndexMut};
use std::collections::{HashMap, HashSet};

use crate::deck::Card;
use crate::termchar::TermChar;
use crate::pos::TermPos;

use super::Grid;

use crossterm::style::Color;

// pub enum LayerType {
//     Card,
//     Outline,
//     Deck,
// }

use super::LayerCell;

#[derive(Clone, Debug)]
pub struct Layer {
    
    // so calling code can pull out specific layers
    card: Option<Card>,

    // for each line number (in the entire screen!) we store all column indices
    // that we've changed.
    dirtied: HashMap<i16, HashSet<i16>>,

    // a None indicates transparency - lower layers should be printed instead
    panel: Grid<LayerCell>,

    // location of top-left corner of this panel
    anchor: TermPos,
}

impl Default for Layer {
    fn default() -> Self {
        Self {
            card: None,
            dirtied: HashMap::new(),
            panel: Grid::default(),
            anchor: TermPos::try_from((0i16, 0i16)).unwrap().chk(),
        }
    }
}

impl Layer {
    pub fn new(card: Option<Card>, height: i16, width: i16, anchor: TermPos) -> Self {
        Self {
            card,
            dirtied: HashMap::new(),
            panel: Grid::new(height.try_into().unwrap(), width.try_into().unwrap(), (None, 0)),
            anchor,
        }
    }


    // make from_safe method for termpos? no new
    pub fn is_dirty(&self, pos: TermPos) -> bool {
        if let Some(x) = self.dirtied.get(pos.y()) {
            x.contains(pos.x())
        } else { false }
    }

    // pub fn get_dirtied(&self) -> &HashSet<i16> {
    // }

    pub fn get_c(&self, pos: TermPos) -> Option<LayerCell>{
        self.panel.get(pos)
    }

    // pub fn fill(&mut self, c: Option<TermChar>) {
    //     for y in 0..(self.height()) {
    //         for x in 0..(self.width()) {
    //             self.set_c(TermPos::new(y, x).chk(), c);
    //         }
    //     }
    // }

    // pub fn new_by_bounds(id: Option<Card>, tl: TermPos, br: TermPos) -> Self {
    //     Self::new(id, br.y() - tl.y(), br.x() - tl.x(), tl)
    // }

    // pub fn height(&self) -> i16 {
    //     i16::try_from(self.panel.height()).unwrap()
    // }

    // pub fn width(&self) -> i16 {
    //     i16::try_from(self.panel.width()).unwrap()
    // }

    // pub fn set_id(&mut self, id: Option<Card>) {
    //     self.id = id;
    // }

    // pub fn corners(&self) -> Vec<TermPos> {
    //     vec![
    //         self.anchor, 
    //         self.anchor + (self.height(), 0), 
    //         self.anchor + (0, self.width()),
    //         self.anchor + (self.height(), self.width())]
    // }

    // pub fn set_c(&mut self, tp: TermPos, tc: Option<TermChar>) {
    //     self.panel[tp] = tc;
    // }

    // pub fn set_anch(&mut self, tp: TermPos) {
    //     self.anchor = tp;
    // }
    
    // pub fn over(self, other: Self) -> Self {
    //     let mut corners = self.corners();
    //     corners.append(&mut other.corners());
    //     let (tl, br) = TermPos::bounding_box(&corners);
    //     let mut stains = Vec::new();

    //     let mut lay = Self::new_by_bounds(None, tl, br);

    //     for tp in tl.range_to(br) {
            
    //         let tc = match (self.contains(tp), other.contains(tp)) {
    //             (true, true) =>
    //                 if let Some(x) = self.panel[tp] {
    //                     Some(x)
    //                 } else {
    //                     other.panel[tp]
    //                 },
    //             (true, false) => self.panel[tp],
    //             (false, true) => other.panel[tp],
    //             (false, false) => None
    //         };

    //         stains.push(lay.set_c(tp, tc));
    //     };

    //     lay
    // }

    // pub fn beneath(self, other: Self) -> Self {
    //     other.over(self)
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
}

impl Index<TermPos> for Layer {
    type Output = LayerCell;

    fn index(&self, pos: TermPos) -> &Self::Output {
        &self.panel[pos]
    }
}

impl IndexMut<TermPos> for Layer {
    fn index_mut(&mut self, pos: TermPos) -> &mut Self::Output {
        &mut self.panel[pos]
    }
}

impl Index<(i16, i16)> for Layer {
    type Output = LayerCell;

    fn index(&self, pos: (i16, i16)) -> &Self::Output {
        let y = usize::try_from(pos.0).unwrap();
        let x = usize::try_from(pos.1).unwrap();
        &self.panel[(y, x)]
    }
}
