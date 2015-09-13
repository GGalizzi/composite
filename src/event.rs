use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

use super::Entity;

pub trait EventDataHolder {
    fn as_type(&self) -> &'static str;
}

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

    pub fn global_events(&mut self) -> ::std::vec::Drain<Holder> {
        self.global.drain(..)
    }
    
    pub fn of_type(&self, s: &str) -> &HashMap<Entity, Vec<Holder>> {
        &self.map[s]
    }

    pub fn for_behavior_of(&mut self, related_events: Vec<&str>, ent: Entity) -> Vec<Holder> {
        let mut all_events = Vec::new();
        for s in related_events {
            all_events.append(&mut self.map.get_mut(s).unwrap_or(&mut HashMap::new()).
                remove(&ent).unwrap_or(vec!()));
        }
        all_events
    }

    pub fn push_for(&mut self, ent: Entity, holder: Holder) {
        let map = self.map.entry(holder.as_type()).or_insert(HashMap::new());
        match map.entry(ent) {
            Occupied(entry) => {entry.into_mut().push(holder);},
            Vacant(entry) => {entry.insert(vec!(holder));}
        }
    }

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
