use std::{io, time};
use super::game::*;

use time::Instant;
use termion::{color, style, cursor};
use color::Color;


use crate::config::*;
use crate::pos::{TermPos, SetPos};

// Note that this module doesn't know about SetPos!! It expects everything to be converted to
// TermPos beforehand. I think this kinda makes sense. This is just about displaying things, it
// doesn't know how Set works.

// need to be able to print card with given background
// need striped background
// need to be able to print layout
// need to be able to print deck (
//
// how to display last found set??
//
// all this is getting ahead of myself. iimplement basic functionality first!
//
//
//
// how do i set background color to black after clearing screen? check truecolor.rs example for
// possible way... cuz i do have to clear the screen every time, damn.




pub fn print_gamestate (buf: &mut impl io::Write, g: GameState) -> io::Result<()> {
    for (pos, c_opt) in g.layout.enumerate_2d(){
        let filled = matches!(c_opt, Some(c));
        let selected = g.selects.contains(&pos);

        if filled && selected {
            // print card with yellow background
        } else if filled && !selected {
            //print card normal
        } else if selected {
            // print yellow background
        }
    }

    // if deck isn't empty, print card stack
    // if there's a last set found, print it
    // if there's a selection, print it

    Ok(())
}

pub fn write_time(buf: &mut impl io::Write, start: Instant, pos:TermPos) -> io::Result<()>{
    mv_cursor(buf, pos)?;
    let elap = start.elapsed();
    write!(buf, "{}{}{}", style::Reset, color::Fg(color::White), color::Bg(color::Black))?;
    write!(buf, "{:02}:{:02}.{:03}", elap.as_secs() / 60, elap.as_secs() % 60, elap.subsec_millis())?;
    Ok(())
}

// pos is position of top left corner of card outline
pub fn print_card(  buf: &mut impl io::Write,
                    pos: TermPos,
                    card: Card,
                    bg: Option<impl Color>,
                    outline: Option<impl Color>) -> io::Result<()> {

    if cfg!(feature="blocky"){ write!(buf, "{}", color::Bg(color::White))?; };
    print_card_outline(buf, pos, color::White)?;
    print_card_contents(buf, pos + (1, 1), card)?;
    Ok(())
}

pub fn print_card_outline(buf: &mut impl io::Write, pos:TermPos, c:Option<impl Color>) -> io::Result<()> {
    write!(buf, "{}{}{}", style::Reset, color::Bg(color::Black), color::Fg(c))?;
    print_literal(buf, pos, RAW_OUTLINE)?;
    Ok(())
}




////////////////////////
/// Private!
////////////////////////

// Just prints whatever is in lit with nothing fancy. Keeps previous styling.
fn print_literal(buf: &mut impl io::Write, pos:TermPos, lit:&str) -> io::Result<()> {
    for (i, ln) in lit.lines().enumerate(){
        mv_cursor(buf, pos + (i as u16, 0))?;
        write!(buf, "{}", ln)?
    };

    Ok(())
}

// Prints the shapes inside the card (up to 3)
fn print_card_contents (buf: &mut impl io::Write, pos:TermPos, card:Card) -> io::Result<()> {

    // offset comes from the requirement that the shapes be centered, no matter how many there are
    let offset = (SHAPE_WIDTH * (3 - card.number as u16)) / 2;

    for i in 0..(card.number as u16){

        // x must be adjusted if e.g. this is the third shape in the row
        let shape_pos = i*SHAPE_WIDTH;

        // there is a small amount of minimum spacing between adjacent shapes, and between the
        // shapes and the card outline
        let spacing = (i+1) * SHAPE_SPACING;

        print_card_shape(buf, pos + (0, offset + shape_pos + spacing), card)?;
    };
    Ok(())
}

// Print a single instance of one of this card's shapes in the specified position.
fn print_card_shape(buf: &mut impl io::Write, pos:TermPos, card: Card) -> io::Result<()> {
    let shape = get_raw_shape(card);
    for (i, ln) in shape.lines().enumerate(){
        print_card_shape_line(buf, pos + (i as u16, 0), ln, card)?;
    };
    Ok(())
}

// Style is reset at beginning of each line.
fn print_card_shape_line(buf: &mut impl io::Write, pos:TermPos, ln:&str, card: Card) -> io::Result<()> {
    write!(buf, "{}", style::Reset)?;

    if card.fill == CardFill::Solid {
        return print_card_shape_line_solid(buf, pos, ln, card);
    }
    
    let mut is_fill = true;

    for (i, ch) in ln.chars().enumerate(){
        if ch == ' ' { continue; };
        mv_cursor(buf, pos + (0, i as u16))?;

        let restyle = (ch == 'x') != is_fill;
        is_fill = ch == 'x';

        if is_fill  {
            if restyle { write!(buf, "{}", get_raw_fill_style(card))?; };
            write!(buf, "{}", get_raw_fill(card))?
        } else { 
            if restyle { write!(buf, "{}", get_raw_edge_style(card))?; };
            write!(buf, "{}", ch)?;
        };
    };

    Ok(())
}

fn mv_cursor(buf: &mut impl io::Write, pos:TermPos) -> io::Result<()> {
    if pos.x() == 0 || pos.y() == 0 {
        panic!("Cursor positions start at 1, not 0.");
    };
    write!(*buf, "{}", cursor::Goto(pos.x(), pos.y()))?;
    Ok(())
}

fn print_card_shape_line_solid(buf: &mut impl io::Write, pos:TermPos, ln:&str, card: Card) -> io::Result<()> {
    let core = ln.trim();
    let first = ln.find(|c:char| !c.is_whitespace() ).unwrap();
    mv_cursor(buf, pos + (0, first as u16))?;
    write!(buf, "{}{}{}", style::Reset, get_raw_solid_style(card), core)?;
    Ok(())
}




//////////////////////////////
/// Helpers
//////////////////////////////

fn get_raw_solid_style(c:Card) -> String {
    if c.fill != CardFill::Solid { panic!(); };
    let col = get_raw_color(c);
    format!("{}{}", color::Fg(col), color::Bg(col))
}

fn get_raw_color(c:Card) -> &'static dyn color::Color {
    match c.color {
        CardColor::Green =>  RAW_GREEN,
        CardColor::Red =>    RAW_RED,
        CardColor::Purple => RAW_PURPLE,
    }
}

fn get_raw_shape(c:Card) -> &'static str {
    let solid_override = (c.fill == CardFill::Solid) || (cfg!(feature="blocky"));
    match (solid_override, c.shape) {
        (true, CardShape::Oval) => RAW_OVAL_SOLID,
        (true, CardShape::Diamond) => RAW_DIAMOND_SOLID,
        (true, CardShape::Squiggle) => RAW_SQUIGGLE_SOLID,
        (_,    CardShape::Oval) => RAW_OVAL,
        (_,    CardShape::Diamond) => RAW_DIAMOND,
        (_,    CardShape::Squiggle) => RAW_SQUIGGLE
    }
}

// Returns styling string for characters at the edge of a shape
fn get_raw_edge_style(c:Card) -> String {
    let col = get_raw_color(c);
    let solid_override = (c.fill == CardFill::Solid) || (cfg!(feature="blocky"));

    if solid_override {
        format!("{}{}", color::Bg(col), color::Fg(col))
    } else {
        format!("{}{}", color::Bg(color::Black), color::Fg(col))
    }
}

// Returns styling string for fill ('x') characters. block compilation isn't relevant here.
fn get_raw_fill_style(c:Card) -> String {
    let col = get_raw_color(c);
    match c.fill {
        CardFill::Solid =>  format!("{}{}", color::Bg(col), color::Fg(col)),
        _ =>                format!("{}{}", color::Bg(color::Black), color::Fg(col)),
    }
}

fn get_raw_fill(c:Card) -> char {
    match c.fill {
        CardFill::Solid => ' ',    // solid shapes don't have a fill, they have a background
        CardFill::Striped => '─',
        CardFill::Empty => ' '
    }
}

