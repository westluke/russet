use super::*;

mod config;
use config::*;




pub fn print_card(buf: &mut impl io::Write, row:u16, col:u16, card:Card) -> io::Result<()> {
    let (y, x) = get_card_yx(row, col);
    print_card_yx(buf, y, x, card)?;
    Ok(())
}

// (y, x) is positiin of to top left corner of card outline
pub fn print_card_yx(buf: &mut impl io::Write, y:u16, x:u16, card:Card) -> io::Result<()> {
    if x == 0 || y == 0 {
        panic!("Cursor positions start at 1, not 0.");
    };
    print_card_outline(buf, y, x)?;
    print_card_contents(buf, y+1, x+1, card)?;
    Ok(())
}

// Print the edges of the card, in white
fn print_card_outline(buf: &mut impl io::Write, y:u16, x:u16) -> io::Result<()> {
    write!(buf, "{}{}{}", ts::Reset, tc::Bg(tc::Black), tc::Fg(tc::White))?;
    print_literal(buf, y, x, RAW_OUTLINE)?;
    Ok(())
}

// Just prints whatever is in lit with nothing fancy. Keeps previous styling.
// Only trick is that spaces are skipped (this permits transparency)
fn print_literal(buf: &mut impl io::Write, y:u16, x:u16, lit:&str) -> io::Result<()> {
    for (i, ln) in lit.lines().enumerate(){
        mv_cursor(buf, y + i as u16, x)?;
        write!(buf, "{}", ln)?
    };

    Ok(())
}

// Prints the shapes inside the card (up to 3)
fn print_card_contents (buf: &mut impl io::Write, y:u16, x:u16, card:Card) -> io::Result<()> {

    // offset comes from the requirement that the shapes be centered, no matter how many there are
    let offset = (SHAPE_WIDTH * (3 - card.number as u16)) / 2;

    for i in 0..(card.number as u16){

        // x must be adjusted if e.g. this is the third shape in the row
        let shape_pos = i*SHAPE_WIDTH;

        // there is a small amount of minimum spacing between adjacent shapes, and between the
        // shapes and the card outline
        let spacing = (i+1) * SHAPE_SPACING;

        print_card_shape(buf, y, x + offset + shape_pos + spacing, card)?;
    };
    Ok(())
}

// Print a single instance of one of this card's shapes in the specified position.
fn print_card_shape(buf: &mut impl io::Write, y: u16, x: u16, card: Card) -> io::Result<()> {
    let shape = get_raw_shape(card);
    for (i, ln) in shape.lines().enumerate(){
        print_card_shape_line(buf, y+(i as u16), x, ln, card)?;
    };
    Ok(())
    
}

// Style is reset at beginning of each line.
fn print_card_shape_line(buf: &mut impl io::Write, y: u16, x: u16, ln:&str, card: Card) -> io::Result<()> {
    write!(buf, "{}", ts::Reset)?;

    if card.fill == CardFill::Solid {
        return print_card_shape_line_solid(buf, y, x, ln, card);
    }
    
    let mut is_fill = false;

    for (i, ch) in ln.chars().enumerate(){
        if ch == ' ' { continue; };
        mv_cursor(buf, y, x + (i as u16))?;

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

pub fn mv_cursor(buf: &mut impl io::Write, y:u16, x:u16) -> io::Result<()> {
    write!(*buf, "{}", termion::cursor::Goto(x, y))?;
    Ok(())
}

fn print_card_shape_line_solid(buf: &mut impl io::Write, y: u16, x: u16, ln:&str, card: Card) -> io::Result<()> {
    let core = ln.trim();
    let first = ln.find(|c:char| !c.is_whitespace() ).unwrap();
    mv_cursor(buf, y, x + (first as u16))?;
    write!(buf, "{}{}{}", ts::Reset, get_raw_solid_style(card), core)?;
    Ok(())
}




fn get_raw_solid_style(c:Card) -> String {
    if c.fill != CardFill::Solid { panic!(); };
    let col = get_raw_color(c);
    format!("{}{}", tc::Fg(col), tc::Bg(col))
}

fn get_raw_color(c:Card) -> &'static dyn tc::Color {
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
        format!("{}{}", tc::Bg(col), tc::Fg(col))
    } else {
        format!("{}{}", tc::Bg(tc::Black), tc::Fg(col))
    }
}

// Returns styling string for fill ('x') characters. block compilation isn't relevant here.
fn get_raw_fill_style(c:Card) -> String {
    let col = get_raw_color(c);
    match c.fill {
        CardFill::Solid =>  format!("{}{}", tc::Bg(col), tc::Fg(col)),
        _ =>                format!("{}{}", tc::Bg(tc::Black), tc::Fg(col)),
    }
}

// Rows / Columns start at (0, 0), from top left.
// Indexing is like with matrices, y-val (row) first.
fn get_card_yx(row:u16, col:u16) -> (u16, u16) {
    let y = (row * CARD_WIDTH) + (row * CARD_SPACING);
    let x = (col * CARD_HEIGHT) + (col * CARD_SPACING);
    (y+1, x+1)
}

fn get_raw_fill(c:Card) -> char {
    match c.fill {
        CardFill::Solid => ' ',    // solid shapes don't have a fill, they have a background
        CardFill::Striped => '─',
        CardFill::Empty => ' '
    }
}

