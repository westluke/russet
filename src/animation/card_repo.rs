use crate::deck::{Card, CardShape, CardColor, CardFill, CardNumber, all_cards};
use crate::term_char::TermChar;
use crate::pos::TermPos;
use crate::util::{*, config::*};
use crate::{Id, IdManager};

use crate::sprites::sprite::Sprite;
use crate::sprites::sprite_tree::*;
use crate::sprites::img::Img;
use crate::sprites::*;

use std::collections::{HashMap, hash_map::DefaultHasher};

use crossterm::style::Color;

#[derive(Clone)]
pub struct EmbodiedCard {
    pub handle: Card,
    pub tree: SpriteTree,
    pub tree_ids: IdManager<SpriteTree>,
    pub sprite_ids: IdManager<Sprite>
}

#[derive(Clone, Default)]
pub struct EmbodiedDeck {
    pub tree: SpriteTree,
    pub tree_ids: IdManager<SpriteTree>,
    pub sprite_ids: IdManager<Sprite>
}

pub struct CardRepo {
    deck: EmbodiedDeck,
    cards: HashMap<Card, EmbodiedCard>
}

impl CardRepo {
    pub fn new(scale: Scale) -> Self {
        make(scale)
    }

    pub fn card(&self, c: Card) -> EmbodiedCard {
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

    pub fn deck(&self) -> EmbodiedDeck {
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

pub fn stamp_shape(scale: Scale, img: &mut Img, card: Card, bg: Color) {
    let num: i16 = card.number.into();

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
        set_shape(scale, img, card, (drop, offset + shape_pos + spacing).finto(), bg);
    };
}

pub fn stamp_question(scale: Scale, img: &mut Img, fg: Color, bg: Color) {
    let mut offset = scale.CARD_WIDTH;
    offset -= scale.SHAPE_WIDTH;
    offset /= 2;

    let mut drop = scale.CARD_HEIGHT;
    drop -= scale.SHAPE_HEIGHT;
    drop /= 2;

    set_s_clear(img, (drop, offset).finto(), String::from(scale.RAW_QUESTION), fg, bg);
}

pub fn stamp_shapes(scale: Scale, img: &Img, bg: Color) -> HashMap<Card, Img> {
    let mut card_bufs = HashMap::new();

    for card in all_cards() {
        let mut clon = img.clone();
        stamp_shape(scale, &mut clon, card, bg);
        card_bufs.insert(card, clon);
    }

    card_bufs
}

fn set_s(img: &mut Img, mut pos: TermPos, s: String, fg: Color, bg: Color) {
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
            img.set(pos.finto(), Opaque(TermChar::new(chars[i], fg, bg)));
            pos = pos + (0, 1).finto();
        };
    };
}

fn set_s_clear(img: &mut Img, mut pos: TermPos, s: String, fg: Color, bg: Color) {
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
            img.set(pos.finto(), Opaque(TermChar::new(chars[i], fg, bg)));
            pos = pos + (0, 1).finto();
        };
    };
}

//// This is kinda a variant of set_s method, necessary cuz we need to do special stuff depending on
//// shape contents
pub fn set_shape(
    scale: Scale,
    img: &mut Img,
    card: Card,
    mut pos: TermPos,
    bg: Color,
) {

    let shape = get_raw_shape(card, scale);
    let colr = get_raw_color(card);

    let start = pos;
    let start_x = start.x();
    let chars: Vec<char> = shape.chars().collect();

    for i in 0..chars.len() {

        // If we hit a newline, advance pos one row down
        if chars[i] == '\n' {
            pos = pos + (1, 0).finto();
            pos = pos.set_x(start_x);

        // Spaces are skipped
        } else if chars[i] == ' ' {
            pos = pos + (0, 1).finto();

        // chars are interpreted and set
        } else {
            let c = get_raw_char(card, chars[i], colr, bg);
            img.set(pos.finto(), c);
            pos = pos + (0, 1).finto();
        };
    };
}

// Should outline be optional?
pub fn card_base(
    scale: Scale,
    edge_fg: Color,
    edge_bg: Color,
    bg: Option<Color>
) -> Img {
    let bg = bg.map_or(Transparent, |c| SpriteCell::Opaque(TermChar::Bg(c)));
    let mut img = Img::rect(scale.CARD_HEIGHT.finto(), scale.CARD_WIDTH.finto(), bg);
    let (tl, tr, bl, br) = (img.top_left(), img.top_right(), img.bottom_left(), img.bottom_right());
    img.set(tl.finto(), Transparent);
    img.set(tr.finto(), Transparent);
    img.set(bl.finto(), Transparent);
    img.set(br.finto(), Transparent);
    set_s_clear(&mut img, (0, 0).finto(), String::from(CARD_TL), edge_fg, edge_bg);
    set_s_clear(&mut img, (0, scale.CARD_WIDTH-2).finto(), String::from(CARD_TR), edge_fg, edge_bg);
    set_s_clear(&mut img, (scale.CARD_HEIGHT-2, 0).finto(), String::from(CARD_BL), edge_fg, edge_bg);
    set_s_clear(&mut img, (scale.CARD_HEIGHT-2, scale.CARD_WIDTH-2).finto(), String::from(CARD_BR), edge_fg, edge_bg);
    for row in 2..(scale.CARD_HEIGHT-2) {
        img.set((row.finto(), 0), Opaque(TermChar::new('┃', edge_fg, edge_bg)));
        img.set((row.finto(), (scale.CARD_WIDTH-1).finto()), Opaque(TermChar::new('┃', edge_fg, edge_bg)));
    };

    for col in 2..(scale.CARD_WIDTH-2) {
        img.set((0, col.finto()), Opaque(TermChar::new('━', edge_fg, edge_bg)));
        img.set(((scale.CARD_HEIGHT-1).finto(), col.finto()), Opaque(TermChar::new('━', edge_fg, edge_bg)));
    };
    img
}

pub fn make(scale: Scale) -> CardRepo {
    let mut outline_thin = card_base(scale, CARD_BORDER, TERM_BG, None);
    // outline_thin.set_anchor((1, -1).finto());

    let mut shadow = card_base(scale, TERM_BG, TERM_BG, Some(TERM_BG));
    // shadow.set_anchor((1, 1).finto());

    let mut outline_good = card_base(scale, GOOD_SET, GOOD_SET, None);
    let mut outline_bad = card_base(scale, BAD_SET, BAD_SET, None);

    let card_active = card_base(scale, ACTIVE_BG, ACTIVE_BG, Some(ACTIVE_BG));
    let card_inactive = card_base(scale, CARD_BG, CARD_BG, Some(CARD_BG));

    let mut deck_active = card_active.clone();
    stamp_question(scale, &mut deck_active, TERM_BG, TERM_BG);

    let mut deck_inactive = card_inactive.clone();
    stamp_question(scale, &mut deck_inactive, TERM_BG, TERM_BG);

    let mut cards_active = stamp_shapes(scale, &card_active, ACTIVE_BG);
    let cards_inactive = stamp_shapes(scale, &card_inactive, CARD_BG);

    let mut cards = HashMap::new();

    // For each card, finalize the associated Imgs, then turn them into presprites and combine them
    // into a SpriteForest. Keep track of Ids in the process, and produce IdManager simultaneously.
    for (handle, inactive) in cards_inactive.into_iter() {
        let mut tree_ids: IdManager<SpriteTree> = IdManager::default();
        let mut sprite_ids: IdManager<Sprite> = IdManager::default();
        let mut tree = SpriteTree::default();

        let mut active_card: Sprite = cards_active.remove(&handle).unwrap().into();
        active_card.set_visible(Invisible);
        sprite_ids.insert((handle, "active").into(), active_card.id());
        tree_ids.insert(
            (handle, "active").into(),
            tree.push_sprite(new_stn(active_card), INHERIT_NONE)
        );

        // Cards are anchored at the top left corner of the ACTIVE/YELLOW variant.
        // Equivalently, at the top left corner of the floating outline of the inactive variant.

        // Inactive card is above its own outline, and to the right.
        let mut inactive_card: Sprite = inactive.into();
        let inactive_border: Sprite = outline_thin.clone().into();
        inactive_card.reanchor((-1, 1).finto());
        inactive_card.reorder(1);
        sprite_ids.insert((handle, "inactive_card").into(), inactive_card.id());
        sprite_ids.insert((handle, "inactive_border").into(), inactive_border.id());

        let mut inactive_subtree = SpriteTree::default();
        tree_ids.insert((handle, "inactive").into(), inactive_subtree.id());

        tree_ids.insert(
            (handle, "inactive_card").into(),
            inactive_subtree.push_sprite(new_stn(inactive_card), INHERIT_NONE)
        );
        tree_ids.insert(
            (handle, "inactive_border").into(),
            inactive_subtree.push_sprite(new_stn(inactive_border), INHERIT_NONE)
        );
        tree.push_tree(inactive_subtree, INHERIT_NONE);
        cards.insert(handle, EmbodiedCard {handle, tree, tree_ids, sprite_ids});
    }

    CardRepo {
        cards,
        deck: Default::default()
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

fn get_raw_char(card: Card, ch: char, colr: Color, card_bg: Color) -> SpriteCell {
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
