use crate::pos::*;
use crate::deck::{Card, Deck};
use crate::layout::*;

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

// Can extend Card definition to include UID later, if necessary.
// But it's a good way to just index.
// But then this brings back the question: what gets canceled by what?
// I think this is still better than the current system. So I can
// figure out cancelling later.

// Note: in order to be unambiguous, and to be able to instantiate one of these changes
// even on an otherwise blank bnaord, we need to include cards AND layoutpos, all the time

// Represents changes that have occurred in the gamestate with VISUAL consequences
#[derive(Copy, Clone, Debug)]
pub enum ChangeAtom {
    Reflow(Card, LayoutPos, LayoutPos),
    GoodMove(Card, LayoutPos, GamePos),
    BadOutline(Card, LayoutPos),
    Select(Card, LayoutPos),
    Deselect(Card, LayoutPos),
    Fade(Card, LayoutPos),
    Deal(Card, LayoutPos),
}

// stamp is handy for identifying which came later in a more concrete way than instants
#[derive(Clone, Debug)]
pub struct ChangeSet {
    changes: Vec<ChangeAtom>,
    stamp: u32
}

impl ChangeSet {
    pub fn new(changes: Vec<ChangeAtom>, stamp: u32) -> Self {
        Self { changes, stamp}
    }
}

impl Default for ChangeSet {
    fn default() -> Self {
        Self { changes: Vec::new(), stamp: 0}
    }
}

#[derive(Clone, Debug)]
pub struct GameState {
    deck: Deck,
    layout: Layout,
    last_set_found: Option<(Card, Card, Card)>,
    selects: Vec<LayoutPos>,
    changes: Vec<ChangeSet>,
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

        for row in (&mut cards).into_iter(){
            for (i, c) in row.into_iter().enumerate() {
                *c = if i <= 3 { deck.pop() } else { None };
            }
        }

        GameState {
            deck,
            layout: Layout::new(cards),
            last_set_found: None,
            selects: Vec::new(),
            changes: Vec::new(),
            id_counter: 0
        }
    }
}

impl GameState {
    pub fn pop_changeset(&mut self) -> ChangeSet {
        self.changes.pop().unwrap()
    }

    pub fn has_changes(&self) -> bool {
        !self.changes.is_empty()
    }

    /// given a user's selection of cards, adjusts game state and self.changes to indicate
    /// necessary next steps
    pub fn select(&mut self, pos: LayoutPos) {

        let card_at = match self.layout[pos] {
            None => return,
            Some(x) => x
        };

        let mut chs = Vec::new();

        if self.selects.contains(&pos) {
            self.selects.retain(|&x| x != pos);
            chs.push(ChangeAtom::Deselect(card_at, pos));
        }

        else if self.selects.len() <= 1 {
            self.selects.push(pos);
            chs.push(ChangeAtom::Select(card_at, pos));
        }

        else if self.selects.len() == 2 {
            self.selects.push(pos);

            let (p0, p1, p2) = (
                self.selects.pop().unwrap(),
                self.selects.pop().unwrap(),
                self.selects.pop().unwrap()
            );

            let (c0, c1, c2) = (
                self.layout[p0].unwrap(),
                self.layout[p1].unwrap(),
                self.layout[p2].unwrap()
            );

            if is_a_set(c0, c1, c2) {

                for p in [p0, p1, p2] { self.layout.remove(p); };
                self.selects.clear();
                self.last_set_found = Some((c0, c1, c2));

                chs.push(ChangeAtom::GoodMove(c0, p0, GamePos::LastFound0));
                chs.push(ChangeAtom::GoodMove(c1, p1, GamePos::LastFound1));
                chs.push(ChangeAtom::GoodMove(c2, p2, GamePos::LastFound2));

                for (c, l0, l1) in self.layout.redistribute() {
                    chs.push(ChangeAtom::Reflow(c, l0, l1));
                };

                for (c, l) in self.layout.refill(&mut self.deck) {
                    chs.push(ChangeAtom::Deal(c, l));
                };

            } else {
                self.selects.clear();
                chs.push(ChangeAtom::BadOutline(c0, p0));
                chs.push(ChangeAtom::BadOutline(c1, p1));
                chs.push(ChangeAtom::BadOutline(c2, p2));
            }
        } else {
            panic!("self.selects should never have more than 3 elements");
        }

        self.changes.push(ChangeSet::new(chs, self.id_counter));
        self.id_counter += 1;
    }

    pub fn enumerate_cards(&self) -> impl Iterator<Item=(LayoutPos, Option<Card>)> {
        self.layout.enumerate_2d()
    }

    pub fn selected(&self, pos: LayoutPos) -> bool {
        self.selects.contains(&pos)
    }

    pub fn last_set_found(&self) -> Option<(Card, Card, Card)> {
        self.last_set_found
    }
}
