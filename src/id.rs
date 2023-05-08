use std::{io, thread, time, marker::PhantomData};
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use uuid::Uuid;
use bimap::{BiMap, Overwritten};

use crate::deck::*;

#[derive(Debug)]
pub struct Id<T>(Uuid, PhantomData<T>);

impl<T> Default for Id<T> {
    fn default() -> Self {
        Self(Uuid::new_v4(), Default::default())
    }
}

impl<T> Hash for Id<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T> Copy for Id<T> {}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Display for Id<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Id({:?}, {:?})", self.0, self.1)
    }
}

impl<T> PartialEq for Id<T> {
    fn eq(&self, oth: &Self) -> bool {
        self.0 == oth.0
    }
}

impl<T> Eq for Id<T> {}


impl<T> Id<T> {
    pub fn from<S>(oth: Id<S>) -> Self {
        Self(oth.0, Default::default())
    }

    pub fn into<S>(self) -> Id<S> {
        Id(self.0, Default::default())
    }
}

#[derive(Hash, Default, Debug, Clone, PartialEq, Eq)]
pub struct IdKey {
    pub card: Option<Card>,
    pub name: Option<String>
}

impl From<String> for IdKey {
    fn from(s: String) -> Self {
        Self { card: None, name: Some(s) }
    }
}

impl From<&str> for IdKey {
    fn from(s: &str) -> Self {
        Self { card: None, name: Some(s.into()) }
    }
}

impl From<Card> for IdKey {
    fn from(c: Card) -> Self {
        Self { card: Some(c), name: None }
    }
}

impl From<(Card, String)> for IdKey {
    fn from((c, s): (Card, String)) -> Self {
        Self { card: Some(c), name: Some(s) }
    }
}

impl From<(Card, &str)> for IdKey {
    fn from((c, s): (Card, &str)) -> Self {
        Self { card: Some(c), name: Some(s.into()) }
    }
}

#[derive(Default, Clone)]
pub struct IdManager<T> (BiMap<IdKey, Id<T>>);

impl<T> IdManager<T> {
    pub fn absorb(&mut self, other: IdManager<T>) {
        for (l, r) in other.0 {
            self.insert(l, r);
        }
    }

    pub fn insert(&mut self, idkey: IdKey, id: Id<T>) -> Overwritten<IdKey, Id<T>> {
        self.0.insert(idkey, id)
    }

    pub fn by_id(&self, id: Id<T>) -> Option<&IdKey> {
        self.0.get_by_right(&id)
    }

    pub fn by_idkey(&self, idkey: IdKey) -> Option<Id<T>> {
        self.0.get_by_left(&idkey).copied()
    }

    // pub fn get;
    // pub fn remove;
    // pub fn merge;
    // pub fn values;
    // pub fn len;
    // pub fn keys;
}
