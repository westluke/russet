use termion::color::{self, Color};


// USE REFCELL INSTEAD

pub const RAW_GREEN: &dyn Color = &color::Green;
pub const RAW_RED: &dyn Color = &color::Red;
pub const RAW_PURPLE: &dyn Color = &color::Blue;

pub const CARD_BG: &dyn Color = &color::White;
pub const GOOD_SET: &dyn Color = &color::Green;
pub const BAD_SET: &dyn Color = &color::Red;

pub const TEXT_BG: &dyn Color = &color::Black;
pub const TEXT_FG: &dyn Color = &color::White;

// NOTE: shape_width must be odd!! otherwise duos can't be centered properly, think about it
// not unless you use even spacing, and any spacing more than 1 looks weird

pub const RAW_OVAL_SOLID: &str = include_str!("../txt/9x9/solids/oval.txt");
pub const RAW_DIAMOND_SOLID: &str = include_str!("../txt/9x9/solids/diamond.txt");
pub const RAW_SQUIGGLE_SOLID: &str = include_str!("../txt/9x9/solids/squiggle.txt");

pub const SHAPE_HEIGHT: u16 = 9;
pub const SHAPE_WIDTH: u16 = 9;
pub const SHAPE_SPACING: u16 = 1;

pub const CARD_HEIGHT: u16 = SHAPE_HEIGHT + 2;
pub const CARD_WIDTH: u16 = (SHAPE_WIDTH * 3) + (4 * SHAPE_SPACING);
pub const CARD_SPACING_VERT: u16 = 1;
pub const CARD_SPACING_HORIZ: u16 = 3;

#[cfg(not(feature = "blocky"))]
mod config_intern {
    use super::*;
    // pub const RAW_OVAL: &str = include_str!("../txt/oval.txt");
    // pub const RAW_DIAMOND: &str = include_str!("../txt/diamond.txt");
    // pub const RAW_SQUIGGLE: &str = include_str!("../txt/squiggle.txt");
    pub const RAW_OVAL: &str = RAW_OVAL_SOLID;
    pub const RAW_DIAMOND: &str = RAW_DIAMOND_SOLID;
    pub const RAW_SQUIGGLE: &str = RAW_SQUIGGLE_SOLID;
}

#[cfg(feature = "blocky")]
mod config_intern{
    use super::*;
    pub const RAW_OVAL: &str = RAW_OVAL_SOLID;
    pub const RAW_DIAMOND: &str = RAW_DIAMOND_SOLID;
    pub const RAW_SQUIGGLE: &str = RAW_SQUIGGLE_SOLID;
}

pub use config_intern::*;
