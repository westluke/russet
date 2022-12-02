use std::{io, thread, time};
use std::env;

use std::sync::{Arc, Mutex, mpsc, mpsc::TryRecvError};
use time::{Duration};

use crossterm::event::{
    self, read, poll,
    Event, KeyEvent, KeyEventKind, MouseEvent, MouseEventKind,
    KeyCode, KeyModifiers, MouseButton};

use crossterm::{terminal, execute};

use log::{info};
use flexi_logger::{FileSpec, Logger, WriteMode};


mod animation;
// mod printing;
mod game;
mod util;

pub mod pos;
pub mod deck;
pub mod framebuf;
pub mod layout;
pub mod termchar;

use game::*;
use animation::*;
use framebuf::*;
use util::*;
// use pos::*;
// use deck::*;

// pub use game::all_cards;




// fn key_to_LayoutPos(c: char) -> Option<LayoutPos> {
//     match c {
//         'q' | 'Q' => Some(LayoutPos::new(0, 0)),
//         'w' | 'W' => Some(LayoutPos::new(0, 1)),
//         'e' | 'E' => Some(LayoutPos::new(0, 2)),
//         'r' | 'R' => Some(LayoutPos::new(0, 3)),
//         't' | 'T' => Some(LayoutPos::new(0, 4)),
//         'y' | 'Y' => Some(LayoutPos::new(0, 5)),
//         'a' | 'A' => Some(LayoutPos::new(1, 0)),
//         's' | 'S' => Some(LayoutPos::new(1, 1)),
//         'd' | 'D' => Some(LayoutPos::new(1, 2)),
//         'f' | 'F' => Some(LayoutPos::new(1, 3)),
//         'g' | 'G' => Some(LayoutPos::new(1, 4)),
//         'h' | 'H' => Some(LayoutPos::new(1, 5)),
//         'z' | 'Z' => Some(LayoutPos::new(2, 0)),
//         'x' | 'X' => Some(LayoutPos::new(2, 1)),
//         'c' | 'C' => Some(LayoutPos::new(2, 2)),
//         'v' | 'V' => Some(LayoutPos::new(2, 3)),
//         'b' | 'B' => Some(LayoutPos::new(2, 4)),
//         'n' | 'N' => Some(LayoutPos::new(2, 5)),
//         _ => None
//     }
// }


            // Event::Key(
            //     KeyEvent{ code, .. }
            // ) => match code {
            //     KeyCode::Delete |
            //     KeyCode::Backspace => {
            //         break;
            //     },

            //     KeyCode::Char(c) => {
            //         let pos_r = key_to_LayoutPos(c);
            //         if let Some(pos) = pos_r {
            //             let res = state.select(pos);
            //             handle_result(res, &tx, &state)?;
            //         };
            //     }
            //     _ => {}
            // },
                // MouseEvent{
                //     kind:MouseEventKind::Down(MouseButton::Left),
                //     column:x, row:y, ..
                // }
            // ) => {
                // let pos_r = TermPos::new(y, x).to_LayoutPos();
                // if let Some(pos) = pos_r {
                //     let res = state.select(pos);
                //     handle_result(res, &tx, &state)?;
                // };
            // },

fn handle_key_event(kc: KeyCode) -> Result<FrameResult>{
    match kc {
        KeyCode::Backspace | KeyCode::Delete => Ok(FrameResult::Quit),
        _ => Ok(FrameResult::Nop)
    }
}

fn handle_mouse_event(column: u16, row: u16) -> Result<FrameResult>{
    Ok(FrameResult::Nop)
}

enum FrameResult {
    Quit,
    Nop,
    Msgs(Vec<Msg>)
}

fn input_frame(state: &GameState) -> Result<FrameResult> {
    if poll(Duration::from_millis(200))? {
        return match read()? {
            Event::Key(
                KeyEvent {
                    code: kc,
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                }
            ) => handle_key_event(kc),
            Event::Mouse(
                MouseEvent {
                    kind: MouseEventKind::Down(MouseButton::Left),
                    column,
                    row,
                    modifiers: KeyModifiers::NONE
                }
            ) => handle_mouse_event(column, row),
            _ => Ok(FrameResult::Nop),
        };
    };

    Ok(FrameResult::Nop)
}




fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    let (sx, rx) = mpsc::channel::<animation::Msg>();
    let (sx_back, rx_back) = mpsc::channel::<animation::BackMsg>();

    terminal::enable_raw_mode()?;
    execute!(io::stdout(), event::EnableMouseCapture)?;

    let _logger = Logger::try_with_str("info")?
        .log_to_file(FileSpec::default().basename("log").suppress_timestamp().suffix("txt"))
        .write_mode(WriteMode::Direct)
        .start()?;

    let gs = GameState::default();
    let (y, x) = TS.dims();

    // what is the right solution here??
    //
    // Ok whats the issues: animation needs to know about gamestate and/or changes to gamestate.
    // main needs to know which panel a click hits. Is that the only information it needs?
    // yeah, actually. Ok, so why not query it via channels? and have animation use recv_timeout to
    // sleep, inistead of fixed frame timeout.
    // sure that might not be super speedy, but neither is mutex locking. And it's a little bit
    // cleaner, I think. makes the interfaces very explicit. and the performance is negligible!!!
    // Stop worrying about it dumbfuck. There's an inherent dependence here anyways, because the
    // frames are moving. Only avoidable if using a cache, and there's no way im doing that.
    // ok u gotta sleep dude

    let handle = thread::spawn(|| {
        animation::animate(rx, sx_back)
    });

    // let initsend = tx.send(Msg::Base(Clone::clone(&state)));

    // idea: to avoid excessive buffering on holding keydown, 
    // impose time limit on pressing the same key twice.

    loop {
        match rx_back.try_recv() {
           Err(TryRecvError::Disconnected) | Ok(_) => break, 
           _ => ()
        }

        match input_frame(&gs) {
            Err(_) => break,
            Ok(FrameResult::Quit) => break,
            _ => ()
        }
    };

    info!("loop over");

    terminal::disable_raw_mode()?;

    info!("after disabling raw mode");

    info!("after stdout capture");

    if let Err(x) = sx.send(Msg::QuitMsg){
        info!("Failed to send Quit message to animation thread, err: {:?}", x);
    }

    info!("Waiting to join animation thread...");

    // Set timeout on join, we don't want to wait too long.
    if let Err(x) = handle.join(){
        info!("Joining animation thread returned err: {:?}", x);
    }

    // Can't executed on stdout until animation thread drops its lock
    execute!(io::stdout(), event::DisableMouseCapture)?;

    Ok(())
}
