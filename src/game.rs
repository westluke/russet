use crate::pos::*;
use crate::deck::{Card, Deck};
use crate::layout::*;
use std::collections::HashSet;
use crate::util::*;

use std::ops::{Index, IndexMut};
use rand::seq::SliceRandom as _;

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

fn find_set(lay:Layout) -> Option<[DealtPos; 3]> {
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

// Can extend Card definition to include UID later, if necessary.
// But it's a good way to just index.
// But then this brings back the question: what gets canceled by what?
// I think this is still better than the current system. So I can
// figure out cancelling later.

// Note: in order to be unambiguous, and to be able to instantiate one of these changes
// even on an otherwise blank bnaord, we need to include cards AND layoutpos, all the time

// Represents changes that have occurred in the gamestate with VISUAL consequences
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum ChangeAtom {
    Reflow(Card, DealtPos, DealtPos),
    GoodMove(Card, DealtPos, GamePos),
    BadOutline(Card, DealtPos),
    Select(Card, DealtPos),
    Deselect(Card, DealtPos),
    Fade(Card, DealtPos),
    Deal(Card, DealtPos),
}

// stamp is handy for identifying which came later in a more concrete way than instants
#[derive(Clone, Debug)]
pub struct ChangeSet {
    pub changes: HashSet<ChangeAtom>,
    pub stamp: u32
}

impl ChangeSet {
    pub fn new(changes: HashSet<ChangeAtom>, stamp: u32) -> Self {
        Self { changes, stamp}
    }
}

impl Default for ChangeSet {
    fn default() -> Self {
        Self { changes: HashSet::new(), stamp: 0}
    }
}

#[derive(Clone, Debug)]
pub struct GameState {
    deck: Deck,
    layout: Layout,
    last_set_found: Option<(Card, Card, Card)>,
    selects: Vec<Card>,
    changesets: Vec<ChangeSet>,
    id_counter: u32
}

// GameState update should take one of these instead of just layoutPos
// pub enum UserInteraction {
//    SelectCard,
//    FindSet,
//    NewGame
// }

impl Default for GameState {
    fn default() -> Self {
        let mut deck = Deck::new();
        let mut cards = [[None; 6]; 3];
        let mut changesets = Vec::new();
        let mut cs = HashSet::new();

        for (row_i, row) in (&mut cards).into_iter().enumerate() {
            for (col_i, cel) in row.into_iter().enumerate() {
                if col_i <= 3 {
                    *cel = deck.pop();
                    cs.insert(ChangeAtom::Deal(cel.unwrap(), DealtPos::new(row_i.finto(), col_i.finto())));
                }
            }
        }

        changesets.push(ChangeSet::new(cs, 0));

        GameState {
            deck,
            layout: Layout::new(cards),
            last_set_found: None,
            selects: Vec::new(),
            changesets,
            id_counter: 1
        }
    }
}

impl GameState {
    pub fn changes(&mut self) -> Vec<ChangeSet> {
        std::mem::take(&mut self.changesets)
    }

    pub fn has_changes(&self) -> bool {
        !self.changesets.is_empty()
    }

    /// given a user's selection of cards, adjusts game state and self.changes to indicate
    /// necessary next steps
    pub fn select(&mut self, card: Card) {

        debug_assert!(
            self.enumerate_cards()
                .find(|(_, c)| c.is_some() && c.unwrap() == card)
                .is_some());

        let mut chs = HashSet::new();

        if self.selects.contains(&card) {
            self.selects.retain(|&x| x != card);
            chs.insert(ChangeAtom::Deselect(card, self.find(card)));
        }

        else if self.selects.len() <= 1 {
            self.selects.push(card);
            chs.insert(ChangeAtom::Select(card, self.find(card)));
        }

        else if self.selects.len() == 2 {
            self.selects.push(card);

            let (c0, c1, c2) = (
                self.selects.pop().unwrap(),
                self.selects.pop().unwrap(),
                self.selects.pop().unwrap()
            );

            let (p0, p1, p2) = (
                self.find(c0),
                self.find(c1),
                self.find(c2),
            );

            if is_a_set(c0, c1, c2) {

                for p in [p0, p1, p2] { self.layout.remove(p); };
                self.selects.clear();
                self.last_set_found = Some((c0, c1, c2));

                chs.insert(ChangeAtom::GoodMove(c0, p0, GamePos::LastFound0));
                chs.insert(ChangeAtom::GoodMove(c1, p1, GamePos::LastFound1));
                chs.insert(ChangeAtom::GoodMove(c2, p2, GamePos::LastFound2));

                for (c, l0, l1) in self.layout.redistribute() {
                    chs.insert(ChangeAtom::Reflow(c, l0, l1));
                };

                for (c, l) in self.layout.refill(&mut self.deck) {
                    chs.insert(ChangeAtom::Deal(c, l));
                };

            } else {
                self.selects.clear();
                chs.insert(ChangeAtom::BadOutline(c0, p0));
                chs.insert(ChangeAtom::BadOutline(c1, p1));
                chs.insert(ChangeAtom::BadOutline(c2, p2));
            }
        } else {
            panic!("self.selects should never have more than 3 elements");
        }

        self.changesets.push(ChangeSet::new(chs, self.id_counter));
        self.id_counter += 1;
    }

    fn find(&self, card: Card) -> DealtPos {
        let (pos, _) = self.enumerate_cards()
            .filter(|(_, c)| c.is_some() && c.unwrap() == card)
            .next()
            .unwrap();
        pos
    }

    pub fn enumerate_cards(&self) -> impl Iterator<Item=(DealtPos, Option<Card>)> {
        self.layout.enumerate_2d()
    }

    // pub fn selected(&self, pos: DealtPos) -> bool {
    //     self.selects.contains(&pos)
    // }

    pub fn last_set_found(&self) -> Option<(Card, Card, Card)> {
        self.last_set_found
    }
}
