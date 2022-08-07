// extern crate termion;

use termion::{color as tc, style}; //, screen, input, event, cursor, clear};
use std::{time, thread, io};

const raw_green: &dyn tc::Color = &tc::Green;
const raw_red: &dyn tc::Color = &tc::Red;
const raw_purple: &dyn tc::Color = &tc::Blue;

const raw_oval: &str = include_str!("../txt/oval.txt");
const raw_diamond: &str = include_str!("../txt/diamond.txt");
const raw_squiggle: &str = include_str!("../txt/squiggle.txt");

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
        CardColor::Green =>  raw_green,
        CardColor::Red =>  raw_red,
        CardColor::Purple =>  raw_purple,
    }
}

fn get_raw_shape(c:Card) -> &'static str {
    match c.shape {
        CardShape::Oval => raw_oval,
        CardShape::Diamond => raw_diamond,
        CardShape::Squiggle => raw_squiggle
    }
}

fn get_raw_fill(c:Card) -> char {
    match c.fill {
        CardFill::Solid => ' ',
        CardFill::Striped => '─',
        CardFill::Empty => ' '
    }
}

fn get_raw_fg_style(c:Card) -> String {
    let col = get_raw_color(c);
    format!("{}{}", tc::Fg(col), style::Bold)
}

fn get_raw_bg_style(c:Card) -> String {
    let col = get_raw_color(c);
    match c.fill {
        CardFill::Solid => format!("{}", tc::Bg(col)),
        CardFill::Striped => format!("{}{}", tc::Fg(col), style::Underline),
        CardFill::Empty => String::new()
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

fn mv_cursor(x:u16, y:u16){
    print!("{}", termion::cursor::Goto(x, y));
}

// fn wait(sec: u64){
//     thread::sleep(time::Duration::new(sec, 0));
// }

fn print_card (x:u16, y:u16, card:Card) -> io::Result<()> {
    if x == 0 || y == 0 {
        panic!("Cursor positions start at 1, not 0.");
    }

    let shape = get_raw_shape(card);
    let mut bg = true;

    use io::Write as _;

    let mut buf = io::BufWriter::with_capacity(100_000, std::io::stdout());
    write!(buf, "{}", style::Reset)?;

    for j in 0..(card.number as u16){
        for (i, ln) in shape.lines().enumerate(){
            mv_cursor(x + j*7, y + (i as u16));

            for ch in ln.chars() {

                let restyle = (ch == 'x') != bg;
                bg = (ch == 'x');

                if restyle {
                    write!(buf, "{}{}", style::Reset, if ch == 'x' {
                        get_raw_bg_style(card) 
                    } else {
                        get_raw_fg_style(card)
                    })?;
                };

                match ch {
                    'x'  => write!(buf, "{}", get_raw_fill(card)),
                    edge => write!(buf, "{}", edge)
                }?;
            };
            writeln!(buf)?;
        };
    };

    Ok(())
}

fn main() -> std::io::Result<()> {
    let c0 = Card{color: CardColor::Red, shape: CardShape::Squiggle, number: CardNumber::One, fill: CardFill::Empty};
    let c1 = Card{color: CardColor::Purple, shape: CardShape::Diamond, number: CardNumber::One, fill: CardFill::Striped};
    // let c1 = Card{color: CardColor::Red, ..c0};
    // let c2 = Card{color: CardColor::Purple, ..c0};

    // print!("{}", screen::ToAlternateScreen);
    // print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    // std::io::stdout().flush()?;
    
    // println!("{:?} -- {:?}", c0, get_raw_fg_style(c0));
    // println!("{:?} -- {:?}", c1, get_raw_fg_style(c1));

    print_card(1, 1, c0)?;
    print_card(1, 20, c1)?;
    // println!("asdlkfjh");
    // thread::sleep(time::Duration::new(5, 0));

    // print!("{}", termion::cursor::Goto(20, 10));
    // std::io::stdout().flush()?;
    // println!();
    // thread::sleep(time::Duration::new(2, 0));
    // println!("asldkfjh");
    // thread::sleep(time::Duration::new(2, 0));

    // thread::sleep(time::Duration::new(2, 0));
    Ok(())
}
