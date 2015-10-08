//! Management and creation of behaviors (a.k.a Systems in most ECSs).
//!
//! A behavior defines how a certain family "behaves",
//! the actions it can take.
//!
//! A behavior must implement the `Behavior` trait and then passed to the `behaviors!` macro.
//!
//! # Example
//!
//! ```
//! pub struct MovementBehavior;
//! 
//! impl Behavior<EntityData, Event> for MovementBehavior {
//!     fn process(&self, events: Vec<Event>, ent: Entity, data: &mut ComponentData<EntityData>, event_manager: &mut EventManager<Event>) {
//!      // implementation
//!     }
//! }
//!
//! behaviors!([MovementBehavior: family: movable, events: movement]]);
//! ```

use std::collections::HashMap;
use std::fmt;

use super::event::{EventDataHolder, EventManager};
use super::family::FamilyDataHolder;
use super::{Entity,EntityDataHolder, ComponentData, EntityManager};

pub trait BehaviorData: fmt::Debug {
    fn family(&self) -> &'static str;
    fn events(&self) -> Vec<&'static str>;
}

pub trait Behavior<T, Event: EventDataHolder>: BehaviorData {
    fn process(&self, Vec<Event>, Entity, &mut T, &mut EventManager<Event>);
}

/// Keeps track of behavior and what family and events they care about.
pub struct BehaviorManager<T, Event: EventDataHolder> {
    pub behaviors: Vec<Box<Behavior<T,Event>>>,
    family_relation: HashMap<&'static str, Vec<usize>>,
    pub processable: Vec<Entity>,
}

impl<T, Event: EventDataHolder> BehaviorManager<T, Event> {
    pub fn new((behaviors, families): (Vec<Box<Behavior<T, Event>>>, HashMap<&'static str, Vec<usize>>)) -> BehaviorManager<T, Event> {
        BehaviorManager {
            behaviors: behaviors,
            family_relation: families,
            processable: vec!(),
        }
    }

    /*pub fn run<EntityData, FamilyData>(&self, manager: &mut EntityManager<EntityData, FamilyData>, event_manager: &mut EventManager<Event>)
    where EntityData: EntityDataHolder, FamilyData: FamilyDataHolder {
        for ent in manager.entities.clone().iter() {
            if manager.data.get(ent).is_none() { continue; } // If Removed during iteration.
            for beh_idx in self.valid_behaviors_for(manager.data[*ent].families()) {
                let ref beh = self.behaviors[beh_idx];
                beh.process::<EntityManager<EntityData, FamilyData>>
                    (event_manager.for_behavior_of(beh.events(), *ent, true),
                            *ent,
                            manager,
                            event_manager);
            }
        }
}*/

    pub fn run(&self, idx: usize, ent: Entity, manager: &mut T, ev_manager: &mut EventManager<Event>) {
        let ref behavior = self.behaviors[idx];
        let relevant_events = ev_manager.for_behavior_of(behavior.events(), ent, false);
        println!("{:?}", behavior);
        behavior.process(relevant_events, ent, manager, ev_manager);
    }

    pub fn add_processable(&mut self, ent: Entity) {
        if self.processable.iter().find(|e| **e == ent).is_some() { return; }
        self.processable.push(ent);
    }
    
    pub fn valid_behaviors_for(&self, families: Vec<&str>) -> Vec<usize> {
        let mut vec = Vec::new();
        for family in families {
            vec.append(&mut self.family_relation.get(family).unwrap_or(&mut vec!()).clone());
        }
        vec.sort();
        vec
    }
}

#[macro_export]
macro_rules! behaviors {
    (manager:$t:ty,
     event:$ev_enum:ty,
     list_name:$fn_name:ident,
     $([$behavior:ident: family: $family:ident, events: $($event:ident),*]),+) => {
        use std::fmt;
        use std::collections::HashMap;
        $(
            impl $crate::behavior::BehaviorData for $behavior {
                fn family(&self) -> &'static str {
                    stringify!($family)
                }

                fn events(&self) -> Vec<&'static str> {
                    vec!(
                        $(
                           stringify!($event) 
                         )*
                        )
                }
            }

            impl fmt::Debug for $behavior {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "{}", stringify!($behavior))
                }
            }
         )+

        #[allow(unused_assignments)]
        pub fn $fn_name() -> (Vec<Box<Behavior<$t, $ev_enum>>>, HashMap<&'static str, Vec<usize>>) {
            use std::collections::hash_map::Entry::{Occupied, Vacant};

            let mut idx = 0;
            let mut beh_vec = Vec::new();
            let mut fam_map = HashMap::new();
            $(
                beh_vec.push(Box::new($behavior) as Box<Behavior<$t, $ev_enum>>);
                match fam_map.entry(stringify!($family)) {
                    Vacant(entry) => { entry.insert(vec!(idx));},
                    Occupied(entry) => { entry.into_mut().push(idx);}
                }
                idx += 1;
             )+
             (beh_vec, fam_map)
        }
    }
}
