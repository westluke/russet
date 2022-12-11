use crate::term_char::*;
use crossterm::style::{Color, StyledContent, ContentStyle};
use log::info;
use std::borrow::Borrow;

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
                {
                // info!("S{}E", s);
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
