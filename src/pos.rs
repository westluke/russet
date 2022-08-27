use crate::config::*;

use std::ops::Add;


#[derive(PartialEq, Eq, Hash)]
pub enum SetPos {
    Deck,
    LastFound0,
    LastFound1,
    LastFound2,
    Dealt{ row:u16, col:u16 }
}

impl SetPos {
    pub fn new_dealt(row:u16, col:u16) -> Self {
        debug_assert!(0 <= row && row <= 2);
        debug_assert!(0 <= col && col <= 5);
        Self::Dealt{row, col}
    }

    pub fn to_TermPos(self) -> TermPos {
        let (width, height) = termion::terminal_size().unwrap();
        let bottom = height - CARD_HEIGHT;
        let right = width - CARD_WIDTH;

        match self {
            Self::Deck =>               TermPos::new(bottom, 1),
            Self::LastFound0 =>         TermPos::new(bottom, right),
            Self::LastFound1 =>         TermPos::new(bottom - 3, right - 3),
            Self::LastFound2 =>         TermPos::new(bottom - 6, right - 6),
            Self::Dealt{row, col}  =>   {
                let y = (row * CARD_HEIGHT) + (row * CARD_SPACING_VERT);
                let x = (col * CARD_WIDTH)  + (col * CARD_SPACING_HORIZ);
                TermPos::new(y, x)
            }
        }
    }

    pub fn row(&self) -> u16 {
        match self {
            &Self::Dealt{row, col} => return row,
            _ => panic!()
        };
    } 

    pub fn col(&self) -> u16 {
        match self {
            &Self::Dealt{row, col} => return col,
            _ => panic!()
        };
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

impl Add<TermPos> for TermPos {
    type Output = Self;
    fn add(self, pos:TermPos) -> Self {
        TermPos::new(self.y + pos.y, self.x + pos.x)
    }
}

