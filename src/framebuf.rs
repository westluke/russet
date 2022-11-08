use std::io::{Write};
use std::ops::{Index, IndexMut};
use std::cmp::{Ordering, min, max};

use crossterm::style::{self, Color, Colors, SetColors, Print};
use crossterm::{queue, cursor};

use crate::pos::*;
use crate::err::*;
use crate::deck::Card;
use crate::termchar::*;

use log::{info, warn, error};




#[derive(Clone, Debug)]
pub struct Grid<T: Copy> {
    grid: Vec<Vec<T>>,
    height: usize,
    width: usize
}

impl<T: Copy> Grid<T> {
    fn new(height: usize, width: usize, fill: T) -> Self {
        Self{ grid: vec![vec![fill; width]; height], height, width }
    }

    // pub fn iter(&self) -> Iterator

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    fn resize(&mut self, height: usize, width: usize, fill: T){
        for v in &mut self.grid {
            v.resize(width, fill)
        };

        self.grid.resize_with(height, || vec![fill; width]);
        self.height = height;
        self.width = width;
    }

    fn refill(&mut self, fill: T) {
        for i in 0..self.height {
            self.grid[i] = vec![fill; self.width]
        }
    }

    // An iterator over IMMUTABLE REFERENCES to the row vectors
    fn row_iter(&self) -> std::slice::Iter<'_, Vec<T>> {
        (&self.grid).into_iter()
    }
}

impl<T: Copy> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (y, x): (usize, usize)) -> &Self::Output {
        &self.grid[y][x]
    }
}

impl<T: Copy> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (y, x): (usize, usize)) -> &mut Self::Output {
        &mut self.grid[y][x]
    }
}

impl<T: Copy> Index<TermPos> for Grid<T> {
    type Output = T;

    fn index(&self, pos: TermPos) -> &Self::Output {
        &self.grid
            [usize::try_from(pos.y()).unwrap()]
            [usize::try_from(pos.x()).unwrap()]
    }
}

impl<T: Copy> IndexMut<TermPos> for Grid<T> {
    fn index_mut(&mut self, pos: TermPos) -> &mut Self::Output {
        &mut self.grid
            [usize::try_from(pos.y()).unwrap()]
            [usize::try_from(pos.x()).unwrap()]
    }
}




#[derive(Copy, Clone, Debug)]
struct ChangeFlag {
    start: TermPos,
    len: i16
}

impl ChangeFlag {
    pub fn new(start: TermPos, len: i16) -> Self {
        Self {start, len}
    }

    pub fn start(&self) -> TermPos {
        self.start
    }

    pub fn len(&self) -> i16 {
        self.len
    }

    fn cmp(&self, oth: &ChangeFlag) -> Ordering {
        let (y0, x0) = <(i16, i16)>::from(self.start);
        let (y1, x1) = <(i16, i16)>::from(oth.start);

        if y0 < y1 {
            Ordering::Less
        } else if y0 > y1 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    pub fn merge_with(self, oth: Self) -> Vec<Self> {
        let (p0, len0) = (self.start, self.len);
        let (p1, len1) = (oth.start, oth.len);
        let (p0y, p0x) = <(i16, i16)>::from(p0);
        let (p1y, p1x) = <(i16, i16)>::from(p1);

        if p0y != p1y {
            vec![self, oth]
        } else {
            let start = min(p0x, p1x);
            let end = max(p0y + len0, p1y + len1);
            vec![
                Self::new(
                    TermPos::try_from((p0y, start)).unwrap(),
                    end - start
                )
            ]
        }
    }

    pub fn merge_into(self, oths: &mut Vec<Self>) {
        // debug_assert!(oth.is_sorted);
        // debug_assert!(!oth.has_dups);

        let partr = |cf: &Self| cf.cmp(&self) == Ordering::Less;
        
        let i = oths.partition_point(partr);
        let mut to_add = self.merge_with(oths[i]);
        oths.remove(i);
        oths.append(&mut to_add);
    }
}

trait Mergeable {
    fn merge(self) -> Self;
}

impl Mergeable for Vec<ChangeFlag> {
    fn merge(mut self) -> Self {
        self.sort_by(|cf0, cf2| cf0.cmp(cf2));
        self.iter()
            .fold(
                Vec::new(),
                |mut acc, e|
                    {e.merge_into(&mut acc); acc}
            )
    }
}

#[derive(Clone, Debug)]
pub struct FrameBufLayer {
    
    // so calling code can pull out specific layers
    id: Option<Card>,

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
    flags: Vec<ChangeFlag>
}

impl Default for FrameBufLayer {
    fn default() -> Self {
        Self {
            id: None,
            panel: Grid::new(0, 0, None),
            anchor: TermPos::try_from((0i16, 0i16)).unwrap(),
            flags: Vec::new()
        }
    }
}

impl FrameBufLayer {
    pub fn new(id: Option<Card>, height: i16, width: i16, anchor: TermPos) -> Self {
        Self {
            id,
            panel: Grid::new(height.try_into().unwrap(), width.try_into().unwrap(), None),
            anchor,
            flags: Vec::new()
        }
    }

    pub fn set_all(&mut self, c: TermChar) {
        
    }

    pub fn new_by_bounds(id: Option<Card>, tl: TermPos, br: TermPos) -> Self {
        Self::new(id, br.y() - tl.y(), br.x() - tl.x(), tl)
    }

    pub fn height(&self) -> i16 {
        i16::try_from(self.panel.height()).unwrap()
    }

    pub fn width(&self) -> i16 {
        i16::try_from(self.panel.width()).unwrap()
    }

    pub fn set_id(&mut self, id: Option<Card>) {
        self.id = id;
    }

    pub fn corners(&self) -> Vec<TermPos> {
        vec![
            self.anchor, 
            self.anchor + (self.height(), 0), 
            self.anchor + (0, self.width()),
            self.anchor + (self.height(), self.width())]
    }

    pub fn set_c(&mut self, tp: TermPos, tc: Option<TermChar>) {
        self.panel[tp] = tc;
    }

    pub fn set_anch(&mut self, tp: TermPos) {
        self.anchor = tp;
    }
    
    pub fn over(self, other: Self) -> Self {
        let mut corners = self.corners();
        corners.append(&mut other.corners());
        let (tl, br) = TermPos::bounding_box(&corners);
        let mut flags = Vec::new();

        let mut lay = Self::new_by_bounds(None, tl, br);

        for tp in tl.range_to(br) {
            
            let tc = match (self.contains(tp), other.contains(tp)) {
                (true, true) =>
                    if let Some(x) = self.panel[tp] {
                        Some(x)
                    } else {
                        other.panel[tp]
                    },
                (true, false) => self.panel[tp],
                (false, true) => other.panel[tp],
                (false, false) => None
            };

            flags.push(lay.set_c(tp, tc));
        };

        lay
    }

    pub fn beneath(self, other: Self) -> Self {
        other.over(self)
    }

    pub(self) fn flags(&self) -> Vec<ChangeFlag> {
        let mut res = Vec::new();

        for i in 0..self.panel.height() {
            res.push((
                self.anchor + (i.try_into().unwrap(), 0_i16),
                i16::try_from(self.panel.width()).unwrap()
            ));
        }

        Vec::new()

        // merge_flagss(res)
    }

    pub fn contains(&self, p: TermPos) -> bool  {
        let (y, x) = <(i16, i16)>::from(self.anchor);
        let (py, px) = <(i16, i16)>::from(p);

        (y <= py) && (py < y + i16::try_from(self.panel.height()).unwrap()) &&
        (x <= px) && (px < x + i16::try_from(self.panel.width()).unwrap())
    }

    pub fn write(&mut self, start: TermPos, buf: &str, fg: Color, bg: Color) {
        let mut res = Vec::new();

        for (y, ln) in buf.lines().enumerate() {

            res.push((
                self.anchor + start + (y, 0),
                i16::try_from(ln.len()).unwrap()
            ));

            for (x, ch) in buf.chars().enumerate() {
                let np = start + (y, x);
                self.panel[np] = Some(TermChar::new(ch, fg, bg));
            }
        };
    }

    pub fn resize(&mut self, height: usize, width: usize) {
        self.panel.resize(height, width, Some(TermChar::default()));
    }
}

impl Index<TermPos> for FrameBufLayer {
    type Output = Option<TermChar>;

    fn index(&self, pos: TermPos) -> &Self::Output {
        &self.panel[pos]
    }
}

impl IndexMut<TermPos> for FrameBufLayer {
    fn index_mut(&mut self, pos: TermPos) -> &mut Self::Output {
        &mut self.panel[pos]
    }
}

pub struct FrameBuf<T: Write> {

    // The underlying Write object (should be a terminal)
    under: T,

    height: i16,
    width: i16,

    // Each change flag marks a line that must be updated, the column in that line
    // where the change starts, and the number of characters changed
    change_flags: Vec<ChangeFlag>,

    // Each layer is an independent "panel" that can be manipulated across the screen, i.e. a
    // playing card sliding around
    layers: Vec<FrameBufLayer>,
}

impl<T: Write> Index<Option<Card>> for FrameBuf<T> {
    type Output = FrameBufLayer;

    fn index(&self, id: Option<Card>) -> &Self::Output {
        self.layers.iter().find(|x| x.id == id).unwrap()
    }
}

impl<T: Write> IndexMut<Option<Card>> for FrameBuf<T> {
    fn index_mut(&mut self, id: Option<Card>) -> &mut Self::Output {
        self.layers.iter_mut().find(|x| x.id == id).unwrap()
    }
}

impl<T: Write> FrameBuf<T> {
    pub fn new(under: T, height: i16, width: i16) -> Self {
        Self {
            under,
            height, width,
            change_flags: Vec::new(),
            layers: Vec::new()
        }
    }

    pub fn push_layer(&mut self, lay: FrameBufLayer) {
        self.layers.push(lay);
        // self.change_flags.append(&mut lay.flags());
        // merge_flagss(self.change_flags);
    }

    fn char_at(&self, pos: TermPos) -> TermChar {
        for lay in self.layers.iter().rev() {
            if !lay.contains(pos) { continue; };
            if let Some(tc) = lay[pos] { return tc; }
        }
        TermChar::default()
    }

    fn queue_acc(&mut self, s: &str, foreground: Option<Color>, background: Option<Color>) {
        queue!(
            self.under,
            SetColors (
                Colors{ foreground, background }
            ),
            Print(s)
        );
    }

    pub fn flush(&mut self)  {
        // check the change flags, for each pix in flags, go through layers from top to bottom
        // then for that flag, construct sequence of commands to queue, based on color changes.
        for i in 0..self.change_flags.len() {

            let f = self.change_flags[i];
            let (y, x): (i16, i16) = f.start().into();
            debug_assert!(f.len() != 0);
            let (mut fg0, mut bg0) = (None, None);
            let mut acc = String::new();

            for i in x..(x+f.len()) {
                let tc = self.char_at(TermPos::new(y, i));
                let (fg, bg) = tc.get_fg_bg();

                if fg0.is_none() { fg0 = fg; };
                if bg0.is_none() { bg0 = Some(bg); };

                if  (fg0.is_some() && fg.is_some() && fg0 != fg) ||
                    (bg0.is_some() && bg0 != Some(bg)) {

                    self.queue_acc(&acc, fg0, bg0);
                    (fg0, bg0) = (None, None);
                    acc = String::new();

                } else {
                    acc.push(tc.get_c());
                }
            }

            if !acc.is_empty() {
                self.queue_acc(&acc, fg0, bg0);
            }
        }

        self.change_flags = Vec::new();
    }
}






// // implementing Write on SmartBuf is actually a bunch of work, for no clear purpose or gain.
