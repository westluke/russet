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
pub mod frametree;

use grid::Grid;
use frametree::FrameTree;
// use layer::{Layer, LayerGroup};

// pub type LayerCell = Option<TermChar>;

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

// this is stupid and wrong, it's not actually optimizing at all
pub struct LineUpdate {
    cs: Vec<LayerCell>
}

impl Default for LineUpdate {
    fn default() -> Self {
        Self { cs: Default::default() }
    }
}

impl LineUpdate {
    pub fn new(length: i16) -> Self {
        Self { cs: vec![Transparent; usize::try_from(length).unwrap()] }
    }

    pub fn set(&mut self, i: i16, c: LayerCell) {
        self.cs[usize::try_from(i).unwrap()] = c;
    }

    // Returns number of characters consumed to find start, Termable produced (if any)
    fn first_termable(&mut self) -> (i16, Option<Termable>) {
        let mut term = None;
        let mut cons = 0;

        for i in 0..self.cs.len() {
            let c_opt = self.cs[i];

            match (&mut term, c_opt) {
                (None, Transparent) => {
                    cons += 1;
                },
                (None, Opaque(c)) => {
                    term = Some(Termable::from(c));
                },
                (Some(_), Transparent) => {
                    self.cs.drain(0..i);
                    return (cons, term);
                },
                (Some(t), Opaque(c)) => {
                    if !t.push(c) {
                        self.cs.drain(0..i);
                        return (cons, term);
                    };
                }
            };
        };

        self.cs.drain(0..);
        (cons, term)
    }

    // Outputs a vector of pairs (i, cont) where cont is a StyledContent ready to be Print'd,
    // and i is the column where cont should be printed
    pub fn finalize(mut self) -> Vec<(i16, StyledContent<Termable>)> {
        let mut out = Vec::new();

        let (mut cons, mut term) = self.first_termable();
        let mut last = 0;

        while let Some(t) = term {
            let len = t.len();
            out.push((last + cons, t.finalize()));
            last += cons + len;
            (cons, term) = self.first_termable();
        };

        out
    }
}

// Termable is a TermChar sequence that can be printed as a single command.
pub enum Termable {
    Bg { n: usize, bg: Color }, // n is just the number of spaces to use
    Fg { s: String, fg: Color, bg: Color }
}

impl std::fmt::Display for Termable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Termable::Bg { n, .. } =>
                write!(f, "{}", " ".repeat(n)),
            Termable::Fg { ref s, .. } =>
                {info!("S{}E", s);
                write!(f, "{}", s)}
        }
    }
}

impl From<TermChar> for Termable {
    fn from(c: TermChar) -> Self {
        match c {
            TermChar::Bg { bg } =>
                Self::Bg { n: 1, bg },
            TermChar::Fg { c, bg, fg } =>
                Self::Fg { s: String::from(c), fg, bg }
        }
    }
}

impl Termable {

    pub fn len(&self) -> i16 {
        let i = match self {
            Self::Bg { n, .. } => *n,
            Self::Fg { s, .. } => s.chars().count()
        };
        i16::try_from(i).unwrap()
    }

    // None cells MUST BE HANDLED EXTERNALLY. This is for adding visible, printable characters,
    // NOT for handling transparency. Returns true iff tc was compatible and added successfully.
    // Therefore returns false iff tc will need a new Termable to be added to.
    pub fn push(&mut self, mut tc: TermChar) -> bool {
        // match tc {
        //     TermChar::Fg { ref mut c, .. } => {
        //         if *c == 'X' {
        //             *c = '┗';
        //         }
        //     },
        //     _ => ()
        // };

        match (self, tc) {
            (Termable::Fg { s, bg: bg0, .. }, TermChar::Bg { bg }) => {
                if *bg0 == bg {
                    s.push(' ');
                    true
                } else { false }
            },
            (Termable::Fg { s, fg: fg0, bg: bg0 }, TermChar::Fg { c, fg, bg }) => {
                if (*bg0 == bg) && (*fg0 == fg) {
                    s.push(c);
                    true
                } else { false }
            }
            (Termable::Bg { n, bg: bg0 }, TermChar::Bg { bg }) => {
                if *bg0 == bg {
                    *n += 1;
                    true
                } else { false }
            },
            (self_, TermChar::Fg { c, fg, bg }) => {
                let (n, bg0) = match self_.borrow() {
                    Termable::Bg { n, bg } => (*n, *bg),
                    _ => panic!("should not be able to reach this arm")

                };

                if bg0 == bg {
                    let mut s = " ".repeat(n);
                    s.push(c);
                    *self_ = Self::Fg { s, fg, bg };
                    true
                } else { false }
            },
        }
    }

    pub fn bg(&self) -> Color {
        match *self {
            Termable::Bg {bg, ..} => bg,
            Termable::Fg {bg, ..} => bg,
        }
    }

    pub fn fg(&self) -> Option<Color> {
        match *self {
            Termable::Bg {..} => None,
            Termable::Fg {fg, ..} => Some(fg),
        }
    }

    fn n(&self) -> usize {
        match *self {
            Termable::Bg { n, .. } => n,
            _ => panic!("this function should never be called on a non-Bg variant!!")
        }
    }

    pub fn finalize(self) -> StyledContent<Termable> {
        let mut style = ContentStyle::new();
        style.foreground_color = self.fg();
        style.background_color = Some(self.bg());
        StyledContent::new(style, self)
    }
}
