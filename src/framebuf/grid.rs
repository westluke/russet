use std::ops::{Index, IndexMut};
use crate::pos::TermPos;

#[derive(Clone, Debug)]
pub struct Grid<T: Copy> {
    grid: Vec<Vec<T>>,
    height: usize,
    width: usize
}

impl<T: Copy + Default> Default for Grid<T> {
    fn default() -> Self {
        Self::new(0, 0, T::default())
    }
}

impl<T: Copy> Grid<T> {
    pub fn new(height: usize, width: usize, fill: T) -> Self {
        Self{ grid: vec![vec![fill; width]; height], height, width }
    }

    // pub fn iter(&self) -> Iterator

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn get(&self, pos: TermPos) -> Option<T> {
        if let Some(row) = self.grid.get(pos.y()) {
            if let Some(cell) = row.get(pos.x()) {
                cell
            } else { None }
        } else { None }
    }

    fn resize(&mut self, height: usize, width: usize, fill: T){
        for v in &mut self.grid {
            v.resize(width, fill)
        };

        self.grid.resize_with(height, || vec![fill; width]);
        self.height = height;
        self.width = width;
    }

    fn refill(&mut self, fill: T) {
        for i in 0..self.height {
            self.grid[i] = vec![fill; self.width]
        }
    }

    // An iterator over IMMUTABLE REFERENCES to the row vectors
    fn row_iter(&self) -> std::slice::Iter<'_, Vec<T>> {
        (&self.grid).into_iter()
    }
}

impl<T: Copy> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (y, x): (usize, usize)) -> &Self::Output {
        &self.grid[y][x]
    }
}

impl<T: Copy> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (y, x): (usize, usize)) -> &mut Self::Output {
        &mut self.grid[y][x]
    }
}

impl<T: Copy> Index<TermPos> for Grid<T> {
    type Output = T;

    fn index(&self, pos: TermPos) -> &Self::Output {
        &self.grid
            [usize::try_from(pos.y()).unwrap()]
            [usize::try_from(pos.x()).unwrap()]
    }
}

impl<T: Copy> IndexMut<TermPos> for Grid<T> {
    fn index_mut(&mut self, pos: TermPos) -> &mut Self::Output {
        &mut self.grid
            [usize::try_from(pos.y()).unwrap()]
            [usize::try_from(pos.x()).unwrap()]
    }
}
