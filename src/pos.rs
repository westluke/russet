use std::convert::From;
use std::ops::Add;

use crate::config::*;
use crate::{SetError as SE, SetErrorKind as SEK, Result};

use crossterm::terminal;




#[derive(PartialEq, Eq, Hash)]
#[derive(Copy, Clone, Debug)]
pub struct LayoutPos {
    row: u16,
    col: u16
}

impl LayoutPos {
    pub fn new(row: u16, col: u16) -> Self {
        debug_assert!(row <= 2 && col <= 5);
        Self{row, col}
    }

    pub fn row(&self) -> u16 { self.row }
    pub fn col(&self) -> u16 { self.col }

    pub fn to_TermPos(self) -> TermPos {
        let y = (self.row * CARD_HEIGHT) + (self.row * CARD_SPACING_VERT) + 1;
        let x = (self.col * CARD_WIDTH)  + (self.col * CARD_SPACING_HORIZ) + 2;
        TermPos::from((y, x))
    }
}




#[derive(PartialEq, Eq, Hash)]
#[derive(Copy, Clone, Debug)]
pub enum SetPos {
    Deck,
    LastFound0,
    LastFound1,
    LastFound2,
    Dealt(LayoutPos)
}

impl SetPos {
    pub fn new_dealt(row:u16, col:u16) -> Self {
        Self::Dealt(LayoutPos::new(row, col))
    }

    pub fn to_TermPos(self) -> TermPos {
        let (width, height) = ts.update();
        let bottom = height - CARD_HEIGHT;
        let right = width - CARD_WIDTH;

        match self {
            Self::Deck =>           TermPos::new(bottom - 3, 2),
            Self::LastFound0 =>     TermPos::new(bottom - 1, right - 48),
            Self::LastFound1 =>     TermPos::new(bottom - 3, right - 24),
            Self::LastFound2 =>     TermPos::new(bottom - 5, right),
            Self::Dealt(pos)  =>    pos.to_TermPos()
        }
    }
}

impl From<LayoutPos> for SetPos {
    fn from(pos: LayoutPos) -> Self {
        Self::Dealt(pos)
    }
}




macro_rules! TermPos_from_type {
    ( $($x:ty)* ) => {
        $(
            impl From<($x, $x)> for TermPos {
                fn from((y, x): ($x, $x)) -> Self {
                    let (y, x) = (u16::try_from(y).unwrap(), u16::try_from(x).unwrap());
                    Self::new(y, x)
                }
            }
        )*
    };
}

macro_rules! type_from_TermPos {
    ( $($x:ty)* ) => {
        $(
            impl From<TermPos> for ($x, $x) {
                fn from(pos: TermPos) -> Self {
                    (<$x>::try_from(pos.y()).unwrap(), <$x>::try_from(pos.x()).unwrap())
                }
            }
        )*
    };
}

macro_rules! add_for_TermPos {
    ( $($x:ty)*) => {
        $(
            impl Add<($x, $x)> for TermPos {
                type Output = Self;

                fn add(self, (y, x): ($x, $x)) -> Self::Output {
                    let (y, x) = (u16::try_from(y).unwrap(), u16::try_from(x).unwrap());
                    Self::new(self.y + y, self.x + x)
                }
            }
        )*
    };
}

#[derive(PartialEq, Eq, Hash)]
#[derive(Copy, Clone, Debug)]
pub struct TermPos {
    y: u16,
    x: u16
}

TermPos_from_type!(u8 u16 u32 u64 usize i8 i16 i32 i64 isize);
type_from_TermPos!(u8 u16 u32 u64 usize i8 i16 i32 i64 isize);
add_for_TermPos!(u8 u16 u32 u64 usize i8 i16 i32 i64 isize);

impl Add<TermPos> for TermPos {
    type Output = Self;

    fn add(self, t: TermPos) -> Self::Output {
        Self::new(self.y + t.y, self.x + t.x)
    }
}

impl TermPos {
    fn in_range(y: u16, x: u16) {
        let (ymax, xmax) = ts.update();
        debug_assert!(0 <= y && y < ymax && 0 <= x && x < xmax);
    }

    pub fn new(y:u16, x:u16) -> Self {
        Self::in_range(y, x);
        Self {y, x}
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

    pub fn to_LayoutPos(&self) -> Option<LayoutPos> {
        // Let's say a card's "y-zone" extends from its top edge to the top edge of the card below
        // it. similarly for "x-zone", left edges.
        //
        // if we return a card C, that means this position was not only in the y-zone of card C, it
        // was in the upper portion (excluding the vertical space between cards).
        //
        // Also, since cards start right up against the top of the screen, these zones basically
        // "tile" the playing area.

        let y_tile = CARD_HEIGHT + CARD_SPACING_VERT;
        let x_tile = CARD_WIDTH + CARD_SPACING_HORIZ;

        if  (self.y % y_tile > CARD_HEIGHT) || 
            (self.x % x_tile > CARD_WIDTH) { return None; };

        Some(LayoutPos::new(self.y / y_tile, self.x / x_tile))
    }
}
