use std::ops::{Deref};
use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};
use std::collections::{HashMap, HashSet};
use std::collections::{hash_map::Iter as MapIter, hash_set::Iter as SetIter};
use std::slice::Iter as VecIter;

use crate::pos::TermPos;
use crate::bounds::Bounds;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct Dirt (Rc<RefCell<HashMap<i16, Vec<i16>>>>);

impl Dirt {

    // Should produce an iterator over column (vec) iterators
    // pub fn iter<'a>(&'a self) -> RefMut<MapIter<'a, i16, Vec<i16>>> {
    //     let x = self.0.borrow();
    //     let iter: RefMut<MapIter<i16, Vec<i16>>> = RefMut::map(x, |t| &mut t.iter());
    //     iter
    // }

    // pub fn merge(&mut self, mut other: Self) {
    //     for (&k, v) in other.borrow_mut().iter() {
    //         let ent = self.borrow_mut()
    //             .entry(k)
    //             .or_default();
    //         ent.extend(v);
    //         ent.sort();
    //     }
    // }

    pub fn borrow(&self) -> Ref<HashMap<i16, Vec<i16>>> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<HashMap<i16, Vec<i16>>> {
        self.0.borrow_mut()
    }

    pub fn set_dirty(&self, p: TermPos) {
        let mut rf = self.0.borrow_mut();
        let v = rf
            .entry(p.y())
            .or_insert(vec![]);

        if let Err(i) = v.binary_search(&p.x()) {
            v.insert(i, p.x());
        }
    }


    // DEFAULT MAKES VISIBLE FALSE

    pub fn dirty_all(&self, bnd: Bounds<i16>) {
        let mut rf = self.0.borrow_mut();
        for y in bnd.y_range() {
            let v = rf
                .entry(y)
                .or_insert(vec![]);

            v.extend(bnd.x_range());
            v.sort();
            v.dedup();

            // Alternatively, consider using partition_point for each individual insert
        }
    }

    pub fn clear(&self) {
        self.0.borrow_mut().clear()
    }
}

