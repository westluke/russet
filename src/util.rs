use crossterm::style::Color;
use std::sync::RwLock;
use std::fmt::Debug;
use crossterm::terminal;
use log::{warn};

pub mod err;
pub mod config;

pub use err::*;
pub use config::*;

pub type Result<T> = std::result::Result<T, SetError>;

pub struct TermSize {
    size: RwLock<(i16, i16)>
}

// Force-From - use when you know this conversion won't fail, but you don't want
// to unwrap constantly
pub trait FFrom<T> {
    fn ffrom (_: T) -> Self;
}

pub trait FInto<T> {
    fn finto (self) -> T;
}

impl<S, T> FFrom<S> for T
where
    T: TryFrom<S>,
    <T as TryFrom<S>>::Error: Debug
{
    fn ffrom(s: S) -> Self {
        Self::try_from(s).unwrap()
    }
}

impl<S, T> FInto<T> for S where T: FFrom<S> {
    fn finto (self) -> T {
        T::ffrom(self)
    }
}

// Simplifies extracting result value on results you know should never fail
pub trait Unwrap<R, E> {
    fn unwrap(self) -> R;
}

impl<R, E> Unwrap<R, E> for std::result::Result<R, E> {
    fn unwrap(self) -> R {
        self.ok().unwrap()
    }
}

impl TermSize {
    pub const fn new () -> Self {
        Self{ size: RwLock::new((0, 0)) }
    }

    pub fn update(&self) -> (i16, i16) {
        let mut lock = self.size.write().unwrap();
        let new = terminal::size();

        // Switch order to be row-major
        if let Ok((x, y)) = new {
            let y = i16::try_from(y).unwrap();
            let x = i16::try_from(x).unwrap();

            if y >= MIN_HEIGHT && x >= MIN_WIDTH {
                *lock = (y, x);
            } else {
                warn!("Terminal too small! Returning last valid value");
            }

        } else {
            warn!("Crossterm failed to determine terminal size! Returning last known value.");
        };

        // if a size check fails, we just go with the last value
        *lock
    }

    pub fn dims(&self) -> (i16, i16) {
        let lock = self.size.read().unwrap();
        *lock
    }

    pub fn height(&self) -> i16 {
        self.dims().1
    }

    pub fn width(&self) -> i16 {
        self.dims().1
    }
}

pub static TS: TermSize = TermSize::new();
