use crate::config::*;
use crate::{SetError as SE, SetErrorKind as SEK};


#[derive(PartialEq, Eq, Hash)]
#[derive(Copy, Clone, Debug)]
pub struct LayoutPos {
    row: u16,
    col: u16
}

impl LayoutPos {
    pub fn new(row: u16, col: u16) -> Result<Self, SE> {
        if row <= 2 && col <= 5 {
            Ok(Self{row, col})
        } else {
            Err(SE::new(SEK::LayoutOob, "layout positions must be somewhere in the 3-row 6-column layout"))
        }
    }

    pub fn row(&self) -> u16 { self.row }
    pub fn col(&self) -> u16 { self.col }
    pub fn set_row(&self, row: u16) -> Result<Self, SE> { Self::new(row, self.col) }
    pub fn set_col(&self, col: u16) -> Result<Self, SE> { Self::new(self.row, col) }

    pub fn to_TermPos(self) -> Result<TermPos, SE> {
        let y = (self.row * CARD_HEIGHT) + (self.row * CARD_SPACING_VERT) + 1;
        let x = (self.col * CARD_WIDTH)  + (self.col * CARD_SPACING_HORIZ) + 2;
        TermPos::new(y, x)
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
    pub fn new_dealt(row:u16, col:u16) -> Result<Self, SE> {
        Ok(Self::Dealt(LayoutPos::new(row, col)?))
    }

    pub fn to_TermPos(self) -> Result<TermPos, SE> {
        let (width, height) = termion::terminal_size().unwrap();
        let bottom = (height - CARD_HEIGHT) + 1;
        let right = width - CARD_WIDTH;

        match self {
            Self::Deck =>           TermPos::new(bottom - 1, 2),
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

#[derive(PartialEq, Eq, Hash)]
#[derive(Copy, Clone, Debug)]
pub struct TermPos {
    y: u16,
    x: u16
}

impl TermPos {
    pub fn new(y:u16, x:u16) -> Result<Self, SE> {
        let (width, height) = termion::terminal_size().expect("this program should be run from a compliant terminal");

        if 1 <= y && y <= height && 1 <= x && x <= width {
            Ok(Self{y, x})
        } else {
            Err(SE::new(
                SEK::SmallScreen,
                "you'll need to increase the size of your terminal window,\
                or decrease your font size, before the game can display properly"))
        }
    }

    pub fn set_x(&self, x:u16) -> Result<Self, SE> {
        Self::new(self.y, x)
    }

    pub fn set_y(&self, y:u16) -> Result<Self, SE> {
        Self::new(y, self.x)
    }

    pub fn x(&self) -> u16 {
        self.x
    }

    pub fn y(&self) -> u16 {
        self.y
    }

    pub fn add (&self, y: i32, x: i32) -> Result<Self, SE> {
        let new_y: i32 = self.y as i32 + y;
        let new_x: i32 = self.x as i32 + x;
        TermPos::new(new_y as u16, new_x as u16)
    }

    pub fn to_LayoutPos(&self) -> Option<LayoutPos> {
        // Let's say a card's "y-zone" extends from its top edge to the top edge of the card below
        // it. similarly for "x-zone", left edges.
        //
        // if we return a card C, that means this position was not only in the y-zone of card C, it
        // was in the upper portion (excluding the vertical space between cards).
        //
        // Also, since cards start right up against the top of the screen, these zones basically
        // "tile" the playing area. (just remember the coordinates are 1-based

        let y_tile = CARD_HEIGHT + CARD_SPACING_VERT;
        let x_tile = CARD_WIDTH + CARD_SPACING_HORIZ;

        if  ((self.y - 1) % y_tile > CARD_HEIGHT) || 
            ((self.x - 1) % x_tile > CARD_WIDTH) { return None; };

        if let Ok(pos) = LayoutPos::new(self.y / y_tile, self.x / x_tile) {
            return Some(pos);
        } else {
            return None;
        }
    }
}
