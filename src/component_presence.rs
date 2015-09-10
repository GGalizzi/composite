//! Optional presence of a component.

use std::ops::{Deref, DerefMut};

/// Type `ComponentPresence` represents the optional presence of a component.
///
/// Used inside the EntityData component.
///
/// This type implements Deref and DerefMut allowing easy access to the component
/// but that only should be used when you are certain that component is present in the entity.
/// 
/// You can check presence with `has_it` or `lacks_it`
#[derive(Debug)]
pub enum ComponentPresence<T> {
    Comp(T),
    None
}

use self::ComponentPresence::*;

impl<T> ComponentPresence<T> {

    /// Returns the inner Component if there is one.
    /// Panics otherwise.
    #[inline]
    pub fn unwrap(self) -> T {
        match self {
            Comp(c) => c,
            None => panic!("called ComponentPresence::unwrap on a None value"),
        }
    }
        
    #[inline]
    pub fn as_ref(&self) -> ComponentPresence<&T> {
        match *self {
            Comp(ref c) => Comp(c),
            None => None,
        }
    }
    #[inline]
    pub fn as_mut(&mut self) -> ComponentPresence<&mut T> {
        match *self {
            Comp(ref mut c) => Comp(c),
            None => None,
        }
    }

    #[inline]
    pub fn has_it(&self) -> bool {
        match *self {
            Comp(_) => true,
            None => false,
        }
    }

    #[inline]
    pub fn lacks_it(&self) -> bool {
        !self.has_it()
    }
}

impl<T> Deref for ComponentPresence<T> {
    type Target = T;

    fn deref<'a>(&'a self) -> &'a T {
        match *self {
            Comp(ref c) => c,
            None => panic!("Lacks component"),
        }
    }
}

impl<T> DerefMut for ComponentPresence<T> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut T {
        match *self {
            Comp(ref mut c) => c,
            None => panic!("Lacks component"),
        }
    }
}
