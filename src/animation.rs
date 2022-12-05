use std::{thread, io, time, sync, collections};

use io::Write;
use time::{Instant, Duration};
use sync::{Arc, Mutex, mpsc::{self, RecvTimeoutError}};
use collections::{HashMap};
use std::hash::{Hash, Hasher};

use crossterm::{terminal, execute, style};
use log::{info};

use crate::game::{*, ChangeAtom::*};
use crate::pos::*;
// use crate::printing;
// use crate::pos::*;
use crate::util::*;
use crate::termchar::*;
use crate::framebuf::{FrameBuf, LayerCell::{self, *}};
use crate::deck::*;


mod cardrepo;
// use cardrepo::CardRepo;




static anim_dur: Duration = Duration::from_millis(500);




// Sent from main thread to animation thread
// ChangeSet and main don't determine animation parameters, this module does
// That includes the amount of time that a card is highlighted, for example

pub enum Msg {
    QuitMsg,
    Nop,
    ChangeMsg(ChangeSet)
}

pub struct ClickMsg(TermPos);

pub enum BackMsg {
    QuitMsg,
    // Collisions(
}

// struct AnimationState {
//     pub buf: FrameBuf,
//     pub anims: Vec<AnimationAtom>
// }

// impl AnimationState {
//     fn new () -> Self {
//         Self {
//             buf: FrameBuf::new(),
//             anims: Vec::new()
//         }
//     }
// }

// struct AnimationAtom {
//     base: ChangeAtom,
//     start: Instant,
//     end: Instant,

//     // For use in "cancelling" by speeding up?
//     // already_cancelled: bool,
    
//     stamp: u32,
// }

// impl AnimationAtom {
//     fn new (base: ChangeAtom, start: Instant, end: Instant) -> Self {
//         Self { base, start, end }
//     }

//     fn expired (&self, i: &Instant) -> bool {
//         i > self.start
//     }

// pub enum Change {
//     BadOutline(Card),
//     GoodMove(Card, LayoutPos),
//     Move(Card, LayoutPos),
//     Fade(Card),
//     Deal(Card, LayoutPos),
//     Select(Card),
//     Deselect(Card)
// }
//

    // issues here: is this animation also keeping tracko f a base state somehow?
    // And how does that interact with deselect?

    // fn involves (&self, card: &Card) -> bool {
    //     use ChangeAtom as CA;

    //     match *self.base {
    //         CA::BadOutline(c, _) => c == *card,
    //         CA::GoodMove(c, _, _) => c == *card,
    //         CA::Move(c, _, _) => c == *card,
    //         CA::Fade(c, _) => c == *card,
    //         CA::Deal(c, _) => c == *card,
    //         CA::Select(c, _) => c == *card,
    //         CA::Deselect(c, _) => c == *card,
    //     }
    // }

    // fn involves (&self, lp: &LayoutPos) -> bool {
    //     use ChangeAtom as CA;

    //     match *self.base {
    //         CA::Move(_, l0, l1) => l == *lp,
    //         CA::GoodMove(_, l0, l1) => l == *lp,
    //         CA::BadOutline(_, l) => l == *lp,
    //         CA::Select(_, l) => l == *lp,
    //         CA::Deselect(_, l) => l == *lp,
    //         CA::Fade(_, l) => l == *lp,
    //         CA::Deal(_, l) => l == *lp,
    //     }
    // }

    // should things even cancel? Well, not yet they shouldn't. This isn't required functionality.
    // stahp for now.

    // fn cancelled (&self, anim: &AnimationAtom) -> bool {
    //     use ChangeAtom as CA;

    //     if (anim.stamp <= self.stamp) { return false; };
            

    //     match *self.base {
    //         CA::Move(_, l0, l1) => match anim.base {
    //             CA::BadOutline(_, l) => (l == l1),
    //             CA::Select(_, l) => (l == l1),
    //             CA::Select(_, l) => (l == l1),
    //         }
    //         CA::BadOutline(c, _, _) =>
    //             anim.stamp >= self.stamp,
    //         CA::GoodMove(c, lp) => 
    //             anim.involves(lp) && match anim.base {
    //                 CA::Move(
    //                 CA::
                    
    //         },
    //     }
        
    // }
// }

// impl From<&ChangeSet> for Vec<AnimationAtom> {
//     fn from(cst: &ChangeSet) -> Self {
//         let mut result = Vec::new();
//         let now = Instant::now();

//         for &chng in &cst.changes {
//             result.push(
//                 AnimationAtom {
//                     base: chng,
//                     start: now,
//                     end: now + anim_dur
//                 }
//             );
//         };

//         result
//     }
// }


// fn ChangeSet_to_AnimationAtoms (cst: ChangeSet) -> Vec<AnimationAtom> {
// }



pub fn sleep_until(i: Instant) {
    loop {
        let start = Instant::now();
        if start > i { return; }
        let diff = i.duration_since(start) + Duration::new(0, 10);
        thread::sleep(diff);
    };
}

// pub fn interp(
//         src:    TermPos, 
//         dst:    TermPos,
//         start:  Instant,
//         end:    Instant,
//         now:    Instant) -> TermPos {

//     if end < now { return dst; };
//     if now < start { return src; };

//     let space_diff = dst - src;
//     let time_diff = end - start;
//     let ratio = (end - now).as_secs_f64() / time_diff.as_secs_f64();

//     src + (space_diff * ratio);
// }




// Returns whether the game is over or not
// Could be because of error, or because of requested quit
// pub fn animate_frame(
//     cst: ChangeSet,
//     state: &mut AnimationState) {

//     use ChangeAtom as CA;

//     // let now = Instant::now();
//     let (width, height) = ts.update();
//     // state.anims.append(Vec::<AnimationAtom>::from(cst));

//     // Ignore animations for now!! Just do immediate updates
//     // perhaps layers iin the buf should be grouped with their badoutline/goodoutline/
//     // selected versions? So those  versions don't have to  be cloned at runtime?
//     // May not be necessary.

//     for chng in &cst.changes {
//         match *chng {
//             CA::Move(c, _, l1) => state.buf.move_layer(c, l1),
//             CA::GoodMove(c, _, l1) => state.buf.move_layer(c, l1),
//             CA::BadOutline(c, _) => state.buf.replace_layer(c, _),
//             CA::Select(c, _) => state.buf.replace_layer(c, _),
//             CA::Deselect(c, _) => state.buf.replace_layer(c, _),
//             CA::Fade(c, _) => state.buf.delete_layer(c),
//             CA::Dead(c, l1) => state.buf.insert_layer(c, l1),
//         }
//     }

//     state.buf.flush();
// }

    // for anim in &state.anims {
    //     let st = anim.start;
    //     let nd = anim.end;

    //     match anim.base {
    //         CA::Move(c, l0, l1) => {
    //             let newpos = interp(l0, l1, st, end, now);
    //             anim.buf.move_layer(c, newpos);
    //         },
    //         CA::GoodMove(c, l0, l1) => {
    //         }
    //     }
    // }

        // have to update the animations, cancel them if necessary,
        // then go through them and, for each one, determine the necessary changes
        // to corresponding framebuflayers

    // how do i handle the cst...

    // buf.flush();


    // if state.is_some() {
    //     printing::print_gamestate(
    //         &mut buf,
    //         state.as_ref().unwrap()
    //     )?;
    // }

    // buf.flush();
    // thread::sleep(Duration::from_millis(200));
// }




pub fn animate(
    game_rcv: mpsc::Receiver<Msg>,
    click_rcv: mpsc::Receiver<ClickMsg>,
    back_snd: mpsc::Sender<BackMsg>
) -> Result<()> {

    // lock standard out to avoid lock thrashing. Already converted to raw mouse terminal by main
    let mut stdout = io::stdout();//.lock();

    // Swap to alternate screen, clear it (this might just be a scrolled-down version of the
    // main screen, but it's fine bc scroll is disabled in raw mode)
    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        terminal::Clear(terminal::ClearType::All),
        terminal::SetSize(1, 1),
        terminal::SetTitle("Set!")
    ).unwrap();

    // mut cuz we need to adapt to size changes
    let (mut width, mut height) = TS.update();
    debug_assert!(width > 0 && height > 0);

    let mut start = Instant::now();

    // GONNA NEED A BETTER SYSTEM FOR KEEPING TRACK OF WHICH KAYER IS WHICH
    // ALSO NEED TO RMEMEBER TO FIX UP LAYER DISPLAY SYSTEM
    
    // let repo7 = CardRepo::new(SIZE_7, TERM_BG, CARD_BG);
    // let repo9 = CardRepo::new(SIZE_9, TERM_BG, CARD_BG);
    // let mut buf = FrameBuf::new(stdout); 
    // let mut deck = repo7.get_deck();
    // let mut deck_active = repo7.get_deck_active();

    let mut deck_id = 0;
    let mut deck_active_id = 1;
    // let mut card_ids: HashMap<(Card, bool), u64> = HashMap::new();

    // deck_active.deactivate();
    // deck.set_anchor((&GamePos::Deck, &SIZE_7).finto());
    // deck_active.set_anchor((&GamePos::Deck, &SIZE_7).finto());

    // buf.push_layer(deck);
    // buf.push_layer(deck_active);

    info!("animation loop starting");

    // Exits only due to error or quit message
    // What should this behavior be like?
    // Obviously if we quit because of a quitmsg, we should complete the rest of the exit
    // procedure. But what about an error? 
    
    loop {
        let game_msg = game_rcv.recv_timeout(Duration::from_millis(200));
        let click_msg = click_rcv.try_recv();
        
        if let Ok(click_msg) = click_msg {
            // Do collision calculations
            // continue so we can handle more possible clicks
            // backx.send(collidemsg);
            continue;
        }

        match game_msg {
            Err(RecvTimeoutError::Disconnected) | Ok(Msg::QuitMsg) => break,
            Err(RecvTimeoutError::Timeout) => (),
            Ok(Msg::Nop) => continue,
            Ok(Msg::ChangeMsg(cs)) => {
                let ChangeSet { changes, stamp } = cs;
                for change in changes {
                    match change {
                        Deal(card, pos) => {
                            // let mut card_lay = repo7.get_card_active(card);
                            // let mut card_lay = repo7.get_card(card);
                            // card_lay.set_anchor(TermPos::from((&pos, &SIZE_7)) + TermPos::ffrom((1, 0)));
                            // buf.push_layer(card_lay);
                        },
                        _ => ()
                    }
                }
            }
        }

        // buf.flush();
    }

    info!("animation loop over");

    // Necessary if we exited loop due to error, rather than forward quitmsg
    back_snd.send(BackMsg::QuitMsg);

    // gamestate shows all statically visible cards.
    // I think, it might make sense later for GameSTate to be keeping an internal record of all
    // changes, all of which get sent to animation, tagged with their timestamps?
    // But should I do that now?


    // Create a new instance, let original stdout get dropped (and therefore unlocked)
    // thanks to Non-Lexical Lifetimes
    let mut stdout2 = io::stdout().lock();

    execute!(
        stdout2,
        terminal::Clear(terminal::ClearType::All),
        terminal::LeaveAlternateScreen,
    ).unwrap();

    Ok(())

    // res
}
