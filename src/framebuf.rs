use std::io::{Write};
use std::borrow::Borrow;
use std::collections::{HashSet};
use std::ops::BitOr;

use crossterm::style::{self, Color, ContentStyle, StyledContent, PrintStyledContent};
use crossterm::{queue, cursor};

use crate::pos::*;
use crate::termchar::*;

use log::{info, warn, error};

mod grid;
pub mod layer;

use grid::Grid;
use layer::Layer;

pub type LayerCell = Option<TermChar>;

pub struct FrameBuf<T: Write> {
    // FrameBuf doesn't have an independent width, it's just the width of the external terminal
    // object
    // height: i16,
    // width: i16,

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

    pub fn push_layer(&mut self, lay: Layer) {
        self.layers.push(lay);
        // self.change_flags.append(&mut lay.flags());
        // merge_flagss(self.change_flags);
    }

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
            let keys: HashSet<i16> = self.layers[lay_i].dirty_lines().collect();
            dirty_lines = dirty_lines.bitor(&keys);
        }

        info!("Starting flush!");
        
        // for every dirty line...
        for row_i in dirty_lines {
            info!("line {} is dirty", row_i);

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

                    // _ opaque -> done, paint with this cell
                    // newly transparent -> fall through, initialize with default background
                    // old transparent -> fall through, do NOT initialize, if we need to change then we'll know later on.

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
                        (false, None) => (),
                    };
                };
            };

            for (col_i, cont) in lnup.finalize() {
                queue!(
                    self.under, 
                    cursor::MoveTo(
                        u16::try_from(col_i).unwrap(),
                        u16::try_from(row_i).unwrap()),
                    PrintStyledContent(cont));
            };
        };

        // clear all layers
        for lay in &mut self.layers {
            lay.clean();
        }

        self.under.flush();
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
        let mut term = None;
        let mut cons = 0;

        for i in 0..self.cs.len() {
            let c_opt = self.cs[i];

            match (&mut term, c_opt) {
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
                (Some(t), Some(c)) => {
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
            info!("last: {}", last);
            info!("cons: {}", cons);
            info!("len: {}", len);
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

    pub fn len(&self) -> i16 {
        let i = match self {
            Self::Bg { n, .. } => *n,
            Self::Fg { s, .. } => s.len()
        };
        i16::try_from(i).unwrap()
    }

    // None cells MUST BE HANDLED EXTERNALLY. This is for adding visible, printable characters,
    // NOT for handling transparency. Returns true iff tc was compatible and added successfully.
    // Therefore returns false iff tc will need a new Termable to be added to.
    pub fn push(&mut self, tc: TermChar) -> bool {
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
