use std::{io, time};
use crate::game::*;

use time::Instant;
use termion::{color, style, cursor};
use color::Color;

use crate::config::*;
use crate::pos::{TermPos, SetPos};
use crate::SetError as SE;



// IDEA: deck is normally flat, but raises when cards are emitted

pub enum CardStyle {
    Button,         // duplicate card bg in border to bottom and left
    ShadowLeft,     // apply SHADOW_BG to bottom and right of card
    ShadowRight,    // apply SHADOW_BG to bottom and right of card
    Flat,           // normal
    Pending         // Use PENDING_BG
}

pub fn print_card(  buf: &mut impl io::Write,
                    pos: TermPos,
                    card: Card,
                    style: CardStyle) -> Result<(), SE> {

    let mut bg = CARD_BG;

    match style {
        CardStyle::Button => {
            print_card_outline(buf, pos.add(1, -1)?, &color::Reset, CARD_BG);
        },
        CardStyle::ShadowLeft => {
            print_card_bg(buf, pos.add(1, -1)?, SHADOW_BG);
        },
        CardStyle::Pending => {
            bg = PENDING_BG;
        },
        _ => unimplemented!()
    };

    print_card_bg(buf, pos, bg);
    print_card_contents(buf, pos, card, bg)?;
    Ok(())
}

fn print_deck(  buf: &mut impl io::Write,
                    pos: TermPos,
                    bg: &dyn Color) -> Result<(), SE> {

    print_card_outline(buf, pos.add(1, -1)?, &color::Reset, bg);
    print_card_bg(buf, pos, bg)?;
    print_deck_contents(buf, pos, &color::Black)?;
    Ok(())
}

pub fn print_gamestate( buf: &mut impl io::Write,
                        g: &GameState) -> Result<(), SE> {

    for (pos, c_opt) in g.layout.enumerate_2d(){
        let filled = c_opt.is_some();
        let selected = g.selects.contains(&pos);

        if filled && selected {
            print_card(buf, pos.to_TermPos()?.add(1, -1)?, c_opt.unwrap(), CardStyle::Pending)?;
        } else if filled && !selected {
            print_card(buf, pos.to_TermPos()?, c_opt.unwrap(), CardStyle::Button)?;
        } else if selected {
            print_card_bg(buf, pos.to_TermPos()?, PENDING_BG)?;
        }
    }

    print_deck(buf, SetPos::Deck.to_TermPos()?, CARD_BG)?;

    if g.last_set_found.is_some() {
        let (c1, c2, c3) = g.last_set_found.unwrap();
        print_card(buf, SetPos::LastFound0.to_TermPos()?, c1, CardStyle::ShadowLeft)?;
        print_card(buf, SetPos::LastFound1.to_TermPos()?, c2, CardStyle::ShadowLeft)?;
        print_card(buf, SetPos::LastFound2.to_TermPos()?, c3, CardStyle::ShadowLeft)?;
    }

    Ok(())
}

pub fn write_time(  buf: &mut impl io::Write,
                    start: Instant,
                    pos:TermPos) -> Result<(), SE> {

    mv_cursor(buf, pos)?;
    let elap = start.elapsed();
    write!(buf, "{}{}{}", style::Reset, color::Fg(color::White), color::Bg(color::Black))?;
    write!(buf, "{:02}:{:02}.{:03}", elap.as_secs() / 60, elap.as_secs() % 60, elap.subsec_millis())?;
    Ok(())
}


fn print_deck_contents( buf: &mut impl io::Write,
                        pos: TermPos,
                        bg: &dyn Color) -> Result<(), SE> {

    let num = 3;
    let offset = (CARD_WIDTH - ((SHAPE_WIDTH * num) + (num + 1))) / 2;

    write!(buf, "{}", color::Bg(bg));

    for i in 0..(num as u16){

        // x must be adjusted if e.g. this is the third shape in the row
        let shape_pos = i*SHAPE_WIDTH;

        // there is a small amount of minimum spacing between adjacent shapes, and between the
        // shapes and the card outline
        let spacing = (i+1) * SHAPE_SPACING;

        print_question(buf, pos.add(2, (offset + shape_pos + spacing) as i32)?, bg)?;
    };
    Ok(())
}

fn print_question(  buf: &mut impl io::Write,
                    pos: TermPos,
                    bg: &dyn Color) -> Result<(), SE> {

    for (i, ln) in RAW_QUESTION.lines().enumerate(){
        for (j, c) in ln.chars().enumerate() {
            if c == ' ' { continue; }
            mv_cursor(buf, pos.add(i as i32, j as i32)?);
            write!(buf, " ");
        };
    };
    Ok(())
}





////////////////////////
/// Private!
////////////////////////

fn print_card_bg(   buf: &mut impl io::Write,
                    pos: TermPos,
                    bg: &dyn Color) -> Result<(), SE> {
    mv_cursor(buf, pos.add(0, 1)?);
    write!(buf, "{}{}", color::Bg(bg), " ".repeat(usize::from(CARD_WIDTH-2)))?;

    for i in 1..(CARD_HEIGHT-1) {
        mv_cursor(buf, pos.add(i as i32, 0)?);
        write!(buf, "{}{}", color::Bg(bg), " ".repeat(usize::from(CARD_WIDTH)))?;
    }

    mv_cursor(buf, pos.add(CARD_HEIGHT as i32 - 1, 1)?);
    write!(buf, "{}{}", color::Bg(bg), " ".repeat(usize::from(CARD_WIDTH-2)))?;

    Ok(())
}

fn print_card_outline( buf: &mut impl io::Write,
                            pos: TermPos,
                            bg: &dyn Color,
                            fg: &dyn Color) -> Result<(), SE> {

    mv_cursor(buf, pos.add(0, 1)?);
    write!(buf, "{}{}{}{}{}", color::Bg(bg), color::Fg(fg), "┏", "━".repeat(usize::from(CARD_WIDTH-4)), "┓")?;
    mv_cursor(buf, pos.add(1, 0)?);
    write!(buf, "{}{}{}",  "┏┛", " ".repeat(usize::from(CARD_WIDTH-4)), "┗┓")?;

    for i in 2..(CARD_HEIGHT-2) {
        mv_cursor(buf, pos.add(i as i32, 0)?);
        write!(buf, "{}{}{}", "┃", " ".repeat(usize::from(CARD_WIDTH-2)), "┃")?;
    }

    mv_cursor(buf, pos.add(CARD_HEIGHT as i32 - 2, 0)?);
    write!(buf, "{}{}{}",  "┗┓", " ".repeat(usize::from(CARD_WIDTH-4)), "┏┛")?;
    mv_cursor(buf, pos.add(CARD_HEIGHT as i32 - 1, 1)?);
    write!(buf, "{}{}{}", "┗", "━".repeat(usize::from(CARD_WIDTH-4)), "┛")?;

    Ok(())
}

// Prints the shapes inside the card (up to 3)
// pos is again the top left corner of the CARD, which will be above the top of any shape
fn print_card_contents( buf: &mut impl io::Write,
                        pos: TermPos,
                        card: Card,
                        bg: &dyn Color) -> Result<(), SE> {

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

        print_card_shape(buf, pos.add(2, (offset + shape_pos + spacing) as i32)?, card, bg)?;
    };
    Ok(())
}

// Print a single instance of one of this card's shapes in the specified position.
fn print_card_shape(    buf: &mut impl io::Write,
                        pos: TermPos,
                        card: Card,
                        bg: &dyn Color) -> Result<(), SE> {

    let shape = get_raw_shape(card);
    for (i, ln) in shape.lines().enumerate(){
        print_card_shape_line(buf, pos.add(i as i32, 0)?, ln, card, bg)?;
    };
    Ok(())
}

// Style is reset at beginning of each line.
fn print_card_shape_line(   buf: &mut impl io::Write,
                            pos: TermPos, 
                            ln: &str,
                            card: Card,
                            bg: &dyn Color) -> Result<(), SE> {

    write!(buf, "{}{}", style::Reset, color::Bg(bg))?;

    let mut is_fill = true;

    for (i, ch) in ln.chars().enumerate(){
        if ch == ' ' { continue; };
        mv_cursor(buf, pos.add(0, i as i32)?)?;

        let restyle = (ch == 'x') != is_fill;
        is_fill = ch == 'x';

        if is_fill  {
            if restyle { write!(buf, "{}", get_raw_fill_style(card, bg))?; };
            write!(buf, "{}", get_raw_fill(card))?
        } else { 
            if restyle { write!(buf, "{}", get_raw_edge_style(card))?; };
            write!(buf, "{}", ' ')?;
        };
    };

    Ok(())
}





//////////////////////////////
/// Helpers
//////////////////////////////

fn mv_cursor(   buf: &mut impl io::Write,
                pos: TermPos) -> Result<(), SE> {

    if pos.x() == 0 || pos.y() == 0 {
        panic!("Cursor positions start at 1, not 0.");
    };
    write!(*buf, "{}", cursor::Goto(pos.x(), pos.y()))?;
    Ok(())
}

fn get_raw_color(c:Card) -> &'static dyn color::Color {
    match c.color {
        CardColor::Green =>  RAW_GREEN,
        CardColor::Red =>    RAW_RED,
        CardColor::Purple => RAW_PURPLE,
    }
}

fn get_raw_shape(c:Card) -> &'static str {
    match c.shape {
        CardShape::Oval => RAW_OVAL,
        CardShape::Diamond => RAW_DIAMOND,
        CardShape::Squiggle => RAW_SQUIGGLE
    }
}

// Returns styling string for characters at the edge of a shape
fn get_raw_edge_style(c:Card) -> String {
    let col = get_raw_color(c);
    format!("{}{}", color::Bg(col), color::Fg(col))
}

// Returns styling string for fill ('x') characters. block compilation isn't relevant here.
fn get_raw_fill_style(c:Card, bg: &dyn Color) -> String {
    let col = get_raw_color(c);
    match c.fill {
        CardFill::Solid =>  format!("{}{}", color::Bg(col), color::Fg(col)),
        _ =>                format!("{}{}", color::Bg(bg), color::Fg(col)),
    }
}

fn get_raw_fill(c:Card) -> char {
    match c.fill {
        CardFill::Solid => ' ',    // solid shapes don't have a fill, they have a background
        CardFill::Striped => '╳', //'╋', //'─', box plus looks better honestly
        CardFill::Empty => ' '
    }
}
