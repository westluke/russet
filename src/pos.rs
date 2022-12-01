use std::convert::From;
use std::ops::{Add, Sub};
use std::iter;
use std::cmp::{min, max};
use std::fmt::Debug;

use crate::util::{*, SetErrorKind as SEK, SetError as SE};

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

    // pub fn bounding_box(v: &Vec<Self>) -> (Self, Self) {
    //     let (mut min_y, mut min_x) = (i16::MIN, i16::MIN);
    //     let (mut max_y, mut max_x) = (i16::MAX, i16::MAX);

    //     for &tp in v {
    //         let (y, x): (i16, i16) = tp.into();
    //         min_y = min(y, min_x);
    //         max_y = max(y, max_y);
    //         min_x = min(x, min_x);
    //         max_x = max(x, max_x);
    //     }

    //     (Self::new(min_y, min_x), Self::new(max_y, max_x))
    // }

    // pub fn pos_range(topleft: Self, bottomright: Self) -> impl Iterator<Item=TermPos> {
    //     let mut pos = topleft;

    //     let clos = move || {
    //         pos = pos + (0i16, 1i16);
    //         if pos.x() > bottomright.x() {
    //             pos = TermPos::from((pos.y() + 1, topleft.x()));
    //         };

    //         if pos.y() > bottomright.y() {
    //             None  
    //         } else {
    //             Some(pos)
    //         }
    //     };

    //     iter::from_fn(clos).fuse()
    // }

    // pub fn range_to(self, bottomright: Self) -> impl Iterator<Item=TermPos> {
    //     Self::pos_range(*self, bottomright)
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

impl<T> FFrom <(T, T)> for TermPos
where
    i16: TryFrom<T>,
    <i16 as TryFrom<T>>::Error: Debug
{
    fn ffrom((y, x): (T, T)) -> Self {
        let (y, x) = (i16::try_from(y).unwrap(), i16::try_from(x).unwrap());
        Self::new(y, x)
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

impl<T> FFrom<TermPos> for (T, T)
where
    T: TryFrom<i16>,
    <T as TryFrom<i16>>::Error: Debug
{
    fn ffrom(pos: TermPos) -> Self {
        (T::try_from(pos.y()).unwrap(), T::try_from(pos.x()).unwrap())
    }
}



// impl<T> Add<(T, T)> for TermPos where i16: TryFrom<T> {
//     type Output = Self;

//     fn add(self, (y, x): (T, T)) -> Self::Output {
//         let (y, x) = (i16::try_from(y).parwnu(), i16::try_from(x).parwnu());
//         Self::new(self.y + y, self.x + x)
//     }
// }

// impl<T> Add<TermPos> for (T, T) where i16: From<T> {
//     type Output = TermPos;

//     fn add(self, tp: TermPos) -> Self::Output {
//         let (y, x) = (i16::try_from(self.0).unwrap(), i16::try_from(self.1).unwrap());
//         TermPos::new(y + tp.y(), x + tp.x())
//     }
// }

// impl From<(&LayoutPos, &Scale)> for TermPos {
//     fn from ((lp, s): (&LayoutPos, &Scale)) -> Self {
//         let (row, col): (i16, i16) = (lp.row.into(), lp.col.into());
//         let y = (row * s.CARD_HEIGHT) + (row * CARD_SPACING_VERT) + WIN_MARGIN_VERT;
//         let x = (col * s.CARD_WIDTH)  + (col * CARD_SPACING_HORIZ) + WIN_MARGIN_HORIZ;
//         TermPos::from((y, x))
    // }
// }

// impl From<(&GamePos, &Scale)> for TermPos {
//     fn from((sp, s): (&GamePos, &Scale)) -> Self {
//         let (width, height) = TS.update();

//         // adjustments by 1 are for the border, which is always offset by 1.
//         let bottom = height - s.CARD_HEIGHT - WIN_MARGIN_VERT - 1;
//         let right = width - s.CARD_WIDTH - WIN_MARGIN_HORIZ;
//         let left = WIN_MARGIN_HORIZ + 1;

//         match sp {
//             GamePos::Deck => TermPos::new(bottom - CARD_SPACING_VERT,  left),
//             GamePos::LastFound0 => TermPos::new(bottom, right - LAST_FOUND_OFFSET * 2),
//             GamePos::LastFound1 => TermPos::new(bottom - CARD_SPACING_VERT, right - LAST_FOUND_OFFSET),
//             GamePos::LastFound2 => TermPos::new(bottom - CARD_SPACING_VERT * 2, right),
//             GamePos::Dealt(pos) => TermPos::from((pos, s))
//         }
//     }
// }

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
