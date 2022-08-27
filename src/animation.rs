use std::{thread, io, time};

use time::{Instant, Duration};
use std::{sync::mpsc};
use io::Write as _;
use io::BufWriter;

use termion::{style, clear, cursor, color};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::IntoRawMode;
use termion::color::Color;
use termion::screen::{ToAlternateScreen, ToMainScreen};

use super::game::*;
use super::printing;
use super::pos::*;


pub enum Msg {
    Quit,
    Base(GameState),                // Canceled by later Base
    Timed(Vec<TimedMsg>, Instant)   // These have to be separate cuz they all happen at the same time
}

// Pending status of cards can just be obtained from the gamestate. The only thing that
// animation.rs really needs help with is movement - interpolating between two gamestates.
// That's something that kinda needs game knowledge behind it.

pub enum TimedMsg {
    StaticCard(SetPos, Option<Card>),                               // Placeholder for both ends during a MoveCard
    MoveCard(SetPos, SetPos, Card, Option<&'static dyn Color>),     // Optional color to be used for the cards border as it moves.
    BadOutline(SetPos),                                             // Red border that flashes when the user incorrectly tries to select a set
}

impl Msg {
    fn print(&self, buf: &mut impl io::Write, i0: Instant) -> io::Result<()> {
        match self {
            &Msg::Quit => panic!(),     // Something's wrong if we're trying to print a Quit message.
            &Msg::Base(g) => printing::print_gamestate(buf, g),
            &Msg::Timed(t_msgs, i1) => {
                for msg in t_msgs {
                    match msg {
                        TimedMsg::StaticCard
                        TimedMsg::MoveCard
                        TimedMsg::BadOutline
                    }
                };
            
            }
        }
    }

    // // TimedMsg's cannot be canceled.
    // // Quit cannot be canceled.
    // // Base is canceled by later base.
    // fn canceled(&self, later_msg: &Msg) -> bool {
    //     match self {
    //         &Msg::Quit => false,
    //         &Msg::Base(b0) => match later_msg {
    //             &Msg::Base(_) => true,
    //             _ => false
    //         },
    //         &Msg::Pending(pos0) => match later_msg {
    //             &Msg::Pending(pos0) => true,
    //             &Msg::ClearPending(pos0) => true,
    //             _ => false,
    //         },
    //         &Msg::ClearPending(pos0) => true,
    //         &x => false
    //     }
    // }

    fn timed_out(&self, i0: Instant) -> bool {
        match self {
            &Msg::Timed(_, i1) => i0 >= i1,
            _ => false
        }
    }
}

pub fn sleep_until(i: Instant) {
    loop {
        let start = Instant::now();
        if start > i { return; }
        let diff = i.duration_since(start) + Duration::new(0, 10);
        thread::sleep(diff);
    };
}



pub fn animate(rx: mpsc::Receiver<AnimationMsg>) -> impl (FnOnce() -> ()) {
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
            printing::write_time(&mut stdout, start, TermPos::new(height, 1));

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
