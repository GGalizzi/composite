use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

use super::Entity;

pub trait EventDataHolder {
    fn as_type(&self) -> &'static str;
}

pub struct EventManager<Holder: EventDataHolder> {
    map: HashMap<&'static str, HashMap<Entity, Vec<Holder>>>,
}

impl<Holder: EventDataHolder> EventManager<Holder> {
    pub fn new() -> EventManager<Holder> {
        EventManager {
            map: HashMap::new(),
        }
    }
    
    pub fn of_type(&self, s: &str) -> &HashMap<Entity, Vec<Holder>> {
        &self.map[s]
    }

    pub fn push_for(&mut self, ent: Entity, holder: Holder) {
        let map = self.map.entry(holder.as_type()).or_insert(HashMap::new());
        match map.entry(ent) {
            Occupied(entry) => {entry.into_mut().push(holder);},
            Vacant(entry) => {entry.insert(vec!(holder));}
        }
    }
}

#[macro_export]
macro_rules! events {
    ($([$evtype:ident, $event:ident]),+) => {
        #[derive(Debug)]
        enum Event {
            $(
                $event($event),
             )+
        }

        impl $crate::event::EventDataHolder for Event {
            fn as_type(&self) -> &'static str {
                match *self {
                    $(
                        Event::$event(_) => stringify!($evtype)
                     )+
                }
            }
        }
    }
}
