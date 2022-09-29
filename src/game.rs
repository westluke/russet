use crate::pos::*;
use std::ops::{Index, IndexMut};
use rand::seq::SliceRandom as _;





#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug, Hash)]
pub enum CardColor {
    Color1, 
    Color2,
    Color3
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug, Hash)]
pub enum CardShape {
    Oval,
    Diamond,
    Squiggle
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug, Hash)]
pub enum CardNumber {
    One=1, Two=2, Three=3
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug, Hash)]
pub enum CardFill {
    Solid, Striped, Empty
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug, Hash)]
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
        let colors = [CardColor::Color1, CardColor::Color2, CardColor::Color3];
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

    fn len(&self) -> usize {
        self.cards.len()
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

    fn take3(&mut self) -> Option<(Card, Card, Card)> {
        if self.cards.len() >= 3 {
            Some((self.pop().unwrap(), self.pop().unwrap(), self.pop().unwrap()))
        } else {
            None
        }
    }
}




#[derive(Clone, Debug)]
pub enum SelectResult {
    Invalid,
    Pending,
    UnPending,
    BadSet(LayoutPos, LayoutPos, LayoutPos),
    GoodSet(LayoutPos, LayoutPos, LayoutPos, Vec<(SetPos, SetPos, Card)>)
}




#[derive(Clone, Debug)]
pub struct GameState {
    deck: Deck,
    layout: Layout,
    selects: Vec<LayoutPos>,
    last_set_found: Option<(Card, Card, Card)>
}




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

    // given a user's selection of cards, adjusts game state and returns SelectResult to indicate
    // changes / necessary animations
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

                let mut moves = self.layout.redistribute();
                moves.append(&mut self.layout.refill(&mut self.deck));
                return SelectResult::GoodSet(p0, p1, p2, moves);

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

        // helper function that takes an iterator over a row of the board, and the row index, and
        // creates an iterator over the row, with each card associated with its full LayoutPos
        fn distribute_enum((i, row): (usize, [Option<Card>; 6])) -> impl Iterator<Item=(LayoutPos, Option<Card>)> {

            // takes a card and corresponding column index to make the enum tuple
            let make_enumerated_card = move |(j, card_option)| {
                (LayoutPos::new(
                    u16::try_from(i).unwrap(),
                    u16::try_from(j).unwrap()
                ).unwrap(), card_option)
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

    pub fn remove(&mut self, p: LayoutPos) -> Option<Card> {
        let spot = self[p];
        self[p] = None;
        spot
    }

    fn count (&self) -> u16 {
        u16::try_from(
            self.iter().filter(|&&c| c != None).count()
        ).unwrap()
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
    fn redistribute(&mut self) -> Vec<(SetPos, SetPos, Card)> {
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
                SetPos::from(extras[i]), 
                SetPos::from(empties[i]),
                self[extras[i]].expect(err)));

            self[empties[i]] = self[extras[i]];
            self[extras[i]] = None;
        }

        to_return
    }

    fn refill(&mut self, deck: &mut Deck) -> Vec<(SetPos, SetPos, Card)> {
        let empties: Vec<LayoutPos> = self.empties().
            into_iter()
            .filter(|&pos| pos.col() <= 3)
            .collect();

        let to_fill = std::cmp::min(empties.len(), deck.len());
        let mut to_return = vec![];

        for i in 0..to_fill {
            let c = deck.pop().unwrap();
            to_return.push((SetPos::Deck, SetPos::from(empties[i]), c));
            self[empties[i]] = Some(c);
        }

        to_return
        
    }

    fn extra3(&mut self, deck: &mut Deck) -> Vec<(SetPos, SetPos, Card)> {
        let empties = self.empties();
        debug_assert!(empties.len() <= 6 && empties.len() >= 3);

        let to_fill = std::cmp::min(3, deck.len());
        let mut to_return = vec![];

        for i in 0..to_fill {
            let c = deck.pop().unwrap();
            to_return.push((SetPos::Deck, SetPos::from(empties[i]), c));
            self[empties[i]] = Some(c);
        }

        to_return
    }
}
