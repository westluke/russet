use std::{io, thread, time, marker::PhantomData};
use std::fmt::{Display, Formatter};
use std::collections::HashMap;
use uuid::Uuid;

use crate::deck::*;

#[derive(Hash, Debug, Eq)]
pub struct Id<T>(Uuid, PhantomData<T>);

impl<T> Default for Id<T> {
    fn default() -> Self {
        Self(Uuid::new_v4(), Default::default())
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

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Self(self.0, self.1)
    }
}

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
    card: Option<Card>,
    name: Option<String>
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

impl IdKey {
    pub fn card(&self) -> Option<Card> {
        self.card
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn set_card(&mut self, c: Option<Card>) {
        self.card = c
    }

    pub fn set_name(&mut self, s: Option<String>) {
        self.name = s
    }
}

#[derive(Default, Debug, Clone)]
pub struct IdManager<T> {
    ids: HashMap<IdKey, Id<T>>
}

impl<T> IdManager<T> {
    pub fn insert(&mut self, idkey: IdKey, id: Id<T>) -> Option<Id<T>> {
        self.ids.insert(idkey, id)
    }
    // pub fn get;
    // pub fn remove;
    // pub fn merge;
    // pub fn values;
    // pub fn len;
    // pub fn keys;
}

// #[derive(Clone, Hash, Debug, Eq)]
// pub struct Id {
//     uuid: Uuid,
//     card: Option<Card>,
//     name: Option<String>,
//     // _marker: marker::PhantomData
// }

// impl Default for Id {
//     fn default() -> Self {
//         Self {
//             uuid: Uuid::new_v4(),
//             card: Default::default(),
//             name: Default::default(),
//             // _marker: Default::default()
//         }
//     }
// }

// impl PartialEq for Id {
//     fn eq(&self, other: &Id) -> bool {
//         self.uuid == other.uuid &&
//         self.card == other.card &&
//         self.name == other.name
//     }
// }

// impl Display for Id {
//     fn fmt(&self, fmt: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
//         write!(fmt, "Id({}", self.uuid)?;

//         if let Some(c) = self.card {
//             write!(fmt, ", {}", c)?;
//         }

//         if let Some(n) = self.name.clone() {
//             write!(fmt, ", {}", n)?;
//         };

//         write!(fmt, ")")
//     }
// }

// impl From<Card> for Id {
//     fn from(c: Card) -> Self {
//         Self {
//             uuid: Uuid::new_v4(),
//             card: Some(c),
//             name: None,
//             // _marker: Default::default()
//         }
//     }
// }

// // impl From<String> for Id {
// //     fn from(s: String) -> Self {
// //         Self {
// //             uuid: Uuid::new_v4(),
// //             card: None,
// //             name: Some(s)
// //         }
// //     }
// // }

// impl From<&str> for Id {
//     fn from(s: &str) -> Self {
//         Self {
//             uuid: Uuid::new_v4(),
//             card: None,
//             name: Some(s.into()),
//             // _marker: Default::default()
//         }
//     }
// }

// // impl<T, S> From<&Id<S>> for Id {
// //     fn from(id: &Id<S>) -> Self {
// //         Self {
// //             uuid: id.uuid,
// //             card: id.card,
// //             name: id.name.clone(),
// //             // _marker: Default::default()
// //         }
// //     }
// // }
