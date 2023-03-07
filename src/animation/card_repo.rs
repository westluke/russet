use crate::deck::{Card, CardShape, CardColor, CardFill, CardNumber, all_cards};
use crate::term_char::TermChar;
use crate::pos::TermPos;
use crate::util::*;
use crate::Id;

use crate::sprite;

use std::hash::{Hash, Hasher};

use std::collections::{HashMap, hash_map::DefaultHasher};

use crossterm::style::Color;

// fn hash_card(c: Card, active: bool) -> u64 {
//     let mut hasher = DefaultHasher::new();
//     c.hash(&mut hasher);
//     active.hash(&mut hasher);
//     hasher.finish()
// }


// GIVE EVERY CARD EVERY LAYER IT WILL EVER Need, WITH DESCRIPTIVE IDs AND TURNED INACTIVE

pub struct CardRepo {
    deck: FrameTree,
    // deck_active: FrameTree,
    cards: HashMap<Card, FrameTree>,
    // cards_active: HashMap<Card, FrameTree>,
    // outline_thin: FrameTree,
    // outline_good: FrameTree,
    // outline_bad: FrameTree
}

impl CardRepo {
    pub fn new(scale: Scale) -> Self {
        make(scale)
    }

    pub fn card(&self, c: Card) -> FrameTree {
        self.cards
            .get(&c)
            .unwrap()
            .clone()
    }

    // pub fn card_active(&self, c: Card) -> FrameTree {
    //     self.cards_active
    //         .get(&c)
    //         .unwrap()
    //         .clone()
    // }

    pub fn deck(&self) -> FrameTree {
        self.deck.clone()
    }

    // pub fn deck_active(&self) -> FrameTree {
    //     self.deck_active.clone()
    // }

    // pub fn outline_thin(&self) -> FrameTree {
    //     self.outline_thin.clone()
    // }

    // pub fn outline_good(&self) -> FrameTree {
    //     self.outline_good.clone()
    // }

    // pub fn outline_bad(&self) -> FrameTree {
    //     self.outline_bad.clone()
    // }
}

pub fn stamp_shape(scale: Scale, buf: FrameTree, card: Card, bg: Color) -> FrameTree {
    let num: i16 = card.number.into();
    let mut card_lay = buf.clone();

    // DIRTY HACK TO MAKE IT LOOK NICER
    let mut shape_spacing = if card.shape == CardShape::Squiggle && card.number == CardNumber::Three {
        SHAPE_SPACING - 1
    } else {SHAPE_SPACING};

    // This is basically the left-margin on the first shape
    // Justification: We're gonna take the full card width, and subtract the combined width of
    // all the shapes, and the combined width of the spaces between each pair of shapes, and
    // divide the result by 2 since there's a margin on the right side too.
    let mut offset = scale.CARD_WIDTH;
    offset -= scale.SHAPE_WIDTH * num;
    offset -= shape_spacing * (num-1);
    offset /= 2;

    // same kinda thing - how far is the top of the shape from the top of the card?
    // subtract shape height from card height and divide by 2.
    let mut drop = scale.CARD_HEIGHT;
    drop -= scale.SHAPE_HEIGHT;
    drop /= 2;

    for i in 0..num {
        // x must be adjusted if e.g. this is the third shape in the row
        // this is the amount of spacing due to that order - not accounting for inter-shape spacing
        let shape_pos = i*scale.SHAPE_WIDTH;

        // there is a small amount of minimum spacing between adjacent shapes
        let spacing = i * shape_spacing;

        // add these to the base offset
        set_shape_rel(scale, &mut card_lay, card, (drop, offset + shape_pos + spacing).finto(), bg);
    };

    card_lay
}

pub fn stamp_question(scale: Scale, buf: FrameTree, fg: Color, bg: Color) -> FrameTree {
    let mut card_lay = buf.clone();

    let mut offset = scale.CARD_WIDTH;
    offset -= scale.SHAPE_WIDTH;
    offset /= 2;

    let mut drop = scale.CARD_HEIGHT;
    drop -= scale.SHAPE_HEIGHT;
    drop /= 2;

    set_s_clear(&mut card_lay, (drop, offset).finto(), String::from(scale.RAW_QUESTION), fg, bg);

    card_lay
}

pub fn stamp_shapes(scale: Scale, buf: FrameTree, bg: Color) -> HashMap<Card, FrameTree> {
    let mut card_bufs: HashMap<Card, FrameTree> = HashMap::new();

    for card in all_cards() {
        card_bufs.insert(card, stamp_shape(scale, buf.clone(), card, bg));
    }

    card_bufs
}

fn set_s(tree: &mut FrameTree, mut pos: TermPos, s: String, fg: Color, bg: Color) -> Result<()> {
    let start_x = pos.x();
    let chars: Vec<char> = s.chars().collect();

    // for every character in the string...
    for i in 0..chars.len() {
        
        // if it's a newline, we jump down one step, and move back to our original column index
        if chars[i] == '\n' {
            pos = pos + (1, 0).finto();
            pos = pos.set_x(start_x);

        // otherwise, we set the cell to this character and advance one step to the right.
        } else {
            tree.set_cell(pos, Opaque(TermChar::new(chars[i], fg, bg)));
            pos = pos + (0, 1).finto();
        };
    };

    Ok(())
}

fn set_s_clear(tree: &mut FrameTree, mut pos: TermPos, s: String, fg: Color, bg: Color) -> Result<()> {
    let start_x = pos.x();
    let chars: Vec<char> = s.chars().collect();

    // for every character in the string...
    for i in 0..chars.len() {
        
        // if it's a newline, we jump down one step, and move back to our original column index
        if chars[i] == '\n' {
            pos = pos + (1, 0).finto();
            pos = pos.set_x(start_x);

        // if it's a space, since this is "clear", we just skip it
        } else if chars[i] == ' ' {
            pos = pos + (0, 1).finto();

        // otherwise, we set the cell to this character and advance one step to the right.
        } else {
            tree.set_cell(pos, Opaque(TermChar::new(chars[i], fg, bg)));
            pos = pos + (0, 1).finto();
        };
    };

    Ok(())
}

// If you want a solid background card, just set fg = bg
pub fn make_card_shape(
    scale: Scale, 
    border_fg: Color,
    border_bg: Color,
    interior_bg: Option<Color>,
    // "outer_bg" is always None
) -> Result<FrameTree> {

    let mut buf;
    // buf = FrameTree::new_leaf(
    //     (scale.CARD_HEIGHT, scale.CARD_WIDTH),
    //     Opaque(TermChar::new('X', Color::Red, Color::Red)), "".into(), true, (0, 0).finto());

    if let Some(colr) = interior_bg {
        buf = FrameTree::new_leaf(
            (scale.CARD_HEIGHT, scale.CARD_WIDTH),
            Opaque(TermChar::new(' ', colr, colr)), "".into(), true, (0, 0).finto(), 0);
        buf.set_cell((0, scale.CARD_WIDTH-1).finto(), Transparent);
        buf.set_cell((scale.CARD_HEIGHT-1, scale.CARD_WIDTH-1).finto(), Transparent);
        buf.set_cell((scale.CARD_HEIGHT-1, 0).finto(), Transparent);
        // buf.set_cell((0, 0).finto(), Opaque(TermChar::new(' ', Color::Green, Color::Green)));
        buf.set_cell((0, 0).finto(), Transparent);
    } else {
        buf = FrameTree::new_leaf(
            (scale.CARD_HEIGHT, scale.CARD_WIDTH),
            Transparent, "".into(), true, (0, 0).finto(), 0);
    }

    // Corners have to be drawn with clear spaces, since they are irregularly shaped.
    // Luckily, they have no real effect on the interior
    set_s_clear(&mut buf, (0, 0).finto(), String::from(CARD_TL), border_fg, border_bg)?;
    set_s_clear(&mut buf, (0, scale.CARD_WIDTH-2).finto(), String::from(CARD_TR), border_fg, border_bg)?;
    set_s_clear(&mut buf, (scale.CARD_HEIGHT-2, 0).finto(), String::from(CARD_BL), border_fg, border_bg)?;
    set_s_clear(&mut buf, (scale.CARD_HEIGHT-2, scale.CARD_WIDTH-2).finto(), String::from(CARD_BR), border_fg, border_bg)?;
    // buf.set_cell((0, 0).finto(), Opaque(TermChar::new(' ', Color::Green, Color::Green)));

    for row in 2..(scale.CARD_HEIGHT-2) {
        buf.set_cell((row, 0).finto(), Opaque(TermChar::new('┃', border_fg, border_bg)));
        buf.set_cell((row, scale.CARD_WIDTH-1).finto(), Opaque(TermChar::new('┃', border_fg, border_bg)));
    };

    for col in 2..(scale.CARD_WIDTH-2) {
        buf.set_cell((0, col).finto(), Opaque(TermChar::new('━', border_fg, border_bg)));
        buf.set_cell((scale.CARD_HEIGHT-1, col).finto(), Opaque(TermChar::new('━', border_fg, border_bg)));
    };

    Ok(buf)
}

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

pub fn make(scale: Scale) -> CardRepo {
    let mut outline_thin = make_card_shape(scale, CARD_BORDER, TERM_BG, None).unwrap();
    outline_thin.set_anchor((1, -1).finto());
    outline_thin.set_id("outline".into());
    // outline_thin.deactivate();

    let mut shadow = make_card_shape(scale, TERM_BG, TERM_BG, Some(TERM_BG)).unwrap();
    shadow.set_anchor((1, 1).finto());
    shadow.set_id("shadow".into());
    shadow.deactivate();

    let mut outline_good = make_card_shape(scale, GOOD_SET, GOOD_SET, None).unwrap();
    let mut outline_bad = make_card_shape(scale, BAD_SET, BAD_SET, None).unwrap();
    outline_good.set_id("good".into());
    outline_good.deactivate();
    outline_bad.set_id("bad".into());
    outline_bad.deactivate();

    let card_active = make_card_shape(scale, ACTIVE_BG, ACTIVE_BG, Some(ACTIVE_BG)).unwrap();
    let card = make_card_shape(scale, CARD_BG, CARD_BG, Some(CARD_BG)).unwrap();

    let mut cards_active = stamp_shapes(scale, card_active.clone(), ACTIVE_BG);
    let mut cards = stamp_shapes(scale, card.clone(), CARD_BG);
    cards = cards.into_iter()
        .map(
            |(k, mut inactive)| {
                let mut active = cards_active.get(&k).unwrap().clone();
                active.set_anchor((1, -1).finto());
                active.deactivate();
                active.set_id("active".into());
                inactive.set_id("inactive".into());
                let mut mom = FrameTree::new_branch(
                    vec![outline_bad.clone(), outline_good.clone(), active, inactive, outline_thin.clone(), shadow.clone()],
                    k.into(),
                    true, (0, 0).finto(), 0);
                mom.activate();
                (k, mom)
            }
        ).collect();

    let deck_active = stamp_question(scale, card_active.clone(), TERM_BG, TERM_BG);
    let mut deck = stamp_question(scale, card.clone(), TERM_BG, TERM_BG);

    deck.shup_tree(outline_thin.clone());
    deck.push_tree(deck_active.clone());

    CardRepo {
        cards,
        deck,
    }
}

fn get_raw_shape(c:Card, s:Scale) -> &'static str {
    match c.shape {
        CardShape::Oval => s.RAW_OVAL,
        CardShape::Diamond => s.RAW_DIAMOND,
        CardShape::Squiggle => s.RAW_SQUIGGLE
    }
}

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

fn get_raw_char(card: Card, ch: char, colr: Color, card_bg: Color) -> LayerCell {
    // Other striped options: '╋', '─'
    match (card.fill, ch) {
        (_, ' ') =>                 Opaque(TermChar::new(' ', card_bg, card_bg)),
        (CardFill::Solid, _) =>     Opaque(TermChar::new(' ', colr, colr)),
        (_, 'X') =>                 Opaque(TermChar::new(' ', colr, colr)),
        (CardFill::Striped, 'o') => Opaque(TermChar::new('╳', colr, card_bg)),
        (CardFill::Empty, 'o') =>   Opaque(TermChar::new(' ', card_bg, card_bg)),
        _ =>                        panic!("Unrecognized character in get_raw_char")
    }
}

//// This is kinda a variant of set_s method, necessary cuz we need to do special stuff depending on
//// shape contents
pub fn set_shape_rel(
    scale: Scale,
    lay: &mut FrameTree,
    card: Card,
    mut pos: TermPos,
    bg: Color,
) -> Result<()> {

    let shape = get_raw_shape(card, scale);
    let colr = get_raw_color(card);

    let start = pos;
    let start_x = start.x();
    let chars: Vec<char> = shape.chars().collect();

    for i in 0..chars.len() {
        if chars[i] == '\n' {
            pos = pos + (1, 0).finto();
            pos = pos.set_x(start_x);
        } else if chars[i] == ' ' {
            pos = pos + (0, 1).finto();
        } else {
            let c = get_raw_char(card, chars[i], colr, bg);
            lay.set_cell(pos, c);
            pos = pos + (0, 1).finto();
        };
    };

    Ok(())
}


// //     pub fn over(&self, other: &Self) -> Self {
// //         let mut corners = self.corners();
// //         corners.append(&mut other.corners());
// //         let (tl, br) = TermPos::bounding_box(corners);

// //         // result is just as wide as necessary to cover both input layers
// //         let mut lay = Self::new_by_bounds(String::new(), tl, br, Transparent);

// //         // For each position in this new layer...
// //         for pos in tl.range_to(br) {
// //             let ch: LayerCell = match (self.covers(pos), other.covers(pos)) {
// //                 // if self is opaque at this position, use the self cell. Otherwise, use other.
// //                 (true, true) =>
// //                     if let cel @ Opaque(_) = self.get_c(pos) {
// //                         cel
// //                     } else {
// //                         other.get_c(pos)
// //                     },
// //                 (true, false) => self.get_c(pos),
// //                 (false, true) => other.get_c(pos),
// //                 (false, false) => Transparent
// //             };

// //             lay.set_c(pos, ch);
// //         };

// //         lay
// //     }
