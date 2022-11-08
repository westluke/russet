use crossterm::style::{self, Color};

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
