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
//!
//! ```
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
//! ```

use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

use super::Entity;
use super::EntityDataHolder;
use super::ComponentData;

/// Implemented automatically by the `events!` macro.
pub trait EventDataHolder: Clone {
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
    pub fn for_behavior_of(&mut self, related_events: Vec<&str>, ent: Entity, clone: bool) -> Vec<Holder> {
        let mut all_events = Vec::new();
        for s in related_events {
            let op = self.map.get_mut(s);
            if op.is_none() {return vec!();}
            let vec = op.unwrap();
            if clone {
                all_events.append(&mut vec.clone().remove(&ent).unwrap_or(vec!()));
            } else {
                all_events.append(&mut vec.remove(&ent).unwrap_or(vec!()));
            }
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

    /// Push the given event to all entities of `family`
    pub fn push_for_family<Data: EntityDataHolder>(&mut self, family: &'static str, data: &ComponentData<Data>, holder: Holder) {
        for e in data.members_of(family) {
            self.push_for(e, holder.clone());
        }
    }

    /// Pushes the given event as a global one.
    pub fn push_global(&mut self, holder: Holder) {
        self.global.push(holder);
    }

    /// Removes the events related to an entity, returns them in case
    /// the user needs to do some clean-up with those.
    pub fn clear_events_for(&mut self, ent: Entity) -> Vec<Holder> {
        let mut events = vec!();
        for (_,h) in self.map.iter_mut() {
            match h.remove(&ent) {
                Some(mut ev) => events.append(&mut ev),
                None => {}
            }
        }
        events
    }
}

#[macro_export]
macro_rules! events {
    ($ev_enum:ident,$([$evtype:ident, $event:ident]),+) => {
        #[derive(Debug, Clone)]
        pub enum $ev_enum {
            $(
                $event($event),
             )+
        }

        impl $crate::event::EventDataHolder for $ev_enum {
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
