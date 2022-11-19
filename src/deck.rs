use rand::seq::SliceRandom;

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

pub fn all_cards() -> Vec<Card> {
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

    cards
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = all_cards();
        cards.shuffle(&mut rand::thread_rng());
        Deck{cards}
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn pop(&mut self) -> Option<Card> {
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
