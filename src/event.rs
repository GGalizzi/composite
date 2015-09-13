//! Event handling system.
//!
//! Events are the way Behaviors communicate with each other, allowing for behaviors to be independent of
//! components they don't care about.
//!
//! For example, some input behavior might fire an event specifying what happens when such button is pressed.
//! Say, an event that's supposed to move the entity.
//!
//! Then a movement behavior will look for Movement events and react accordingly.
//!
//! Now you can simply make different input behaviors (say one for keyboard, another from an AI) that act on
//! different components, and the movement behavior will just keep working, it doesn't care where that movement
//! event came from.
//!
//! # Defining Events
//! #[derive(Debug)]
//! pub struct MovementEvent {
//!     pub x: i32,
//!     pub y: i32,
//! }
//!
//! #[derive(Debug)]
//! pub struct InputEvent {
//!     pub pressed_key: Key,
//! }
//!
//! // Event type first, then the struct.
//! events!([input, InputEvent],
//!         [movement, MovementEvent]);

use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

use super::Entity;

/// Implemented automatically by the `events!` macro.
pub trait EventDataHolder {
    fn as_type(&self) -> &'static str;
}


/// Holds all events.
///
/// EventManager can quickly give you which events are relevant for a certain behavior
/// both by getting the appropiate events by type, and by who's the entity being processed.
pub struct EventManager<Holder: EventDataHolder> {
    map: HashMap<&'static str, HashMap<Entity, Vec<Holder>>>,
    global: Vec<Holder>,
}

impl<Holder: EventDataHolder> EventManager<Holder> {
    pub fn new() -> EventManager<Holder> {
        EventManager {
            map: HashMap::new(),
            global: Vec::new(),
        }
    }

    /// Global Events are events meant to be checked outside of a `Behavior`
    ///
    /// Meant for things that modify some game state outside of the scope of a single entity.
    /// This function drains the Vector, emptying it.
    pub fn global_events(&mut self) -> ::std::vec::Drain<Holder> {
        self.global.drain(..)
    }

    /// Returns all events of a given type.
    pub fn of_type(&self, s: &str) -> &HashMap<Entity, Vec<Holder>> {
        &self.map[s]
    }

    /// Given the events a behavior listens to, and the entity it will process, return the relevant events.
    pub fn for_behavior_of(&mut self, related_events: Vec<&str>, ent: Entity) -> Vec<Holder> {
        let mut all_events = Vec::new();
        for s in related_events {
            all_events.append(&mut self.map.get_mut(s).unwrap_or(&mut HashMap::new()).
                remove(&ent).unwrap_or(vec!()));
        }
        all_events
    }

    /// Push the given event, for the given entity to process at some point.
    pub fn push_for(&mut self, ent: Entity, holder: Holder) {
        let map = self.map.entry(holder.as_type()).or_insert(HashMap::new());
        match map.entry(ent) {
            Occupied(entry) => {entry.into_mut().push(holder);},
            Vacant(entry) => {entry.insert(vec!(holder));}
        }
    }

    /// Pushes the given event as a global one.
    pub fn push_global(&mut self, holder: Holder) {
        self.global.push(holder);
    }
}

#[macro_export]
macro_rules! events {
    ($([$evtype:ident, $event:ident]),+) => {
        #[derive(Debug)]
        pub enum Event {
            $(
                $event($event),
             )+
        }

        impl $crate::event::EventDataHolder for Event {
            fn as_type(&self) -> &'static str {
                match *self {
                    $(
                        Event::$event(_) => stringify!($evtype),
                     )+
                }
            }
        }
    }
}
