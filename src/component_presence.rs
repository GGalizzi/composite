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
///
/// # Examples
///
/// ```
/// let mut manager = EntityManager::new();
/// let ent = manager.new_entity();
/// manager.add_component_to(ent, Position{x:1, y:2});
/// // Makes use of `Deref` to get a reference of `Position` out of `ComponentPresence`
/// println!("pos: {:?}", manager.data[ent].position.x);
/// // Makes use of `DerefMut` to get a mutable reference of `Position`
/// manager.data[ent].position.x += 5;
/// ```
///
/// # Panics
/// Trying to access a component that isn't held by the entity will panic.
/// 
/// ```
/// let mut manager = EntityManager::new();
/// let ent = manager.new_entity();
/// println!("pos: {:?}", manager.data[ent].position.x);
/// ```
///
/// Always be sure that the entity does pocess the component.
/// 
/// If you are only accessing components inside a `System` then that is already handled
/// for the systems required components.
///
/// Otherwise use `has_it` or a `match`.
#[derive(Debug, Clone)]
pub enum ComponentPresence<T> {
    Has(T),
    Lacks
}

use self::ComponentPresence::*;

impl<T> ComponentPresence<T> {

    /// Returns the inner Component if there is one.
    /// Panics otherwise.
    #[inline]
    pub fn unwrap(&mut self) -> &mut T {
        match *self {
            Has(ref mut c) => c,
            Lacks => panic!("called ComponentPresence::unwrap on a None value"),
        }
    }
        
    #[inline]
    pub fn as_ref(&self) -> ComponentPresence<&T> {
        match *self {
            Has(ref c) => Has(c),
            Lacks => Lacks,
        }
    }
    #[inline]
    pub fn as_mut(&mut self) -> ComponentPresence<&mut T> {
        match *self {
            Has(ref mut c) => Has(c),
            Lacks => Lacks,
        }
    }

    #[inline]
    pub fn has_it(&self) -> bool {
        match *self {
            Has(_) => true,
            Lacks => false,
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
            Has(ref c) => c,
            Lacks => panic!("Lacks component"),
        }
    }
}

impl<T> DerefMut for ComponentPresence<T> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut T {
        match *self {
            Has(ref mut c) => c,
            Lacks => panic!("Lacks component"),
        }
    }
}
