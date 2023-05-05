use std::ops::{Index, IndexMut};
use std::iter::Map;
use crate::bounds::{Bounds, BoundsIter};

use crate::util::{*, SetErrorKind as SEK, SetError as SE};

#[derive(Clone, Debug, Default)]
pub struct Grid<T: Copy> {
    grid: Vec<Vec<T>>,
    height: usize,
    width: usize
}

pub struct GridEnumerator<'a, T: Copy> {
    grid: &'a Grid<T>,
    iter: BoundsIter<usize>
}

impl<'a, T: Copy> Iterator for GridEnumerator<'a, T> {
    type Item = ((usize, usize), T);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((y, x)) = self.iter.next() {
            Some((
                (y, x),
                self.grid.get((y, x))
                    .expect("shouldn't be possible for object to produce invalid iterator over itself")
            ))
        } else {
            None
        }
    }
}

impl<T: Copy> Grid<T> {
    pub fn new(height: usize, width: usize, fill: T) -> Self {
        Self{ grid: vec![vec![fill; width]; height], height, width }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn enumerate(&self) -> GridEnumerator<T> {
        GridEnumerator { grid: self, iter: Bounds::mk((0, 0), (self.height, self.width)).into_iter()}
    }

    pub fn get(&self, (row_i, col_i): (usize, usize)) -> Result<T> {
        if let Some(row) = self.grid.get(row_i) {
            if let Some(cell) = row.get(col_i) {
                Ok(*cell)
            } else { Err(SE::new(SEK::OutOfBounds, "column index too big")) }
        } else { Err(SE::new(SEK::OutOfBounds, "row index too big")) }
    }

    pub fn set(&mut self, (row_i, col_i): (usize, usize), cel: T) -> Result<T> {
        if let Some(row) = self.grid.get_mut(row_i) {
            if let Some(cell) = row.get_mut(col_i) {
                let ret = *cell;
                *cell = cel;
                Ok(ret)
            } else { Err(SE::new(SEK::OutOfBounds, "column index too big")) }
        } else { Err(SE::new(SEK::OutOfBounds, "row index out too big")) }
    }

    // fn resize(&mut self, height: usize, width: usize, fill: T){
    //     for v in &mut self.grid {
    //         v.resize(width, fill)
    //     };

    //     self.grid.resize_with(height, || vec![fill; width]);
    //     self.height = height;
    //     self.width = width;
    // }

    // fn refill(&mut self, fill: T) {
    //     for i in 0..self.height {
    //         self.grid[i] = vec![fill; self.width]
    //     }
    // }

    // An iterator over IMMUTABLE REFERENCES to the row vectors
    // fn row_iter(&self) -> std::slice::Iter<'_, Vec<T>> {
    //     (&self.grid).into_iter()
    // }
}
