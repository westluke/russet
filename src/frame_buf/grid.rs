use std::ops::{Index, IndexMut};
use std::iter::Map;
use crate::pos::TermPos;

use crate::util::{*, SetErrorKind as SEK, SetError as SE};

#[derive(Clone, Debug)]
pub struct Grid<T: Copy> {
    grid: Vec<Vec<T>>,
    height: usize,
    width: usize
}

impl<T: Copy + Default> Default for Grid<T> {
    fn default() -> Self {
        Self::new(1, 1, T::default())
    }
}

impl<T: Copy> Grid<T> {
    pub fn new(height: usize, width: usize, fill: T) -> Self {
        debug_assert!(height >= 1 && width >= 1);
        Self{ grid: vec![vec![fill; width]; height], height, width }
    }

    pub fn enumerate(&self) -> impl Iterator<Item=(TermPos, T)> + '_ {
        let under = TermPos::new(0, 0).range_to((self.height-1, self.width-1).finto());
        under.map(|p| (p, self.get(p).unwrap()))
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn get(&self, pos: TermPos) -> Option<T> {
        let pos_tup: Result<(usize, usize)> = pos.try_into();
        if let Ok((row_i, col_i)) = pos_tup {
            if let Some(row) = self.grid.get(row_i) {
                if let Some(cell) = row.get(col_i) {
                    Some(*cell)
                } else { None }
            } else { None }
        } else { None }
    }

    pub fn set(&mut self, pos: TermPos, cel: T) -> Result<()> {
        let pos_tup: Result<(usize, usize)> = pos.try_into();
        if let Ok((row_i, col_i)) = pos_tup {
            if let Some(row) = self.grid.get_mut(row_i) {
                if let Some(cell) = row.get_mut(col_i) {
                    *cell = cel;
                    Ok(())
                } else { Err(SE::new(SEK::PanelOob, "column index out of bounds in Grid::get")) }
            } else { Err(SE::new(SEK::PanelOob, "row index out of bounds in Grid::get")) }
        } else { Err(SE::new(SEK::PanelOob, "TermPos has negative components in Grid::get")) }
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
