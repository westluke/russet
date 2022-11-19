use std::ops::{Index, IndexMut};

use crate::deck::Card;
use crate::termchar::TermChar;
use crate::pos::TermPos;

use super::stain::{Stain, StainSet};
use super::Grid;

use crossterm::style::Color;


#[derive(Clone, Debug)]
pub struct Layer {
    
    // so calling code can pull out specific layers
    card: Option<Card>,

    // 
    stamp: u32,

    // a None indicates transparency - lower layers should be printed instead
    panel: Grid<Option<TermChar>>,

    // location of top-left corner of this panel
    anchor: TermPos,

    // polled in animation thread just before write to term
    // for something like deletion, this is fine because deletion is done THROUGH Framebuf
    // although, should they be stored here, or in FrameBuf?
    // Doesn't seem possible to reasonably store them in framebuf, but user shouldn't be bothered
    // with these. So store them locally.
    // IF logic for changes gets too intsense, should be very easy to do some basic caching based
    // on ordering
    stains: StainSet
}

impl Default for Layer {
    fn default() -> Self {
        Self {
            id: None,
            panel: Grid::new(0, 0, None),
            anchor: TermPos::try_from((0i16, 0i16)).unwrap(),
            stains: StainSet::new()
        }
    }
}

impl Layer {
    pub fn new(id: Option<Card>, height: i16, width: i16, anchor: TermPos) -> Self {
        Self {
            id,
            panel: Grid::new(height.try_into().unwrap(), width.try_into().unwrap(), None),
            anchor,
            stains: StainSet::new()
        }
    }

    pub fn stained_rows(&self) -> std::collections::hash_map::Keys<i16, Stain> {
        self.stains.keys()
    }

    pub fn get_stain(&self, row: i16) -> Option<Stain> {
        self.stains.get_stain(row)
    }

    // pub fn get_c(&self, 

    // pub fn fill(&mut self, c: Option<TermChar>) {
    //     for y in 0..(self.height()) {
    //         for x in 0..(self.width()) {
    //             self.set_c(TermPos::new(y, x), c);
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
    type Output = Option<TermChar>;

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
    type Output = Option<TermChar>;

    fn index(&self, pos: (i16, i16)) -> &Self::Output {
        let y = usize::try_from(pos.0).unwrap();
        let x = usize::try_from(pos.1).unwrap();
        &self.panel[(y, x)]
    }
}
