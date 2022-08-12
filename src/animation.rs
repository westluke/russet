use std::{thread, io, time};

use time::{Instant, Duration};
use std::{sync::mpsc};
use io::Write as _;

use termion::style;




pub enum Msg {
    Quit,
    Select,
    Deselect,
    RevealSet,
    FoundSet,
    Take3,
    Redistribute,
    GameOver
}


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

pub fn animate(buf: impl io::Write, rx: mpsc::Receiver<Msg>) -> io::Result<()> {
    let mut buf = io::BufWriter::with_capacity(100_000, io::stdout());
    write!(buf, "asdlkfjahsdflkfjh{}", style::Reset)?;
    Ok(())

    // in a loop, run frame animation
    // check terminal size with termion::terminal_size, compare to last known.
}
