use std::ops::{Add, Range};
use crate::pos::TermPos;
use crate::util::FInto as _;
use std::cmp::{min, max};

#[derive(Copy, Clone)]
pub struct Bounds<T: Copy + Add<Output=T> + From<u8> + Ord> {
    y_min: T,
    y_max: T,
    x_min: T,
    x_max: T,
}

#[derive(Copy, Clone)]
pub struct BoundsIter<T: Copy + Add<Output=T> + From<u8> + Ord> {
    rng: Bounds<T>,
    y: T,
    x: T
}

impl<T: Copy + Add<Output=T> + From<u8> + Ord> Bounds<T> {
    pub fn shift(self, shift: (T, T)) -> Self {
        Self {
            y_min: self.y_min + shift.0,
            y_max: self.y_max + shift.0,
            x_min: self.x_min + shift.1,
            x_max: self.x_max + shift.1,
        }
    }

    pub fn merge(self, oth: Self) -> Self {
        Self {
            y_min: min(self.y_min, oth.y_min),
            y_max: max(self.y_max, oth.y_max),
            x_min: min(self.x_min, oth.x_min),
            x_max: max(self.x_max, oth.x_max)
        }
    }

    pub fn y_range(self) -> Range<T> {
        self.y_min..(self.y_max + 1.into())
    }

    pub fn x_range(self) -> Range<T> {
        self.x_min..(self.x_max + 1.into())
    }
}

impl<T: Copy + Add<Output=T> + From<u8> + Ord> IntoIterator for Bounds<T> {
    type Item = (T, T);
    type IntoIter = BoundsIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        BoundsIter {
            rng: self,
            y: self.y_min,
            x: self.x_min
        }
    }
}

impl<T: Copy + Add<Output=T> + From<u8> + Ord> Iterator for BoundsIter<T> {
    type Item = (T, T);
    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.rng.x_max {
            if self.y >= self.rng.y_max {
                None
            } else {
                self.x = self.rng.x_min;
                self.y = self.y + 1.into();
                Some((self.y, self.x))
            }
        } else {
            self.x = self.x + 1.into();
            Some((self.y, self.x))
        }
    }
}

impl<T: Copy + Add<Output=T> + From<u8> + Ord> From<Range<(T, T)>> for Bounds<T> {
    fn from(rng: Range<(T, T)>) -> Self {
        let Range { start, end } = rng;
        let (y_min, x_min) = start;
        let (y_max, x_max) = end;
        Self {
            y_min,
            y_max,
            x_min,
            x_max,
        }
    }
}

impl From<Range<TermPos>> for Bounds<i16> {
    fn from(rng: Range<TermPos>) -> Self {
        let Range { start, end } = rng;
        let (y_min, x_min) = start.finto();
        let (y_max, x_max) = end.finto();

        Self {
            y_min,
            y_max,
            x_min,
            x_max,
        }
    }
}

impl<T: Copy + Add<Output=T> + From<u8> + Ord> From<Range<(T, T)>> for BoundsIter<T> {
    fn from(rng: Range<(T, T)>) -> Self {
        let Range { start, .. } = rng;
        let (y_min, x_min) = start;
        Self {
            rng: rng.into(),
            y: y_min,
            x: x_min,
        }
    }
}

impl From<Range<TermPos>> for BoundsIter<i16> {
    fn from(rng: Range<TermPos>) -> Self {
        let Range { start, .. } = rng;
        let (y_min, x_min) = start.finto();
        Self {
            rng: rng.into(),
            y: y_min,
            x: x_min,
        }
    }
}
