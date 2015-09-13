use std::collections::HashMap;

use super::event::{EventDataHolder, EventManager};
use super::family::FamilyDataHolder;
use super::EntityDataHolder;

pub trait BehaviorData {
    fn family(&self) -> &'static str;
    fn events(&self) -> Vec<&'static str>;
}

pub trait Behavior<EntityData: EntityDataHolder, Event: EventDataHolder>: BehaviorData {
    fn process(&self, &mut EntityData, &mut EventManager<Event>, Vec<Event>);
}

pub struct BehaviorManager<EntityData: EntityDataHolder, Event: EventDataHolder> {
    behaviors: Vec<Box<Behavior<EntityData, Event>>>,
    family_relation: HashMap<&'static str, Vec<usize>>,
}

impl<EntityData: EntityDataHolder, Event: EventDataHolder> BehaviorManager<EntityData, Event> {
    pub fn new((behaviors, families): (Vec<Box<Behavior<EntityData, Event>>>, HashMap<&'static str, Vec<usize>>)) -> BehaviorManager<EntityData, Event> {
        BehaviorManager {
            behaviors: behaviors,
            family_relation: families,
        }
    }

    pub fn run<FamilyData: FamilyDataHolder>(&self, manager: &mut super::EntityManager<EntityData, FamilyData>, event_manager: &mut EventManager<Event>) {
        for ent in manager.entities.iter().cloned() {
            for beh_idx in self.valid_behaviors_for(manager.data[ent].families()) {
                self.behaviors[beh_idx].process(&mut manager.data[ent], event_manager, vec!());
            }
        }
    }

    fn valid_behaviors_for(&self, families: Vec<&str>) -> Vec<usize> {
        let mut vec = Vec::new();
        println!("{:?}", self.family_relation);
        for family in families {
            println!("{:?}", family);
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
        fn behavior_list() -> (Vec<Box<Behavior<EntityData, Event>>>, HashMap<&'static str, Vec<usize>>) {
            use std::collections::hash_map::Entry::{Occupied, Vacant};

            let mut idx = 0;
            let mut beh_vec = Vec::new();
            let mut fam_map = HashMap::new();
            $(
                beh_vec.push(Box::new($behavior) as Box<Behavior<EntityData, Event>>);
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
