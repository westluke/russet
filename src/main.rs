// extern crate termion;

use termion::{color, style, screen, input, event, cursor, clear};
use std::{time, thread};
use std::fs::File;
use std::io::Read;
use std::include_str;

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum CardColor {
    Green, Red, Purple
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum CardShape {
    Oval, Diamond, Squiggle
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum CardNumber {
    One, Two, Three
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum CardFill {
    Solid, Striped, Empty
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct Card {
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

fn print_card(x:i32, y:i32, card:Card) {
}

fn main() -> std::io::Result<()> {
    println!("{}Red", color::Fg(color::Red));
    println!("{}Blue", color::Fg(color::Blue));
    println!("{}Blue'n'Bold{}", style::Bold, style::Reset);
    println!("{}Just plain italic", style::Italic);

    let mut oval = String::new();
    let mut diamond = String::new();
    let mut squiggle = String::new();
    File::open("../oval.txt")?.read_to_string(&mut oval)?;
    File::open("../diamond.txt")?.read_to_string(&mut diamond)?;
    File::open("../squiggle.txt")?.read_to_string(&mut squiggle)?;

    // let c0 = Card{color: CardColor::Green, shape: CardShape::Oval, number: CardNumber::One, fill: CardFill::Solid};
    // let c1 = Card{color: CardColor::Red, ..c0};
    // let c2 = Card{color: CardColor::Purple, ..c0};

    println!("{}", screen::ToAlternateScreen);
    // println!("{}", is_a_set(c0, c1, c2));
    thread::sleep(time::Duration::new(2, 0));
    Ok(())
}
