use crate::deck::*;
use crate::pos::*;

use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Debug)]
pub struct Layout {
    // Columns 1-4 are the main section, should always be filled during normal play
    // Columns 5-6 are the extra section, filled only if user thinks there are no sets
    cards: [[Option<Card>; 6]; 3],
}

impl<T> Index<(T, T)> for Layout where usize: From<T> {
    type Output = Option<Card>;

    fn index(&self, (i, j): (T, T)) -> &Self::Output {
        &self.cards[usize::from(i)][usize::from(j)]
    }
}

impl<T> IndexMut<(T, T)> for Layout where usize: From<T> {
    fn index_mut (&mut self, (i, j): (T, T)) -> &mut Self::Output {
        &mut self.cards[usize::from(i)][usize::from(j)]
    }
}

impl Index<DealtPos> for Layout {
    type Output = Option<Card>;
    
    fn index(&self, pos:DealtPos) -> &Self::Output {
        &self.cards[usize::from(pos.row())][usize::from(pos.col())]
    }
}

impl IndexMut<DealtPos> for Layout {
    fn index_mut(&mut self, pos:DealtPos) -> &mut Self::Output {
        &mut self.cards[usize::from(pos.row())][usize::from(pos.col())]
    }
}

impl Layout {
    pub fn new(cards: [[Option<Card>; 6]; 3]) -> Self {
        Self { cards }
    }
    
    fn iter (&self) -> impl Iterator<Item=&Option<Card>> {
        self.cards.iter().flatten()
    }

    pub fn enumerate_2d (self) -> impl Iterator<Item=(DealtPos, Option<Card>)>{

        // helper function that takes an iterator over a row of the board, and the row index, and
        // creates an iterator over the row, with each card associated with its full DealtPos
        fn distribute_enum((i, row): (usize, [Option<Card>; 6])) -> impl Iterator<Item=(DealtPos, Option<Card>)> {

            // takes a card and corresponding column index to make the enum tuple
            let make_enumerated_card = move |(j, card_option)| {
                (DealtPos::new(
                    u8::try_from(i).unwrap(),
                    u8::try_from(j).unwrap()
                ), card_option)
            };

            row.into_iter()
                .enumerate()
                .map(make_enumerated_card)
        }

        self.cards.into_iter()
            .enumerate()
            .map(distribute_enum)
            .flatten()
    }

    pub fn remove(&mut self, p: DealtPos) {
        // let spot = self[p];
        self[p] = None;
        // spot
    }

    fn count (&self) -> u16 {
        u16::try_from(
            self.iter().filter(|&&c| c != None).count()
        ).unwrap()
    }

    fn empties (&self) -> Vec<DealtPos> {
        self.enumerate_2d()
            .filter(|&(pos, c)| c == None)
            .map(|(pos, c)| pos)
            .collect()
    }

    fn extras (&self) -> Vec<DealtPos> {
        self.enumerate_2d()
            .filter(|&(pos, c)|
                c != None &&
                pos.col() >= 4 )
            .map(|(pos, c)| pos)
            .collect()
    }

    // Tries to fill gaps in the main section using cards from the extra sections
    // Note: if extra sections are empty, NO GAPS ARE FILLED IN MAIN SECTION
    // (main section cards are never moved)
    // Returns Vec of tuples (p0, p1, c) where c is moved card, p0 is c's initial location, and p1
    // is c's final location
    pub fn redistribute(&mut self) -> Vec<(Card, DealtPos, DealtPos)> {
        let empties: Vec<DealtPos> = self.empties().
            into_iter()
            .filter(|&pos| pos.col() <= 3)
            .collect();

        let extras = self.extras();
        let mut to_return = vec![];

        let to_fill = std::cmp::min(empties.len(), extras.len());

        for i in 0..to_fill {
            to_return.push((
                self[extras[i]].expect("extras should not contain any nones"),
                DealtPos::from(extras[i]), 
                DealtPos::from(empties[i]),
            ));

            self[empties[i]] = self[extras[i]];
            self[extras[i]] = None;
        }

        to_return
    }

    pub fn refill(&mut self, deck: &mut Deck) -> Vec<(Card, DealtPos)> {
        let empties: Vec<DealtPos> = self.empties().
            into_iter()
            .filter(|&pos| pos.col() <= 3)
            .collect();

        let to_fill = std::cmp::min(empties.len(), deck.len());
        let mut to_return = vec![];

        for i in 0..to_fill {
            let c = deck.pop().unwrap();
            to_return.push((c, DealtPos::from(empties[i])));
            self[empties[i]] = Some(c);
        }

        to_return
    }

    fn extra3(&mut self, deck: &mut Deck) -> Vec<(GamePos, GamePos, Card)> {
        let empties = self.empties();
        debug_assert!(empties.len() <= 6 && empties.len() >= 3);

        let to_fill = std::cmp::min(3, deck.len());
        let mut to_return = vec![];

        for i in 0..to_fill {
            let c = deck.pop().unwrap();
            to_return.push((GamePos::Deck, GamePos::from(empties[i]), c));
            self[empties[i]] = Some(c);
        }

        to_return
    }
}
