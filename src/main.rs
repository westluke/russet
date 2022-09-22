use std::{io, thread, time};
use std::fmt::Write as _;

use std::sync::mpsc::{self, SendError};
use time::{Instant, Duration};

use crossterm::event::{self, read, poll, Event, KeyEvent, KeyCode, MouseEvent, MouseEventKind, MouseButton};
use crossterm::terminal;
use crossterm::execute;

use log::{info, warn, error};
use flexi_logger::{FileSpec, Logger, WriteMode, FlexiLoggerError};


mod animation;
mod printing;
mod game;
mod config;

pub mod pos;
pub mod smartbuf;

use game::*;
use animation::*;
use pos::*;

// Should send a message to the other thread to shut down gracefully
const threaderr: &str = "thread communication should never fail";


pub type Result<T> = std::result::Result<T, SetError>;

// const test: Result<(), ()> = Ok(());

#[derive(Debug)]
pub enum SetErrorKind {
    Io(io::Error),
    TryFromInt(std::num::TryFromIntError),
    Fmt,
    SmallScreen,
    LayoutOob,
    SendErr,
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

impl From<std::num::TryFromIntError> for SetError {
    fn from(err: std::num::TryFromIntError) -> Self {
        Self::new(SetErrorKind::TryFromInt(err), "tryfromint error detected")
    }
}

impl From<std::fmt::Error> for SetError {
    fn from(err: std::fmt::Error) -> Self {
        Self::new(SetErrorKind::Fmt, "fmt error detected")
    }
}

impl From<SendError<Msg>> for SetError {
    fn from(err: SendError<Msg>) -> Self {
        Self::new(SetErrorKind::SendErr, "send error detected")
    }
}

impl From<FlexiLoggerError> for SetError {
    fn from(err: FlexiLoggerError) -> Self {
        Self::new(SetErrorKind::SendErr, "send error detected")
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

fn handle_result (res: SelectResult, tx: &mpsc::Sender<Msg>, state: &GameState) -> Result<()>{
    match res {
        SelectResult::Pending |
        SelectResult::UnPending => {
            let res = tx.send(
                Msg::Base(
                    Clone::clone(
                        &state
                    )
                )
            );
            if res.is_err() { info!("failed to send message to animation thread, err: {:?}", res); };
            res?;
        },

        SelectResult::BadSet(p0, p1, p2) => {
            let res = tx.send(
                Msg::Timed(
                    vec![TimedMsg::BadOutline(p0), TimedMsg::BadOutline(p1), TimedMsg::BadOutline(p2)],
                    Instant::now() + Duration::from_secs(1)
            ));

            if res.is_err() { info!("failed to send message to animation thread, err: {:?}", res); };
            res?;

            let res = tx.send(
                Msg::Base(
                    Clone::clone(
                        &state
                    )
                )
            );

            if res.is_err() { info!("failed to send message to animation thread, err: {:?}", res); };
            res?;
        },

        SelectResult::GoodSet(p0, p1, p2, _) => {
            let res = tx.send(
                Msg::Base(
                    Clone::clone(
                        &state
                    )
                )
            );

            if res.is_err() { info!("failed to send message to animation thread, err: {:?}", res); };
            res?;
        },
        _ => ()
    };

    Ok(())
}

        // Event::Key(Key::Delete) | 
        // Event::Key(Key::Backspace) | 
        // Event::Key(Key::Ctrl('c')) |
        // Event::Key(Key::Ctrl('z')) |
        // Event::Key(Key::Ctrl('\\')) => break,

        // Event::Key(Key::Char(c)) => {
        // },

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel::<animation::Msg>();
    let stdin = io::stdin();

    terminal::enable_raw_mode()?;
    execute!(io::stdout(), event::EnableMouseCapture)?;
    info!("Raw mode enabled");

    let _logger = Logger::try_with_str("info")?
        .log_to_file(FileSpec::default().basename("log").suppress_timestamp().suffix("txt"))
        .write_mode(WriteMode::Direct)
        .start()?;

    let handle = thread::spawn(|| { animation::animate(rx) });
    info!("Animation thread spawned");

    let res = || -> Result<()> {

        let mut state = GameState::new();
        let initsend = tx.send(Msg::Base(Clone::clone(&state)));

        // idea: to avoid excessive buffering on holding keydown, 
        // impose time limit on pressing the same key twice.

        loop {
            if poll(Duration::from_millis(200))? {
                match read()? {

                    Event::Key(
                        KeyEvent{ code, .. }
                    ) => match code {
                        KeyCode::Delete |
                        KeyCode::Backspace => {
                            break;
                        },

                        KeyCode::Char(c) => {
                            let pos_r = key_to_LayoutPos(c);
                            if let Some(pos) = pos_r {
                                let res = state.select(pos);
                                handle_result(res, &tx, &state)?;
                            };
                        }
                        _ => {}
                    },

                    Event::Mouse(
                        MouseEvent{
                            kind:MouseEventKind::Down(MouseButton::Left),
                            column:x, row:y, ..
                        }
                    ) => {
                        let pos_r = TermPos::new(y, x).unwrap().to_LayoutPos();
                        if let Some(pos) = pos_r {
                            let res = state.select(pos);
                            handle_result(res, &tx, &state)?;
                        };
                    },
                    _ => (),
                };
            };
        };

        Ok(())
    } ();
    info!("Closure exited");

    terminal::disable_raw_mode()?;
    execute!(io::stdout(), event::DisableMouseCapture)?;
    info!("Raw mode exited");

    if let Err(x) = tx.send(Msg::Quit){
        info!("Failed to send Quit message to animation thread, err: {:?}", x);
    }

    info!("Waiting for animation thread to be joined");

    if let Err(x) = handle.join(){
        info!("Joining animation thread returned err: {:?}", x);
    }

    Ok(())
}
