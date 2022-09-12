use std::io::{Write};
use std::fmt::Write as _;
use std::cmp::min;
use std::rc::Rc;

use crate::pos::*;
use crate::Result;
use crate::{color_to_fg_str, color_to_bg_str};

use termion::{color, cursor};
use color::{Color, Fg, Bg};

use once_cell::sync::Lazy;

static DEFAULT_TERMCHAR: Lazy<TermChar> = Lazy::new(|| {
    let (fg, bg) = (
        Rc::new(color_to_fg_str(&color::Reset).unwrap()),
        Rc::new(color_to_bg_str(&color::Reset).unwrap())
    );

    TermChar{c: ' ', fg, bg}
});

// what's the actual problem here? The duplication of strings.
// what's really needed is an abstract representation of color. 
// But termion doesn't support that, cuz its color interface seriously sucks.
// And I don't want to make it. And this is all early optimization anyway...
// Ugh fuck fine just use the duplicated strings, and if I run into issues I'll fix
// termion's colors.


// Writes iff character has been updated since last write
// What exactly does this mean here?
// Keeps record of whats on screen. Record can be changed only on calls to display()
// When display() is called, it figures out what needs to be changed
// Also, it needs to be able to resize...
// Ack this does break the abstraction a little, cuz it needs to intercept cursor movements.
// and it succks to try to do that just by inspecting what's being written.
// So we need an actual mv cursor method on this struct
//
// Also need to be able to store and write backgrouond color.
// That suggests, instead of a string, this should store some kind of abstract character that
// stores chaaracter, fg, bg, etc.

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

// Should be Rc because calling code will probably need to CONSTRUCT a Color on the spot,
// but not be able to keep ownership of it. Additionally, calling code will only provide
// us with a single instance of a given color, but that instance must be duplicated
// across all characters inserted into the grid. Since Color isn't a subtrait of Clone,
// that means we need to store in Rc.
//
// Actually? fuck all that. This termion interface sucks and I'll just skip all that.
// Just store the actual strings used to do the color changing. Can still keep it in an Rc
// to avoid duplication
pub struct TermChar {
   pub c: char,
   pub fg: Rc<String>,
   pub bg: Rc<String>,
}

impl Default for TermChar {
    fn default() -> Self {
        let (fg, bg) = (
            Rc::new(color_to_fg_str(&color::Reset).unwrap()),
            Rc::new(color_to_bg_str(&color::Reset).unwrap())
        );

        Self{c: ' ', fg, bg}
    }
}

pub struct SmartBuf<T: Write> {
    under: T,
    counter: u32,
    cache: Grid<(u32, TermChar)>,
    cursor: TermPos,
}

impl<T: Write> SmartBuf<T> {
    pub fn clear() {
    }

    pub fn write_color(&mut self, fg: Rc<dyn Color>, bg: Rc<dyn Color>, buf: &str) -> Result<usize> {
        
        for (i, ln) in buf.lines().enumerate() {
            for (j, c) in ln.chars().enumerate() {
                self.cache.write_cell(
                    self.cursor.add(i.try_into()?, j.try_into()?)?,
                    (self.counter, TermChar{c, fg:Rc::clone(&fg), bg:Rc::clone(&bg)})
                );
            };
        };
        Ok(buf.len())
    }

    pub fn display(&mut self) -> Result<()> {

        for (i, row) in self.cache.row_iter().enumerate() {
            let mut s = String::new();
            let (mut fg, mut bg);

            for (j, &(cnt, ref chr)) in row.iter().enumerate() {
                (fg, bg) = (chr.fg, chr.bg);
                write!( self.under, "{}{}{}{}",
                        cursor::Goto(self.cursor.x() + u16::try_from(j)?, self.cursor.y() + u16::try_from(i)?),
                        color::Fg(fg.as_ref()),
                        color::Bg(bg.as_ref()), chr.c)?;
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

impl<T: Write> Write for SmartBuf<T> {
    // Writes at current cursor, WITHOUT STYLING (reset all colors)
    // Newlines cause cursor to move down?
    // Overflow causes... what?
    
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if let Err(x) = self.write_color(Rc::new(color::Reset), Rc::new(color::Reset), "asdlkfjh"){
            Err(std::io::Error::new(std::io::ErrorKind::Other, "write failed!"))
        } else {
            Ok(buf.len())
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        if let Err(x) = self.display() {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "flush failed!"))
        } else {
            Ok(())
        }
    }
}
    
