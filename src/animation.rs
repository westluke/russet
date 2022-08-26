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



// Animation can't use underlying gamestate to know which cards to print, since they may be out of
// date. So for each animation itll need to be TOLD which cards to use.
//
// Now, do I want to send msgs in abstractc form to this module, which then creates the actual
// closure'd animations? Or are the animations fully constructed when initially sent?
//
// I think the latter makes more sense. Cuz otherwise we make extrra work for animation.rs in
// figuring out which cards to use. So, instead, animation.rs should provide a bunch of
// animation-constructing funcctions? maybe?
//
// Nah cuz that also relies on understanding set game structure. I think main.rs should do all the
// work of constructing animations.
//
// So, what do animations look like? Well, for any given instant, they should provide a stringi to
// be printed, and the string includes where to move on the screen. They have a type, and they have
// a stopping time (optional) and conditions under which they are canceled.
//
// how does this module interact with animations?
//
// actually I do wanna limit the traffic across the channel, so main just sends the information
// necessary to CONSTRUCT the animation.
//
// And how are animations cancelled?
//
// aaaagh should i send over the cards as well? Or just send a gamestate and have them reference
// using the setpos? Now imi kinda feeling the latter, that seems like a more high-level approach
// that maiin.rs should take. but then what about transitional gamestates? Agh butt thats full of
// stuff that animation.rs doesn't and shouldn't keep track of
//
// Ah. I see. the messages should be much finer-grained. move this  card from here to here, for
// instance.
//
// how to handle persistence?
//
// also cards should have white background


// there CANT be any delay until you're allowed to select cards. So how do I design the animations
// around that?
//
// For instance, say a card A is being moved from the top of a deck to a just-vacated space in the
// layout (vacated by card B), which is currently marked by a static blank card
//
// And a user inputs a set that includes card A in that currently-blank space. Then what happens?
//
// option 1: Card A redirects mid-flight to discard pile. that kinda sounds like a nightmare to
// implement, and would also be visually unclear.
//
// option 2: Card A teleports to the vacant space, then begins animation to disccard pile. I'd
// prefer to avoid this, it ruins the whole continuous-movement thing.
//
// option 3: Card A finishes animation to vacant space, perhaps faster now, then immediately begnis
// animation to discard pile. I think this is probably the best option.
//
// So that implies that previous animation BLOCKS must finish before a new block can start. More
// precisely, I could make it so that any animation at a particular SetPos must finish before any
// new animations AT THAT SETPOS can start.
//
// Except, this isn't the case for all animations!! Outlines shouldn't be dependent on other
// animations, they can always play. They should happen instantly. no waiting.
//
// So its just staticcard and movecard that have waiting relations?
// yup and they come in blocks so some single actions can have multiple animation responses.
// also badoutlines are temporary, but they're the only temporary outline. hm.


// other potential msgs: updatescore, resettime?

pub enum Msg {
    Quit,
    Base(GameState),                // Canceled by later Base
    Pending(SetPos),                // Canceled by ClearPending. A yellow striped background to the spot/card that indicates a partial set selection
    ClearPending(SetPos),           // Canceled by itself!
    Timed(Vec<TimedMsg>, Instant)   // These have to be separate cuz they all happen at the same time
}

pub enum TimedMsg {
    StaticCard(SetPos, Option<Card>),                               // Placeholder for both ends during a MoveCard
    MoveCard(SetPos, SetPos, Card, Option<&'static dyn Color>),     // Optional color to be used for the cards border as it moves.
    BadOutline(SetPos),                                             // Red border that flashes when the user incorrectly tries to select a set
}

// ok, the question of what cancels what and what WAITS ON what are different questions. hmm.
// Ok, there's a couple things to note. one, goodoutlines shouldn't appear aroun the card SPOT,
// they should appear on the card itself. which will begin moving immediately. So that implieis
// they can't be cancelled or anything.
//
// can i make the pending outline not 

impl Msg {
    fn to_string(&self, i: Instant) -> String {
        String::new()
    }

    // TimedMsg's cannot be canceled.
    // Quit cannot be canceled.
    // Base is canceled by later base.
    // PendingOutline is canceled by 
    fn reduce(&self, later_msg: &Msg) -> Option<Msg> {
        None
    }
}

// how to convert block to animations that are easily presentable?
// they should be in impl of animationmsgblock
    

// How do Ii specify iniitial state / card animations?
// don't want to make it dependnet on referring back to the gamestate.
// so it should be a card, plus two points, expressed in abstract (not pixel) notation.

// and it should also be informed of changes to the underlying base game state.


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
