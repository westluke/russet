use std::{io, thread, time, marker};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

use crate::deck::*;

#[derive(Clone, Hash, Debug, Eq)]
pub struct Id {
    uuid: Uuid,
    card: Option<Card>,
    name: Option<String>,
    // _marker: marker::PhantomData
}

impl Default for Id {
    fn default() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            card: Default::default(),
            name: Default::default(),
            // _marker: Default::default()
        }
    }
}

impl PartialEq for Id {
    fn eq(&self, other: &Id) -> bool {
        self.uuid == other.uuid &&
        self.card == other.card &&
        self.name == other.name
    }
}

impl Display for Id {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "Id({}", self.uuid)?;

        if let Some(c) = self.card {
            write!(fmt, ", {}", c)?;
        }

        if let Some(n) = self.name.clone() {
            write!(fmt, ", {}", n)?;
        };

        write!(fmt, ")")
    }
}

impl From<Card> for Id {
    fn from(c: Card) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            card: Some(c),
            name: None,
            // _marker: Default::default()
        }
    }
}

// impl From<String> for Id {
//     fn from(s: String) -> Self {
//         Self {
//             uuid: Uuid::new_v4(),
//             card: None,
//             name: Some(s)
//         }
//     }
// }

impl From<&str> for Id {
    fn from(s: &str) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            card: None,
            name: Some(s.into()),
            // _marker: Default::default()
        }
    }
}

// impl<T, S> From<&Id<S>> for Id {
//     fn from(id: &Id<S>) -> Self {
//         Self {
//             uuid: id.uuid,
//             card: id.card,
//             name: id.name.clone(),
//             // _marker: Default::default()
//         }
//     }
// }
