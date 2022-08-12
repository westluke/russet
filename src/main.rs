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

use game::*;


enum Msg {
    Quit,
    Select,
    Deselect,
    RevealSet,
    FoundSet,
    Take3,
    Redistribute,
    GameOver
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

    // let (tx, rx) = mpsc
    
    let stdin = io::stdin();
    let handle = thread::spawn(|| {

        // Perfect yes amazing showstopping incredible. In raw mode all input (including scrolls!) 
        // are consumed by stdin. Interesting, that. The terminal must be reading mouse events from the host OS and
        // re-interpreting them as escape codes? And sending those to child process.
        let mut stdout = MouseTerminal::from(
            io::stdout().into_raw_mode().unwrap()
        );

        let mut buf_stdout = BufWriter::with_capacity(100_000, stdout);
        write!(buf_stdout, "{}{}{}q to exit. Click, click, click!", ToAlternateScreen, clear::All, cursor::Goto(1, 1)).unwrap();

        let c0 = Card{color:CardColor::Green, fill:CardFill::Solid, number:CardNumber::Two, shape:CardShape::Squiggle};

        // let (y, mut x) = (0, 0);

        loop {
            write!(buf_stdout, "{}{}", style::Reset, clear::All);
            let gs = GameState::new();
            // printing::print_card(&mut buf_stdout, y, x, c0);
            // x += 1;
            buf_stdout.flush().unwrap();
            thread::sleep(Duration::from_millis(1000));
        };
        
    });

    handle.join().unwrap();

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Delete) => { write!(io::stdout(), "{}{}", clear::All, ToMainScreen)?; break},
            _ => ()
        }
    }

    Ok(())

}
