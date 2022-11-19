use crate::framebuf::FrameBufLayer;
use crate::deck::{Card, CardShape, CardColor, CardFill, CardNumber, all_cards};
use crate::termchar::TermChar;
use crate::pos::TermPos;
use crate::config::*;

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
pub struct CardRepo {
    card_bufs: HashMap<Card, FrameBufLayer>,
    deck_buf: FrameBufLayer,
    card_bufs_active: HashMap<Card, FrameBufLayer>,
    deck_buf_active: FrameBufLayer,
    outline_buf: FrameBufLayer
}

impl CardRepo {
    pub fn new(scale: Size, term_bg: Color, card_bg: Color) -> Self {
        let mut g = BufGen::new(scale, term_bg, card_bg);
        Self {
            card_bufs: g.card_bufs(),
            deck_buf: g.deck_buf(),
            card_bufs_active: g.card_bufs_active(),
            deck_buf_active: g.deck_buf_active(),
            outline_buf: g.outline_buf()
        }
    }

    pub fn get_card(&self, c: Card) -> FrameBufLayer {
        self.card_bufs
            .get(&c)
            .unwrap()
            .clone()
    }

    pub fn get_deck(&self) -> FrameBufLayer {
        self.deck_buf.clone()
    }

    pub fn get_card_active(&self, c: Card) -> FrameBufLayer {
        self.card_bufs_active
            .get(&c)
            .unwrap()
            .clone()
    }

    pub fn get_deck_active(&self) -> FrameBufLayer {
        self.deck_buf_active.clone()
    }

    pub fn get_outline(&self) -> FrameBufLayer {
        self.outline_buf.clone()
    }

    pub fn apply_outline(&self, buf: FrameBufLayer) -> FrameBufLayer {
        let out = self.get_outline();
        out.translate(1, -1);
        buf.under(out)
    }

}

struct BufGen {
    scale: Scale,
    term_bg: Color,
    card_bg: Color
    outline_buf: Option<FrameBufLayer>,
    card_bg_buf: Option<FrameBufLayer>,
    card_bg_buf_active: Option<FrameBufLayer>
    card_bufs: Option<FrameBufLayer>,
    card_bufs_selected: Vec<FrameBufLayer>,
    // question_buf: FrameBufLayer,
    // question_buf_active
}

impl BufGen {
    pub fn make(scale: Scale, term_bg: Color, card_bg: Color) -> Self {
        let result = Self {
            scale,
            term_bg,
            card_bg,
            outline_buf: None,
            card_bg_buf: None,
            card_bg_buf_active: None
        }

        result.outline_buf = result.mk_outline_buf();
        result.card_bg_buf = 

    }

    pub fn provide(self) -> (

    // Converts template character to the character that should actually be
    // displayed
    fn to_TermChar(&self, c: char, card: Card) -> Option<TermChar> {
        if c == ' ' {
            None
        } else if c == 'x' {
            let col = get_raw_color(card);
            match card.fill {
                CardFill::Solid => Some(TermChar::new(' ', col, col)),
                CardFill::Striped => Some(TermChar::new('╳', col, self.card_bg)),
                CardFill::Empty => None,
            }
        } else if c == 'X' {
            Some(TermChar::new(' ', self.card_bg, self.card_bg))
        } else {
            panic!()
        }
    }

    fn shape_buf(&self, c: Card) -> FrameBufLayer {
        let shape = get_raw_shape(c, self.scale);
        let mut buf = FrameBufLayer::new(
            None,
            self.scale.shape_height,
            self.scale.shape_width
        );

        for (y, ln) in shape.lines().enumerate() {
            for (x, ch) in ln.chars().enumerate() {
                buf.set_c(
                    TermPos::from((y, x)),
                    self.to_TermChar(ch, c)
                );
            };
        };

        buf
    }

    // fn fancy_outline_buf(&self) -> FrameBufLayer {}

    fn outline_buf(&self) -> FrameBufLayer {
        let (ly, lx) = (self.scale.card_height-1, self.scale.card_width-1);
        let mut buf = FrameBufLayer::new(
            None,
            self.scale.card_height,
            self.scale.card_width
        );

        let mk_edge = |c| {
            Some(TermChar::new(c, self.card_bg, self.term_bg))
        };


        for y in 1..ly {
            buf.set_c(
                TermPos::from((y, 0)),
                mk_edge('┃')
            );

            buf.set_c(
                TermPos::from((y, lx)),
                mk_edge('┃')
            );
        }

        for x in 1..lx {
            buf.set_c(
                TermPos::from((0, x)),
                mk_edge('━')
            );

            buf.set_c(
                TermPos::from((ly, x)),
                mk_edge('━')
            );
        }

        buf.set_c(
            TermPos::from((0, 0)),
            mk_edge('┏')
        );
        buf.set_c(
            TermPos::from((ly, 0)),
            mk_edge('┗')
        );

        buf.set_c(
            TermPos::from((0, lx)),
            mk_edge('┓')
        );
        buf.set_c(
            TermPos::from((ly, lx)),
            mk_edge('┛')
        );

        buf
    }

    fn card_bg_buf(&self) -> FrameBufLayer {
        let mut buf = FrameBufLayer::new(
            None,
            self.scale.card_height,
            self.scale.card_width
        );
        let (ly, lx) = (self.scale.card_height-1, self.scale.card_width-1);

        for y in 0..ly {
            for x in 0..lx {
                buf.set_c(
                    TermPos::from((y, x)),
                    Some(TermChar::new(' ', self.card_bg, self.card_bg))
                );
            }
        }

        buf
    }

    fn card_buf_from_card(&self, c: Card, s: Scale, bg: Color) -> FrameBufLayer {
        let shape = get_raw_shape(c, self.scale);
        let mut shape_buf_v = vec![self.shape_buf(c)];
        let mut result = self.card_bg_buf();
        let outln = self.outline_buf();

        if c.number == 1 {
            shape_buf_v[0].set_anch(...);
        }

        else {
            let src = &mut shape_buf_v[0];
            src.set_anch(...);
            for i in 1..c.number {
                let mut dup = src.clone();
                dup.set_anch(...)
                shape_buf_v.push(dup)
            }
        }

        for buf in shape_buf_v {
            result = result.beneath(buf);
        }

        result.set_anch(TermPos::from(0, 1))
        result = result.over(outln);

        result
    }
}


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
        CardShape::Oval => s.raw_oval,
        CardShape::Diamond => s.raw_diamond,
        CardShape::Squiggle => s.raw_squiggle
    }
}

fn get_fg_bg(c:Card, bg: Color) -> (Color, Color) {
    let col = get_raw_color(c);
    match c.fill {
        CardFill::Solid =>  (col, col),
        _ =>                (col, bg)
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
