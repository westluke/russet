use std::cmp::{Ordering, min, max};
use std::collections;
use collections::HashMap;

// Indicates "dirty" portions of a FrameBufLayer, starting at start,
// and extending len TermChars to the right
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Stain {
    start: i16,
    end: i16
}

// fuck why did I need an intersect method on these again?
// and what did I decide about putting stains in the main FrameBuf?
// Ah intersect is for when several stains at different layers interfere, and I need
// to know which portions to take from each of them.

impl Stain {
    pub fn new(start: i16, end: i16) -> Self {
        Self {start, end}
    }

    pub fn start(&self) -> i16 {
        self.start
    }

    pub fn end(&self) -> i16 {
        self.end
    }

    pub fn len(&self) -> i16 {
        self.end - self.start
    }

    pub fn start_end(&self) -> (i16, i16) {
        (self.start, self.end)
    }

    // Assuming self and oth are on the same line, what's the total stain
    pub fn union(&self, oth: &Self) -> Self {
        Self::new(min(self.start, oth.start), max(self.end, oth.end))
    }

    pub fn intersect(&self, oth: &Self) -> Option<Self> {
        let start = max(self.start, oth.start);
        let end = min(self.end, oth.end);
        if start >= end {
            None
        } else {
            Some(Self::new(start, end))
        }
    }
}

// Stores stains per layer. Indices to stains should be ABSOLUTE line numbers,
// not relative. i.e. anchor + relative.
// Hmm. Should Stains store their own line numbers? Is there a point?
// doesn't really seem like it...
#[derive(Clone, Debug)]
pub struct StainSet {
    stains: HashMap<i16, Stain>,
    sorted: bool
}

impl StainSet {
    pub fn new() -> Self {
        Self {stains: HashMap::new(), sorted: true}
    }

    pub fn keys(&self) -> collections::hash_map::Keys<i16, Stain> {
        self.stains.keys()
    }

    pub fn get_stain(&self, row: i16) -> Option<Stain> {
        self.stains.get(&row).copied()
    }

    fn union_one(&mut self, ln: i16, s: &Stain) {
        if let Some(v) = self.stains.get_mut(&ln) {
            *v = v.union(s);
        };
    }

    fn intersect_one(&mut self, ln: i16, s: &Stain) {
        if let Some(v) = self.stains.get_mut(&ln) {
            if let Some(i) = v.intersect(s) {
                *v = i;
            } else {
                self.stains.remove(&ln);
            }
        }
    }

    pub fn union(&mut self, oth: &Self) {
        for k in self.stains.keys().chain(oth.stains.keys()) {
            match (self.stains.get_mut(k), oth.stains.get(k)) {
                (Some(v0), Some(v1)) =>
                    {self.stains.insert(*k, v0.union(v1));},
                (None, Some(v1)) =>
                    {self.stains.insert(*k, *v1);},
                (Some(v0), None) =>
                    (),
                (None, None) =>
                    (),
            }
        }
    }

    pub fn intersect(&mut self, oth: &Self) {
        for k in self.stains.keys().chain(oth.stains.keys()) {
            match (self.stains.get_mut(k), oth.stains.get(k)) {
                (Some(v0), Some(v1)) => 
                    if let Some(i) = v0.intersect(v1) {
                        self.stains.insert(*k, i);
                    } else {
                        self.stains.remove(k);
                    },
                (None, Some(v1)) =>
                    {self.stains.insert(*k, *v1); },
                (Some(v0), None) =>
                    (),
                (None, None) =>
                    (),
            }
        }
    }
}
