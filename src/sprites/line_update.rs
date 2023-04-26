use super::SpriteCell::{*, self};
// use super::DirtyBit::{*, self};
use super::termable::*;

use crossterm::style::StyledContent;

use crate::term_char::TermChar;

#[derive(Default, Debug, Copy, Clone)]
pub enum UpdateCell {
    Dirty(TermChar),
    #[default] Clean
}

use UpdateCell::*;

// There's an additional optimization step I'm missing here, which would be knowing which
// SpriteCell is already in place on the screen. But I'm not sure I want to do that.
//
//
//
// Ugh, SpriteCell isn't the right abstraction here. It's conceptually very different.
// Its not Opaque/Transparent, it's Stale/Dirty.

#[derive(Debug, Default)]
pub struct LineUpdateBuilder {
    pub cells: Vec<UpdateCell>,
    pub start: i16
}


impl LineUpdateBuilder {
    fn set(&mut self, real_index: i16, c: UpdateCell) {
        if let Some(s) = self.start {
            assert!(s <= real_index);
            let vec_index = usize::try_from(real_index - s).unwrap();
            let min_vec_len = vec_index + 1;
            if (min_vec_len > self.cells.len()) {
                self.cells.resize(min_vec_len, Default::default());
            }
            self.cells[vec_index] = c;
        } else {
            assert!(self.cells.len() == 0);
            self.start = Some(real_index);
            self.cells.push(c);
        }
    }

    // fn dirty_only(start: i16, cells: Vec<UpdateCell>) -> Vec<(i16, Vec<UpdateCell>)> {
    //     for (i, cel) in cells.enumerate() {
    //     }
    // }
    //
    // {{{

    // splits cells in contiguous groups of dirty cells, so the termchars can be unwrapped
    // fn dirty_only(&self) -> Vec<(i16, Vec<TermChar>)> {
    //     let ret = Vec::new();
    //     let mut latest = (self.start, Vec::new());
            
    //     for (i, cel) in self.cells.iter().enumerate() {
    //         let i = i + self.start;

    //         if let Dirty(tc) = cel {
    //             if latest.1.is_empty() {
    //                 latest.0 = i;
    //             }
    //             latest.1.push(tc);
    //         }

    //         else {
    //             if latest.1.is_empty() {
    //                 continue;
    //             } else {
    //                 ret.push(latest);
    //                 latest = (i, Vec::new())
    //             }
    //         }
    //     }

    //     ret
    // }

    fn termables(dirties: Vec<(i16, Vec<TermChar>)>) -> Vec<(i16, StyledContent<Termable>)> {
        for (start, run) in dirties {
        }
    }


    fn build(&mut self) -> Vec<StyledContent<Termable>> {
        // let Self {cells, start} = self;
        let ret = Vec::new();
        let dirty_only = self.cells.split(|c| c == Stale);

        for slice in dirty_only {
            Termable::extract(slice);
        }

        loop {
            if let Some(term_pair) = Termable::get_one(self) {
                ret.push(term_pair);
            } else {
                break;
            }
        }

        ret

        // assert!(self.start.is_some() && self.cells.len() > 0);
        // let first = self.cells.first().unwrap();
        // if let Dirty(f) = first {
        // } else {
        //     panic!("First element of self.cells should always be Dirty");
        // }
        // vec![]
    }

}

// // this is stupid and wrong, it's not actually optimizing at all
// // Why did I write that again? Don't totally remember...
// pub struct LineUpdate {
//     cs: Vec<SpriteCell>,
//     start: i16,
//     stop: i16
// }

// impl Default for LineUpdate {
//     fn default() -> Self {
//         Self { cs: Default::default(), start: -1, stop: -1 }
//     }
// }

// impl LineUpdate {
//     // pub fn new(start: i16) -> Self {
//     //     Self { gtgt }
//     // }

//     pub fn set(&mut self, i: i16, c: LayerCell) {
//         self.cs[usize::try_from(i).unwrap()] = c;
//     }

//     // Returns number of characters consumed to find start, Termable produced (if any)
//     fn first_termable(&mut self) -> (i16, Option<Termable>) {
//         let mut term = None;
//         let mut cons = 0;

//         for i in 0..self.cs.len() {
//             let c_opt = self.cs[i];

//             match (&mut term, c_opt) {
//                 (None, Transparent) => {
//                     cons += 1;
//                 },
//                 (None, Opaque(c)) => {
//                     term = Some(Termable::from(c));
//                 },
//                 (Some(_), Transparent) => {
//                     self.cs.drain(0..i);
//                     return (cons, term);
//                 },
//                 (Some(t), Opaque(c)) => {
//                     if !t.push(c) {
//                         self.cs.drain(0..i);
//                         return (cons, term);
//                     };
//                 }
//             };
//         };

//         self.cs.drain(0..);
//         (cons, term)
//     }

//     // Outputs a vector of pairs (i, cont) where cont is a StyledContent ready to be Print'd,
//     // and i is the column where cont should be printed
//     pub fn finalize(mut self) -> Vec<(i16, StyledContent<Termable>)> {
//         let mut out = Vec::new();

//         let (mut cons, mut term) = self.first_termable();
//         let mut last = 0;

//         while let Some(t) = term {
//             let len = t.len();
//             out.push((last + cons, t.finalize()));
//             last += cons + len;
//             (cons, term) = self.first_termable();
//         };

//         out
//     }
// }

