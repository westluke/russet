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

