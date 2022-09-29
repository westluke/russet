use std::io::{Write};
use std::ops::{Index, IndexMut};
use std::collections::HashMap;
use std::cmp::{Ordering, min, max};

use crossterm::style::{self, Color, Stylize as _};
use crossterm::{queue, cursor};

use crate::pos::*;
use crate::Result;
use crate::game::Card;

use log::{info, warn, error};




type ChangeFlag = (TermPos, u16);

pub struct Grid<T: Copy> {
    grid: Vec<Vec<T>>,
    height: usize,
    width: usize
}

impl<T: Copy> Grid<T> {
    fn new(height: usize, width: usize, fill: T) -> Self {
        Self{ grid: vec![vec![fill; width]; height], height, width }
    }

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
        &self.grid[usize::from(pos.y())][usize::from(pos.x())]
    }
}

impl<T: Copy> IndexMut<TermPos> for Grid<T> {
    fn index_mut(&mut self, pos: TermPos) -> &mut Self::Output {
        &mut self.grid[usize::from(pos.y())][usize::from(pos.x())]
    }
}




#[derive (Clone, Copy, Debug, PartialEq, Eq)]
pub enum TermChar {
    Space       {bg: Color},
    Printable   {c: char, fg: Color, bg: Color}
}

impl Default for TermChar {
    fn default () -> Self {
        Self::Space {bg: Color::Reset}
    }
}

impl TermChar {
    pub fn new(c: char, fg: Color, bg: Color) -> Self {
        if c == ' ' {
            Self::Space{bg}
        } else {
            Self::Printable{c, fg, bg}
        }
    }

    pub fn get_fg_bg(&self) -> (Option<Color>, Color) {
        match self {
            Self::Space{bg} => (None, *bg),
            Self::Printable{c:_, fg, bg} => (Some(*fg), *bg)
        }
    }

    pub fn get_c(&self) -> char {
        match self {
            Self::Space{..} => ' ',
            Self::Printable{c, ..} => *c
        }
    }

    pub fn is_space(&self) -> bool {
        match self {
            Self::Space{..} => true,
            Self::Printable{..} => false
        }
    }

    pub fn is_printable(&self) -> bool {
        match self {
            Self::Space{..} => false,
            Self::Printable{..} => true
        }
    }

    fn style_cmd(&self) -> style::Colors {
        match self {
            Self::Space{bg} => style::Colors{foreground: None, background: Some(*bg)},
            Self::Printable{fg, bg, ..} => style::Colors{foreground: Some(*fg), background: Some(*bg)}
        }
    }

    fn matches(&self, fg0: Color, bg0: Color) -> bool {
        match self {
            TermChar::Space{bg} =>
                bg0 == *bg,
            TermChar::Printable{c:_, fg, bg} =>
                (fg0 == *fg) && (bg0 == *bg)
        }
    }
}


// Here's the thing, theres 3 stages:
// Figure out what needs to be printed / what's been updated
// Update the cache to represent what will be printed
//
// Ok: consider normal cases. How do we know whether a pixel needs to be updated or not? Have to go
// through list of layers, see if any of them are newer. But what about moved layers? Easy solution: moving or deleting a layer
// generates a temporary, one-iteration layer in its place.
//
// alternative: we go through list of layers, and if we find none, that means we print reset? Does
// that actually work? If no layers apply, print reset. Mmmm, yeah, that seems t owork actually.
//
// How do counters work with that?
// Also, ok, layers should be traversed in reverse order, we stop when we successfully print
// something. How do we know whether to print? Whether the layer was updated this cycle.
// How do we know that?
//
// ok but there's a tricky thing here. we might print even if it wasn't updated this cycle, because
// a layer on top disappeared, exposing a layer underneath. So tthat's not quite tthe right idea,
// unless deletting / moving a layer implicitly primes all overlapping layers underneath.
//
// What's the real underlying idea here. We update the cache when a change is made. That's the
// idea. Changes are made to layers. We must mark them for them to be accounted for? Except no,
// that's not quite right. Changes to layers may affect the pixels underneath the layer, but we
// don't necessarily know we can make the right update just based on the primed layer.
//
// Layer changes prime all CACHE PIXELS underneath that layer. that's it.
// And layers are always complete, they don't use counters? Yeah, I think so.
//
// This is separate step! ignore it for now
// make command sequence (based on changes to what was printed last) and print it
//
// remember that cache is not base state, it's just last printed.


pub struct FrameBufLayer {

    // a None indicates transparency - lower layers should be printed instead
    panel: Grid<(Option<TermChar>)>,

    // location of top-left corner of this panel
    anchor: TermPos,
}




fn merge_flag(flag0: ChangeFlag, flag1: ChangeFlag) -> Vec<ChangeFlag> {
    let (p0, len0) = flag0;
    let (p1, len1) = flag1;
    let (p0y, p0x) = <(u16, u16)>::from(p0);
    let (p1y, p1x) = <(u16, u16)>::from(p1);

    if p0y != p1y {
        vec![flag0, flag1]
    } else {
        let start = min(p0x, p1x);
        let end = max(p0y + len0, p1y + len1);
        vec![(TermPos::try_from((p0y, start)).unwrap(), end - start)]
    }
}

// Assumes flags is sorted by y_index, and has no duplicate y_indices
fn merge_flags(flag: ChangeFlag, flags: &mut Vec<ChangeFlag>) {
    let (p, len) = flag;
    let (py, px) = (p.y(), p.x());

    let partr = |&(p, len):&(TermPos, u16)| p.y() < py;
    
    let i = flags.partition_point(partr);
    let to_add = merge_flag(flag, flags[i]);
    flags.remove(i);
    flags.append(&mut to_add);
}

fn cmp_flag(&(p0, len0): &ChangeFlag, &(p1, len1): &ChangeFlag) -> Ordering {
    let (y0, x0) = <(u16, u16)>::from(p0);
    let (y1, x1) = <(u16, u16)>::from(p1);
    if y0 < y1 {
        Ordering::Less
    } else if y0 > y1 {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

fn merge_flagss(flags: Vec<ChangeFlag>) -> Vec<ChangeFlag> {
    flags.sort_by(cmp_flag);
    flags.iter().fold(Vec::new(), |acc, e| {merge_flags(*e, &mut acc); acc})
}

// write tests for these?




// All modifying operations return change flags, indicating which portions of the underlying cache
// should be marked for update.

impl FrameBufLayer {
    pub fn new(height: u16, width: u16) -> (Self, Vec<ChangeFlag>) {

        // Since all cells are none, this doesn't actually induce any change flags.
        // Also, I use unwrap since that could only fail if we weren't connected to a terminal, 
        // but if that were the case we would have failed already.
        
        (Self {
            panel: Grid::new(height.into(), width.into(), None),
            anchor: TermPos::try_from((0u16, 0u16)).unwrap()
        }, Vec::new())
    }

    fn contains(&self, p: TermPos) -> bool  {
        let (y, x) = <(u16, u16)>::from(self.anchor);
        let (py, px) = <(u16, u16)>::from(p);

        (y <= py) && (py < y + u16::try_from(self.panel.height()).unwrap()) &&
        (x <= px) && (px < x + u16::try_from(self.panel.width()).unwrap())
    }

    pub fn write(&mut self, start: TermPos, buf: &str, fg: Color, bg: Color) -> Result<Vec<ChangeFlag>> {
        let mut res = Vec::new();

        for (y, ln) in buf.lines().enumerate() {

            res.push((
                ((self.anchor + start)? + (y, 0))?,
                u16::try_from(ln.len())?
            ));

            for (x, ch) in buf.chars().enumerate() {
                let np = (start + (y, x))?;
                self.panel[np] = Some(TermChar::new(ch, fg, bg));
            }
        }

        Ok(res)
    }

    pub fn translate(&mut self, np: TermPos) -> Vec<ChangeFlag> {
        Vec::new()
    }

    pub fn resize(&mut self, height: usize, width: usize) -> Vec<ChangeFlag> {
        Vec::new()
    }
}






pub struct FrameBuf<T: Write> {

    // The underlying Write object (should be a terminal)
    under: T,

    // The characters that will be written to the screen.
    cache: Grid<TermChar>,

    // Each change flag marks a line that must be updated, the column in that line
    // where the change starts, and the number of characters changed
    change_flags: Vec<(TermPos, u16)>,

    // Each layer is an independent "panel" that can be manipulated across the screen, i.e. a
    // playing card sliding around
    layers: HashMap<Card, FrameBufLayer>,
}

impl<T: Write> FrameBuf<T> {
    pub fn new(under: T, height: u16, width: u16) -> Self {
        Self {
            under,
            cache: Grid::new(height.into(), width.into(), TermChar::default()),
            change_flags: Vec::new(),
            layers: HashMap::new()
        }
    }

    // pub fn push_layer(&mut self, lay: FrameBufLayer) {
    //     self.cache.push(lay);
    //     self.change_flags. add everything that willbe affected by this one!
    // }

    // pub fn 
}

//     pub fn write_dyn(&mut self, buf: &str, fg: Color, bg: Color, pos: TermPos) -> Result<()> {
//         for (i, ln) in buf.lines().enumerate() {
//             for (j, c) in ln.chars().enumerate() {
//                 let npos = (pos + (i, j))?;
//                 self.dynamic[npos] = (self.counter, TermChar::new(c, fg, bg));
//             };
//         };
//         Ok(())
//     }

//     pub fn write_stat(&mut self, buf: &str, fg: Color, bg: Color, pos: TermPos) -> Result<()> {
//         for (i, ln) in buf.lines().enumerate() {
//             for (j, c) in ln.chars().enumerate() {
//                 let npos = (pos + (i, j))?;
//                 self.staticc[npos] = (self.counter, TermChar::new(c, fg, bg));
//             };
//         };
//         Ok(())
//     }

//     fn char_at(&self, pos: TermPos) -> TermChar {
//         let (cnt_s, tc_s) = self.staticc[pos];
//         let (cnt_d, tc_d) = self.dynamic[pos];
//         if (self.counter <=) || (cnt < self.counter - 1) {
//             TermChar::default()
//         } else {
//             tc
//         }
//     }


//     fn norm(latest: u32, (cnt, tc): (u32, TermChar)) -> TermChar {
//         if cnt < latest {
//             TermChar::default()
//         } else {
//             tc
//         }
//     }

//     // cache is implicitly "cleared" after each display
//     // (actually done just by incrementing counter)
//     // still not right: if char is old, but new char is same as old char, do nothing.
//     // requires splitting string again
//     // still, this should be working
//     pub fn flush(mut self) -> Result<Self> {


//         // Ok, here's a question. what was the last character written here?
//         // Let's say it's blank, and has always been blank. Then it has an outdated count, and the character is blank.
//         // Let's say it's blank, because some previous iteration failed to put a character there.
//         // Then every subsequent grid will also have an outdated count
//         // Say it's a real character. then prev will have an up-to-date count
//         // ok here's the issue. the problem is not avoiding printing on all duplicated cells,
//         // the problem is avoiding printing on all duplicated cells AT THE EDGES. I strongly doubt
//         // that breaking the print into many smaller prints speeds anything up, and i would not be
//         // surprised if it makes things slower.
//         // So, how do I do that? can be done in a single pass. Record the start, and record the
//         // last seen new char. record all chars from start, and cut it down to end at last before
//         // printing. and I still need to intercept color changes. But actually that's much easier
//         // if I just intercept the commands and take their string values instead...
//         // I could also try printing the entire screen all at once... but then what's the point of
//         // all this?

//         if !self.write_flag { return Ok(()); };
//         // bruh what
        
//         let (mut fg0, mut bg0) = (Color::Reset, Color::Reset);
//         queue! (
//             self.under,
//             style::SetColors(style::Colors{foreground: fg0, background: bg0})
//         );
        
//         for i in 0..self.cache.height {
//             let mut s = String::new();
//             let mut s_start: u16 = 0;
//             let mut s_end: u16 = 0;
//             let mut init = true;

//             // Ugh I should still use the command api.
//             for j in 0..self.cache.width {
//                 let mut chr = Self::norm(self.counter, self.cache[(i, j)]);
//                 let mut pchr = Self::norm(self.counter, self.prev[(i, j)]); 

//                 let (fresh, matches) = (chr != pchr, chr.matches(fg0, bg0));

//                 match (true, true, true) {

//                     // FUCK this is complicated.
//                     // fresh char, so we gotta bump the end of the printed section of string
//                     (!init, fresh, matches) => s_end = u16::try_from(i)?,

//                     // we have to push it, since we might find another fresh one later
//                     (!init, !fresh, matches) => s.push(chr.get_c()),

//                     (!init, _, !matches) => {
//                             (fg0, bg0) = chr.get_fg_bg();
//                             queue!(
//                                 self.under,
//                                 cursor::MoveTo(s_start, i),
//                                 style::PrintStyledContent(s.with(fg_to_write).on(bg0))
//                                 style::SetColors(chr.style_cmd())
//                             );
//                         }
//                     }

//                     // not a fresh char, but we have to add it anyways, to avoid extra print calls
//                     // } (true, false) => {
//                         s.push(chr.get_c());

//                         if !chr.matches(fg0, bg0){
//                             (fg0, bg0) = chr.get_fg_bg();
//                             queue!(
//                                 self.under,
//                                 style::SetColors(chr.style_cmd())
//                             );
//                         }
//                     }

//                     (true, true) _ => continue
                    
                
//                 }


//                 if chr != pchr {

//                     // in case this is the last fresh char we see

//                     // printing must begin here
//                     if !init {
//                         s_start = u16::try_from(j)?;
//                         init = true;

//                     }
//                 }

//                 let (i, j) = (u16::try_from(i)?, u16::try_from(j)?);

//                 // No point in writing this one, skip it
//                 if prev_chr == chr {
//                     let fg_to_write = fg0.unwrap_or(Color::Reset);
//                     queue!( self.under,
//                             cursor::MoveTo(s_start, i),
//                             style::PrintStyledContent(s.with(fg_to_write).on(bg0))
//                     )?;

//                     s = String::new();
//                     s_start = j+1;
//                     fg = chr.fg;
//                     bg = chr.bg;
//                     // write the current string, set s_start forward
//                 } else if !matches(fg0, bg0, chr) {
//                     queue!( self.under,
//                             cursor::MoveTo(s_start, i),
//                             style::PrintStyledContent(s.with(fg).on(bg))
//                     )?;

//                     s = String::new();
//                     s.push(chr.c);
//                     s_start = j;
//                     fg = chr.fg;
//                     bg = chr.bg;
//                     // write the current string, start new one
//                 } else {
//                     s.push(chr.get_c());

//                     if let (None, TermChar::Printable{fg, ..}) = (fg0, chr) {
//                         fg0 = Some(fg);
//                     };
//                 };

//                 // This cell is old, so pretend its empty (write background)
//                 if cnt < self.counter {
//                     chr = TermChar{ c: ' ', fg: Color::Reset, bg: Color::Reset };
//                 } else if (chr.c == ' ') && (chr.fg != chr.bg) {
//                     chr.fg = chr.bg;    // to avoid unnecessary new strings
//                 };

//                 // same formatting as current string, so just extend it
//                 if (chr.fg == fg) && (chr.bg == bg) {
//                     s.push(chr.c);
//                 }

//                 // formatting has changed, so we'll need a separate write. so end the string here,
//                 // start a new one
//                 else {

//                 }
//             }

//             if !s.is_empty() {
//                 let i = u16::try_from(i)?;
//                 queue!( self.under,
//                         cursor::MoveTo(s_start, i),
//                         style::PrintStyledContent(s.with(fg).on(bg))
//                 )?;
//             }
//         }
        
//         self.under.flush()?;
//         self.counter += 1;

//         // temp takes ownership - low cost
//         let temp = self.cache;

//         // cache trades ownership - low cost
//         self.cache = self.prev;

//         // prev trades ownership - low cost
//         self.prev = temp;

//         // modified self on stack trades ownership back to caller, should be low cost
//         Ok(self)
//     }

//     pub fn resize(&mut self, height:u16, width:u16) {
//         debug_assert!(height >= 1 && width >= 1);
//         self.cache.resize(usize::from(height), usize::from(width), (self.counter, TermChar::default()));
//     }
// }

// // implementing Write on SmartBuf is actually a bunch of work, for no clear purpose or gain.
