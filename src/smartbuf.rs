use std::io::{Write};
use std::ops::{Index, IndexMut};

use crossterm::style::{self, Color, Stylize as _};
use crossterm::{queue, execute, cursor, QueueableCommand};

use crate::pos::*;
use crate::Result;

use log::{info, warn, error};




pub struct Grid<T: Copy> {
    grid: Vec<Vec<T>>,
    height: usize,
    width: usize

}

impl<T: Copy> Grid<T> {
    fn new(height: usize, width: usize, fill: T) -> Self {
        Self{ grid: vec![vec![fill; width]; height], height, width }
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
        &self.grid[usize::from(pos.y())][usize::from(pos.x())]
    }
}

impl<T: Copy> IndexMut<TermPos> for Grid<T> {
    fn index_mut(&mut self, pos: TermPos) -> &mut Self::Output {
        &mut self.grid[usize::from(pos.y())][usize::from(pos.x())]
    }
}




#[derive (Clone, Copy, Debug, PartialEq, Eq)]
pub enum TermChar {
    Space       {bg: Color},
    Printable   {c: char, fg: Color, bg: Color}
}

impl Default for TermChar {
    fn default () -> Self {
        Self::Space {bg: Color::Reset}
    }
}

impl TermChar {
    pub fn new(c: char, fg: Color, bg: Color) -> Self {
        if c == ' ' {
            Self::Space{bg}
        } else {
            Self::Printable{c, fg, bg}
        }
    }

    pub fn get_fg_bg(&self) -> (Option<Color>, Color) {
        match self {
            Self::Space{bg} => (None, *bg),
            Self::Printable{c:_, fg, bg} => (Some(*fg), *bg)
        }
    }

    pub fn get_c(&self) -> char {
        match self {
            Self::Space{..} => ' ',
            Self::Printable{c, ..} => *c
        }
    }

    pub fn is_space(&self) -> bool {
        match self {
            Self::Space{..} => true,
            Self::Printable{..} => false
        }
    }

    pub fn is_printable(&self) -> bool {
        match self {
            Self::Space{..} => false,
            Self::Printable{..} => true
        }
    }

    fn style_cmd(&self) -> style::Colors {
        match self {
            Self::Space{bg} => style::Colors{foreground: None, background: Some(*bg)},
            Self::Printable{fg, bg, ..} => style::Colors{foreground: Some(*fg), background: Some(*bg)}
        }
    }
}




// T should be some terminal-like type
pub struct SmartBuf<T: Write> {
    under: T,
    counter: u32,
    cache: Grid<(u32, TermChar)>,
    prev: Grid<(u32, TermChar)>
}

impl<T: Write> SmartBuf<T> {
    pub fn new(under: T, height: usize, width: usize) -> Self {
        Self {
            under,
            counter: 0,
            cache: Grid::new(height, width, (0, TermChar::default())),
            prev: Grid::new(height, width, (0, TermChar::default())),
        }
    }

    pub fn write(&mut self, buf: &str, fg: Color, bg: Color, pos: TermPos) -> Result<()> {
        for (i, ln) in buf.lines().enumerate() {
            for (j, c) in ln.chars().enumerate() {
                let npos = (pos + (i, j))?;
                self.cache[npos] = (self.counter, TermChar::new(c, fg, bg));
            };
        };
        Ok(())
    }

    fn get_last_displayed(&mut self, pos: TermPos) -> TermChar {
        let (cnt, tc) = self.prev[pos];
        if (self.counter == 0) || (cnt < self.counter - 1) {
            TermChar::default()
        } else {
            tc
        }
    }


    fn norm(latest: u32, (cnt, tc): (u32, TermChar)) -> TermChar {
        if cnt < latest {
            TermChar::default()
        } else {
            tc
        }
    }

    // cache is implicitly "cleared" after each display
    // (actually done just by incrementing counter)
    // still not right: if char is old, but new char is same as old char, do nothing.
    // requires splitting string again
    // still, this should be working
    pub fn flush(mut self) -> Result<Self> {
        fn matches(fg0: Option<Color>, bg0: Color, tc: TermChar) -> bool {
            match tc {
                TermChar::Space{bg} =>
                    bg0 == bg,
                TermChar::Printable{c:_, fg, bg} =>
                    (fg0.unwrap_or(fg) == fg) && (bg0 == bg)
            }
        }


        // Ok, here's a question. what was the last character written here?
        // Let's say it's blank, and has always been blank. Then it has an outdated count, and the character is blank.
        // Let's say it's blank, because some previous iteration failed to put a character there.
        // Then every subsequent grid will also have an outdated count
        // Say it's a real character. then prev will have an up-to-date count
        // ok here's the issue. the problem is not avoiding printing on all duplicated cells,
        // the problem is avoiding printing on all duplicated cells AT THE EDGES. I strongly doubt
        // that breaking the print into many smaller prints speeds anything up, and i would not be
        // surprised if it makes things slower.
        // So, how do I do that? can be done in a single pass. Record the start, and record the
        // last seen new char. record all chars from start, and cut it down to end at last before
        // printing. and I still need to intercept color changes. But actually that's much easier
        // if I just intercept the commands and take their string values instead...
        
        for i in 0..self.cache.height {
            let (mut fg0, mut bg0) = (Color::Reset, Color::Reset);
            let mut s = String::new();
            let mut s_start: u16 = 0;

            for j in 0..self.cache.width {
                let mut chr = Self::norm(self.counter, self.cache[(i, j)]);
                let mut pchr = Self::norm(self.counter, self.prev[(i, j)]); 

                // printing must begin here
                if chr != pchr {
                    
                }

                let (i, j) = (u16::try_from(i)?, u16::try_from(j)?);

                // No point in writing this one, skip it
                if prev_chr == chr {
                    let fg_to_write = fg0.unwrap_or(Color::Reset);
                    queue!( self.under,
                            cursor::MoveTo(s_start, i),
                            style::PrintStyledContent(s.with(fg_to_write).on(bg0))
                    )?;

                    s = String::new();
                    s_start = j+1;
                    fg = chr.fg;
                    bg = chr.bg;
                    // write the current string, set s_start forward
                } else if !matches(fg0, bg0, chr) {
                    queue!( self.under,
                            cursor::MoveTo(s_start, i),
                            style::PrintStyledContent(s.with(fg).on(bg))
                    )?;

                    s = String::new();
                    s.push(chr.c);
                    s_start = j;
                    fg = chr.fg;
                    bg = chr.bg;
                    // write the current string, start new one
                } else {
                    s.push(chr.get_c());

                    if let (None, TermChar::Printable{fg, ..}) = (fg0, chr) {
                        fg0 = Some(fg);
                    };
                };

                // This cell is old, so pretend its empty (write background)
                if cnt < self.counter {
                    chr = TermChar{ c: ' ', fg: Color::Reset, bg: Color::Reset };
                } else if (chr.c == ' ') && (chr.fg != chr.bg) {
                    chr.fg = chr.bg;    // to avoid unnecessary new strings
                };

                // same formatting as current string, so just extend it
                if (chr.fg == fg) && (chr.bg == bg) {
                    s.push(chr.c);
                }

                // formatting has changed, so we'll need a separate write. so end the string here,
                // start a new one
                else {

                }
            }

            if !s.is_empty() {
                let i = u16::try_from(i)?;
                queue!( self.under,
                        cursor::MoveTo(s_start, i),
                        style::PrintStyledContent(s.with(fg).on(bg))
                )?;
            }
        }
        
        self.under.flush()?;
        self.counter += 1;

        // temp takes ownership - low cost
        let temp = self.cache;

        // cache trades ownership - low cost
        self.cache = self.prev;

        // prev trades ownership - low cost
        self.prev = temp;

        // modified self on stack trades ownership back to caller, should be low cost
        Ok(self)
    }

    pub fn resize(&mut self, height:u16, width:u16) {
        debug_assert!(height >= 1 && width >= 1);
        self.cache.resize(usize::from(height), usize::from(width), (self.counter, TermChar::default()));
    }
}

// implementing Write on SmartBuf is actually a bunch of work, for no clear purpose or gain.
