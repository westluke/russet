use std::{io, time};
use crate::game::*;

use time::Instant;
use io::Write;

use crate::config;
use crate::pos::{TermPos, SetPos};
use crate::SetError as SE;
use crate::smartbuf::SmartBuf;

use crossterm::style::Color;
use log::{info, warn, error};




// IDEA: deck is normally flat, but raises when cards are emitted

pub enum CardStyle {
    Button,         // duplicate card bg in border to bottom and left
    ShadowLeft,     // apply SHADOW_BG to bottom and right of card
    ShadowRight,    // apply SHADOW_BG to bottom and right of card
    Flat,           // normal
    Pending         // Use PENDING_BG
}

pub fn print_card(  buf: &mut SmartBuf<impl Write>,
                    pos: TermPos,
                    card: Card,
                    style: CardStyle) -> Result<(), SE> {

    let mut bg = config::CARD_BG;

    match style {
        CardStyle::Button => {
            print_card_outline(buf, pos.add(1, -1)?, config::CARD_BG);
        },
        CardStyle::ShadowLeft => {
            print_card_bg(buf, pos.add(1, -1)?, config::SHADOW);
        },
        CardStyle::Pending => {
            bg = config::PENDING_BG;
        },
        _ => unimplemented!()
    };

    print_card_bg(buf, pos, bg);
    print_card_contents(buf, pos, card, bg)?;
    Ok(())
}

fn print_deck(  buf: &mut SmartBuf<impl Write>,
                pos: TermPos,
                bg: Color) -> Result<(), SE> {

    print_card_outline(buf, pos.add(1, -1)?, bg);
    // print_card_bg(buf, pos, bg)?;
    // print_deck_contents(buf, pos, Color::Black)?;
    Ok(())
}

pub fn print_gamestate( buf: &mut SmartBuf<impl Write>,
                        g: &GameState) -> Result<(), SE> {

    // for (pos, c_opt) in g.enumerate_cards(){
    //     let filled = c_opt.is_some();
    //     let selected = g.selected(pos);

    //     if filled && selected {
    //         print_card(buf, pos.to_TermPos()?.add(1, -1)?, c_opt.unwrap(), CardStyle::Pending)?;
    //     } else if filled && !selected {
    //         print_card(buf, pos.to_TermPos()?, c_opt.unwrap(), CardStyle::Button)?;
    //     } else if selected {
    //         print_card_bg(buf, pos.to_TermPos()?, config::PENDING_BG)?;
    //     }
    // }

    print_deck(buf, SetPos::Deck.to_TermPos()?, config::CARD_BG)?;

    // if g.last_set_found().is_some() {
    //     let (c1, c2, c3) = g.last_set_found().unwrap();
    //     print_card(buf, SetPos::LastFound0.to_TermPos()?, c1, CardStyle::ShadowLeft)?;
    //     print_card(buf, SetPos::LastFound1.to_TermPos()?, c2, CardStyle::ShadowLeft)?;
    //     print_card(buf, SetPos::LastFound2.to_TermPos()?, c3, CardStyle::ShadowLeft)?;
    // }

    Ok(())
}

pub fn write_time(  buf: &mut SmartBuf<impl Write>,
                    start: Instant,
                    pos:TermPos) -> Result<(), SE> {

    let elap = start.elapsed();
    buf.write(
        "00",
        // &format!("{:02}:{:02}.{:03}", elap.as_secs() / 60, elap.as_secs() % 60, elap.subsec_millis()),
        Color::White, Color::Reset, pos)?;
    Ok(())
}

fn print_deck_contents( buf: &mut SmartBuf<impl Write>,
                        pos: TermPos,
                        bg: Color) -> Result<(), SE> {

    let num = 3;
    let offset = (config::CARD_WIDTH - ((config::SHAPE_WIDTH * num) + (num + 1))) / 2;

    for i in 0..(num as u16){

        // x must be adjusted if e.g. this is the third shape in the row
        let shape_pos = i*config::SHAPE_WIDTH;

        // there is a small amount of minimum spacing between adjacent shapes, and between the
        // shapes and the card outline
        let spacing = (i+1) * config::SHAPE_SPACING;

        print_question(buf, pos.add(2, (offset + shape_pos + spacing) as i32)?, bg)?;
    };
    Ok(())
}

fn print_question(  buf: &mut SmartBuf<impl Write>,
                    pos: TermPos,
                    bg: Color) -> Result<(), SE> {

    for (i, ln) in config::RAW_QUESTION.lines().enumerate(){
        for (j, c) in ln.chars().enumerate() {
            if c == ' ' { continue; }
            buf.write(
                " ", bg, bg,
                pos.add(
                    i32::try_from(i)?,
                    i32::try_from(j)?
                )?
            )?;
        };
    };
    Ok(())
}





////////////////////////
/// Private!
////////////////////////

fn print_card_bg(   buf: &mut SmartBuf<impl Write>,
                    pos: TermPos,
                    bg: Color) -> Result<(), SE> {
    buf.write(
        &(" ".repeat(usize::from(config::CARD_WIDTH-2))),
        bg, bg, pos.add(0, 1)?
    )?;

    for i in 1..(config::CARD_HEIGHT-1) {
        buf.write(
            &(" ".repeat(usize::from(config::CARD_WIDTH))),
            bg, bg, pos.add(i32::from(i), 0)?
        )?;
    }

    buf.write(
        &(" ".repeat(usize::from(config::CARD_WIDTH-2))),
        bg, bg, pos.add(i32::from(config::CARD_HEIGHT) - 1, 1)?
    )?;

    Ok(())
}

fn print_card_outline( buf: &mut SmartBuf<impl Write>,
                            pos: TermPos,
                            fg: Color ) -> Result<(), SE> {

    buf.write(
        &format!("┏{}┓", "━".repeat(usize::from(config::CARD_WIDTH-4))),
        fg, Color::Reset, pos.add(0, 1)?
    )?;

    buf.write(
        &format!("┏┛{}┗┓", " ".repeat(usize::from(config::CARD_WIDTH-4))),
        fg, Color::Reset, pos.add(1, 0)?
    )?;

    for i in 2..(config::CARD_HEIGHT-2) {
        buf.write(
            &format!("┃{}┃", " ".repeat(usize::from(config::CARD_WIDTH-2))),
            fg, Color::Reset, pos.add(i32::from(i), 0)?
        )?;
    }

    buf.write(
        &format!("┗┓{}┏┛", " ".repeat(usize::from(config::CARD_WIDTH-4))),
        fg, Color::Reset, pos.add(i32::from(config::CARD_HEIGHT - 2), 0)?
    )?;

    buf.write(
        &format!("┗{}┛", "━".repeat(usize::from(config::CARD_WIDTH-4))),
        fg, Color::Reset, pos.add(i32::from(config::CARD_HEIGHT) - 1, 1)?
    )?;

    Ok(())
}

// Prints the shapes inside the card (up to 3)
// pos is again the top left corner of the CARD, which will be above the top of any shape
fn print_card_contents( buf: &mut SmartBuf<impl Write>,
                        pos: TermPos,
                        card: Card,
                        bg: Color) -> Result<(), SE> {

    // offset comes from the requirement that the shapes be centered, no matter how many there are
    // when there are 3 shapes, the middle one is separated from the left border by SHAPE_WIDTH + 2
    // blocks. the left one is separated from the left border by just 1 block.
    // 
    // basically: if there are n shapes, there are n+1 spacing columns, and n*shape_width shape
    // columns. The remaining space should be split between the left and right sides.
    let num = card.number as u16;
    let offset = (config::CARD_WIDTH - ((config::SHAPE_WIDTH * num) + (num + 1))) / 2;

    for i in 0..(card.number as u16){

        // x must be adjusted if e.g. this is the third shape in the row
        let shape_pos = i*config::SHAPE_WIDTH;

        // there is a small amount of minimum spacing between adjacent shapes, and between the
        // shapes and the card outline
        let spacing = (i+1) * config::SHAPE_SPACING;

        print_card_shape(buf, pos.add(2, i32::from(offset + shape_pos + spacing))?, card, bg)?;
    };
    Ok(())
}

// Print a single instance of one of this card's shapes in the specified position.
fn print_card_shape(    buf: &mut SmartBuf<impl Write>,
                        pos: TermPos,
                        card: Card,
                        bg: Color) -> Result<(), SE> {

    let shape = get_raw_shape(card);
    for (i, ln) in shape.lines().enumerate(){
        print_card_shape_line(buf, pos.add(i32::try_from(i)?, 0)?, ln, card, bg)?;
    };
    Ok(())
}

// Style is reset at beginning of each line.
fn print_card_shape_line(   buf: &mut SmartBuf<impl Write>,
                            pos: TermPos, 
                            ln: &str,
                            card: Card,
                            bg: Color) -> Result<(), SE> {

    for (i, ch) in ln.chars().enumerate(){
        if ch == ' ' { continue; };
        let is_fill = ch == 'x';

        let (fgx, bgx) = get_fg_bg(card, bg);

        if is_fill {
            buf.write(
                get_raw_fill(card),
                fgx, bgx,
                pos.add(0, i32::try_from(i)?)?
            )?;
        } else {
            buf.write(
                " ",
                fgx, bgx,
                pos.add(0, i32::try_from(i)?)?
            )?;
        };
    };

    Ok(())
}





//////////////////////////////
/// Helpers
//////////////////////////////

fn get_raw_color(c:Card) -> Color {
    match c.color {
        CardColor::Color1 => config::COLOR_1,
        CardColor::Color2 => config::COLOR_2,
        CardColor::Color3 => config::COLOR_3,
    }
}

fn get_raw_shape(c:Card) -> &'static str {
    match c.shape {
        CardShape::Oval => config::RAW_OVAL,
        CardShape::Diamond => config::RAW_DIAMOND,
        CardShape::Squiggle => config::RAW_SQUIGGLE
    }
}

// Returns styling string for fill ('x') characters. block compilation isn't relevant here.
fn get_fg_bg(c:Card, bg: Color) -> (Color, Color) {
    let col = get_raw_color(c);
    match c.fill {
        CardFill::Solid =>  (col, col),
        _ =>                (col, bg)
    }
}

fn get_raw_fill(c:Card) -> &'static str {
    match c.fill {
        CardFill::Solid => " ",    // solid shapes don't have a fill, they have a background
        CardFill::Striped => "╳", //'╋', //'─', box plus looks better honestly
        CardFill::Empty => " "
    }
}
