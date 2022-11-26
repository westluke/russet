use std::io::{Write};
use std::collections::{HashMap, HashSet};
use std::ops::{Index, IndexMut};
use std::cmp::{Ordering, min, max};

use crossterm::style::{self, Color, ContentStyle, StyledContent, Colors, SetColors, Print};
use crossterm::{queue, cursor};

use crate::pos::*;
use crate::err::*;
use crate::deck::Card;
use crate::termchar::*;

use log::{info, warn, error};

mod grid;
mod stain;
mod layer;

use grid::Grid;
use layer::Layer;

pub type LayerCell = Option<TermChar>;

pub struct FrameBuf<T: Write> {
    height: i16,
    width: i16,

    // The underlying Write object (should be a terminal, probably stdout)
    under: T,

    // Each layer is an independent "panel" that can be manipulated across the screen, i.e. a
    // playing card sliding around
    layers: Vec<Layer>,
}

// impl<T: Write> Index<Option<Card>> for FrameBuf<T> {
//     type Output = Layer;

//     fn index(&self, id: Option<Card>) -> &Self::Output {
//         self.layers.iter().find(|x| x.id == id).unwrap()
//     }
// }

// impl<T: Write> IndexMut<Option<Card>> for FrameBuf<T> {
//     fn index_mut(&mut self, id: Option<Card>) -> &mut Self::Output {
//         self.layers.iter_mut().find(|x| x.id == id).unwrap()
//     }
// }

impl<T: Write> FrameBuf<T> {
    pub fn new(under: T, height: i16, width: i16) -> Self {
        Self {
            under,
            height, width,
            layers: Vec::new()
        }
    }

    // pub fn push_layer(&mut self, lay: Layer) {
    //     self.layers.push(lay);
    //     // self.change_flags.append(&mut lay.flags());
    //     // merge_flagss(self.change_flags);
    // }

    // fn char_at(&self, pos: TermPos) -> TermChar {
    //     for lay in self.layers.iter().rev() {
    //         if !lay.contains(pos) { continue; };
    //         if let Some(tc) = lay[pos] { return tc; }
    //     }
    //     TermChar::default()
    // }

    // fn queue_acc(&mut self, s: &str, foreground: Option<Color>, background: Option<Color>) {
    //     queue!(
    //         self.under,
    //         SetColors (
    //             Colors{ foreground, background }
    //         ),
    //         Print(s)
    //     );
    // }

    pub fn flush(&mut self)  {

        // can optimize by pre-fetching dirtied line numbers
        let mut dirty_lines = HashSet::new();

        for lay_i in 0..self.layers.len() {
            let keys: HashSet = lay_i.dirtu_lines().collect();
            dirty_lines.union(keys);
        }
        
        // for every dirty line...
        for row_i in dirty_lines {

            // start a new line update
            let mut lnup = LineUpdate::new(self.width);

            // for every cell in this line...
            for col_i in 0..self.width {

                // for every layer...
                for lay_i in 0..self.layers.len() {
                    let lay = self.layers.get(lay_i).unwrap();
                    let pos = TermPos::from((row_i, col_i)).chk();
                    let dirty = lay.is_dirty(pos);
                    let opaq = lay.get_c(pos);

                    match (dirty, opaq) {
                        // If we hit an opaque cell, we're done -- we won't see changes past this
                        (_, Some(c)) => {
                            lnup.set(col_i, c);
                            break;
                        },

                        // If we hit a newly transparent cell, we have to keep going,
                        // waiting for the next opaque cell. But in case we fall ALL the way
                        // through, we put in the default value (terminal background)
                        (true, None) => lnup.set(col_i, Default::default()),

                        // If we hit an old transparent cell, we just fall through
                        (true, Some(c)) => (),
                    };
                };
            };

            execute the lnup!
        };
    }
}

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
        Self { cs: vec![None; usize::try_from(length).unwrap()] }
    }

    pub fn set(&mut self, i: i16, c: LayerCell) {
        self.cs[usize::try_from(i).unwrap()] = c;
    }

    // Returns number of characters consumed to find start, Termable produced (if any)
    fn first_termable(&mut self) -> (i16, Option<Termable>) {
        let term = None;
        let mut cons = 0;

        for i in 0..self.cs.len() {
            let c_opt = self.cs[i];

            match (term, c_opt) {
                (None, None) => {
                    cons += 1;
                },
                (None, Some(c)) => {
                    term = Some(Termable::from(c));
                },
                (Some(_), None) => {
                    self.cs.drain(0..i);
                    return (cons, term);
                },
                (Some(ref mut t), Some(c)) => {
                    if !t.push(c) {
                        self.cs.drain(0..i);
                        return (cons, term);
                    };
                }
            };
        };

        (cons, term)
    }

    // Outputs a vector of pairs (i, cont) where cont is a StyledContent ready to be Print'd,
    // and i is the column where cont should be printed
    pub fn finalize(self) -> Vec<(i16, StyledContent<Termable>)> {
        let mut out = Vec::new();
        let mut col = 0i16;

        let (mut cons, mut term) = self.first_termable();
        let mut last = 0;

        while let Some(t) = term {
            out.push(last + cons, t.finalize());
            last += cons + t.len();
            cons, term = self.first_termable();

            
        }

        // Would love to refactor this...
        // extract first termable function?
        
        // For each cell in this line...
        for c_i in 0..self.cs.len() {

            // If there's something there...
            if let Some(c) = self.cs[c_i] {
                
                // And if there's a termable currently being built...
                if let Some(ref mut t) = term {

                    // then try pushing it onto the current termable, but if this requires
                    // splitting off a new termable...
                    if let Some(t2) = t.push(c) {

                        // then the old one is complete, we can push it and start the new one.
                        out.push((col, t.finalize()));
                        term = Some(t2);
                    };

                // if there's no current termable, start a new one
                } else {
                    col = i16::try_from(c_i).unwrap();
                    term = Some(Termable::from(c));
                };
            // if there's nothing in this cell and we have a termable in progress, push it
            } else if let Some(ref mut t) = term {
                out.push((col, t.finalize()));
            };
        };

        // If there was a termable in progress when we finished, push it
        if let Some(ref mut t) = term {
            out.push((col, t.finalize()));
        };

        out
    }
}

// What's the right abstraction here?
// we need to construct these sequences, but when/where do we break them?
// Really what we care about is printable sequences.
// Ok so, Termable is a TermChar sequence that can be printed as a single command.
// and then TermableSet will be a set of those
pub enum Termable {
    Bg { n: usize, bg: Color }, // n is just the number of spaces to use
    Fg { s: String, fg: Color, bg: Color }
}

impl std::fmt::Display for Termable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Termable::Bg { n, .. } =>
                write!(f, "{}", " ".repeat(n)),
            Termable::Fg { s, .. } =>
                write!(f, "{}", s),
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

    // None cells MUST BE HANDLED EXTERNALLY. This is for adding visible, printable characters,
    // NOT for handling transparency. Returns true iff tc was compatible and added successfully.
    // Therefore returns false iff tc will need a new Termable to be added to.
    pub fn push(&mut self, tc: TermChar) -> bool {
        match (*self, tc) {
            (Termable::Bg { ref mut n, bg: bg0 }, TermChar::Bg { bg }) => {
                if bg0 == bg {
                    *n += 1;
                    None
                } else { Some(Self::from(tc)) }
            }
            (Termable::Bg { n, bg: bg0 }, TermChar::Fg { c, fg, bg }) => {
                if bg0 == bg {
                    let mut s = " ".repeat(n);
                    s.push(c);
                    *self = Self::Fg { s, fg, bg };
                    None
                } else { Some(Self::from(tc)) }
            }
            (Termable::Fg { ref mut s, fg, bg: bg0 }, TermChar::Bg { bg }) => {
                if bg0 == bg {
                    s.push(' ');
                    None
                } else { Some(Self::from(tc)) }
            }
            (Termable::Fg { ref mut s, fg: fg0, bg: bg0 }, TermChar::Fg { c, fg, bg }) => {
                if (bg0 == bg) && (fg0 == fg) {
                    s.push(c);
                    None
                } else { Some(Self::from(tc)) }
            }
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

    pub fn finalize(self) -> StyledContent<Termable> {
        let mut style = ContentStyle::new();
        style.foreground_color = self.fg();
        style.background_color = Some(self.bg());
        StyledContent::new(style, self)
    }
}
