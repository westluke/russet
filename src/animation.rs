use termion::{style as ts, color as tc};
use std::{thread, io, time};
use time::{Instant, Duration};


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

pub fn animate() -> io::Result<()> {
    use io::Write as _;
    let mut buf = io::BufWriter::with_capacity(100_000, io::stdout());
    write!(buf, "asdlkfjahsdflkfjh{}", ts::Reset)?;
    Ok(())

    // in a loop, run frame animation
}
