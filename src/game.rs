use crate::pos::*;
use std::ops::{Index, IndexMut};
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

    fn peek_n(&self, n:usize) -> &[Card] {
        let len = self.cards.len();
        if n >= len { panic!(); };
        let ret = self.cards.get(len-3..len);
        match ret {
            None => panic!(),
            Some(x) => x
        }
    }
}




#[derive(Copy, Clone, Debug)]
pub enum SelectResult {
    Invalid,
    Pending,
    UnPending,
    BadSet(LayoutPos, LayoutPos, LayoutPos),
    GoodSet(LayoutPos, LayoutPos, LayoutPos)
}




#[derive(Clone, Debug)]
pub struct GameState {
    pub deck: Deck,
    pub layout: Layout,
    pub selects: Vec<LayoutPos>,
    pub last_set_found: Option<(Card, Card, Card)>
}

// make layout.fill(vec<cards>) func? that tries to drain vec into layout?

impl GameState {
    pub fn new() -> Self {
        let mut deck = Deck::new();
        let mut cards = [[None; 6]; 3];

        for row in (&mut cards).into_iter(){
            for (i, c) in row.into_iter().enumerate() {
                *c = if i <= 3 { deck.pop() } else { None };
            }
        }

        GameState{deck, layout: Layout{cards}, selects: Vec::new(), last_set_found: None}
    }

    pub fn select(&mut self, pos: LayoutPos) -> SelectResult {
        if self.selects.contains(&pos) {
            self.selects.retain(|&x| x != pos);
            return SelectResult::UnPending;

        } else if self.layout[pos].is_none() {
            return SelectResult::Invalid;

        } else if self.selects.len() <= 1 {
            self.selects.push(pos);
            return SelectResult::Pending;

        } else if self.selects.len() == 2 {
            self.selects.push(pos);
            let err = "selects should have exactly 3 elements here";

            // Should be fine since theoretically none of these pops can fail
            let (p0, p1, p2) = (
                self.selects.pop().expect(err),
                self.selects.pop().expect(err),
                self.selects.pop().expect(err),
            );

            let err = "these positions in layout should be filled if this line is reached";

            if super::is_a_set( self.layout[p0].expect(err),
                                self.layout[p1].expect(err),
                                self.layout[p2].expect(err)) {

                self.selects.clear();
                self.last_set_found = Some((
                    self.layout.remove(p0).unwrap(),
                    self.layout.remove(p1).unwrap(),
                    self.layout.remove(p2).unwrap()));
                return SelectResult::GoodSet(p0, p1, p2);

            } else {
                self.selects.clear();
                self.layout.remove(p0);
                self.layout.remove(p1);
                self.layout.remove(p2);
                return SelectResult::BadSet(p0, p1, p2);
            }
            
        } else {
            panic!("self.selects should never have more than 3 elements");
        }
    }
}




#[derive(Copy, Clone, Debug)]
pub struct Layout {
    // Columns 1-4 are the main section, should always be filled during normal play
    // Columns 5-6 are the extra section, filled only if user thinks there are no sets
    cards: [[Option<Card>; 6]; 3],
}

// make it use setpos instead
impl Index<(u16, u16)> for Layout {
    type Output = Option<Card>;
    
    fn index(&self, (i, j):(u16, u16)) -> &Self::Output {
        &self.cards[usize::from(i)][usize::from(j)]
    }
}

impl IndexMut<(u16, u16)> for Layout {
    fn index_mut(&mut self, (i, j):(u16, u16)) -> &mut Self::Output {
        &mut self.cards[usize::from(i)][usize::from(j)]
    }
}

impl Index<LayoutPos> for Layout {
    type Output = Option<Card>;
    
    fn index(&self, pos:LayoutPos) -> &Self::Output {
        &self.cards[usize::from(pos.row())][usize::from(pos.col())]
    }
}

impl IndexMut<LayoutPos> for Layout {
    fn index_mut(&mut self, pos:LayoutPos) -> &mut Self::Output {
        &mut self.cards[usize::from(pos.row())][usize::from(pos.col())]
    }
}

impl Layout {

    fn iter (&self) -> impl Iterator<Item=&Option<Card>> {
        self.cards.iter().flatten()
    }

    pub fn enumerate_2d (self) -> impl Iterator<Item=(LayoutPos, Option<Card>)>{
        self.cards.into_iter()
            .enumerate()
            .map( move |(i, c_arr)|
                c_arr.into_iter()
                    .enumerate()
                    .map( move |(j, c)| (LayoutPos::new(i as u16, j as u16).unwrap(), c) )
            )
            .flatten()
    }

    pub fn remove(&mut self, p: LayoutPos) -> Option<Card> {
        let spot = self[p];
        self[p] = None;
        spot
    }

    fn count (&self) -> u16 {
        self.iter().filter(|&&c| c != None)
            .count() as u16
    }

    fn empties (&self) -> Vec<LayoutPos> {
        self.enumerate_2d()
            .filter(|&(pos, c)| c == None)
            .map(|(pos, c)| pos)
            .collect()
    }

    fn extras (&self) -> Vec<LayoutPos> {
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
    fn redistribute(&mut self) -> Vec<(LayoutPos, LayoutPos, Card)> {
        let empties: Vec<LayoutPos> = self.empties().
            into_iter()
            .filter(|&pos| pos.col() <= 3)
            .collect();

        let extras = self.extras();
        let mut to_return = vec![];

        let to_fill = std::cmp::min(empties.len(), extras.len());
        let err = "extras should not contain any nones";

        for i in 0..to_fill {
            to_return.push((
                extras[i], 
                empties[i],
                self[extras[i]].expect(err)));

            self[empties[i]] = self[extras[i]];
            self[extras[i]] = None;
        }

        to_return
    }
}
