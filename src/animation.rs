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



// better idea: make the game background-agnostic.
// ok no, cuz i think i need to denote the cards using a white background anyways.
// otherwise itll look weird.
// ok so it's not background-agnostic, it just has to contrast with white.
// incorrect sets flash red (ENTIRE CARD flashes red)
// correct sets flash green, then move
// pending sets? idk yet. Maybe pending spot flashes yellow, and gets halo of card-colored pluses?
// text gets printed in white, on black background (customizable eventually)
//


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
    MoveCard(SetPos, SetPos, Card, &'static (dyn Color + Sync)),     // Optional color to be used for the cards border as it moves.
    BadOutline(SetPos),                                             // Red border that flashes when the user incorrectly tries to select a set
}

impl Msg {
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



pub fn animate(rx: mpsc::Receiver<Msg>) -> impl (FnOnce() -> ()) {
    // let mut animations = vec![];

    move || {
        let mut stdout = MouseTerminal::from(io::stdout().lock()).into_raw_mode().unwrap();

        let (mut width, mut height) = termion::terminal_size().unwrap();
        write!(stdout, "{}{}{}", ToAlternateScreen, clear::All, cursor::Hide).unwrap();

        let mut start = Instant::now();
        let mut state: Option<GameState> = None;


        loop {
            write!(stdout, "{}{}{}", style::Reset, color::Fg(color::Yellow), clear::All);

            let msg = rx.try_recv();
            printing::write_time(&mut stdout, start, TermPos::new(height, 1));

            if let Ok(m) = msg {
                match m {
                    Msg::Quit => { 
                        write!(stdout, "{}{}{}", clear::All, cursor::Show, ToMainScreen).unwrap();
                        stdout.suspend_raw_mode();
                        stdout.flush();
                        return ();
                    },
                    Msg::Base(g) => state = Some(g),
                    Msg::Timed(_, _) => ()
                };
            };

            if state.is_some() {
                printing::print_gamestate(&mut stdout, state.as_ref().unwrap());
            }

            stdout.flush().unwrap();
            thread::sleep(Duration::from_millis(100));
        };
    }
}
