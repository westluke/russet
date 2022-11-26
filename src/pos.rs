use std::convert::From;
use std::ops::Add;
use std::iter;
use std::cmp::{min, max};

use crate::err::*;
use crate::config::*;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct LayoutPos {
    row: u8,
    col: u8
}

impl LayoutPos {
    pub fn new(row: u8, col: u8) -> Self {
        debug_assert!(
            row <= 2 && 
            col <= 5
        );

        Self{row, col}
    }

    pub fn row(self) -> u8 { self.row }
    pub fn col(self) -> u8 { self.col }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum GamePos {
    Deck,
    LastFound0,
    LastFound1,
    LastFound2,
    Dealt(LayoutPos)
}

impl GamePos {
    pub fn new_dealt(row:u8, col:u8) -> Self {
        Self::Dealt(LayoutPos::new(row, col))
    }
}

impl From<LayoutPos> for GamePos {
    fn from(pos: LayoutPos) -> Self {
        Self::Dealt(pos)
    }
}

impl<T> From <(T, T)> for TermPos where i16: TryFrom<T> {
    fn from((y, x): (T, T)) -> Self {
        let (y, x) = (i16::try_from(y).parwnu(), i16::try_from(x).parwnu());
        Self::new(0, 0)
    }
}

impl<T> From<TermPos> for (T, T) where T: TryFrom<i16> {
    fn from(pos: TermPos) -> Self {
        (T::try_from(pos.y()).parwnu(), T::try_from(pos.x()).parwnu())
    }
}

impl<T> Add<(T, T)> for TermPos where i16: TryFrom<T> {
    type Output = Self;

    fn add(self, (y, x): (T, T)) -> Self::Output {
        let (y, x) = (i16::try_from(y).parwnu(), i16::try_from(x).parwnu());
        Self::new(self.y + y, self.x + x)
    }
}

impl<T> Add<TermPos> for (T, T) where i16: From<T> {
    type Output = TermPos;

    fn add(self, tp: TermPos) -> Self::Output {
        let (y, x) = (i16::try_from(self.0).unwrap(), i16::try_from(self.1).unwrap());
        TermPos::new(y + tp.y(), x + tp.x())
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct TermPos {
    y: i16,
    x: i16
}

impl From<(&LayoutPos, &Scale)> for TermPos {
    fn from ((lp, s): (&LayoutPos, &Scale)) -> Self {
        let (row, col): (i16, i16) = (lp.row.into(), lp.col.into());
        let y = (row * s.CARD_HEIGHT) + (row * CARD_SPACING_VERT) + WIN_MARGIN_VERT;
        let x = (col * s.CARD_WIDTH)  + (col * CARD_SPACING_HORIZ) + WIN_MARGIN_HORIZ;
        TermPos::from((y, x))
    }
}

// shit. should the visual presentation be more tightly integrated with main?
// so i can more easily know when something has been clicked?
// yes, mutex
//
// also, should main.rs be doing more of thte terminal setup? No, cuz I think 
// the part done by animation is the part that gives you the handle?

impl From<(&GamePos, &Scale)> for TermPos {
    fn from((sp, s): (&GamePos, &Scale)) -> Self {
        let (width, height) = TS.update();

        // adjustments by 1 are for the border, which is always offset by 1.
        let bottom = height - s.CARD_HEIGHT - WIN_MARGIN_VERT - 1;
        let right = width - s.CARD_WIDTH - WIN_MARGIN_HORIZ;
        let left = WIN_MARGIN_HORIZ + 1;

        match sp {
            GamePos::Deck => TermPos::new(bottom - CARD_SPACING_VERT,  left),
            GamePos::LastFound0 => TermPos::new(bottom, right - LAST_FOUND_OFFSET * 2),
            GamePos::LastFound1 => TermPos::new(bottom - CARD_SPACING_VERT, right - LAST_FOUND_OFFSET),
            GamePos::LastFound2 => TermPos::new(bottom - CARD_SPACING_VERT * 2, right),
            GamePos::Dealt(pos) => TermPos::from((pos, s))
        }
    }
}

impl Add<TermPos> for TermPos {
    type Output = Self;

    fn add(self, t: TermPos) -> Self::Output {
        Self::new(self.y + t.y, self.x + t.x)
    }
}

impl TermPos {
    // fn pos_in_range(y: i16, x: i16, ymax: i16, xmax: i16) {
    //     debug_assert!(0 <= y && y < ymax && 0 <= x && x < xmax);
    // }

    // pub fn in_range(&self, ymin: i16, xmin: i16, ymax: i16, xmax: i16) {
    //     debug_assert!(
    //         ymin <= self.y &&
    //         self.y < ymax && 
    //         xmin <= self.x &&
    //         self.x < xmax
    //     );
    // }

    pub fn chk(self) -> Self {
        let (h, w) = TS.get();

        debug_assert!(
            0 <= self.y &&
            self.y < h &&
            0 <= self.x &&
            self.x < w
        );

        self
    }

    pub fn new(y:i16, x:i16) -> Self {
        let (ymax, xmax) = TS.get();
        // Self::pos_in_range(y, x, ymax, xmax);
        Self {y, x}
    }

    pub fn set_x(self, x:i16) -> Self {
        Self::new(self.y, x)
    }

    pub fn set_y(self, y:i16) -> Self {
        Self::new(y, self.x)
    }

    pub fn x(self) -> i16 {
        self.x
    }

    pub fn y(self) -> i16 {
        self.y
    }

    pub fn bounding_box(v: &Vec<Self>) -> (Self, Self) {
        let (mut min_y, mut min_x) = (i16::MIN, i16::MIN);
        let (mut max_y, mut max_x) = (i16::MAX, i16::MAX);

        for &tp in v {
            let (y, x): (i16, i16) = tp.into();
            min_y = min(y, min_x);
            max_y = max(y, max_y);
            min_x = min(x, min_x);
            max_x = max(x, max_x);
        }

        (Self::new(min_y, min_x), Self::new(max_y, max_x))
    }

    pub fn pos_range(topleft: Self, bottomright: Self) -> impl Iterator<Item=TermPos> {
        let mut pos = topleft;

        let clos = move || {
            pos = pos + (0i16, 1i16);
            if pos.x() > bottomright.x() {
                pos = TermPos::from((pos.y() + 1, topleft.x()));
            };

            if pos.y() > bottomright.y() {
                None  
            } else {
                Some(pos)
            }
        };

        iter::from_fn(clos).fuse()
    }

    pub fn range_to(self, bottomright: Self) -> impl Iterator<Item=TermPos> {
        Self::pos_range(*self, bottomright)
    }

    pub fn to_LayoutPos(self, s: Scale) -> Option<LayoutPos> {
        // Let's say a card's "y-zone" extends from its top edge to the top edge of the card below
        // it. similarly for "x-zone", left edges.
        //
        // if we return a card C, that means this position was not only in the y-zone of card C, it
        // was in the upper portion (excluding the vertical space between cards).
        //
        // Also, since cards start right up against the top of the screen, these zones basically
        // "tile" the playing area.

        let y_tile = s.CARD_HEIGHT + CARD_SPACING_VERT;
        let x_tile = s.CARD_WIDTH + CARD_SPACING_HORIZ;

        if  (self.y % y_tile > s.CARD_HEIGHT) || 
            (self.x % x_tile > s.CARD_WIDTH) { return None; };

        Some(
            LayoutPos::new(
                u8::try_from(self.y / y_tile).unwrap(),
                u8::try_from(self.x / x_tile).unwrap()
            )
        )
    }
}
