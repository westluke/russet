use std::{thread, io, time};

use time::{Instant, Duration};
use std::{sync::mpsc};
use io::{BufWriter, Write as _};

use crossterm::{style::Color, terminal, execute};

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
    MoveCard(SetPos, SetPos, Card, Color),     // Optional color to be used for the cards border as it moves.
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
        let mut stdout = io::stdout().lock();

        // Swap to alternate screen, clear it (this might just be a scrolled-down version of the
        // main screen, but it's fine bc scroll is disabled in raw mode)
        execute!(
            stdout,
            terminal::EnterAlternateScreen,
            terminal::Clear,
            terminal::SetSize(1, 1),
            terminal::SetTitle("Set!")
        );

        terminal::enable_raw_mode()?;

        // mut cuz we need to adapt to size changes
        let (mut width, mut height) = terminal::size()?;

        let mut start = Instant::now();
        let mut state: Option<GameState> = None;

        // Wrap stdout in a buf that only writes characters when necessary
        // Note: SmartBuf is NOT Write. Which means we can't execute! on it directly.
        let mut buf = SmartBuf::new(stdout); 

        loop {
            let msg = rx.try_recv();
            printing::write_time(&mut buf, start, TermPos::new(height-1, 0)?)?;

            if let Ok(m) = msg {
                match m {
                    Msg::Quit => break,
                    Msg::Base(g) => state = Some(g),
                    Msg::Timed(_, _) => ()
                };
            };

            if state.is_some() {
                printing::print_gamestate(
                    &mut buf,
                    state.as_ref().unwrap()
                )?;
            }

            buf.flush()?;
            thread::sleep(Duration::from_millis(50));
        };

        buf.flush()?;

        // Create a new instance, let original stdout get dropped (and therefore unlocked)
        // thanks to Non-Lexical Lifetimes
        let stdout2 = io::stdout().lock();

        execute!(
            stdout2,
            terminal::Clear,
            terminal::LeaveAlternateScreen,
        );

        Ok(())
    }
}
