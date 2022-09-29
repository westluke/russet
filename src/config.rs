use crossterm::style::Color;
use std::sync::RwLock;
use crossterm::terminal;
use log::{info, warn, error};



pub struct TermSize {
    size: RwLock<(u16, u16)>
}

impl TermSize {
    pub const fn new () -> Self {
        Self{ size: RwLock::new((0, 0)) }
    }

    pub fn update(&self) -> (u16, u16) {
        let lock = self.size.write().unwrap();
        let new = terminal::size();

        // Switch order to be row-major
        if let Ok((x, y)) = new {
            *lock = (y, x);
        } else {
            warn!("Crossterm failed to determine terminal size!");
        };

        // if a size check fails, we just go with the last value
        *lock
    }

    pub fn get(&self) -> (u16, u16) {
        let lock = self.size.read().unwrap();
        return *lock;
    }
}

pub static ts: TermSize = TermSize::new();








pub const COLOR_1: Color = Color::Green;
pub const COLOR_2: Color = Color::Red;
pub const COLOR_3: Color = Color::Blue;

pub const CARD_BG: Color = Color::White;
pub const QUESTION_BG: Color = Color::White;
pub const PENDING_BG: Color = Color::Yellow;

pub const CARD_BORDER: Color = Color::White;
pub const SHADOW: Color = Color::White;
pub const GOOD_SET: Color = Color::Green;
pub const BAD_SET: Color = Color::Red;






pub const CARD_VERT_MARGIN: u16 = 2;
pub const CARD_HEIGHT: u16 = SHAPE_HEIGHT + CARD_VERT_MARGIN;
pub const CARD_WIDTH: u16 = (SHAPE_WIDTH * 3) + (4 * SHAPE_SPACING);
pub const CARD_SPACING_VERT: u16 = 2;
pub const CARD_SPACING_HORIZ: u16 = 2;


// NOTE: shape_width must be odd!! otherwise duos can't be centered properly, think about it
// not unless you use even spacing, and any spacing more than 1 looks weird
pub const RAW_OVAL: &str = include_str!("../txt/9x9/oval.txt");
pub const RAW_DIAMOND: &str = include_str!("../txt/9x9/diamond.txt");
pub const RAW_SQUIGGLE: &str = include_str!("../txt/9x9/squiggle.txt");
pub const RAW_QUESTION: &str = include_str!("../txt/9x9/question.txt");

pub const SHAPE_HEIGHT: u16 = 9;
pub const SHAPE_WIDTH: u16 = 9;
pub const SHAPE_SPACING: u16 = 1;

// pub const RAW_OVAL: &str = include_str!("../txt/7x7/oval.txt");
// pub const RAW_DIAMOND: &str = include_str!("../txt/7x7/diamond.txt");
// pub const RAW_SQUIGGLE: &str = include_str!("../txt/7x7/squiggle.txt");
// pub const RAW_QUESTION: &str = include_str!("../txt/7x7/question.txt");

// pub const SHAPE_HEIGHT: u16 = 7;
// pub const SHAPE_WIDTH: u16 = 7;
// pub const SHAPE_SPACING: u16 = 1;
