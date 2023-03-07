use std::cell::RefCell;
use std::io::{Write};

use super::Sprite;

// How will dirt calculations work?
// cache a grid of dirt marks?
pub struct ZVec<'a> ( Vec<&'a RefCell<Sprite>> );

impl<'a> ZVec<'a> {
    pub fn new() -> Self {
        Self (Vec::new())
    }

    pub fn insert(&mut self) {
    }

    pub fn write(writer: impl Write) {
    }
}

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
