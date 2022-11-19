use crossterm::style::{self, Color, StyledContent, ContentStyle};
use std::fmt;

#[derive (Clone, Copy, Debug, PartialEq, Eq)]
pub enum TermChar {
    Bg {bg: Color},
    Fg {c: char, fg: Color, bg: Color}
}

impl Default for TermChar {
    fn default () -> Self {
        Self::Bg {bg: Color::Reset}
    }
}

impl TermChar {
    pub fn new(c: char, fg: Color, bg: Color) -> Self {
        if c == ' ' {
            Self::Bg{bg}
        } else {
            Self::Fg{c, fg, bg}
        }
    }

    pub fn solid(c: Color) -> Self {
        Self::Bg{bg: c}
    }

    pub fn get_fg_bg(&self) -> (Option<Color>, Color) {
        match self {
            Self::Bg{bg} => (None, *bg),
            Self::Fg{c:_, fg, bg} => (Some(*fg), *bg)
        }
    }

    pub fn get_c(&self) -> char {
        match self {
            Self::Bg{..} => ' ',
            Self::Fg{c, ..} => *c
        }
    }

    pub fn is_space(&self) -> bool {
        match self {
            Self::Bg{..} => true,
            Self::Fg{..} => false
        }
    }

    pub fn is_printable(&self) -> bool {
        match self {
            Self::Bg{..} => false,
            Self::Fg{..} => true
        }
    }

    fn style_cmd(&self) -> style::Colors {
        match self {
            Self::Bg{bg} => style::Colors{foreground: None, background: Some(*bg)},
            Self::Fg{fg, bg, ..} => style::Colors{foreground: Some(*fg), background: Some(*bg)}
        }
    }

    fn matches(&self, fg0: Color, bg0: Color) -> bool {
        match self {
            TermChar::Bg{bg} =>
                bg0 == *bg,
            TermChar::Fg{c:_, fg, bg} =>
                (fg0 == *fg) && (bg0 == *bg)
        }
    }
}

// What's the right abstraction here?
// we need to construct these sequences, but when/where do we break them?
// Really what we care about is printable sequences.
// Ok so, Termable is a TermChar sequence that can be printed as a single command.
// and then TermableSet will be a set of those
pub enum Termable {
    Empty,
    Bg { n: usize, bg: Color }, // n is just the number of spaces to use
    Fg { s: String, fg: Color, bg: Color }
}

impl fmt::Display for Termable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Termable::Empty =>
                Ok(()),
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
    pub fn new() -> Self {
        Self::Empty
    }
        
    // If c isn't compatible with this Termable, we return a new Termable initialized with c.
    pub fn push(&mut self, tc: TermChar) -> Option<Self>{
        match (*self, tc) {
            (Termable::Empty, _) => {
                *self = Self::from(tc);
                None
            }
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

    pub fn bg(&self) -> Option<Color> {
        match *self {
            Termable::Empty => None,
            Termable::Bg {bg, ..} => Some(bg),
            Termable::Fg {bg, ..} => Some(bg),
        }
    }

    pub fn fg(&self) -> Option<Color> {
        match *self {
            Termable::Empty => None,
            Termable::Bg {..} => None,
            Termable::Fg {fg, ..} => Some(fg),
        }
    }

    pub fn finalize(self) -> Option<StyledContent<Termable>> {
        if let Termable::Empty = self { return None; };
        let mut style = ContentStyle::new();
        style.foreground_color = self.fg();
        style.background_color = self.bg();
        Some(StyledContent::new(style, self))
    }
}

// pub struct TermableSet {
    
// }


// Dont use this, use StyledContent from Crossterm instead! jk, they can't modify content on the
// fly. So I gotta use my own.
