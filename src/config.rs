// USE REFCELL INSTEAD

// pub const RAW_GREEN: &dyn Color = &color::Green;
// pub const RAW_RED: &dyn Color = &color::Red;
// pub const RAW_PURPLE: &dyn Color = &color::Blue;

// pub const CARD_BG: &dyn Color = &color::White;
// pub const CARD_BORDER: &dyn Color = &color::Reset;
// pub const QUESTION_BG: &dyn Color = &color::Reset;
// pub const PENDING_BG: &dyn Color = &color::LightYellow;
// pub const SHADOW_BG: &dyn Color = &color::Reset;
// pub const GOOD_SET: &dyn Color = &color::Green;
// pub const BAD_SET: &dyn Color = &color::Red;

// pub const TEXT_BG: &dyn Color = &color::Black;
// pub const TEXT_FG: &dyn Color = &color::White;

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





pub const CARD_HEIGHT: u16 = SHAPE_HEIGHT + 4;
pub const CARD_WIDTH: u16 = (SHAPE_WIDTH * 3) + (4 * SHAPE_SPACING);
pub const CARD_SPACING_VERT: u16 = 2;
pub const CARD_SPACING_HORIZ: u16 = 2;
