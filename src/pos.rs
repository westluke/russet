use crate::config::*;

use std::ops::Add;


#[derive(PartialEq, Eq, Hash)]
#[derive(Copy, Clone, Debug)]
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
        let bottom = (height - CARD_HEIGHT) + 1;
        let right = width - CARD_WIDTH;

        match self {
            Self::Deck =>               TermPos::new(bottom-1, 2),
            Self::LastFound0 =>         TermPos::new(bottom - 1, right - 48),
            Self::LastFound1 =>         TermPos::new(bottom - 3, right - 24),
            Self::LastFound2 =>         TermPos::new(bottom - 5, right),
            Self::Dealt{row, col}  =>   {
                let y = (row * CARD_HEIGHT) + (row * CARD_SPACING_VERT) + 1;
                let x = (col * CARD_WIDTH)  + (col * CARD_SPACING_HORIZ) + 2;
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

#[derive(PartialEq, Eq, Hash)]
#[derive(Copy, Clone, Debug)]
pub struct TermPos {
    y: u16,
    x: u16
}

impl TermPos {
    pub fn new(y:u16, x:u16) -> Self {
        let (width, height) = termion::terminal_size().unwrap();
        debug_assert!(1 <= y && y <= height);
        debug_assert!(1 <= x && x <= width);
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
    fn add(self, (y, x):(u16, u16)) -> Self {
        TermPos::new(self.y + y, self.x + x)
    }
}

impl Add<(i32, i32)> for TermPos {
    type Output = Self;
    fn add(self, (y, x):(i32, i32)) -> Self {
        let new_y = i32::from(self.y) + y;
        let new_x = i32::from(self.x) + x;
        debug_assert!(new_y > 0 && new_x > 0);
        TermPos::new(new_y as u16, new_x as u16)
    }
}

impl Add<TermPos> for TermPos {
    type Output = Self;
    fn add(self, pos:TermPos) -> Self {
        TermPos::new(self.y + pos.y, self.x + pos.x)
    }
}

