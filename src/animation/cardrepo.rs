// use crate::framebuf::layer::{Layer, LayerCell::{self, *}};
use crate::framebuf::{LayerCell::{self, *}};
use crate::deck::{Card, CardShape, CardColor, CardFill, CardNumber, all_cards};
use crate::termchar::TermChar;
use crate::pos::TermPos;
use crate::util::*;

use std::hash::{Hash, Hasher};

use std::collections::{HashMap, hash_map::DefaultHasher};

use crossterm::style::Color;

fn hash_card(c: Card, active: bool) -> u64 {
    let mut hasher = DefaultHasher::new();
    c.hash(&mut hasher);
    active.hash(&mut hasher);
    hasher.finish()
}

//pub struct CardRepo {
//    deck: Layer,
//    deck_active: Layer,
//    cards: HashMap<Card, Layer>,
//    cards_active: HashMap<Card, Layer>,
//    shadow: Layer,
//    outline_thin: Layer,
//    outline_good: Layer,
//    outline_bad: Layer
//}

//impl CardRepo {
//    pub fn new(scale: Scale, term_bg: Color, card_bg: Color) -> Self {
//        make(scale, term_bg, card_bg)
//    }

//    pub fn get_card(&self, c: Card) -> Layer {
//        self.cards
//            .get(&c)
//            .unwrap()
//            .clone()
//    }

//    pub fn get_card_active(&self, c: Card) -> Layer {
//        self.cards_active
//            .get(&c)
//            .unwrap()
//            .clone()
//    }

//    pub fn get_deck(&self) -> Layer {
//        self.deck.clone()
//    }

//    pub fn get_deck_active(&self) -> Layer {
//        self.deck_active.clone()
//    }

//    pub fn get_shadow(&self) -> Layer {
//        self.shadow.clone()
//    }

//    pub fn get_outline_thin(&self) -> Layer {
//        self.outline_thin.clone()
//    }

//    pub fn get_outline_good(&self) -> Layer {
//        self.outline_good.clone()
//    }

//    pub fn get_outline_bad(&self) -> Layer {
//        self.outline_bad.clone()
//    }
//}

//pub fn stamp_shape(scale: Scale, buf: Layer, card: Card, bg: Color) -> Layer {
//    let num: i16 = card.number.into();
//    let mut card_lay = buf.clone();

//    // This is basically the left-margin on the first shape
//    // Justification: We're gonna take the full card width, and subtract the combined width of
//    // all the shapes, and the combined width of the spaces between each pair of shapes, and
//    // divide the result by 2 since there's a margin on the right side too.
//    let mut offset = scale.CARD_WIDTH;
//    offset -= scale.SHAPE_WIDTH * num;
//    offset -= SHAPE_SPACING * (num-1);
//    offset /= 2;

//    // same kinda thing - how far is the top of the shape from the top of the card?
//    // subtract shape height from card height and divide by 2.
//    let mut drop = scale.CARD_HEIGHT;
//    drop -= scale.SHAPE_HEIGHT;
//    drop /= 2;

//    for i in 0..num {
//        // x must be adjusted if e.g. this is the third shape in the row
//        // this is the amount of spacing due to that order - not accounting for inter-shape spacing
//        let shape_pos = i*scale.SHAPE_WIDTH;

//        // there is a small amount of minimum spacing between adjacent shapes
//        let spacing = i * SHAPE_SPACING;

//        // add these to the base offset
//        set_shape_rel(scale, &mut card_lay, card, (drop, offset + shape_pos + spacing).finto(), bg);
//    };

//    card_lay
//}

//pub fn stamp_question(scale: Scale, buf: Layer, fg: Color, bg: Color) -> Layer {
//    let mut card_lay = buf.clone();

//    let mut offset = scale.CARD_WIDTH;
//    offset -= scale.SHAPE_WIDTH;
//    offset /= 2;

//    let mut drop = scale.CARD_HEIGHT;
//    drop -= scale.SHAPE_HEIGHT;
//    drop /= 2;

//    card_lay.set_s_clear((drop, offset).finto(), String::from(scale.RAW_QUESTION), fg, bg);

//    card_lay
//}

//pub fn stamp_shapes(scale: Scale, buf: Layer, bg: Color) -> HashMap<Card, Layer> {
//    let mut card_bufs: HashMap<Card, Layer> = HashMap::new();

//    for card in all_cards() {
//        card_bufs.insert(card, stamp_shape(scale, buf.clone(), card, bg));
//    }

//    card_bufs
//}

//// If you want a solid background card, just set fg = bg
//pub fn make_card_shape(
//    scale: Scale, 
//    border_fg: Color,
//    border_bg: Color,
//    interior_bg: Option<Color>,
//    // "outer_bg" is always None
//) -> Result<Layer> {

//    let mut buf;
//    if let Some(colr) = interior_bg {
//        buf = Layer::new(
//            0, scale.CARD_HEIGHT, scale.CARD_WIDTH, 
//            (0, 0).finto(), Opaque(TermChar::new(' ', colr, colr)));
//        buf.set_c((0, scale.CARD_WIDTH-1).finto(), Transparent)?;
//        buf.set_c((scale.CARD_HEIGHT-1, scale.CARD_WIDTH-1).finto(), Transparent)?;
//        buf.set_c((scale.CARD_HEIGHT-1, 0).finto(), Transparent)?;
//        buf.set_c((0, 0).finto(), Transparent)?;
//    } else {
//        buf = Layer::new(
//            0, scale.CARD_HEIGHT, scale.CARD_WIDTH, 
//            (0, 0).finto(), Transparent);
//    }

//    // Corners have to be drawn with clear spaces, since they are irregularly shaped.
//    // Luckily, they have no real effect on the interior
//    buf.set_s_clear((0, 0).finto(), String::from(CARD_TL), border_fg, border_bg)?;
//    buf.set_s_clear((0, scale.CARD_WIDTH-2).finto(), String::from(CARD_TR), border_fg, border_bg)?;
//    buf.set_s_clear((scale.CARD_HEIGHT-2, 0).finto(), String::from(CARD_BL), border_fg, border_bg)?;
//    buf.set_s_clear((scale.CARD_HEIGHT-2, scale.CARD_WIDTH-2).finto(), String::from(CARD_BR), border_fg, border_bg)?;

//    for row in 2..(scale.CARD_HEIGHT-2) {
//        buf.set_c((row, 0).finto(), Opaque(TermChar::new('┃', border_fg, border_bg)))?;
//        buf.set_c((row, scale.CARD_WIDTH-1).finto(), Opaque(TermChar::new('┃', border_fg, border_bg)))?;
//    };

//    for col in 2..(scale.CARD_WIDTH-2) {
//        buf.set_c((0, col).finto(), Opaque(TermChar::new('━', border_fg, border_bg)))?;
//        buf.set_c((scale.CARD_HEIGHT-1, col).finto(), Opaque(TermChar::new('━', border_fg, border_bg)))?;
//    };

//    Ok(buf)
//}

//// how am i gonna handle groups of layers? like a card with a good outline on top of it.
//// I could give the outline its own id, and just manually move them together, but that kinda sounds
//// like a pain.
//// Honestly, it kinda feels more reasonable to alter the framebuf structure so that it stores a
//// stack of GROUPS of layers, rather than single layers, so I can go into the group and make
//// modifications as needed.
////
//// Within each group, I can index by strings, that sounds easy.
////
//// And how do I index into the framebuf? Could also just do it by strings, now that I think about
//// it... Yeah, that will be much better for debugging too. hash_card should really be
//// stringify_card.

//pub fn make(scale: Scale, term_bg: Color, card_bg: Color) -> CardRepo {
//    let mut outline_thin = make_card_shape(scale, CARD_BORDER, term_bg, None).unwrap();
//    outline_thin.set_anchor((1, -1).finto());

//    let mut shadow = make_card_shape(scale, term_bg, term_bg, Some(term_bg)).unwrap();
//    let mut outline_good = make_card_shape(scale, GOOD_SET, GOOD_SET, None).unwrap();
//    let mut outline_bad = make_card_shape(scale, BAD_SET, BAD_SET, None).unwrap();

//    let card_active = make_card_shape(scale, ACTIVE_BG, ACTIVE_BG, Some(ACTIVE_BG)).unwrap();
//    let card = make_card_shape(scale, card_bg, card_bg, Some(card_bg)).unwrap();

//    let mut cards_active = stamp_shapes(scale, card_active.clone(), ACTIVE_BG);
//    let mut cards = stamp_shapes(scale, card.clone(), card_bg);
//    cards = cards.into_iter()
//        .map(
//            |(k, c0)| {
//                let mut c1 = c0.over(&outline_thin);
//                c1.set_id(hash_card(k, false));
//                (k, c1)
//            }
//        ).collect();

//    cards_active = cards_active.into_iter()
//        .map(
//            |(k, c0)| {
//                let mut c1 = c0.over(&outline_thin);
//                c1.set_id(hash_card(k, true));
//                (k, c1)
//            }
//        ).collect();

//    let deck_active = stamp_question(scale, card_active.clone(), term_bg, term_bg);
//    let mut deck = stamp_question(scale, card.clone(), term_bg, term_bg);

//    deck = deck.over(&outline_thin);

//    CardRepo {
//        cards,
//        cards_active,
//        deck,
//        deck_active,
//        shadow,
//        outline_thin,
//        outline_good,
//        outline_bad,
//    }
//}

//fn get_raw_shape(c:Card, s:Scale) -> &'static str {
//    match c.shape {
//        CardShape::Oval => s.RAW_OVAL,
//        CardShape::Diamond => s.RAW_DIAMOND,
//        CardShape::Squiggle => s.RAW_SQUIGGLE
//    }
//}

//fn get_bg(c:Card, bg: Color) -> Color {
//    let col = get_raw_color(c);
//    match c.fill {
//        CardFill::Solid => col,
//        _ =>               bg
//    }
//}

//fn get_raw_color(c: Card) -> Color {
//    match c.color {
//        CardColor::Color1 => COLOR_1,
//        CardColor::Color2 => COLOR_2,
//        CardColor::Color3 => COLOR_3
//    }
//}

//fn get_raw_char(card: Card, ch: char, colr: Color, card_bg: Color) -> LayerCell {
//    // Other striped options: '╋', '─'
//    match (card.fill, ch) {
//        (_, ' ') =>                 Opaque(TermChar::new(' ', card_bg, card_bg)),
//        (CardFill::Solid, _) =>     Opaque(TermChar::new(' ', colr, colr)),
//        (_, 'X') =>                 Opaque(TermChar::new(' ', colr, colr)),
//        (CardFill::Striped, 'o') => Opaque(TermChar::new('╳', colr, card_bg)),
//        (CardFill::Empty, 'o') =>   Opaque(TermChar::new(' ', card_bg, card_bg)),
//        _ =>                        panic!("Unrecognized character in get_raw_char")
//    }
//}

//// This is kinda a variant of set_s method, necessary cuz we need to do special stuff depending on
//// shape contents
//pub fn set_shape_rel(
//    scale: Scale,
//    lay: &mut Layer,
//    card: Card,
//    mut pos: TermPos,
//    bg: Color,
//) -> Result<()> {

//    let shape = get_raw_shape(card, scale);
//    let colr = get_raw_color(card);

//    let start = pos;
//    let start_x = start.x();
//    let chars: Vec<char> = shape.chars().collect();

//    for i in 0..chars.len() {
//        if chars[i] == '\n' {
//            pos = pos + (1, 0).finto();
//            pos = pos.set_x(start_x);
//        } else if chars[i] == ' ' {
//            pos = pos + (0, 1).finto();
//        } else {
//            let c = get_raw_char(card, chars[i], colr, bg);
//            lay.set_c_rel(pos, c)?;
//            pos = pos + (0, 1).finto();
//        };
//    };

//    Ok(())
//}
