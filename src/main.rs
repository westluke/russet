use std::{io, thread, time, marker};
use std::env;
use std::fmt::{Display, Formatter};

use std::sync::{Arc, Mutex, mpsc, mpsc::TryRecvError};
use time::{Duration};

use crossterm::event::{
    self, read, poll,
    Event, KeyEvent, KeyEventKind, MouseEvent, MouseEventKind,
    KeyCode, KeyModifiers, MouseButton};

use crossterm::{terminal, execute};

use log::{info};
use flexi_logger::{FileSpec, Logger, WriteMode};










/////////////////////////////////////////////////
/// SEPARATE CLICKABILITY AND VISIBILITY!!!! WHILE MOVING CARDS SHOULD NOT BE CLICKABLE
/// OR THERE SPOT SHOULD BE CLICKABLE? IDK THINK ABOUIT IT
////////////////////////////////////////////////



// main handles input, sends msgs to animation containing all the information animation needs to
// render. animation handles, well, ongoing animations and writing to screen.
//
// Eventually, will have third thread, taking care of doing the actual low-level writing to
// terminal and waiting for those to complete.









mod animation;
// mod printing;
mod game;
mod util;

pub mod pos;
pub mod deck;
pub mod sprites;
pub mod layout;
pub mod term_char;
pub mod bounds;
pub mod id;

use game::*;
use animation::*;
use util::*;
pub use id::*;
pub use pos::TermPos;


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

// Result of interpreting input
enum InputResult {
    Quit,
    Nop,
    Msgs(Vec<Msg>),
    Click(TermPos)
}

// Result of interpreting collision reported by animation thread.
enum BackMsgResult {
    Quit,
    Nop,
    Msgs(Vec<Msg>)
}

fn handle_key_event(kc: KeyCode) -> InputResult {
    match kc {
        KeyCode::Backspace | KeyCode::Delete => InputResult::Quit,
        _ => InputResult::Nop
    }
}

fn handle_back_msg(state: &mut GameState, msg: std::result::Result<BackMsg, TryRecvError>) -> BackMsgResult {
    match msg {
        Err(TryRecvError::Disconnected)
        | Ok(BackMsg::QuitMsg) => return BackMsgResult::Quit,
        Err(TryRecvError::Empty) => return BackMsgResult::Nop,
        Ok(BackMsg::Collisions(mut cards)) => 
            if let Some(c) = cards.pop() {
                state.select(c);
                let csets = state.changes();
                let msgs = csets.into_iter().map(|c| Msg::ChangeMsg(c)).collect();
                BackMsgResult::Msgs(msgs)
            } else {
                BackMsgResult::Nop
            }
    }
}

fn handle_input_frame(state: &mut GameState, input: crossterm::Result<Event>) -> InputResult {
    use InputResult::*;
    use Event::*;

    if let Err(_) = input { return Quit; };
    let input = input.unwrap();

    return match input {
        Key(
            KeyEvent {
                code: kc,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press, ..
            }
        ) => handle_key_event(kc),

        Mouse(
            MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Left),
                column,
                row,
                modifiers: KeyModifiers::NONE
            }
        ) => {
            // info!("CLICK DETECTED!!");
            Click((row, column).finto())
        }

        Resize(y, x) => Quit,
        _ => Nop
    };
}

fn main() -> Result<()> {

    env::set_var("RUST_BACKTRACE", "1");

    let (snd, anim_rcv) = mpsc::channel::<animation::Msg>();
    let (click_snd, click_rcv) = mpsc::channel::<TermPos>();
    let (anim_snd, rcv) = mpsc::channel::<animation::BackMsg>();

    terminal::enable_raw_mode()?;
    execute!(io::stdout(), event::EnableMouseCapture)?;

    let _logger = Logger::try_with_str("info")?
        .log_to_file(FileSpec::default().basename("log").suppress_timestamp().suffix("txt"))
        .write_mode(WriteMode::Direct)
        .start()?;

    let mut gs = GameState::default();

    let handle = thread::spawn(|| {
        animation::animate(anim_rcv, click_rcv, anim_snd)
    });

    for chng in gs.changes() {
        snd.send(Msg::ChangeMsg(chng));
    }

    // idea: to avoid excessive buffering on holding keydown, 
    // impose time limit on pressing the same key twice.
    //
    // Also, I think maybe screen size changes should be detected HERE, rather than in animation.

    loop {
        match handle_back_msg(&mut gs, rcv.try_recv()) {
            BackMsgResult::Quit => break,
            BackMsgResult::Nop => (),
            BackMsgResult::Msgs (msgs) => {
                for msg in msgs {
                    snd.send(msg);
                };
            }
        };

        if poll(Duration::from_millis(10))? {
            match handle_input_frame(&mut gs, read()) {
                InputResult::Quit => break,
                InputResult::Msgs(msgs) => {
                    for msg in msgs {
                        snd.send(msg);
                    }
                }
                InputResult::Click(cmsg) => { click_snd.send(cmsg); }
                _ => ()
            }
        }
    };

    terminal::disable_raw_mode()?;

    if let Err(x) = snd.send(Msg::QuitMsg){
        info!("Failed to send Quit message to animation thread, err: {:?}", x);
    }

    // Set timeout on join, we don't want to wait too long.
    if let Err(x) = handle.join(){
        info!("Joining animation thread returned err: {:?}", x);
    }

    // Can't executed on stdout until animation thread drops its lock
    execute!(io::stdout(), event::DisableMouseCapture)?;

    Ok(())
}
