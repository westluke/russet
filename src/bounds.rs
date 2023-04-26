use std::ops::{Add, RangeInclusive};
use crate::pos::TermPos;
use crate::util::FInto as _;
use std::cmp::{min, max};

#[derive(Copy, Clone, Debug)]
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
    pub fn mk((y_min, x_min): (T, T), (y_max, x_max): (T, T)) -> Self {
        Self { y_min, y_max, x_min, x_max }
    }

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

    pub fn y_range(self) -> RangeInclusive<T> {
        self.y_min..=self.y_max
    }

    pub fn x_range(self) -> RangeInclusive<T> {
        self.x_min..=self.x_max
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
