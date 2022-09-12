use std::io::{Write};
use std::fmt::Write as _;
use std::cmp::min;
use std::sync::Arc;

use crossterm::style::Color;
use once_cell::sync::Lazy;

use crate::pos::*;
use crate::Result;




pub struct Grid<T: Default> {
    grid: Vec<Vec<T>>,

}

impl<T: Default> Grid<T> {
    fn new(height: usize, width: usize, fill: T) -> Self {
        let mut grid: Vec<Vec<T>> = Vec::new();
        grid.resize_with(
            height,
            || {
                let mut row = Vec::new();
                row.resize_with(width, Default::default);
                row
            }
        );

        Self{grid}
    }

    fn resizer (width: usize) -> Box<dyn Fn() -> Vec<T>> {
        Box::new (move || {
            let mut row = Vec::new();
            row.resize_with(width, Default::default);
            row
        })
    }

    fn write_cell(&mut self, p: TermPos, c:T) {
        self.grid[p.x() as usize][p.y() as usize] = c;
    }

    fn get_cell(&self, p: TermPos) -> &T {
        &self.grid[p.x() as usize][p.y() as usize]
    }

    fn resize(&mut self, height: usize, width: usize){
        self.grid.resize_with(height, Self::resizer(width))
    }

    fn row_iter(&self) -> std::slice::Iter<Vec<T>> {
        (&self.grid).into_iter()
    }
}




pub struct TermChar {
   pub c: char,
   pub fg: Color,
   pub bg: Color
}

impl Default for TermChar {
    fn default() -> Self {
        Self{c: ' ', fg: Color::Reset, bg: Color::Reset}
    }
}




// T should be some terminal-like type
pub struct SmartBuf<T: Write> {
    under: T,
    counter: u32,
    cache: Grid<(u32, TermChar)>,
    cursor: TermPos,
}

impl<T: Write> SmartBuf<T> {
    pub fn new(under: T) -> Self {
        Self {
            under,
            counter: 0,
            cache: Grid::new(),
            cursor: TermPos::new(0, 0)
        }
    }

    pub fn write_color(&mut self, fg: Color, bg: Color, buf: &str) -> Result<usize> {
        for (i, ln) in buf.lines().enumerate() {
            for (j, c) in ln.chars().enumerate() {
                self.cache.write_cell(
                    self.cursor.add(i.try_into()?, j.try_into()?)?,
                    (self.counter, TermChar{c, fg, bg})
                );
            };
        };
        Ok(buf.len())
    }

    // cache is implicitly "cleared" after each display
    // (actually done just by incrementing counter)
    pub fn display(&mut self) -> Result<()> {

        for (i, row) in self.cache.row_iter().enumerate() {
            let mut s = String::new();
            let (mut fg, mut bg);

            for (j, &(cnt, ref chr)) in row.iter().enumerate() {
                (fg, bg) = (chr.fg, chr.bg);
                // write!( self.under, "{}{}{}{}",
                //         cursor::Goto(self.cursor.x() + u16::try_from(j)?, self.cursor.y() + u16::try_from(i)?),
                //         color::Fg(fg.as_ref()),
                //         color::Bg(bg.as_ref()), chr.c)?;
                // restyle if necessary
                // construct string, THEN write
            }
        }
        
        self.under.flush();
        self.counter += 1;

        Ok(())
    }

    pub fn mv_cursor(&mut self, p: TermPos) {
        self.cursor = p;
    }

    pub fn resize(&mut self, height:u16, width:u16) -> Result<()> {
        debug_assert!(height >= 1 && width >= 1);
        self.cache.resize(height as usize, width as usize);
        self.cursor.set_x(min(width-1, self.cursor.x()))?;
        self.cursor.set_y(min(height-1, self.cursor.y()))?;
        Ok(())
    }
}

// implementing Write on SmartBuf is actually a bunch of work, for no clear purpose or gain.
