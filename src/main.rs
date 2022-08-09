use termion::{color as tc, style as ts, clear as tcl};
use termion::screen::{self, AlternateScreen}; //, screen, input, event, cursor, clear};
use std::{io, thread, time};
use time::{Instant, Duration};
use std::{vec::Vec, collections::HashSet, sync::mpsc};

mod animation;
mod printing;

// #[derive(Clone, Debug)]
// struct GameState {
//     deck: HashSet<Card>,
//     in_play: HashSet<Card>
// }

// impl GameState {
//     fn take_3 (&mut self) {
//     }
// }

// #[derive(Copy, Clone, Debug)]
// enum CardSpot<'a> {
//     Empty,
//     Filled(&'a Card)
// }

// #[derive(Copy, Clone, Debug)]
// struct Layout<'a, 'b> {
//     main: [[&'a CardSpot<'b>; 4]; 3],
//     add0: [&'a CardSpot<'b>; 3],
//     add1: [&'a CardSpot<'b>; 3]
// }

// impl<'a, 'b> Layout<'a, 'b> {
//     fn fill (self, game:&GameState){
//     }
// }

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
enum CardColor {
    Green, 
    Red,
    Purple
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
enum CardShape {
    Oval,
    Diamond,
    Squiggle
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
enum CardNumber {
    One=1, Two=2, Three=3
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
enum CardFill {
    Solid, Striped, Empty
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct Card {
    color: CardColor,
    shape: CardShape,
    number: CardNumber,
    fill: CardFill
}

fn is_a_set(c0:Card, c1:Card, c2:Card) -> bool {
    let mut colors = vec![c0.color, c1.color, c2.color];
    let mut shapes = vec![c0.shape, c1.shape, c2.shape];
    let mut numbers = vec![c0.number, c1.number, c2.number];
    let mut fills = vec![c0.fill, c1.fill, c2.fill];
    colors.sort();
    colors.dedup();
    shapes.sort();
    shapes.dedup();
    numbers.sort();
    numbers.dedup();
    fills.sort();
    fills.dedup();

    (colors.len() == 1 || colors.len() == 3) &&
    (shapes.len() == 1 || shapes.len() == 3) &&
    (numbers.len() == 1 || numbers.len() == 3) &&
    (fills.len() == 1 || fills.len() == 3)
}

fn main() -> std::io::Result<()> {
    // let (tx, rx) = mpsc::channel::<fn(u32) -> String>();

    use io::Write as _;


    println!("{}", screen::ToAlternateScreen);
    // printing::mv_cursor(&mut io::stdout(), 1, 1);
    io::stdout().flush();
    thread::sleep(Duration::new(4, 0));
    io::stdout().flush();
    // println!("{} Writing to main screen!", tcl::AfterCursor);
    thread::sleep(Duration::new(4, 0));

    // print!("{}", screen::ToAlternateScreen);
    // println!("{}", tcl::All);
    // print!("{}", screen::ToMainScreen);
    // println!("{}", tcl::All);
    // print!("{}", screen::ToAlternateScreen);
    // print!("{}", screen::ToMainScreen);
    // io::stdout().flush();


    // println!("hello there");
    // println!("visible?");
    // println!("{}", screen::ToMainScreen);
    Ok(())

    // let handle = thread::spawn(|| {
    //     use io::Write as _;
    //     let mut buf = io::BufWriter::with_capacity(100_000, io::stdout());
    //     let c0 = Card{color: CardColor::Green, shape: CardShape::Squiggle, number: CardNumber::Three, fill: CardFill::Striped};
    //     write!(buf, "{}", screen::ToAlternateScreen);

    //     let (mut x, y) = (1, 1);
    //     loop {
    //         // go through list of animations, animate one frame of each.
    //         // try receiving new animations into the list
    //         // sleep until next frame
            
    //         // write!(buf, "{}", tcl::All);
    //         printing::print_card_yx(&mut buf, y, x, c0);
    //         x += 1;
    //         buf.flush();
    //         io::stdout().flush();

    //         animation::sleep_until(Instant::now() + Duration::from_millis(500));
    //         printing::print_card_yx(&mut buf, y, x, c0);
    //         // panic!();
    //     }
    // });

// animation::animate);

    // handle.join();
    // Ok(())
    //
    // use io::Write as _;
    // let mut buf = io::BufWriter::with_capacity(100_000, io::stdout());
    // write!(buf, "{}", ts::Reset)?;

    // let colors = [CardColor::Green, CardColor::Red, CardColor::Purple];
    // let shapes = [CardShape::Oval, CardShape::Diamond, CardShape::Squiggle];
    // let numbers = [CardNumber::One, CardNumber::Two, CardNumber::Three];
    // let fills = [CardFill::Solid, CardFill::Striped, CardFill::Empty];

    // // no, needs to be vec since we want to remove them over time.
    // let mut deck = Vec::with_capacity(81);

    // for c in colors.iter() {
    //     for s in shapes.iter() {
    //         for n in numbers.iter() {
    //             for f in fills.iter() {
    //                 deck.push(Card{color:*c, shape:*s, number:*n, fill:*f});
    //             }
    //         }
    //     }
    // }

    // let c0 = Card{color: CardColor::Red, shape: CardShape::Squiggle, number: CardNumber::Three, fill: CardFill::Empty};
    // let c1 = Card{color: CardColor::Purple, shape: CardShape::Diamond, number: CardNumber::One, fill: CardFill::Striped};
    // let c1 = Card{color: CardColor::Red, ..c0};
    // let c2 = Card{color: CardColor::Purple, ..c0};

    // print!("{}", screen::ToAlternateScreen);
    // print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    // std::io::stdout().flush()?;
    
    // println!("{:?} -- {:?}", c0, get_raw_fg_style(c0));
    // println!("{:?} -- {:?}", c1, get_raw_fg_style(c1));

    // print_card(&mut buf, 1, 2, c0)?;
    // writeln!(buf);
    // writeln!(buf);
    // print_card(&mut buf, 1, 10, c1)?;
    // print_card_outline(&mut buf, 1, 1)?;
    // writeln!(buf)?;
    // print_card(1, 20, c1)?;
    // println!("asdlkfjh");
    // thread::sleep(time::Duration::new(5, 0));

    // print!("{}", termion::cursor::Goto(20, 10));
    // std::io::stdout().flush()?;
    // println!();
    // thread::sleep(time::Duration::new(2, 0));
    // println!("asldkfjh");
    // thread::sleep(time::Duration::new(2, 0));

    // thread::sleep(time::Duration::new(2, 0));
    // buf.flush()?;
    // io::stdout().flush()?;
    // Ok(())
}


// what do i want here? I want the cards to be able to animate moving to their new positions.
// I want cards to remain in their positions when the three extra cards are added. And for whatever
// remains of those three cards to float over and fill in the gaps when a set is found.
// And ideally id like to keep presentation and abstract game state separate. So what really
// determines the game state? the cards remaining in the deck, the cards in play on the board, and
// thats it. The abstract game state does not care where those cards are on the board or anything
// like that.
//
// So it should just be a vector representing the deck and a vector representing the cards in play.
// Really, it should be a multiset (maybe I want to permit multiple duplicate cards? nah thats
// kinda dumb.) So it should be a set. And then there should be another struct representing the
// LAYOUT of the cards on the table. And it could just contain references to the gamestate to
// ensure they stay in sync.
//
// oof ok so if i want nice animations its gonna get a LOT more complicated.
// I'll handle user nput iin the main thread. But ALL prntnig wll get handled in a separate thread,
// and I'll have to send it animations, probably by sending it closures that yield animation frame
// strings given a frame number input. 
//
// I may also want these to be ccapable of being somewhat independent of terminal size, which
// necessitates making 6-wide and maybe even 4-wide versions of the shapes, as well as 10 or 12.
