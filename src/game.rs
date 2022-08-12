use std::io;
use std::ops::{Index, IndexMut};
use super::printing;

use rand::seq::SliceRandom as _;




#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub enum CardColor {
    Green, 
    Red,
    Purple
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub enum CardShape {
    Oval,
    Diamond,
    Squiggle
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub enum CardNumber {
    One=1, Two=2, Three=3
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub enum CardFill {
    Solid, Striped, Empty
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct Card {
    pub color: CardColor,
    pub shape: CardShape,
    pub number: CardNumber,
    pub fill: CardFill
}




#[derive(Clone, Debug)]
pub struct Deck {
    cards: Vec<Card>
}

impl Deck {
    fn new() -> Deck {
        let colors = [CardColor::Green, CardColor::Red, CardColor::Purple];
        let shapes = [CardShape::Oval, CardShape::Diamond, CardShape::Squiggle];
        let numbers = [CardNumber::One, CardNumber::Two, CardNumber::Three];
        let fills = [CardFill::Solid, CardFill::Striped, CardFill::Empty];

        let mut cards = Vec::with_capacity(81);

        for c in colors.iter() {
            for s in shapes.iter() {
                for n in numbers.iter() {
                    for f in fills.iter() {
                        cards.push(Card{color:*c, shape:*s, number:*n, fill:*f});
                    }
                }
            }
        }

        cards.shuffle(&mut rand::thread_rng());
        Deck{cards}
    }

    fn pop(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    fn is_empty(&self) -> bool {
        return self.cards.is_empty();
    }
}




pub struct GameState {
    pub deck: Deck,
    pub layout: Layout,
    pub selected: [Option<(usize, usize)>; 3]
}

impl GameState {
    pub fn new () -> Self {
        let mut deck = Deck::new();
        let mut cards = [[None; 6]; 3];

        for row in (&mut cards).into_iter(){
            for (i, c) in row.into_iter().enumerate() {
                *c = if i <= 3 { deck.pop() } else { None };
            }
        }

        GameState{deck, layout: Layout{cards}, selected: [None; 3]}
    }
}





#[derive(Copy, Clone, Debug)]
pub struct Layout {
    // Columns 1-4 are the main section, should always be filled during normal play
    // Columns 5-6 are the extra section, filled only if user thinks there are no sets
    cards: [[Option<Card>; 6]; 3],
}

impl Index<(usize, usize)> for Layout {
    type Output = Option<Card>;
    
    fn index(&self, (i, j):(usize, usize)) -> &Self::Output {
        &self.cards[i][j]
    }
}

impl IndexMut<(usize, usize)> for Layout {
    fn index_mut(&mut self, (i, j):(usize, usize)) -> &mut Self::Output {
        &mut self.cards[i][j]
    }
}

impl Layout {

    fn iter (&self) -> impl Iterator<Item=&Option<Card>> {
        self.cards.iter().flatten()
    }

    pub fn enumerate_2d (self) -> impl Iterator<Item=((usize, usize), Option<Card>)>{
        self.cards.into_iter()
            .enumerate()
            .map( move |(i, c_arr)|
                c_arr.into_iter()
                    .enumerate()
                    .map( move |(j, c)| ((i, j), c) )
            )
            .flatten()
    }

    fn count (&self) -> u16 {
        self.iter().filter(|&&c| c != None)
            .count() as u16
    }

    fn empties (&self) -> Vec<(usize, usize)> {
        self.enumerate_2d()
            .filter(|&((i, j), c)| c == None)
            .map(|((i, j), c)| (i, j))
            .collect()
    }

    fn extras (&self) -> Vec<(usize, usize)> {
        self.enumerate_2d()
            .filter(|&((i, j), c)|
                c != None &&
                j >= 4 )
            .map(|((i, j), c)| (i, j))
            .collect()
    }

    pub fn print (&self, buf: &mut impl io::Write) {
        for ((i, j), c_opt) in self.enumerate_2d(){
            if let Some(c) = c_opt {
                printing::print_card(buf, i as u16, j as u16, c);
            }
        }
    }

    // Tries to fill gaps in the main section using cards from the extra sections
    // Note: if extra sections are empty, NO GAPS ARE FILLED IN MAIN SECTION
    // (main section cards are never moved)
    // Returns Vec of tuples (p0, p1, c) where c is moved card, p0 is c's initial location, and p1
    // is c's final location
    fn redistribute(&mut self) -> Vec<((usize, usize), (usize, usize), Card)> {
        let empties: Vec<(usize, usize)> = self.empties().
            into_iter()
            .filter(|&(i, j)| j <= 3)
            .collect();
        let extras = self.extras();
        let mut to_return = vec![];

        let to_fill = std::cmp::min(empties.len(), extras.len());
        for i in 0..to_fill {
            to_return.push((extras[i], empties[i], self[extras[i]].unwrap()));
            self[empties[i]] = self[extras[i]];
            self[extras[i]] = None;
        }

        to_return
    }
        
    // // Fills the first 3 empty spots found
    // // Returns true if change was made
    // fn take3 (&mut self, d:Deck) -> bool{
    //     let empties = self.empties();
    //     if empties.is_empty() { return false; };
    //     debug_assert!(empties.len() % 3 == 0);

    //     if d.is_empty() || empties.is_empty() {
    //         return false;
    //     }

    //     for e in empties {
    //         self[e] = d.
            
    //     }

    //     true
    // }

}
