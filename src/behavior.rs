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

use super::event::{EventDataHolder, EventManager};
use super::family::FamilyDataHolder;
use super::{Entity,EntityDataHolder, ComponentData, EntityManager};

pub trait BehaviorData {
    fn family(&self) -> &'static str;
    fn events(&self) -> Vec<&'static str>;
}

pub trait Behavior<EntityData: EntityDataHolder, FamilyData: FamilyDataHolder, Event: EventDataHolder>: BehaviorData {
    fn process(&self, Vec<Event>, Entity, &mut EntityManager<EntityData, FamilyData>, &mut EventManager<Event>);
}

/// Keeps track of behavior and what family and events they care about.
pub struct BehaviorManager<EntityData: EntityDataHolder, FamilyData: FamilyDataHolder, Event: EventDataHolder> {
    pub behaviors: Vec<Box<Behavior<EntityData, FamilyData, Event>>>,
    family_relation: HashMap<&'static str, Vec<usize>>,
}

impl<EntityData: EntityDataHolder, FamilyData: FamilyDataHolder, Event: EventDataHolder> BehaviorManager<EntityData, FamilyData, Event> {
    pub fn new((behaviors, families): (Vec<Box<Behavior<EntityData, FamilyData, Event>>>, HashMap<&'static str, Vec<usize>>)) -> BehaviorManager<EntityData, FamilyData, Event> {
        BehaviorManager {
            behaviors: behaviors,
            family_relation: families,
        }
    }

    pub fn run(&self, manager: &mut EntityManager<EntityData, FamilyData>, event_manager: &mut EventManager<Event>) {
        for ent in manager.entities.clone().iter() {
            if manager.data.get(ent).is_none() { continue; } // If Removed during iteration.
            for beh_idx in self.valid_behaviors_for(manager.data[*ent].families()) {
                let ref beh = self.behaviors[beh_idx];
                beh.process(event_manager.for_behavior_of(beh.events(), *ent, true),
                            *ent,
                            manager,
                            event_manager);
            }
        }
    }

    pub fn valid_behaviors_for(&self, families: Vec<&str>) -> Vec<usize> {
        let mut vec = Vec::new();
        for family in families {
            vec.append(&mut self.family_relation.get(family).unwrap_or(&mut vec!()).clone());
        }
        vec
    }
}

#[macro_export]
macro_rules! behaviors {
    ($([$behavior:ident: family: $family:ident, events: $($event:ident),*]),+) => {

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
         )+

        #[allow(unused_assignments)]
        pub fn behavior_list() -> (Vec<Box<Behavior<EntityData, FamilyData, Event>>>, HashMap<&'static str, Vec<usize>>) {
            use std::collections::hash_map::Entry::{Occupied, Vacant};

            let mut idx = 0;
            let mut beh_vec = Vec::new();
            let mut fam_map = HashMap::new();
            $(
                beh_vec.push(Box::new($behavior) as Box<Behavior<EntityData, FamilyData, Event>>);
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
