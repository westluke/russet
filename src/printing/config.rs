use termion::color::{self, Color};

pub const RAW_GREEN: &dyn Color = &color::Green;
pub const RAW_RED: &dyn Color = &color::Red;
pub const RAW_PURPLE: &dyn Color = &color::Blue;

pub const RAW_OVAL_SOLID: &str = include_str!("../../txt/solids/oval_solid.txt");
pub const RAW_DIAMOND_SOLID: &str = include_str!("../../txt/solids/diamond_solid.txt");
pub const RAW_SQUIGGLE_SOLID: &str = include_str!("../../txt/solids/squiggle_solid.txt");

pub const RAW_OUTLINE: &str = include_str!("../../txt/outline.txt");

pub const SHAPE_HEIGHT: u16 = 8;
pub const SHAPE_WIDTH: u16 = 8;
pub const SHAPE_SPACING: u16 = 1;

pub const CARD_HEIGHT: u16 = 10;
pub const CARD_WIDTH: u16 = 30;
pub const CARD_SPACING_VERT: u16 = 1;
pub const CARD_SPACING_HORIZ: u16 = 3;

#[cfg(not(feature = "blocky"))]
mod config_intern {
    pub const RAW_OVAL: &str = include_str!("../../txt/oval.txt");
    pub const RAW_DIAMOND: &str = include_str!("../../txt/diamond.txt");
    pub const RAW_SQUIGGLE: &str = include_str!("../../txt/squiggle.txt");
}

#[cfg(feature = "blocky")]
mod config_intern{
    use super::*;
    pub const RAW_OVAL: &str = RAW_OVAL_SOLID;
    pub const RAW_DIAMOND: &str = RAW_DIAMOND_SOLID;
    pub const RAW_SQUIGGLE: &str = RAW_SQUIGGLE_SOLID;
}

pub use config_intern::*;
