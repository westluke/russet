use std::convert::From;
use std::ops::{Add, Sub};
use std::iter;
use std::cmp::{min, max};
use std::fmt::Debug;
use std::collections::HashSet;

use crate::util::{*, SetErrorKind as SEK, SetError as SE};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct DealtPos {
    row: u8,
    col: u8
}

impl DealtPos {
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
    Dealt(DealtPos)
}

impl GamePos {
    pub fn new_dealt(row:u8, col:u8) -> Self {
        Self::Dealt(DealtPos::new(row, col))
    }
}

impl From<DealtPos> for GamePos {
    fn from(pos: DealtPos) -> Self {
        Self::Dealt(pos)
    }
}


#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct TermPos {
    y: i16,
    x: i16
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
        let (h, w) = TS.dims();

        debug_assert!(
            0 <= self.y &&
            self.y < h &&
            0 <= self.x &&
            self.x < w
        );

        self
    }

    pub fn onscreen(self) -> bool {
        let (h, w) = TS.dims();

        0 <= self.y &&
        self.y < h &&
        0 <= self.x &&
        self.x < w
    }

    pub fn new(y:i16, x:i16) -> Self {
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

    // returns upper-leftmost point and lower-rightmost point in this vector
    pub fn bounding_box<T>(v: T) -> (Self, Self)
    where
        T: IntoIterator<Item=Self>
    {
        let (mut min_y, mut min_x) = (i16::MAX, i16::MAX);
        let (mut max_y, mut max_x) = (i16::MIN, i16::MIN);

        for tp in v.into_iter() {
            let (y, x): (i16, i16) = tp.finto();
            min_y = min(y, min_y);
            max_y = max(y, max_y);
            min_x = min(x, min_x);
            max_x = max(x, max_x);
        }

        (Self::new(min_y, min_x), Self::new(max_y, max_x))
    }

    // Visits every pos in the rectangle defined by self as top left, and bottomright, inclusive.
    fn pos_range(self, bottomright: Self) -> impl Iterator<Item=TermPos> {
        // log::info!("self: {:?}, br: {:?}", self, bottomright);
        let mut pos = self;

        // we'll pass this into from_fn, so it'll be called over and over to produce iterator.
        let clos = move || {

            // advance one step to the right.
            pos = pos + (0i16, 1i16).finto();

            // If we've passed the right edge...
            if pos.x() > bottomright.x() {

                // take one step downwards and return to the starting column.
                pos = (pos.y() + 1, self.x()).finto();

                // If, in doing so, we've passed the bottom edge, then we're done.
                if pos.y() > bottomright.y() {
                    None
                } else {
                    Some(pos)
                }
            } else {
                Some(pos)
            }
        };

        iter::from_fn(clos).fuse()
    }

    pub fn range_to(self, bottomright: Self) -> impl Iterator<Item=TermPos> {
        debug_assert!(self.x() <= bottomright.x() && self.y() <= bottomright.y());
        self.pos_range(bottomright)
    }

    // pub fn from_Pos(pos: &GamePos, scale: &Scale, active: bool) {
    // }

    // pub fn from_GamePos(pos: &GamePos, scale: &Scale, active: bool) {
    // }

}

impl<T> TryFrom <(T, T)> for TermPos
where
    i16: TryFrom<T>,
    <i16 as TryFrom<T>>::Error: Debug
{
    type Error = SE;
    fn try_from((y, x): (T, T)) -> Result<Self> {
        if let (Ok(y), Ok(x)) = (y.try_into(), x.try_into()) {
            Ok(Self::new(y, x))
        } else {
            Err(SE::new(SEK::Conversion, "conversion error!"))
        }
    }
}

impl<T> TryFrom<TermPos> for (T, T)
where
    T: TryFrom<i16>,
    <T as TryFrom<i16>>::Error: Debug
{
    type Error = SE;
    fn try_from(pos: TermPos) -> Result<Self> {
        if let (Ok(y), Ok(x)) = (pos.y().try_into(), pos.x().try_into()) {
            Ok((y, x))
        } else {
            Err(SE::new(SEK::Conversion, "conversion error!"))
        }
    }
}

// Again, returns correct positioning for INACTIVE card.
impl From<(&DealtPos, &Scale)> for TermPos {
    fn from ((lp, s): (&DealtPos, &Scale)) -> Self {
        let (row, col): (i16, i16) = (lp.row.into(), lp.col.into());
        let y = WIN_MARGIN_VERT + (row * (s.CARD_HEIGHT + 1 + CARD_SPACING_VERT));
        let x = WIN_MARGIN_HORIZ + (col * (s.CARD_WIDTH + CARD_SPACING_HORIZ));
        (y, x).finto()
    }
}

impl From<(&GamePos, &Scale)> for TermPos {
    fn from((sp, s): (&GamePos, &Scale)) -> Self {

        // these are accurate, i just messed up calculations I think
        let (height, width) = TS.update();

        // Ignoring outline for now:
        // placing a card at height makes it just outside the viewing window.
        // placing one at height - s.CARD_HEIGHT makes the entire main body just visible.
        // But that looks bad, so we enforce a minimum distance from the bottom, which is
        // WIN_MARGIN_VERT.
        let bottom = height - s.CARD_HEIGHT - WIN_MARGIN_VERT;
        let right = width - s.CARD_WIDTH - WIN_MARGIN_HORIZ;
        let left = WIN_MARGIN_HORIZ;

        let pos = match sp {
            // subtract card_spacing_vert so it aligns with lastfound1
            GamePos::Deck => TermPos::new(bottom - CARD_SPACING_VERT,  left),
            GamePos::LastFound0 => TermPos::new(bottom, right - LAST_FOUND_OFFSET * 2),
            GamePos::LastFound1 => TermPos::new(bottom - CARD_SPACING_VERT, right - LAST_FOUND_OFFSET),
            GamePos::LastFound2 => TermPos::new(bottom - CARD_SPACING_VERT * 2, right),

            // to allow for correction below (stupid, i know)
            GamePos::Dealt(pos) => TermPos::from((pos, s)) + (1, 0).finto()
        };

        // to make this work for an inactive card, have to shift up.
        // Justification: consider a card positioned at pos, flush with the left and bottom 
        // borders (so, as if we didn't have window margins). Once border is turned on, anchor
        // point shifts relative to card body - anchor point is now one cell to left of top-left
        // card point. So border does not extend offscreen on left. But it DOES extend one cell
        // offscreen on bottom. So must shift up.
        //
        // If you're trying to place an active card instead, take the result of this function and
        // shift down by 1 (add (1, 0))
        pos + (-1, 0).finto()
    }
}

impl Add<TermPos> for TermPos {
    type Output = Self;

    fn add(self, t: TermPos) -> Self::Output {
        Self::new(self.y + t.y, self.x + t.x)
    }
}

impl Sub<TermPos> for TermPos {
    type Output = Self;

    fn sub(self, t: TermPos) -> Self::Output {
        Self::new(self.y - t.y, self.x - t.x)
    }
}
