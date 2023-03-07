use std::{io, thread, time};
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
use uuid::Uuid;









/////////////////////////////////////////////////
/// SEPARATE CLICKABILITY AND VISIBILITY!!!! WHILE MOVING CARDS SHOULD NOT BE CLICKABLE
/// OR THERE SPOT SHOULD BE CLICKABLE? IDK THINK ABOUIT IT
////////////////////////////////////////////////












mod animation;
// mod printing;
mod game;
mod util;

pub mod pos;
pub mod deck;
pub mod sprite;
pub mod layout;
pub mod term_char;

use game::*;
use animation::*;
use util::*;
use deck::*;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Id {
    uuid: Uuid,
    card: Option<Card>,
    name: Option<String>
}

impl Default for Id {
    fn default() -> Self {
        Self { uuid: Uuid::new_v4(), ..Default::default() }
    }
}

impl Display for Id {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "Id({}", self.uuid)?;

        if let Some(c) = self.card {
            write!(fmt, ", {}", c)?;
        }

        if let Some(n) = self.name.clone() {
            write!(fmt, ", {}", n)?;
        };

        write!(fmt, ")")
    }
}

impl From<Card> for Id {
    fn from(c: Card) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            card: Some(c),
            name: None
        }
    }
}

// impl From<String> for Id {
//     fn from(s: String) -> Self {
//         Self {
//             uuid: Uuid::new_v4(),
//             card: None,
//             name: Some(s)
//         }
//     }
// }

impl From<&str> for Id {
    fn from(s: &str) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            card: None,
            name: Some(s.into())
        }
    }
}

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

enum FrameResult {
    Quit,
    Nop,
    Msgs(Vec<Msg>),
    Click(ClickMsg)
}

fn handle_key_event(kc: KeyCode) -> FrameResult {
    match kc {
        KeyCode::Backspace | KeyCode::Delete => FrameResult::Quit,
        _ => FrameResult::Nop
    }
}

fn handle_back_msg(state: &mut GameState, msg: std::result::Result<BackMsg, TryRecvError>) -> FrameResult {
    match msg {
        Err(TryRecvError::Disconnected)
        | Ok(BackMsg::QuitMsg) => return FrameResult::Quit,
        Err(TryRecvError::Empty) => return FrameResult::Nop,
        Ok(BackMsg::Collisions(None)) => return FrameResult::Nop,
        Ok(BackMsg::Collisions(Some(v))) => {
            // info!("HANDLING COLLISIONS IN MAIN");
            // info!("Collision IDs
            for id in v {
                // info!("ID: {}", id);
                if let Some(c) = id.card {
                    state.select(c);
                    let csets = state.changes();
                    let msgs = csets.into_iter().map(|c| Msg::ChangeMsg(c)).collect();
                    return FrameResult::Msgs(msgs);
                }
            };
            return FrameResult::Nop;
        }
    }
}

fn handle_input_frame(state: &mut GameState, input: crossterm::Result<Event>) -> FrameResult {
    use FrameResult::*;
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
            Click(ClickMsg((row, column).finto()))
        }

        Resize(y, x) => FrameResult::Quit,

        _ => FrameResult::Nop
    };
}

fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");

    let (game_snd, game_rcv) = mpsc::channel::<animation::Msg>();
    let (click_snd, click_rcv) = mpsc::channel::<animation::ClickMsg>();
    let (back_snd, back_rcv) = mpsc::channel::<animation::BackMsg>();

    terminal::enable_raw_mode()?;
    execute!(io::stdout(), event::EnableMouseCapture)?;

    let _logger = Logger::try_with_str("info")?
        .log_to_file(FileSpec::default().basename("log").suppress_timestamp().suffix("txt"))
        .write_mode(WriteMode::Direct)
        .start()?;

    let mut gs = GameState::default();

    let handle = thread::spawn(|| {
        animation::animate(game_rcv, click_rcv, back_snd)
    });

    for chng in gs.changes() {
        game_snd.send(Msg::ChangeMsg(chng));
    }

    // idea: to avoid excessive buffering on holding keydown, 
    // impose time limit on pressing the same key twice.
    //
    // Also, I think maybe screen size changes should be detected HERE, rather than in animation.

    loop {
        match handle_back_msg(&mut gs, back_rcv.try_recv()) {
            FrameResult::Quit => break,
            FrameResult::Nop => (),
            FrameResult::Msgs (msgs) => {
                for msg in msgs {
                    game_snd.send(msg);
                };
            }
            FrameResult::Click (cmsg) => {
                click_snd.send(cmsg);
            }
        };

        if poll(Duration::from_millis(10))? {
            match handle_input_frame(&mut gs, read()) {
                FrameResult::Quit => break,
                FrameResult::Msgs(msgs) => {
                    for msg in msgs {
                        game_snd.send(msg);
                    }
                }
                FrameResult::Click(cmsg) => { click_snd.send(cmsg); }
                _ => ()
            }
        }
    };

    terminal::disable_raw_mode()?;

    if let Err(x) = game_snd.send(Msg::QuitMsg){
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
