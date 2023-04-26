use crate::term_char::*;
use crossterm::style::{Color, StyledContent, ContentStyle};
// use super::line_update::{LineUpdateBuilder, UpdateCell::{self, *}};
use log::info;
use std::borrow::Borrow;
use crate::util::{FInto, FFrom};

// a Termable represents a TermChar sequence that can be printed as a single command.

#[derive(Default, Clone, Debug)]
pub enum Termable {
    // n is just the number of spaces to use
    Bg { n: usize, bg: Color },
    Fg { s: String, fg: Color, bg: Color },
    #[default] Empty
}

impl std::fmt::Display for Termable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::Bg { n, .. } => write!(f, "{}", " ".repeat(n)),
            Self::Fg { ref s, .. } => write!(f, "{}", s),
            Self::Empty => Ok(())
        }
    }
}

impl Termable {
    pub fn new(c: TermChar) -> Self {
        match c {
            TermChar::Bg (bg) =>
                Self::Bg { n: 1, bg },
            TermChar::Fg { c, bg, fg } =>
                Self::Fg { s: String::from(c), fg, bg }
        }
    }

    // // Given a run of contiguous TermChars to be printed on a line, at starting point `start`,
    // // produce a vector of Styled Termables representing the same line.
    // pub fn extract(start: i16, run: Vec<TermChar>) -> Vec<(i16, StyledContent<Self>)> {
    //     let ret = Vec::new();
    //     let last_start = start;
    //     let mut last_term;

    //     if let Some(&first) = run.first() {
    //         last_term = Termable::new(first);
    //     } else {
    //         return ret;
    //     }

    //     for (i, &tc) in run.iter().enumerate().skip(1) {
    //         if !last_term.push(tc) {
    //             ret.push((last_start, last_term.finalize()));
    //             last_term = Termable::new(tc);
    //             last_start = start + i16::ffrom(i);
    //         }
    //     }

    //     ret.push((last_start, last_term.finalize()));
    //     ret
    // }

    pub fn len(&self) -> i16 {
        let i = match self {
            Self::Bg { n, .. } => *n,
            Self::Fg { s, .. } => s.chars().count(),
            Self::Empty => 0
        };
        i16::try_from(i).unwrap()
    }


    // None cells MUST BE HANDLED EXTERNALLY. This is for adding visible, printable characters,
    // NOT for handling transparency. Returns true iff tc was compatible and added successfully.
    // Therefore returns false iff tc will need a new Termable to be added to.
    pub fn push(mut self: &mut Self, tc: TermChar) -> bool {
        match (&mut self, tc) {
            (Termable::Fg { s, bg: bg0, .. }, TermChar::Bg(bg)) => {
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
            (Termable::Bg { n, bg: bg0 }, TermChar::Bg(bg)) => {
                if *bg0 == bg {
                    *n += 1;
                    true
                } else { false }
            },
            (Termable::Bg { n, bg: bg0 }, TermChar::Fg { c, fg, bg }) => {
                if *bg0 == bg {
                    let mut s = " ".repeat(*n);
                    s.push(c);
                    *self = Self::Fg { s, fg, bg };
                    true
                } else { false }
            },
            (Termable::Empty, tc) => {
                *self = Self::new(tc);
                true
            }
        }

        // *self = Self::new(Default::default());
        // true
    }

    pub fn finalize(self) -> StyledContent<Termable> {
        let mut style = ContentStyle::new();
        let (bg0, fg0);
        match self {
            Termable::Bg {bg, ..} => {bg0 = Some(bg); fg0 = None;},
            Termable::Fg {fg, bg, ..} => {bg0 = Some(bg); fg0 = Some(fg);},
            Termable::Empty => {bg0 = None; fg0 = None;}
        };
        style.foreground_color = fg0;
        style.background_color = bg0;
        StyledContent::new(style, self)
    }
}
