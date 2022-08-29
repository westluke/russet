use std::{io, time};
use super::game::*;

use time::Instant;
use termion::{color, style, cursor};
use color::Color;


use crate::config::*;
use crate::pos::{TermPos, SetPos};


// ok so i should make separate functions to print just outline, and to print background as well
// (which requires overwriting)
// Note-I think cards should both have borders (box drawing) AND have rounded corners



pub fn print_gamestate( buf: &mut impl io::Write,
                        g: &GameState) -> io::Result<()> {

    for (pos, c_opt) in g.layout.enumerate_2d(){
        let filled = c_opt.is_some();
        let selected = g.selects.contains(&pos);

        if filled && selected {
            print_card(buf, pos.to_TermPos(), c_opt.unwrap(), &color::LightYellow)?;
        } else if filled && !selected {
            print_card(buf, pos.to_TermPos(), c_opt.unwrap(), CARD_BG)?;
        } else if selected {
            print_card_bg(buf, pos.to_TermPos(), &color::LightYellow)?;
        }
    }

    print_card_bg(buf, SetPos::Deck.to_TermPos(), CARD_BG)?;
    // print_card_outline(buf, SetPos::Deck.to_TermPos() + (-1, 1), &color::Black, &color::White)?;
    // print_card_outline(buf, SetPos::Deck.to_TermPos() + (-2, 2), &color::Black, &color::White)?;

    if g.last_set_found.is_some() {
        let (c1, c2, c3) = g.last_set_found.unwrap();
        print_card(buf, SetPos::LastFound0.to_TermPos(), c1, CARD_BG)?;
        print_card(buf, SetPos::LastFound1.to_TermPos(), c2, CARD_BG)?;
        print_card(buf, SetPos::LastFound2.to_TermPos(), c3, CARD_BG)?;
    }

    Ok(())
}

pub fn write_time(  buf: &mut impl io::Write,
                    start: Instant,
                    pos:TermPos) -> io::Result<()>{

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
                    bg: &dyn Color) -> io::Result<()> {

    // if cfg!(feature="blocky"){ 
    //     print_card_outline(buf, pos, outline, outline)?;
    // } else {
    //     print_card_outline(buf, pos, bg, outline)?;
    // }

    print_card_bg(buf, pos, bg)?;
    print_card_contents(buf, pos, card, bg)?;
    Ok(())
}

pub fn print_card_outline(  buf: &mut impl io::Write,
                            pos: TermPos,
                            bg: &dyn Color,
                            fg: &dyn Color) -> io::Result<()> {

    write!(buf, "{}{}{}", style::Reset, color::Bg(bg), color::Fg(fg))?;
    // print_literal(buf, pos, RAW_OUTLINE)?;
    Ok(())
}

pub fn print_card_outline_over( buf: &mut impl io::Write,
                                pos: TermPos,
                                bg: &dyn Color,
                                fg: &dyn Color) -> io::Result<()> {

    write!(buf, "{}{}{}", style::Reset, color::Bg(bg), color::Fg(fg))?;
    // print_literal_over(buf, pos, RAW_OUTLINE)?;
    Ok(())
}




////////////////////////
/// Private!
////////////////////////

// Just prints whatever is in lit with nothing fancy. Keeps previous styling. Also skips spaces (so
// doesn't overwrite what's underneath)
// fn print_literal(   buf: &mut impl io::Write,
//                     pos:TermPos,
//                     lit:&str) -> io::Result<()> {

//     for (i, ln) in lit.lines().enumerate(){
//         for (j, c) in ln.chars().enumerate() {
//             if c != ' ' {
//                 mv_cursor(buf, pos + (i as u16, j as u16))?;
//                 write!(buf, "{}", c)?;
//             };
//         };
//     };

//     Ok(())
// }

fn print_card_bg(   buf: &mut impl io::Write,
                    pos: TermPos,
                    bg: &dyn Color) -> io::Result<()> {
    for i in 0..CARD_HEIGHT {
        mv_cursor(buf, pos + (i, 0));
        write!(buf, "{}{}", color::Bg(bg), " ".repeat(usize::from(CARD_WIDTH)))?;
    }

    Ok(())
}

// Prints the shapes inside the card (up to 3)
// pos is again the top left corner of the CARD, which will be above the top of any shape
fn print_card_contents( buf: &mut impl io::Write,
                        pos: TermPos,
                        card: Card,
                        bg: &dyn Color) -> io::Result<()> {

    // offset comes from the requirement that the shapes be centered, no matter how many there are
    // when there are 3 shapes, the middle one is separated from the left border by SHAPE_WIDTH + 2
    // blocks. the left one is separated from the left border by just 1 block.
    // 
    // basically: if there are n shapes, there are n+1 spacing columns, and n*shape_width shape
    // columns. The remaining space should be split between the left and right sides.
    let num = card.number as u16;
    let offset = (CARD_WIDTH - ((SHAPE_WIDTH * num) + (num + 1))) / 2;

    for i in 0..(card.number as u16){

        // x must be adjusted if e.g. this is the third shape in the row
        let shape_pos = i*SHAPE_WIDTH;

        // there is a small amount of minimum spacing between adjacent shapes, and between the
        // shapes and the card outline
        let spacing = (i+1) * SHAPE_SPACING;

        print_card_shape(buf, pos + (1_u16, offset + shape_pos + spacing), card, bg)?;
    };
    Ok(())
}

// Print a single instance of one of this card's shapes in the specified position.
fn print_card_shape(    buf: &mut impl io::Write,
                        pos: TermPos,
                        card: Card,
                        bg: &dyn Color) -> io::Result<()> {

    let shape = get_raw_shape(card);
    for (i, ln) in shape.lines().enumerate(){
        print_card_shape_line(buf, pos + (i as u16, 0), ln, card, bg)?;
    };
    Ok(())
}


// Style is reset at beginning of each line.
fn print_card_shape_line(   buf: &mut impl io::Write,
                            pos: TermPos, 
                            ln: &str,
                            card: Card,
                            bg: &dyn Color) -> io::Result<()> {

    write!(buf, "{}{}", style::Reset, color::Bg(bg))?;

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

fn mv_cursor(   buf: &mut impl io::Write,
                pos: TermPos) -> io::Result<()> {

    if pos.x() == 0 || pos.y() == 0 {
        panic!("Cursor positions start at 1, not 0.");
    };
    write!(*buf, "{}", cursor::Goto(pos.x(), pos.y()))?;
    Ok(())
}

fn print_card_shape_line_solid( buf: &mut impl io::Write,
                                pos: TermPos,
                                ln: &str,
                                card: Card) -> io::Result<()> {
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
        format!("{}", color::Fg(col))
    }
}

// Returns styling string for fill ('x') characters. block compilation isn't relevant here.
fn get_raw_fill_style(c:Card) -> String {
    let col = get_raw_color(c);
    match c.fill {
        CardFill::Solid =>  format!("{}{}", color::Bg(col), color::Fg(col)),
        _ =>                format!("{}", color::Fg(col)),
    }
}

fn get_raw_fill(c:Card) -> char {
    match c.fill {
        CardFill::Solid => ' ',    // solid shapes don't have a fill, they have a background
        CardFill::Striped => '╳', //'╋', //'─', box plus looks better honestly
        CardFill::Empty => ' '
    }
}

