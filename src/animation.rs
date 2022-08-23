use std::{thread, io, time};

use time::{Instant, Duration};
use std::{sync::mpsc};
use io::Write as _;
use io::BufWriter;

use termion::{style, clear, cursor, color};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::IntoRawMode;
use termion::screen::{ToAlternateScreen, ToMainScreen};

use super::game::*;
use super::printing;
use super::pos::*;




// Animation can't use underlying gamestate to know which cards to print, since they may be out of
// date. So for each animation itll need to be TOLD which cards to use.
//
// Now, do I want to send msgs in abstractc form to this module, which then creates the actual
// closure'd animations? Or are the animations fully constructed when initially sent?
//
// I think the latter makes more sense. Cuz otherwise we make extrra work for animation.rs in
// figuring out which cards to use. So, instead, animation.rs should provide a bunch of
// animation-constructing funcctions? maybe?
//
// Nah cuz that also relies on understanding set game structure. I think main.rs should do all the
// work of constructing animations.
//
// So, what do animations look like? Well, for any given instant, they should provide a stringi to
// be printed, and the string includes where to move on the screen. They have a type, and they have
// a stopping time (optional) and conditions under which they are canceled.
//
// how does this module interact with animations?
pub enum Msg {
    Quit,
    Base(GameState), // always rendered
    Select(SetPos),               // Canceled by deselect at same loc
    Deselect(SetPos),             // does not induce animation
    // thats gross. time to do pixelpos/rowpos? yeah i think so.
    RevealSet(SetPos, SetPos, SetPos),                      // Canceled by ANY subsequent messages
    FoundSet,                       // Uncancellable finite-time animation
    Take3,                          // Uncancellable finite-time animation
    Redistribute,                   // Uncancellable finite-time animation
    GameOver
}

// How do Ii specify iniitial state / card animations?
// don't want to make it dependnet on referring back to the gamestate.
// so it should be a card, plus two points, expressed in abstract (not pixel) notation.

// and it should also be informed of changes to the underlying base game state.


// struct Animation {
// }

// impl Animation {
//     fn next(sz:u32) -> String {
//     }
// }

pub fn sleep_until(i: Instant) {
    loop {
        let start = Instant::now();
        if start > i { return; }
        let diff = i.duration_since(start) + Duration::new(0, 10);
        thread::sleep(diff);
    };
}

pub fn animate(rx: mpsc::Receiver<Msg>) -> impl (FnOnce() -> ()) {
    // let mut animations = vec![];

    move || {
        let mut stdout = MouseTerminal::from(io::stdout().lock()).into_raw_mode().unwrap();

        let (mut width, mut height) = termion::terminal_size().unwrap();
        write!(stdout, "{}{}{}", ToAlternateScreen, clear::All, cursor::Hide).unwrap();

        let mut start = Instant::now();


        // in a loop, run frame animation
        // check terminal size with termion::terminal_size, compare to last known.
        // if there are no new events and no animations in the buffer, don't clear screen (no
        // point)
        // could also maybe specify which animations require clearing screen?
        loop {
            // write!(buf_stdout, "{}{}", style::Reset, clear::All);
            let msg = rx.try_recv();
            printing::write_time(&mut stdout, start, height, 1);

            let gs = GameState::new();
            gs.layout.print(&mut stdout);

            if let Ok(m) = msg {
                match m {
                    Msg::Quit => { 
                        write!(stdout, "{}{}{}", clear::All, cursor::Show, ToMainScreen).unwrap();
                        stdout.suspend_raw_mode();
                        stdout.flush();
                        return ();
                    },
                    Msg::Select{row, col} => {
                        printing::print_card_outline(&mut stdout, row, col, color::Yellow);
                        
                    },
                    _ => ()
                }
            }

            // x += 1;
            stdout.flush().unwrap();
            thread::sleep(Duration::from_millis(1000));
        };
    }
}
