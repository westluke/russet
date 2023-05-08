use std::ops::{Add, Range};
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


// nah, images with zero rows or columns CAN be meaningful.
// and I SHOULD be able to have an empty boundsiter. Need to change this.
// Does this imply I should use an exclusive model instead? Maybe. Otherwise it needs special handling.
// Yeah, should use exclusive model.

#[derive(Copy, Clone)]
pub struct BoundsIter<T: Copy + Add<Output=T> + From<u8> + Ord> {
    rng: Bounds<T>,

    // represents the NEXT thing to return, including potentially None
    next: Option<(T, T)>
}

impl<T: Copy + Add<Output=T> + From<u8> + Ord> Bounds<T> {
    pub fn mk((y_min, x_min): (T, T), (y_max, x_max): (T, T)) -> Self {
        Self { y_min, y_max, x_min, x_max }
    }

    pub fn is_empty(self) -> bool {
        self.y_max <= self.y_min || self.x_max <= self.x_min
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

    pub fn y_range(self) -> Range<T> {
        self.y_min..self.y_max
    }

    pub fn x_range(self) -> Range<T> {
        self.x_min..self.x_max
    }

    pub fn contains(self, (y, x): (T, T)) -> bool {
        self.y_min <= y && y < self.y_max &&
        self.x_min <= x && x < self.x_max
    }
}

impl<T: Copy + Add<Output=T> + From<u8> + Ord> IntoIterator for Bounds<T> {
    type Item = (T, T);
    type IntoIter = BoundsIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        let next = if self.is_empty() { None } else { Some((self.x_min, self.y_min)) };
        BoundsIter {
            rng: self,
            next
        }
    }
}

// Interesting? Was this using exclusive semantics the whole time?
// Seems like it. Weird...
// Should write test cases for this file.
// Oh wait this impl is SO wrong. It never hits x_min, y_min? wtf? so stuff must be wrong with the rest of the code using it...

impl<T: Copy + Add<Output=T> + From<u8> + Ord> Iterator for BoundsIter<T> {
    type Item = (T, T);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((y, x)) = self.next {
            if x+1.into() >= self.rng.x_max {
                if y+1.into() >= self.rng.y_max { self.next = None; }
                else { self.next = Some((y+1.into(), self.rng.x_min)) };
            } else {
                self.next = Some((y, x+1.into()));
            };
            Some((y, x))
        } else {
            None
        }
    }
}
