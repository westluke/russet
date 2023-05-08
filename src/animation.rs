use std::{thread, io, time, sync};

use time::{Instant, Duration};
use sync::{mpsc::{self, TryRecvError, RecvTimeoutError}};

use crossterm::{terminal, execute};
use log::{info};

use crate::game::{*, ChangeAtom::*};
use crate::pos::*;
use crate::util::*;
use crate::sprites::sprite_tree::{*, InheritanceType::*};
use crate::sprites::sprite_manager::SpriteManager;
use crate::sprites::sprite::Sprite;
use crate::id::*;
use crate::deck::Card;

mod card_repo;
use card_repo::{make, EmbodiedCard, EmbodiedDeck};


// Sent from main thread to animation thread
// ChangeSet and main don't determine animation parameters, this module does
// That includes the amount of time that a card is highlighted, for example
pub enum Msg {
    QuitMsg,
    Nop,
    ChangeMsg(ChangeSet)
}

pub enum BackMsg {
    QuitMsg,

    // hm ok actually this id system is kinda gross for collisions.
    // Can I fix it easily? How do I know what got collided?
    // I think I need a better ID system yeah.
    // So what should an ID be? I think a struct of Option<Card> and Option<String>
    Collisions(Vec<Card>)
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


// fn change_activation(ft: &mut FrameTree, card: Card, name: &'static str, active: bool) {
//     let card_buf = ft.find_mut(&(card.into())).unwrap();
//     let layer = card_buf.find_mut(&(name.into())).unwrap();
//     if active {
//         layer.activate();
//     } else {
//         layer.deactivate();
//     };
// }

// fn show_outline(ft: &mut FrameTree, card: Card) {
//     change_activation(ft, card, "outline", true);
//     change_activation(ft, card, "shadow", false);
// }
// fn hide_outline(ft: &mut FrameTree, card: Card) {
//     change_activation(ft, card, "outline", false);
// }

// fn show_shadow(ft: &mut FrameTree, card: Card) {
//     change_activation(ft, card, "outline", false);
//     change_activation(ft, card, "shadow", true);
// }
// fn hide_shadow(ft: &mut FrameTree, card: Card) {
//     change_activation(ft, card, "shadow", false);
// }

// fn show_good(ft: &mut FrameTree, card: Card) {
//     change_activation(ft, card, "good", true);
//     change_activation(ft, card, "bad", false);
// }
// fn hide_good(ft: &mut FrameTree, card: Card) {
//     change_activation(ft, card, "good", false);
// }

// fn show_bad(ft: &mut FrameTree, card: Card) {
//     change_activation(ft, card, "good", false);
//     change_activation(ft, card, "bad", true);
// }
// fn hide_bad(ft: &mut FrameTree, card: Card) {
//     change_activation(ft, card, "bad", false);
// }

// fn make_active(ft: &mut FrameTree, card: Card) {
//     change_activation(ft, card, "active", true);
//     change_activation(ft, card, "inactive", false);
// }
// fn make_inactive(ft: &mut FrameTree, card: Card) {
//     change_activation(ft, card, "active", false);
//     change_activation(ft, card, "inactive", true);
// }

// JUST HAD A BRAINWAVE. CAN SEPARATE OUT CACHING BY HAVING EVERY SUBTREE STORE A WITNESS (ALL THE SAME WITNESS, THROUGH AN ARC/MUTEX).
// CHILDREN INHERIT WITNESS OF PARENT.
// EVERY UPDATE GETS REGISTERED WITH THE WITNESS. FLUSHING PULLS DATA FROM WITNESS, ONLY EXTRACTS FROM THE NECESSARY TREES.
// CAN CACHE DIRT AND BOUNDS THIS WAY.
// WITNESS KEEPS A LOCAL "DIGITAL DUPLICATE" OF THE ENTIRE TREE, UPDATES WITH EACH REGISTRATION OF CHANGE
//
// Also, this game is russet, but rendering engine should be called russetry


// fn write_time(ft: &mut FrameTree) {
    // find time layer and write to it
// }


pub fn animate(
    rcv: mpsc::Receiver<Msg>,
    click_rcv: mpsc::Receiver<TermPos>,
    snd: mpsc::Sender<BackMsg>
) -> Result<()> {

    // lock standard out to avoid lock thrashing. Already converted to raw mouse terminal by main
    let mut stdout = io::stdout().lock();

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

    info!("animation loop starting");

    let repo9 = make(SIZE_9);
    let repo7 = make(SIZE_7);
    let mut man = SpriteManager::default();
    let mut tree_ids: IdManager<SpriteTree> = Default::default();
    let mut sprite_ids: IdManager<Sprite> = Default::default();

    loop {
        let game_msg = rcv.recv_timeout(Duration::from_millis(10));
        let click_msg = click_rcv.try_recv();

        match click_msg {
            Err(TryRecvError::Disconnected) => break,
            Err(TryRecvError::Empty) => (),
            Ok(pos) => {
                info!("CLICK REGISTERED IN ANIMATION THREAD");
                let mut cards: Vec<Card> = man.tree
                    .collide(pos)
                    .into_iter()
                    .filter_map(|id| sprite_ids.by_id(id).map(|idk| idk.card).flatten())
                    .collect();
                cards.dedup();
                info!("cards clicked: {:?}", cards);

                snd.send(
                    BackMsg::Collisions(cards)
                );
            }
        }

        // SpriteManager should maybe spin up its own thread? Or no, that should be a different object, cuz we might have many sprite managers...
        // Actually, we could pretend that SpriteManager just writes straight to stdout, and even implement it that way at first, but later on
        // make a custom write object that sends writes to another thread to be written to terminal!
        
        match game_msg {
            Err(RecvTimeoutError::Disconnected) | Ok(Msg::QuitMsg) => break,
            Err(RecvTimeoutError::Timeout) => (),
            Ok(Msg::Nop) => continue,
            Ok(Msg::ChangeMsg(cs)) => {
                let ChangeSet { changes, stamp: _ } = cs;
                info!("changeset: {:?}", changes);
                for change in changes {
                    match change {
                        // Reflow(c, _, dst) => {
                        //     info!("REFLOW");
                        //     let card_buf = buf.tree_mut().find_mut(&(c.into())).unwrap();
                        //     card_buf.set_anchor((&dst, &SIZE_7).finto());
                        // },
                       
                        // GoodMove(c, _, dst) => {
                        //     info!("GOODMOVE");
                        //     show_good(buf.tree_mut(), c);
                        //     show_shadow(buf.tree_mut(), c);
                        //     hide_outline(buf.tree_mut(), c);
                        //     make_inactive(buf.tree_mut(), c);

                            
                        //     let card_buf = buf.tree_mut().find_mut(&(c.into())).unwrap();
                        //     card_buf.set_anchor((&dst, &SIZE_7).finto());
                        // },
                        
                        // BadOutline(c, _) => {
                        //     info!("BADOUTLINE");
                        //     show_bad(buf.tree_mut(), c);
                        //     make_inactive(buf.tree_mut(), c);
                        // },
                        // Select(c, _) => {
                        //     info!("SELECT: {:?}", c);
                        //     make_active(buf.tree_mut(), c);
                        // },
                        // Deselect(c, _) => {
                        //     info!("DESELECT");
                        //     make_inactive(buf.tree_mut(), c);
                        // },

                        // Fade(Card, DealtPos),
                        
                        Deal(card, pos) => {
                            info!("DEAL");
                            let EmbodiedCard {mut tree, tree_ids: _tree_ids, sprite_ids: _sprite_ids, ..}= repo7.card(card);
                            tree.reanchor(TermPos::from((&pos, &SIZE_7)), Children);
                            tree.register_dirt(Some(&man.dirt));
                            tree_ids.absorb(_tree_ids);
                            sprite_ids.absorb(_sprite_ids);

                            man.tree.push_tree(tree, Inheritances {anchor: Children, order: Children, ..INHERIT_NONE} );
                            man.refresh_sprites();
                        },
                        _ => ()
                    }
                }
            }
        }

        man.write(&mut stdout);
    }

    info!("animation loop over");

    // Necessary if we exited loop due to error, rather than forward quitmsg
    snd.send(BackMsg::QuitMsg);

    // Create a new instance, let original stdout get dropped (and therefore unlocked)
    // thanks to Non-Lexical Lifetimes
    let mut stdout2 = io::stdout().lock();

    execute!(
        stdout2,
        terminal::Clear(terminal::ClearType::All),
        terminal::LeaveAlternateScreen,
    ).unwrap();

    Ok(())
}
