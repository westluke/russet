use crossterm::style::Color;

// NOTE: shape_width must be odd!! otherwise duos can't be centered properly, think about it
// not unless you use even spacing, and any spacing more than 1 looks weird

#[derive(Copy, Clone)]
#[allow(non_snake_case)]
pub struct Scale {
    pub SHAPE_HEIGHT: i16,
    pub SHAPE_WIDTH: i16,
    pub RAW_OVAL: &'static str,
    pub RAW_DIAMOND: &'static str,
    pub RAW_SQUIGGLE: &'static str,
    pub RAW_QUESTION: &'static str,

    // These two are computed. Also, CARD_HEIGHT describes height of card WITHOUT
    // offset outline
    pub CARD_HEIGHT: i16,
    pub CARD_WIDTH: i16
}

impl Scale {
    #[allow(non_snake_case)]
    pub const fn new(
                SHAPE_HEIGHT: i16,
                SHAPE_WIDTH: i16,
                RAW_OVAL: &'static str,
                RAW_DIAMOND: &'static str,
                RAW_SQUIGGLE: &'static str,
                RAW_QUESTION: &'static str) -> Self {
        
        Self {  SHAPE_HEIGHT,
                SHAPE_WIDTH,
                RAW_OVAL,
                RAW_DIAMOND,
                RAW_SQUIGGLE,
                RAW_QUESTION,
                CARD_HEIGHT: SHAPE_HEIGHT + CARD_INTERNAL_MARGIN_VERT * 2,
                CARD_WIDTH: (SHAPE_HEIGHT * 3) + (SHAPE_SPACING * 4)
        }
    }
}

pub const SIZE_9: Scale = Scale::new(
    9, 9,
    include_str!("../../txt/9x9/oval.txt"),
    include_str!("../../txt/9x9/diamond.txt"),
    include_str!("../../txt/9x9/squiggle.txt"),
    include_str!("../../txt/9x9/question.txt")
);

pub const SIZE_7: Scale = Scale::new(
    7, 7,
    include_str!("../../txt/7x7/oval.txt"),
    include_str!("../../txt/7x7/diamond.txt"),
    include_str!("../../txt/7x7/squiggle.txt"),
    include_str!("../../txt/7x7/question.txt")
);

pub const TERM_BG: Color = Color::Reset;

pub const MIN_HEIGHT: i16 = SIZE_7.CARD_HEIGHT * 4 + CARD_SPACING_VERT * 5;
pub const MIN_WIDTH: i16 = SIZE_7.CARD_WIDTH * 4 + CARD_SPACING_HORIZ * 5;

pub const COLOR_1: Color = Color::Green;
pub const COLOR_2: Color = Color::Red;
pub const COLOR_3: Color = Color::Blue;

pub const CARD_BG: Color = Color::White;
pub const QUESTION_BG: Color = Color::White;
pub const ACTIVE_BG: Color = Color::Yellow;

pub const CARD_BORDER: Color = Color::White;
pub const SHADOW: Color = Color::Black;
pub const GOOD_SET: Color = Color::Green;
pub const BAD_SET: Color = Color::Red;

pub const WIN_MARGIN_VERT: i16 = 1;
pub const WIN_MARGIN_HORIZ: i16 = 1;

pub const CARD_INTERNAL_MARGIN_VERT: i16 = 1;
pub const CARD_SPACING_VERT: i16 = 2;
pub const CARD_SPACING_HORIZ: i16 = 2;
pub const SHAPE_SPACING: i16 = 1;

pub const LAST_FOUND_OFFSET: i16 = 20;

pub const PIPE_BR: &'static str = "┛";
pub const PIPE_BL: &'static str = "┗";
pub const PIPE_TL: &'static str = "┏";
pub const PIPE_TR: &'static str = "┓";

pub const CARD_BR: &'static str = "\
┏┛
┛";
pub const CARD_BL: &'static str = "\
┗┓
 ┗";
// pub const CARD_TL: &'static str = "\
//  ┏
// ┏┛";

pub const CARD_TL: &'static str = "\
 X
XX";
// pub const CARD_TL: &'static str = "asdlf";
pub const CARD_TR: &'static str = "\
┓
┗┓";
