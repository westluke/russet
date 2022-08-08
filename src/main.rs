use termion::{color as tc, style as ts, clear as tcl}; //, screen, input, event, cursor, clear};
use std::{io, thread, time};
use time::{Instant, Duration};
use std::{vec::Vec, collections::HashSet, sync::mpsc};

mod animation;

const RAW_GREEN: &dyn tc::Color = &tc::Green;
const RAW_RED: &dyn tc::Color = &tc::Red;
const RAW_PURPLE: &dyn tc::Color = &tc::Blue;

const RAW_OVAL: &str = include_str!("../txt/oval.txt");
const RAW_DIAMOND: &str = include_str!("../txt/diamond.txt");
const RAW_SQUIGGLE: &str = include_str!("../txt/squiggle.txt");

const RAW_OVAL_SOLID: &str = include_str!("../txt/solids/oval_solid.txt");
const RAW_DIAMOND_SOLID: &str = include_str!("../txt/solids/diamond_solid.txt");
const RAW_SQUIGGLE_SOLID: &str = include_str!("../txt/solids/squiggle_solid.txt");

const RAW_OUTLINE: &str = include_str!("../txt/outline.txt");
const BG_CHARACTERS: &str = include_str!("../txt/bg_chars.txt");

const SHAPE_HEIGHT: u16 = 8;
const SHAPE_WIDTH: u16 = 8;
const SHAPE_SPACING: u16 = 1;

const CARD_HEIGHT: u16 = 10;
const CARD_WIDTH: u16 = 31;
const CARD_SPACING: u16 = 2;

// Rows / Columns start at 1, from top left.
fn get_card_xy(col:u16, row:u16) -> (u16, u16) {
    let y = ((col - 1) * CARD_HEIGHT) + ((col - 1) * CARD_SPACING);
    let x = ((row - 1) * CARD_WIDTH) + ((row - 1) * CARD_SPACING);
    (y, x)
}

#[derive(Clone, Debug)]
struct GameState {
    deck: HashSet<Card>,
    in_play: HashSet<Card>
}

impl GameState {
    fn take_3 (&mut self) {
    }
}

#[derive(Copy, Clone, Debug)]
enum CardSpot<'a> {
    Empty,
    Filled(&'a Card)
}

#[derive(Copy, Clone, Debug)]
struct Layout<'a, 'b> {
    main: [[&'a CardSpot<'b>; 4]; 3],
    add0: [&'a CardSpot<'b>; 3],
    add1: [&'a CardSpot<'b>; 3]
}

impl<'a, 'b> Layout<'a, 'b> {
    fn fill (self, game:&GameState){
    }
}

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
struct Card {
    color: CardColor,
    shape: CardShape,
    number: CardNumber,
    fill: CardFill
}

fn get_raw_color(c:Card) -> &'static dyn tc::Color {
    match c.color {
        CardColor::Green =>  RAW_GREEN,
        CardColor::Red =>  RAW_RED,
        CardColor::Purple =>  RAW_PURPLE,
    }
}

fn get_raw_shape(c:Card) -> &'static str {
    match c.shape {
        CardShape::Oval => RAW_OVAL,
        CardShape::Diamond => RAW_DIAMOND,
        CardShape::Squiggle => RAW_SQUIGGLE
    }
}

fn get_raw_fill(c:Card) -> char {
    match c.fill {
        CardFill::Solid => ' ',
        CardFill::Striped => '─',
        CardFill::Empty => ' '
    }
}

// Returns styling string for foreground characters (characters whose background is always black.)
fn get_raw_fg_style(c:Card) -> String {
    let col = get_raw_color(c);
    format!("{}{}", tc::Bg(tc::Black), tc::Fg(col))
}

// Returns styling string for background characters (characters that may receive a colored
// background if their card is Solid)
// This could be a character like a space, but it could also be an edge or 'x'
fn get_raw_bg_style(c:Card) -> String {
    let col = get_raw_color(c);
    match c.fill {
        CardFill::Solid => format!("{}{}", tc::Bg(col), tc::Fg(col)),
        CardFill::Striped => format!("{}{}{}", tc::Bg(tc::Black), tc::Fg(col), ts::Underline),
        CardFill::Empty => format!("{}{}", tc::Bg(tc::Black), tc::Fg(col))
    }
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

fn mv_cursor(buf: &mut impl io::Write, x:u16, y:u16) -> io::Result<()> {
    write!(*buf, "{}", termion::cursor::Goto(x, y))?;
    Ok(())
}

fn print_shape(buf: &mut impl io::Write, x:u16, y:u16, shape:&str) -> io::Result<()> {
    write!(buf, "{}{}", ts::Reset, tc::Fg(tc::White))?;

    for (i, ln) in shape.lines().enumerate(){
        for (j, ch) in ln.chars().enumerate() {
            match ch {
                ' '  => (),
                _ => {
                    mv_cursor(buf, x + j as u16, y + i as u16)?;
                    write!(buf, "{}", ch)? }
            };
        };
    };

    Ok(())
}

// Refers to top left corner of card outline
fn print_card(buf: &mut impl io::Write, x:u16, y:u16, card:Card) -> io::Result<()> {
    if x == 0 || y == 0 {
        panic!("Cursor positions start at 1, not 0.");
    };
    print_card_contents(buf, x+1, y+1, card)?;
    print_card_outline(buf, x, y)?;
    Ok(())
}

fn print_card_outline(buf: &mut impl io::Write, x:u16, y:u16) -> io::Result<()> {
    print_shape(buf, x, y, RAW_OUTLINE)?;
    Ok(())
}

fn is_bg_char(c:char) -> bool {
    BG_CHARACTERS.contains(c)
}

fn print_card_contents (buf: &mut impl io::Write, x:u16, y:u16, card:Card) -> io::Result<()> {
    if card.fill == CardFill::Solid { return print_card_contents_solid(buf, x, y, card) }

    let shape = get_raw_shape(card);
    // let mut bg = true;

    write!(buf, "{}{}", ts::Reset, get_raw_fg_style(card))?;
    let offset = (SHAPE_WIDTH * (3 - card.number as u16)) / 2;

    for j in 0..(card.number as u16){
        let shape_pos = j*SHAPE_WIDTH;
        let spacing = (j+1) * SHAPE_SPACING;

        for (i, ln) in shape.lines().enumerate(){
            for (h, ch) in ln.chars().enumerate() {
                mv_cursor(buf, x + offset + shape_pos + spacing + (h as u16), y + (i as u16))?;

                // let restyle = is_bg_char(ch) != bg;
                // bg = is_bg_char(ch);

                // if restyle {
                //     write!(buf, "{}{}", ts::Reset, if bg {
                //         get_raw_bg_style(card) 
                //     } else {
                //         get_raw_fg_style(card)
                //     })?;
                // };

                match ch {
                    'x' => write!(buf, "{}", get_raw_fill(card))?,
                    ' ' => (),
                    _ =>   write!(buf, "{}", ch)?
                };
            };
        };
    };

    Ok(())
}

fn print_card_contents_solid (buf: &mut impl io::Write, x:u16, y:u16, card:Card) -> io::Result<()> {
    let shape = get_raw_shape(card);
    let mut bg = true;

    write!(buf, "{}", ts::Reset)?;
    let offset = (SHAPE_WIDTH * (3 - card.number as u16)) / 2;

    for j in 0..(card.number as u16){
        let shape_pos = j*SHAPE_WIDTH;
        let spacing = (j+1) * SHAPE_SPACING;

        for (i, ln) in shape.lines().enumerate(){
            for (h, ch) in ln.chars().enumerate() {
                mv_cursor(buf, x + offset + shape_pos + spacing + (h as u16), y + (i as u16))?;

                let restyle = is_bg_char(ch) != bg;
                bg = is_bg_char(ch);

                if restyle {
                    write!(buf, "{}{}", ts::Reset, if bg {
                        get_raw_bg_style(card) 
                    } else {
                        get_raw_fg_style(card)
                    })?;
                };

                match ch {
                    'x' => write!(buf, "{}", get_raw_fill(card))?,
                    ' ' => (),
                    _ =>   write!(buf, "{}", ch)?
                };
            };
        };
    };

    Ok(())
}

// fn print_layout (buf: &mut impl io::Write, lay:Layout) {
// }

fn main() -> std::io::Result<()> {
    let (tx, rx) = mpsc::channel::<fn(u32) -> String>();

    let handle = thread::spawn(|| {
        use io::Write as _;
        let mut buf = io::BufWriter::with_capacity(100_000, io::stdout());
        let c0 = Card{color: CardColor::Green, shape: CardShape::Oval, number: CardNumber::Three, fill: CardFill::Solid};


        let (mut x, y) = (1, 1);
        loop {
            // go through list of animations, animate one frame of each.
            // try receiving new animations into the list
            // sleep until next frame
            
            write!(buf, "{}", tcl::All);
            print_card(&mut buf, x, y, c0);
            x += 1;
            buf.flush();
            io::stdout().flush();

            animation::sleep_until(Instant::now() + Duration::from_millis(10));
        }
    });

// animation::animate);

    handle.join();
    Ok(())
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
