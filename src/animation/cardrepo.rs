use crate::framebuf::layer::Layer;
use crate::deck::{Card, CardShape, CardColor, CardFill, CardNumber, all_cards};
use crate::termchar::TermChar;
use crate::pos::TermPos;
use crate::util::*;

use std::collections::HashMap;

use crossterm::style::Color;

// Should TermPos be able to be negative? No, that doesn't really make sense.
// But i need to be able to add negative numbers. Should those just be tuples?
// yeah, why not. That's simpler. TermPos is useful for any value that does indeed
// represent a position in the terminal. Could make an additional TermDiff,
// but that's kinda pointless. So just use separate y, x, values. tuples?
// nahhh, thats also extra overhead
//
// Nope, conceptually makes sense for TermPos to be negative. So I should allow that.

/// Stores framebuflayers to be cloned on demand
///
/// How do I deal with yellow highlighted cards?
/// Do I want to instead store transparent versions of all the cards?
/// No, that kinda sucks... especially cuz that doesn't work with the backgrounds
/// on the actually visible characters.
///
/// Actually makes more sense to store two versions of every card:
/// highlighted/flat, and normal/raised.
///
/// same for question.
///
/// Hopefully the construction process isn't too slow.

// pub struct CardRepo {
//     cards: HashMap<Card, FrameBufLayer>,
//     deck: FrameBufLayer,
//     cards_active: HashMap<Card, FrameBufLayer>,
//     deck_active: FrameBufLayer,
//     outline0: FrameBufLayer,
//     outline1: FrameBufLayer,
//     outline2: FrameBufLayer
// }

// impl CardRepo {
//     pub fn new(scale: Size, term_bg: Color, card_bg: Color) -> Self {
//         let mut g = BufGen::new(scale, term_bg, card_bg);
//         Self {
//             card_bufs: g.card_bufs(),
//             deck_buf: g.deck_buf(),
//             card_bufs_active: g.card_bufs_active(),
//             deck_buf_active: g.deck_buf_active(),
//             outline_buf: g.outline_buf()
//         }
//     }

//     pub fn get_card(&self, c: Card) -> FrameBufLayer {
//         self.card_bufs
//             .get(&c)
//             .unwrap()
//             .clone()
//     }

//     pub fn get_deck(&self) -> FrameBufLayer {
//         self.deck_buf.clone()
//     }

    // pub fn get_card_active(&self, c: Card) -> FrameBufLayer {
//         self.card_bufs_active
//             .get(&c)
//             .unwrap()
//             .clone()
//     }

//     pub fn get_deck_active(&self) -> FrameBufLayer {
//         self.deck_buf_active.clone()
//     }

//     pub fn get_outline(&self) -> FrameBufLayer {
//         self.outline_buf.clone()
//     }

//     pub fn apply_outline(&self, buf: FrameBufLayer) -> FrameBufLayer {
//         let out = self.get_outline();
//         out.translate(1, -1);
//         buf.under(out)
//     }

// }

// Fuck it, this works. Just do it, running out of time
// struct BufGen {
//     scale: Scale,
//     term_bg: Color,
//     card_bg: Color

//     outline_buf: Option<FrameBufLayer>, // the 3d effect

//     card_bg_buf: Option<FrameBufLayer>, // white default background
//     card_bg_buf_active: Option<FrameBufLayer> // the yellow background on selection

//     card_bufs: Option<Vec<FrameBufLayer>>, // completed 3d cards
//     card_bufs_selected: Option<Vec<FrameBufLayer>>, // completed cards with yellow/2d

//     deck_buf: Option<FrameBufLayer>, // represents deck, 3d, white
//     deck_buf_active: Option<FrameBufLayer> // becomes 2d, yellow
// }

// impl BufGen {

pub fn stamp_shape(scale: Scale, buf: Layer, card: Card, bg: Color) -> Layer {
    let num: i16 = card.number.into();
    let mut card_lay = buf.clone();

    // This is basically the left-margin on the first shape
    // Justification: We're gonna take the full card width, and subtract the combined width of
    // all the shapes, and the combined width of the spaces between each pair of shapes, and
    // divide the result by 2 since there's a margin on the right side too.
    let mut offset = scale.CARD_WIDTH;
    offset -= scale.SHAPE_WIDTH * num;
    offset -= SHAPE_SPACING * (num-1);
    offset /= 2;

    // same kinda thing - how far is the top of the shape from the top of the card?
    // subtract shape height from card height and divide by 2.
    let mut drop = scale.CARD_HEIGHT;
    drop -= scale.SHAPE_HEIGHT;
    drop /= 2;

    for i in 0..num{
        let bg_actual = get_bg(card, bg);

        // x must be adjusted if e.g. this is the third shape in the row
        // this is the amount of spacing due to that order - not accounting for inter-shape spacing
        let shape_pos = i*scale.SHAPE_WIDTH;

        // there is a small amount of minimum spacing between adjacent shapes
        let spacing = i * SHAPE_SPACING;

        // add these to the base offset
        card_lay.set_s((drop, offset + shape_pos + spacing).finto(), String::from(get_raw_shape(card, scale)), get_raw_color(card), bg_actual);
    };

    card_lay
}

pub fn stamp_question(scale: Scale, buf: Layer, fg: Color, bg: Color) -> Layer {
    let mut card_lay = buf.clone();

    let mut offset = scale.CARD_WIDTH;
    offset -= scale.SHAPE_WIDTH;
    offset /= 2;

    let mut drop = scale.CARD_HEIGHT;
    drop -= scale.SHAPE_HEIGHT;
    drop /= 2;

    card_lay.set_s((drop, offset).finto(), String::from(scale.RAW_QUESTION), fg, bg);

    card_lay
}

pub fn stamp_shapes(scale: Scale, buf: Layer, bg: Color) -> HashMap<Card, Layer> {
    let mut card_bufs: HashMap<Card, Layer> = HashMap::new();

    for card in all_cards() {
        card_bufs.insert(card, stamp_shape(scale, buf, card, bg));
    }

    card_bufs
}

// If you want a solid background card, just set fg = bg
pub fn make_card_shape(scale: Scale, fg: Color, bg: Color) -> Layer {
    let buf = Layer::new(None, scale.CARD_HEIGHT, scale.CARD_WIDTH, (0, 0).finto(), None);
    buf.set_s((0, 0).finto(), String::from(CARD_TL), fg, bg);
    buf.set_s((0, scale.CARD_WIDTH-2).finto(), String::from(CARD_TR), fg, bg);
    buf.set_s((scale.CARD_HEIGHT-2, 0).finto(), String::from(CARD_BL), fg, bg);
    buf.set_s((scale.CARD_HEIGHT-2, scale.CARD_WIDTH-2).finto(), String::from(CARD_BR), fg, bg);

    for row in 1..(scale.CARD_HEIGHT-1) {
        buf.set_c((row, 0).finto(), Some(TermChar::new('┃', fg, bg)));
        buf.set_c((row, scale.CARD_WIDTH-1).finto(), Some(TermChar::new('┃', fg, bg)));
    };

    for col in 1..(scale.CARD_WIDTH-1) {
        buf.set_c((0, col).finto(), Some(TermChar::new('━', fg, bg)));
        buf.set_c((scale.CARD_HEIGHT-1, col).finto(), Some(TermChar::new('━', fg, bg)));
    };

    buf
}

pub fn make(scale: Scale, term_bg: Color, card_bg: Color) -> Layer {
    let outline_buf = make_card_shape(scale, CARD_BORDER, TERM_BG);
    let card_bg_buf = make_card_shape(scale, CARD_BG, CARD_BG);
    let card_bg_buf_active = make_card_shape(scale, ACTIVE_BG, ACTIVE_BG);

    let card_bufs = stamp_shapes(scale, card_bg_buf.clone(), CARD_BG);
    let card_bufs_active = stamp_shapes(scale, card_bg_buf_active.clone(), ACTIVE_BG);

    let deck_buf = stamp_question(scale, card_bg_buf.clone(), TERM_BG, CARD_BG);
    let deck_buf_active = stamp_question(scale, card_bg_buf.clone(), TERM_BG, ACTIVE_BG);

    let shadow_buf = make_card_shape(scale, TERM_BG, TERM_BG);

    outline_buf
}



//         result.outline_buf = result.mk_outline_buf();
//         result.card_bg_buf = 

//     }

//     pub fn provide(self) -> (

//     // Converts template character to the character that should actually be
//     // displayed
//     fn to_TermChar(&self, c: char, card: Card) -> Option<TermChar> {
//         if c == ' ' {
//             None
//         } else if c == 'x' {
//             let col = get_raw_color(card);
//             match card.fill {
//                 CardFill::Solid => Some(TermChar::new(' ', col, col)),
//                 CardFill::Striped => Some(TermChar::new('╳', col, self.card_bg)),
//                 CardFill::Empty => None,
//             }
//         } else if c == 'X' {
//             Some(TermChar::new(' ', self.card_bg, self.card_bg))
//         } else {
//             panic!()
//         }
//     }

//     fn shape_buf(&self, c: Card) -> FrameBufLayer {
//         let shape = get_raw_shape(c, self.scale);
//         let mut buf = FrameBufLayer::new(
//             None,
//             self.scale.shape_height,
//             self.scale.shape_width
//         );

//         for (y, ln) in shape.lines().enumerate() {
//             for (x, ch) in ln.chars().enumerate() {
//                 buf.set_c(
//                     TermPos::from((y, x)),
//                     self.to_TermChar(ch, c)
//                 );
//             };
//         };

//         buf
//     }

//     // fn fancy_outline_buf(&self) -> FrameBufLayer {}

//     fn outline_buf(&self) -> FrameBufLayer {
//         let (ly, lx) = (self.scale.card_height-1, self.scale.card_width-1);
//         let mut buf = FrameBufLayer::new(
//             None,
//             self.scale.card_height,
//             self.scale.card_width
//         );

//         let mk_edge = |c| {
//             Some(TermChar::new(c, self.card_bg, self.term_bg))
//         };


//         for y in 1..ly {
//             buf.set_c(
//                 TermPos::from((y, 0)),
//                 mk_edge('┃')
//             );

//             buf.set_c(
//                 TermPos::from((y, lx)),
//                 mk_edge('┃')
//             );
//         }

//         for x in 1..lx {
//             buf.set_c(
//                 TermPos::from((0, x)),
//                 mk_edge('━')
//             );

//             buf.set_c(
//                 TermPos::from((ly, x)),
//                 mk_edge('━')
//             );
//         }

//         buf.set_c(
//             TermPos::from((0, 0)),
//             mk_edge('┏')
//         );
//         buf.set_c(
//             TermPos::from((ly, 0)),
//             mk_edge('┗')
//         );

//         buf.set_c(
//             TermPos::from((0, lx)),
//             mk_edge('┓')
//         );
//         buf.set_c(
//             TermPos::from((ly, lx)),
//             mk_edge('┛')
//         );

//         buf
//     }

//     fn card_bg_buf(&self) -> FrameBufLayer {
//         let mut buf = FrameBufLayer::new(
//             None,
//             self.scale.card_height,
//             self.scale.card_width
//         );
//         let (ly, lx) = (self.scale.card_height-1, self.scale.card_width-1);

//         for y in 0..ly {
//             for x in 0..lx {
//                 buf.set_c(
//                     TermPos::from((y, x)),
//                     Some(TermChar::new(' ', self.card_bg, self.card_bg))
//                 );
//             }
//         }

//         buf
//     }

//     fn card_buf_from_card(&self, c: Card, s: Scale, bg: Color) -> FrameBufLayer {
//         let shape = get_raw_shape(c, self.scale);
//         let mut shape_buf_v = vec![self.shape_buf(c)];
//         let mut result = self.card_bg_buf();
//         let outln = self.outline_buf();

//         if c.number == 1 {
//             shape_buf_v[0].set_anch(...);
//         }

//         else {
//             let src = &mut shape_buf_v[0];
//             src.set_anch(...);
//             for i in 1..c.number {
//                 let mut dup = src.clone();
//                 dup.set_anch(...)
//                 shape_buf_v.push(dup)
//             }
//         }

//         for buf in shape_buf_v {
//             result = result.beneath(buf);
//         }

//         result.set_anch(TermPos::from(0, 1))
//         result = result.over(outln);

//         result
//     }


//// Style is reset at beginning of each line.
//fn print_card_shape_line(   buf: &mut SmartBuf<impl Write>,
//                            pos: TermPos, 
//                            ln: &str,
//                            card: Card,
//                            bg: Color) -> Result<(), SE> {

//    for (i, ch) in ln.chars().enumerate(){
//        if ch == ' ' { continue; };
//        let is_fill = ch == 'x';

//        let (fgx, bgx) = get_fg_bg(card, bg);

//        if is_fill {
//            buf.write(
//                get_raw_fill(card),
//                fgx, bgx,
//                pos.add(0, i32::try_from(i)?)?
//            )?;
//        } else {
//            buf.write(
//                " ",
//                fgx, bgx,
//                pos.add(0, i32::try_from(i)?)?
//            )?;
//        };
//    };

//    Ok(())
//}

fn get_raw_shape(c:Card, s:Scale) -> &'static str {
    match c.shape {
        CardShape::Oval => s.RAW_OVAL,
        CardShape::Diamond => s.RAW_DIAMOND,
        CardShape::Squiggle => s.RAW_SQUIGGLE
    }
}

// fn get_fg_bg(c:Card, bg: Color) -> (Color, Color) {
//     let col = get_raw_color(c);
//     match c.fill {
//         CardFill::Solid =>  (col, col),
//         _ =>                (col, bg)
//     }
// }

fn get_bg(c:Card, bg: Color) -> Color {
    let col = get_raw_color(c);
    match c.fill {
        CardFill::Solid => col,
        _ =>               bg
    }
}

fn get_raw_color(c: Card) -> Color {
    match c.color {
        CardColor::Color1 => COLOR_1,
        CardColor::Color2 => COLOR_2,
        CardColor::Color3 => COLOR_3
    }
}

// fn get_raw_fill(c:Card) -> &'static str {
//     match c.fill {
//         CardFill::Solid => " ",    // solid shapes don't have a fill, they have a background
//         CardFill::Striped => "╳", //'╋', //'─', box plus looks better honestly
//         CardFill::Empty => " "
//     }
// }
