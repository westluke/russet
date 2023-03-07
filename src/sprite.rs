use std::collections::{HashSet, HashMap};
use std::cell::RefCell;

use crate::pos::*;
use crate::term_char::*;
use crate::util::*;
use crate::Id;

use uuid::Uuid;

mod grid;
// mod line_update;
mod termable;

mod sprite_anchor_tree;
mod sprite_order_tree;
mod sprite_onto_tree;
mod sprite_manager;

pub use sprite_manager::SpriteManager;
use grid::Grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LayerCell {
    Opaque(TermChar),
    #[default]
    Transparent,
}

pub use LayerCell::*;

impl LayerCell {
    pub fn is_opaque(&self) -> bool {
        return !(*self == Transparent)
    }

    pub fn is_transparent(&self) -> bool {
        return *self == Transparent
    }

    pub fn bg() -> Self {
        Self::Opaque(TermChar::new(' ', TERM_BG, TERM_BG))
    }
}

#[derive(Debug, Clone)]
pub struct PreSprite {
    img: Grid<LayerCell>,
    anchor: (i16, i16),
    id: Id,

    // Only in the case of ties do we advance to additional entries
    zs: Vec<i16>
}

// There is no reason we should be making up default values when getters/setters fail! We can just
// report them, legitimately, as errors. Defaulting can always be done in the calling code.

impl PreSprite {
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            img: Grid::new(height, width, LayerCell::default()),
            anchor: (0, 0),
            id: Id::default(),
            zs: Vec::<i16>::default()
        }
    }

    pub fn mk(img: Grid<LayerCell>, anchor: (i16, i16), id: Id, zs: Vec<i16>) -> Self {
        Self { img, anchor, id, zs }
    }

    // pub fn with_size(height: usize, width: usize)
    pub fn get(&self, pos: TermPos) -> Result<LayerCell> {
        self.img.get(pos)
    }

    pub fn set(&mut self, pos: TermPos, cel: LayerCell) -> Result<LayerCell> {
        self.img.set(pos, cel)
    }
}

#[derive(Debug, Clone, Default)]
pub struct PreSpriteBuilder {
    anchor: (i16, i16),
    id: Id,
    zs: Vec<i16>
}

impl PreSpriteBuilder {
    pub fn anchor(&mut self, anchor: (i16, i16)) -> &mut Self {
        self.anchor = anchor;
        self
    }

    pub fn id(&mut self, id: Id) -> &mut Self {
        self.id = id;
        self
    }

    pub fn zs(&mut self, zs: Vec<i16>) -> &mut Self {
        self.zs = zs;
        self
    }

    // Grid doesn't have a sensible default, so we require img at the final step
    pub fn build(&self, img: Grid<LayerCell>) -> PreSprite {
        PreSprite::mk(img, self.anchor, self.id, self.zs)
    }
}

// Wait. Would it be better for the tree structures to be just integrated into the sprite?
// i.e. each sprite owns RCs of its children?
// And how do I avoid loops??? Careful coding, that's it I think.
// Yeah, I think so. Ugh no, its not, I forgot, cuz not every branch is a sprite.
// Ok, and all my alternative solutions seem to be converging to my existing idea. Trees storing sprites.
//
// I could just have Sprite contain a PreSprite but, eh, it's not worth the tradeoff.
// JK yes it is, cuz then i can use presprite methods.
#[derive(Debug, Clone)]
pub struct Sprite<'a> {
    pre_sprite: PreSprite,
    dirt: &'a RefCell<HashMap<i16, HashSet<i16>>>,
}

impl<'a> Sprite<'a> {
    pub fn new(pre_sprite: PreSprite, dirt: &'a RefCell<HashMap<i16, HashSet<i16>>>, zs: Vec<i16>) -> Self {
        // Actually, should Sprites always be initialized with maximal dirt? Lol wait this dirt is
        // external. So also it doesn't have to be Rc? Try it with reference first I guess.
        // Cuz multiple ownership of dirt is unnecessary here, the manager owns it.
        // sprites don't rly need to be wrapped in Rcs either, the manager vector will own them.
        // Modification methods could also pass out custom ref objects to sprites?
        // That or operate using IDs
        Self { pre_sprite, dirt, zs }
    }

    pub fn reanchor(&mut self) {
    }

    pub fn reorder(&mut self) {
    }

    pub fn destroy(&mut self) {
    }

    pub fn dirty_all(&self) {
    }

    pub fn get(&self, pos: TermPos) -> Result<LayerCell> {
        self.img.get(pos)
    }
    pub fn set(&mut self, pos: TermPos, cel: LayerCell) -> Result<LayerCell> {
        self.img.set(pos, cel)
    }
}

#[derive(Default)]
pub struct SpriteTree<'a> {
    node: Option<&'a RefCell<Sprite<'a>>>,
    children: Vec<SpriteTree<'a>>,
    id: Uuid
}

// Who should get the IDs? Sprites, or the nodes that contain them?
// The thing is, nodes DON'T contain them, they just contain references to them.
// So it would be odd if sprites had no id of their own?

// I could make every node have a sprite, and just let sprites be empty. But that seems kinda ugly to me.
// Here's anotehr idea: should every node have its own id, regardless of whether it contains a sprite?
// Are they really separate enough things?
// I think so, actually.

// Idea: instead of having dirt be local to sprite, and recompiled every draw,
// every sprite gets a handle to a single dirt hash. tradeoffs?
// Oh right and remember that I need to split up processing further, isolate io to new thread (io is blocking apparently?)
// io thread just gets text to print, nothing more (minimize its work)


// Should there be a z_tree as well??
// And what are my options for reaching into these data structures and changing sprites?
// by id? by cursor? what?
// I think just by id. That's more than fast enough.


//pub struct FrameBuf<T: Write> {
//    // The underlying Write object (should be a terminal, probably stdout)
//    under: T,

//    // Each layer is an independent "panel" that can be manipulated across the screen, i.e. a
//    // playing card sliding around. Start of the vec is top of the stack
//    frame_tree: FrameTree
//}

//impl<T: Write> FrameBuf<T> {
//    pub fn new(under: T, frame_tree: FrameTree) -> Self {
//        Self { under, frame_tree }
//    }

//    pub fn tree(&self) -> &FrameTree {
//        &self.frame_tree
//    }

//    pub fn tree_mut(&mut self) -> &mut FrameTree {
//        &mut self.frame_tree
//    }


//    // What's the right abstraction for frame tree here?
//    // Am I SURE I just want to hand over mutable access to internals?
//    // Yeah I think it's fine actually. Cuz everything is still mediated by method api anyways.
//    //
//    // Also, means all of these functions can be provided just by leaf nodes, pretty much.

//    // // Pushes layer onto the TOP (most visible part) of the stack
//    // pub fn push_layer(&mut self, id: String, lay: Layer) {
//    //     self.layer_groups.insert(0, LayerGroup::new(id, vec![lay]));
//    // }

//    // // Slides layer under the BOTTOM (least visible part) of the stack
//    // pub fn shup_layer(&mut self, id: String, lay: Layer) {
//    //     self.layer_groups.push(LayerGroup::new(id, vec![lay]));
//    // }

//    // pub fn push_layer_group(&mut self, layg: LayerGroup) {
//    //     self.layer_groups.insert(0, layg);
//    // }

//    // pub fn shup_layer_group(&mut self, layg: LayerGroup) {
//    //     self.layer_groups.push(layg);
//    // }

//    // pub fn get_layer_mut(&mut self, gid: String, id: String) -> Option<&mut Layer> {
//    //     for lay in &mut self.layer_groups {
//    //         if lay.get_id() == gid {
//    //             return lay.get_layer_mut(id);
//    //         }
//    //     }
//    //     return None;
//    // }

//    // pub fn get_layer_group_mut(&mut self, gid: String) -> Option<&mut LayerGroup> {
//    //     for lay in &mut self.layer_groups {
//    //         if lay.get_id() == gid {
//    //             return Some(lay);
//    //         }
//    //     }
//    //     None
//    // }

//    // Hmm. theoretically, if this wasn't performing well enough,
//    // I could use several work-stealing threads to parallelize it pretty easily.
//    // That would be pretty cool if I ever split this out into a separate rendering engine.
//    pub fn flush(&mut self)  {
//        // self.frame_tree.propagate_dirt();
        
//        let mut dirt: HashMap<i16, HashSet<i16>> = self.frame_tree.dirt();
//        dirt.retain(|&k, mut v| {
//            v.retain(|&e| e >= 0 && e < TS.width());
//            k >= 0 && k < TS.height()
//        });

//        // info!("{:?}", dirt);
//        // info!("height: {:?}", TS.height());

//        for (row, cols) in dirt {
//            // info!("{:?}", row);
//            // if row < 0 || row >= TS.height() { continue; };
//            let mut lnup = LineUpdate::new(TS.width());

//            for col in cols {
//                // if col < 0 || col >= TS.width() { continue; };
//                let cel = self.frame_tree.cell((row, col).finto());
//                // info!("{:?}", cel);
//                lnup.set(col, cel);
//                // info!("just set in linupdate: {:?}, {:?}", col, cel);
//            }

//            queue!(self.under, cursor::MoveTo(0, row.finto()));

//            for (col, styled) in lnup.finalize() {
//                queue!(
//                    self.under,
//                    cursor::MoveToColumn(col.finto()),
//                    PrintStyledContent(styled)
//                );
//            }
//        }

//        self.frame_tree.clean();
//        self.under.flush();
//    }

//        // but i feel like im missing some possible benefits associated just with using this new
//        // tree structure...
//        //
//        // should i be allowed to reach in and modify nodes directly????
//        // I think so. Can I account for that in a nice way?
//        //
//        // Ok, I think I should be doing all of these checks at flushtime anyways.
//        // Doesn't really make sense to have an active tracker of dirties.
//        // Why?
//        //
//        // Well, how would you track that? It would require traversing the whole tree everytime you
//        // just wanted to modify a node, and that kinda sucks. If you extract mut node references,
//        // that makes more sense, but then you can't update dirties higher up the tree.
//        //
//        // So every node's dirty map is just the sum of the dirty maps of its children, if any.
//        // Also, is there any point in caching an output result at every branch in the tree?
//        // Not clear that it makes any sense... I suppose, if I really needed the performance,
//        // each cell in the cache could be associated with the leaf it came from, and each dirty
//        // marker could also be associated with a leaf, and then if I see the cell in the cache is
//        // before the dirty leaf, just keep it.... But that's so complicated lmao.
//        // Your existing implementation was fast enough. Just do the same thing but recursive,
//        // and now you get the benefit of grouped translation! Stop being so fancy.
//        //
//        // first day of symptoms was thursday.
//        // so allowed to come back tuesday
//        // self.frame_tree.propagate_dirt();

//        // let dirtied: HashMap<i16, <HashSet<i16>> = self.frame_tree.get_dirt();

//        // for every dirty line:
//            // start a new line update
//            // for each dirty cell:
//                // fill in the line update
//                //

//    // }
//}
