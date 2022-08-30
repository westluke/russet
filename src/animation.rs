use std::{thread, io, time};

use time::{Instant, Duration};
use std::{sync::mpsc};
use io::Write as _;

use termion::{style, clear, cursor, color};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::IntoRawMode;
use termion::color::Color;
use termion::screen::{ToAlternateScreen, ToMainScreen};

use crate::game::*;
use crate::printing;
use crate::pos::*;
use crate::SetError as SE;



pub enum Msg {
    Quit,
    Base(GameState),                // Canceled by later Base
    Timed(Vec<TimedMsg>, Instant)   // These have to be separate cuz they all happen at the same time
}

pub enum TimedMsg {
    StaticCard(SetPos, Option<Card>),                               // Placeholder for both ends during a MoveCard
    MoveCard(SetPos, SetPos, Card, &'static (dyn Color + Sync)),     // Optional color to be used for the cards border as it moves.
    BadOutline(LayoutPos),                                             // Red border that flashes when the user incorrectly tries to select a set
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

pub fn animate(rx: mpsc::Receiver<Msg>) -> impl (FnOnce() -> Result<(), SE>) {
    move || {

        // lock standard out to avoid lock thrashing, then convert it to a MouseTerminal and set it
        // to raw mode. Don't think order matters for mouse/raw
        let mut stdout = MouseTerminal::from(io::stdout().lock())
                            .into_raw_mode()?;

        // mut cuz we need to adapt to size changes
        let (mut width, mut height) = termion::terminal_size()?;

        // Swap to alternate screen, clear it (this might just be a scrolled-down version of the
        // main screen, but it's fine bc scroll is disabled in raw mode)
        write!(stdout, "{}{}{}", ToAlternateScreen, clear::All, cursor::Hide)?;

        let mut start = Instant::now();
        let mut state: Option<GameState> = None;

        loop {
            write!(stdout, "{}{}{}", style::Reset, color::Fg(color::Yellow), clear::All)?;

            let msg = rx.try_recv();
            printing::write_time(&mut stdout, start, TermPos::new(height, 1)?)?;

            if let Ok(m) = msg {
                match m {
                    Msg::Quit => break,
                    Msg::Base(g) => state = Some(g),
                    Msg::Timed(_, _) => ()
                };
            };

            if state.is_some() {
                printing::print_gamestate(
                    &mut stdout,
                    state.as_ref().unwrap()
                )?;
            }

            stdout.flush()?;
            thread::sleep(Duration::from_millis(100));
        };

        write!(stdout, "{}{}{}", clear::All, cursor::Show, ToMainScreen)?;
        stdout.suspend_raw_mode()?;
        stdout.flush()?;

        Ok(())
    }
}
