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
use pos::*;




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

fn find_set(lay:Layout) -> Option<[SetPos; 3]> {
    let cards:Vec<_> = lay.enumerate_2d().filter(|(_, c)| *c != None).collect();
    for i in 0..cards.len() {
        for j in i..cards.len() {
            for k in j..cards.len() {
                let (pos0, c0) = cards[i];
                let (pos1, c1) = cards[j];
                let (pos2, c2) = cards[k];
                return if is_a_set(c0.unwrap(), c1.unwrap(), c2.unwrap()) {
                    Some([pos0, pos1, pos2])
                } else { None }
            }
        }
    }
    None
}

fn key_to_SetPos(c: char) -> SetPos {
    match c {
        'q' | 'Q' => SetPos::new_dealt(0, 0),
        'w' | 'W' => SetPos::new_dealt(0, 1),
        'e' | 'E' => SetPos::new_dealt(0, 2),
        'r' | 'R' => SetPos::new_dealt(0, 3),
        't' | 'T' => SetPos::new_dealt(0, 4),
        'y' | 'Y' => SetPos::new_dealt(0, 5),
        'a' | 'A' => SetPos::new_dealt(1, 0),
        's' | 'S' => SetPos::new_dealt(1, 1),
        'd' | 'D' => SetPos::new_dealt(1, 2),
        'f' | 'F' => SetPos::new_dealt(1, 3),
        'g' | 'G' => SetPos::new_dealt(1, 4),
        'h' | 'H' => SetPos::new_dealt(1, 5),
        'z' | 'Z' => SetPos::new_dealt(2, 0),
        'x' | 'X' => SetPos::new_dealt(2, 1),
        'c' | 'C' => SetPos::new_dealt(2, 2),
        'v' | 'V' => SetPos::new_dealt(2, 3),
        'b' | 'B' => SetPos::new_dealt(2, 4),
        'n' | 'N' => SetPos::new_dealt(2, 5),
        _ => panic!()
    }
}

fn main() -> std::io::Result<()> {
    let (tx, rx) = mpsc::channel::<animation::Msg>();
    let stdin = io::stdin();
    let handle = thread::spawn(animation::animate(rx));

    let mut state = GameState::new();

    tx.send(Msg::Base(Clone::clone(&state)));

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

            Event::Key(Key::Char(c)) => {
                let res = state.select(key_to_SetPos(c));
                match res {
                    SelectResult::Pending | SelectResult::UnPending => tx.send(Msg::Base(Clone::clone(&state))).unwrap(),
                    SelectResult::BadSet(p0, p1, p2) => {
                        tx.send(Msg::Timed(
                            vec![TimedMsg::BadOutline(p0), TimedMsg::BadOutline(p1), TimedMsg::BadOutline(p2)],
                            Instant::now() + Duration::from_secs(1)
                        )).unwrap();
                        tx.send(Msg::Base(Clone::clone(&state))).unwrap();
                    },
                    SelectResult::GoodSet(p0, p1, p2) => {
                        tx.send(Msg::Base(Clone::clone(&state))).unwrap();
                    },
                    _ => ()
                };
            },

            _ => ()
        };
    };

    Ok(())
}
