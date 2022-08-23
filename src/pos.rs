use crate::config::*;

use std::ops::Add;

pub enum SetPos {
    Deck,
    FoundSet,
    Dealt{ row:u16, col:u16 }
}

impl SetPos {
    pub fn new(row:u16, col:u16) -> Self {
        debug_assert!(0 <= row && row <= 2);
        debug_assert!(0 <= col && col <= 5);
        Self::Dealt{row, col}
    }

    pub fn to_TermPos(self) -> TermPos {
        match self {
            Self::Deck =>               TermPos::new(1, 1),
            Self::FoundSet =>           TermPos::new(1, 1),
            Self::Dealt{row, col}  =>   {
                let y = (row * CARD_HEIGHT) + (row * CARD_SPACING_VERT);
                let x = (col * CARD_WIDTH)  + (col * CARD_SPACING_HORIZ);
                TermPos::new(y, x)
            }
        }
    }
}

pub struct TermPos {
    y: u16,
    x: u16
}

impl TermPos {
    pub fn new(y:u16, x:u16) -> Self {
        let (width, height) = termion::terminal_size().unwrap();
        debug_assert!(1 <= y && y <= height);
        debug_assert!(0 <= x && x <= width);
        Self{y, x}
    }

    pub fn set_x(&self, x:u16) -> Self {
        Self::new(self.y, x)
    }

    pub fn set_y(&self, y:u16) -> Self {
        Self::new(y, self.x)
    }

    pub fn x(&self) -> u16 {
        self.x
    }

    pub fn y(&self) -> u16 {
        self.y
    }
}

impl Add<(u16, u16)> for TermPos {
    type Output = Self;
    fn add(self, (y, x):(u16, u16)) -> Self{
        TermPos::new(self.y + y, self.x + x)
    }
}
