use std::{io, thread, time};

use std::sync::mpsc;
use time::{Instant, Duration};

use termion::input::TermRead;
use termion::event::{Key, Event, MouseButton, MouseEvent};

mod animation;
mod printing;
mod game;
mod config;

pub mod pos;

use game::*;
use animation::*;
use pos::*;

const threaderr: &str = "thread communication should never fail";



#[derive(Debug)]
pub enum SetErrorKind {
    Io(io::Error),
    SmallScreen,
    LayoutOob
}

#[derive(Debug)]
pub struct SetError {
    pub kind: SetErrorKind,
    pub msg: String
}

impl SetError {
    fn new(kind: SetErrorKind, msg: &str) -> Self {
        Self{ kind, msg: String::new() }
    }
}

impl From<io::Error> for SetError {
    fn from(err: io::Error) -> Self {
        Self::new(SetErrorKind::Io(err), "io error detected")
    }
}

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

fn find_set(lay:Layout) -> Option<[LayoutPos; 3]> {
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

fn key_to_LayoutPos(c: char) -> Option<LayoutPos> {
    match c {
        'q' | 'Q' => Some(LayoutPos::new(0, 0).unwrap()),
        'w' | 'W' => Some(LayoutPos::new(0, 1).unwrap()),
        'e' | 'E' => Some(LayoutPos::new(0, 2).unwrap()),
        'r' | 'R' => Some(LayoutPos::new(0, 3).unwrap()),
        't' | 'T' => Some(LayoutPos::new(0, 4).unwrap()),
        'y' | 'Y' => Some(LayoutPos::new(0, 5).unwrap()),
        'a' | 'A' => Some(LayoutPos::new(1, 0).unwrap()),
        's' | 'S' => Some(LayoutPos::new(1, 1).unwrap()),
        'd' | 'D' => Some(LayoutPos::new(1, 2).unwrap()),
        'f' | 'F' => Some(LayoutPos::new(1, 3).unwrap()),
        'g' | 'G' => Some(LayoutPos::new(1, 4).unwrap()),
        'h' | 'H' => Some(LayoutPos::new(1, 5).unwrap()),
        'z' | 'Z' => Some(LayoutPos::new(2, 0).unwrap()),
        'x' | 'X' => Some(LayoutPos::new(2, 1).unwrap()),
        'c' | 'C' => Some(LayoutPos::new(2, 2).unwrap()),
        'v' | 'V' => Some(LayoutPos::new(2, 3).unwrap()),
        'b' | 'B' => Some(LayoutPos::new(2, 4).unwrap()),
        'n' | 'N' => Some(LayoutPos::new(2, 5).unwrap()),
        _ => None
    }
}

fn handle_result (res: SelectResult, tx: &mpsc::Sender<Msg>, state: &GameState) {
    match res {
        SelectResult::Pending |
        SelectResult::UnPending => {
            tx.send(
                Msg::Base(
                    Clone::clone(
                        &state
                    )
                )
            ).expect(threaderr);
        },

        SelectResult::BadSet(p0, p1, p2) => {
            tx.send(
                Msg::Timed(
                    vec![TimedMsg::BadOutline(p0), TimedMsg::BadOutline(p1), TimedMsg::BadOutline(p2)],
                    Instant::now() + Duration::from_secs(1)
            )).expect(threaderr);

            tx.send(
                Msg::Base(
                    Clone::clone(
                        &state
                    )
                )
            ).expect(threaderr);
        },

        SelectResult::GoodSet(p0, p1, p2) => {
            tx.send(
                Msg::Base(
                    Clone::clone(
                        &state
                    )
                )
            ).expect(threaderr);
        },
        _ => ()
    };
}

fn main() -> Result<(), SetError> {
    let (tx, rx) = mpsc::channel::<animation::Msg>();
    let stdin = io::stdin();
    let handle = thread::spawn(animation::animate(rx));

    let mut state = GameState::new();
    tx.send(Msg::Base(Clone::clone(&state))).expect(threaderr);

    let mut last_mouse_pressed: Option<MouseButton> = None;

    // idea: to avoid excessive buffering on holding keydown, 
    // impose time limit on pressing the same key twice.

    for c in stdin.events() {
        let evt = c.unwrap();

        match evt {
            Event::Key(Key::Delete) | 
            Event::Key(Key::Backspace) | 
            Event::Key(Key::Ctrl('c')) |
            Event::Key(Key::Ctrl('z')) |
            Event::Key(Key::Ctrl('\\')) => break,

            Event::Key(Key::Char(c)) => {
                let pos_r = key_to_LayoutPos(c);
                if let Some(pos) = pos_r {
                    let res = state.select(pos);
                    handle_result(res, &tx, &state);
                }
            },

            Event::Mouse(MouseEvent::Press(MouseButton::Left, x, y))  => {
                // eprintln!("coords: {}, {}", y, x);
                // panic!();
                let pos_r = TermPos::new(y, x).unwrap().to_LayoutPos();
                if let Some(pos) = pos_r {
                    let res = state.select(pos);
                    handle_result(res, &tx, &state);
                }
                
            },
            _ => (),
        };
    };

    tx.send(Msg::Quit).expect(threaderr);
    if let Err(x) = handle.join() { panic!("{:?}", x); };

    Ok(())
}
