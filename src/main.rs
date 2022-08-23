use std::{io, thread, time, iter};
use termion::{color, style, clear, cursor};

use std::{collections::HashSet, sync::mpsc};
use time::{Instant, Duration};
use io::BufWriter;
use io::Write as _;
use rand::seq::SliceRandom as _;

use termion::screen::{ToAlternateScreen, ToMainScreen};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::IntoRawMode;
use termion::event::{Key, Event, MouseEvent};
use color::Color;

mod animation;
mod printing;
mod game;
mod config;

pub mod pos;

use game::*;
use animation::*;

// Maybe eventually have these, but they should be legit tuple structs,
// with impls, and similarly for the SelectionState (although that should be enum with None, One,
// Two most likely)





fn all_diff_or_all_same<T: Eq> (a:T, b:T, c:T) -> bool {
    ((a == b) && (b == c) && (a == c)) || 
    ((a != b) && (b != c) && (a != c))
}

fn is_a_set(c0:Card, c1:Card, c2:Card) -> bool {
    all_diff_or_all_same(c0.color, c1.color, c2.color) &&
    all_diff_or_all_same(c0.shape, c1.shape, c2.shape) && 
    all_diff_or_all_same(c0.number, c1.number, c2.number) &&
    all_diff_or_all_same(c0.fill, c1.fill, c2.fill)
}

fn find_set(lay:Layout) -> Option<[(usize, usize); 3]> {
    let cards:Vec<_> = lay.enumerate_2d().filter(|(_, c)| *c != None).collect();
    for i in 0..cards.len() {
        for j in i..cards.len() {
            for k in j..cards.len() {
                let ((y0, x0), c0) = cards[i];
                let ((y1, x1), c1) = cards[j];
                let ((y2, x2), c2) = cards[k];
                return if is_a_set(c0.unwrap(), c1.unwrap(), c2.unwrap()) {
                    Some([(y0, x0), (y1, x1), (y2, x2)])
                } else { None }
            }
        }
    }
    None
}

fn main() -> std::io::Result<()> {
    let (tx, rx) = mpsc::channel::<animation::Msg>();
    let stdin = io::stdin();
    let handle = thread::spawn(animation::animate(rx));

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Delete) | 
            Event::Key(Key::Backspace) | 
            Event::Key(Key::Ctrl('c')) |
            Event::Key(Key::Ctrl('z')) |
            Event::Key(Key::Ctrl('\\')) => {
                tx.send(Msg::Quit).unwrap();
                handle.join().unwrap();
                break;
            },

            Event::Key(Key::Char('q')) => {
                tx.send(Msg::Select{row:0, col:0}).unwrap();
            },

            _ => ()
        }
    }


    Ok(())

}
