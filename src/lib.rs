//! An Entity Component System for game development.
//! 
//! Currently used for personal use (for a roguelike game), this library is highly unstable, and a WIP.
#![allow(dead_code)]
#![feature(append,drain)]
use std::iter;
use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use std::collections::hash_map::Entry::{Occupied, Vacant};

pub mod component_presence;
pub mod family;
pub mod builder;
pub mod event;
pub mod behavior;

use family::{FamilyDataHolder, FamilyMap};
use event::{EventDataHolder};
pub use event::EventManager;
pub use behavior::BehaviorManager;
pub use behavior::Behavior;
pub use component_presence::ComponentPresence;

/// Type Entity is simply an ID used as indexes.
pub type Entity = u32;

/// The components macro defines all the structs and traits that manage
/// the component part of the ECS.
#[macro_export]
macro_rules! components {
    ($data:ident:
        $([$access:ident, $ty:ty]),+
            ) => {

        use $crate::component_presence::ComponentPresence;
        use $crate::component_presence::ComponentPresence::*;
        use $crate::{EntityDataHolder, Component, Entity, ComponentData};
        use $crate::family::{FamilyMap};
        use std::fmt;

        pub struct $data {
            pub components: Vec<&'static str>,
            pub families: Vec<&'static str>,
            $(
                pub $access: ComponentPresence<$ty>,
                )+
        }

        impl $data {
            pub fn new_empty() -> $data {
                $data {
                    components: Vec::new(),
                    families: Vec::new(),
                    $(
                        $access: Lacks,
                        )+
                }
            }
        }

        impl fmt::Debug for $data {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                let mut b = fmt.debug_struct("EntityData");
                b.field("has components", &self.components);
                b.field("belongs to families", &self.families);
                /*$(
                    if self.$access.has_it() {
                        b.field(stringify!($access), &self.$access);
                    }
                 )+*/
                b.finish()
            }
        }

        impl EntityDataHolder for $data {
            fn new() -> Self {
                $data::new_empty()
            }

            fn match_families(&self, families: &FamilyMap) -> Vec<&'static str> {
                let mut v: Vec<&str> = vec!();

                // Tuple has the requirements/forbidden vectors
                for (family, tuple) in families {
                    if $crate::family::matcher(tuple, &self.components) {
                        v.push(family)
                    }
                }
                v
            }

            fn set_families(&mut self, families: Vec<&'static str>) {
                self.families = families;
            }

            fn belongs_to_family(&self, family: &str) -> bool {
                self.families.contains(&family)
            }

            fn families(&self) -> Vec<&'static str> {
                self.families.clone()
            }
        }

        $(
            impl Component<$data> for $ty {
                fn add_to(self, ent: Entity, data: &mut ComponentData<$data>) {
                    let ent_data: &mut $data = data.components.get_mut(&ent).expect("no entity");
                    ent_data.components.push(stringify!($access));
                    ent_data.$access = Has(self);
                }
            }
            )+
    }
}


/// This is a marker trait to be used by the `components!` macro.
///
/// This trait is implemented by `EntityData` which is a struct generated
/// by the `components!` macro.
///
/// `EntityData` will be of the form:
/// 
/// ```
/// struct EntityData {
///     component1: ComponentPresence<Component1>,
///     component2: ComponentPresence<Component2>,
///     //etc...
/// }
/// ```
///
/// So it will have one field per component defined in the call to `components!`
/// You'll access these fields directly when indexing the `data` field of the `EntityManager`
pub trait EntityDataHolder {
    fn new() -> Self;

    /// Takes a map of all the defined families,
    /// and returns the families that match this entity.
    fn match_families(&self, &FamilyMap) -> Vec<&'static str>;

    /// Sets the families this entity belongs to to `families`
    fn set_families(&mut self, Vec<&'static str>);

    fn belongs_to_family(&self, &'static str) -> bool;

    /// Gets the known families this ent belongs to.
    fn families(&self) -> Vec<&'static str>;
}

/// ComponentData knows which entities have which components.
pub struct ComponentData<D: EntityDataHolder> {
    /// components holds the components owned by a certain entity.
    pub components: HashMap<Entity, D>,
    /// Family to list of entities.
    pub families: HashMap<&'static str, Vec<Entity>>,
}

/// This trait marks a struct as a component. (Automatically handled by macro `components!`)
///
/// It should implement the `add_to` function, which is automatically generated
/// by the `components!` macro.
pub trait Component<D: EntityDataHolder> {

    /// Adds self to the specified entity. Called by the `EntityManager`
    fn add_to(self, ent: Entity, data: &mut ComponentData<D>);
}

impl<D: EntityDataHolder> ComponentData<D> {
    pub fn new() -> ComponentData<D> {
        ComponentData {
            components: HashMap::new(),
            families: HashMap::new(),
        }
    }

    pub fn get(&self, ent: &Entity) -> Option<&D> {
        self.components.get(ent)
    }

    pub fn get_mut(&mut self, ent: &Entity) -> Option<&mut D> {
        self.components.get_mut(ent)
    }

    pub fn create_component_data_for(&mut self, ent: Entity) {
        self.components.insert(ent, D::new());
    }

    pub fn clear_family_data_for(&mut self, ent: Entity) {
        for family in self[ent].families() {
            self.remove_from_family(family, ent);
            debug_assert!(!self.families[family].contains(&ent))
        }
    }
    
    pub fn delete_component_data_for(&mut self, ent: Entity) {
        self.clear_family_data_for(ent);
        self.components.remove(&ent);
    }

    fn remove_from_family(&mut self, family: &str, ent: Entity) {
        let mut idx: Option<usize> = None;
        {
            let vec = self.families.get_mut(family).expect("No such family");
            let op = vec.iter().enumerate().find(|&(_,e)| *e == ent);
            idx = Some(op.expect("Entity not found in this family").0);
        }

        if let Some(idx) = idx {
            self.families.get_mut(family).unwrap().swap_remove(idx);
        } else { panic!("Entity not found for family"); }
    }

    pub fn set_family_relation(&mut self, family: &'static str, ent: Entity) {
        match self.families.entry(family) {
            Vacant(entry) => {entry.insert(vec!(ent));},
            Occupied(entry) => {
                let v = entry.into_mut();
                if v.contains(&ent) { return; }
                v.push(ent);
            },
        }
    }

    pub fn members_of(&self, family: &'static str) -> Vec<Entity> {
        match self.families.get(family) {
            Some(vec) => vec.clone(),
            None => vec!(),
        }
    }

    pub fn any_member_of(&self, family: &'static str) -> bool {
        !self.families.get(family).expect("no such family").is_empty()
    }
}

impl<D: EntityDataHolder> Index<Entity> for ComponentData<D> {
    type Output = D;

    fn index(&self, index: Entity) -> &D {
        &self.components.get(&index).expect(&format!("no entity {:?}", index))
    }
}

impl<D: EntityDataHolder> IndexMut<Entity> for ComponentData<D> {
    fn index_mut(&mut self, index: Entity) -> &mut D {
        self.components.get_mut(&index).expect("no entity")
    }
}

/// The `EntityManager` type manages all the entities.
///
/// It is in charge of creating and destroying entities.
/// It also takes care of adding or removing components, through the `ComponentData` it contains.
///
/// # Examples
///
/// Creating a new manager, and adding some (predefined) components to a new entity.
///
/// ```
/// let mut manager = EntityManager::new();
/// let ent = manager.new_entity();
/// manager.add_component_to(ent, Position{x: 1, y: 2});
/// ```
pub struct EntityManager<D: EntityDataHolder, F: FamilyDataHolder> {
    next_idx: usize,
    reusable_idxs: Vec<usize>,
    active: Vec<bool>,
    pub data: ComponentData<D>,
    /// Contains a list of all defined families, along with its requirements.
    families: F,
}

impl<D: EntityDataHolder, F: FamilyDataHolder> EntityManager<D, F> {

    /// Creates a new EntityManager
    pub fn new() -> EntityManager<D, F> {
        EntityManager{
            next_idx: 0,
            reusable_idxs: vec!(),
            active: vec!(),
            data: ComponentData::new(),
            families: F::new(),
        }
    }

    /// Creates a new entity, assigning it an unused ID, returning that ID for further use.
    pub fn new_entity(&mut self) -> Entity {
        let idx = match self.reusable_idxs.pop() {
            None => {
                let idx = self.next_idx;
                self.next_idx += 1;
                idx
            }
            Some(idx) => idx,
        };

        // Extend the vec if the idx is bigger.
        if self.active.len() <= idx {
            let padding = idx + 1 - self.active.len();
            self.active.extend(iter::repeat(false).take(padding));
            debug_assert!(self.active.len() == idx+1);
        }

        debug_assert!(!self.active[idx]);
        self.active[idx] = true;
        
        let ent = idx as Entity;
        self.data.create_component_data_for(ent);
        ent
    }

    /// Deletes the entity, removes all data related to it.
    ///
    /// Returns a list of events that were related to it, in case you need to do some clean up with them.
    pub fn delete_entity<Event>(&mut self, ent: Entity, events: &mut EventManager<Event>) -> Vec<Event>
    where Event: event::EventDataHolder {
        self.delete_entity_ignore_events(ent);
        events.clear_events_for(ent)
    }

    pub fn delete_entity_ignore_events(&mut self, ent: Entity) {
        let idx = ent as usize;
        
        assert!(self.active[idx]);
        self.reusable_idxs.push(idx);
        self.active[idx] = false;
        
        self.data.delete_component_data_for(ent);
    }

    pub fn build_ent<'a, A,B: EventDataHolder>(&'a mut self, ent: Entity, processor: &'a mut BehaviorManager<A,B>) -> EntityBuilder<D,A,B> {
        EntityBuilder::new(ent, processor, &mut self.data, self.families.all_families())
    }

    /// Adds the specified component to the entity.
    pub fn add_component_to<A,B:EventDataHolder,C: Component<D>>(&mut self, e: Entity, c: C,
                                             processor: &mut BehaviorManager<A,B>) {
        //c.add_to(e, &mut self.data);
        self.build_ent(e, processor).add_component(c).finalize();
    }
}

/// Used by `EntityManager` to add components to an Entity.
///
/// An object of this type is obtained by calling `add_component` from an EntityManager
pub struct EntityBuilder<'a, EntData: 'a + EntityDataHolder, T: 'a, Ev: event::EventDataHolder + 'a> {
    data: &'a mut ComponentData<EntData>,
    families: &'a FamilyMap,
    processor: &'a mut BehaviorManager<T,Ev>,
    ent: Entity,
}

impl<'a, EntData: 'a + EntityDataHolder, T, Ev: event::EventDataHolder> EntityBuilder<'a, EntData, T, Ev> {
    
    pub fn new(ent: Entity, processor: &'a mut BehaviorManager<T,Ev>, data: &'a mut ComponentData<EntData>,
               families: &'a FamilyMap) -> EntityBuilder<'a, EntData, T, Ev> {
        EntityBuilder {
            data: data,
            families: families,
            processor: processor,
            ent: ent,
        }
    }

    pub fn add_component<Comp: Component<EntData>>(self, comp: Comp) -> EntityBuilder<'a, EntData, T, Ev> {
        comp.add_to(self.ent, self.data);
        self
    }

    pub fn finalize(mut self) -> Entity {
        self.add_all_related_data();
        self.ent
    }

    pub fn add_all_related_data(&mut self) {
        let mut families: Vec<&str> = self.data[self.ent].match_families(self.families);
        families.sort();
        families.dedup();

        // Clean up current component data,
        self.data.clear_family_data_for(self.ent);

        // Give the ComponentDataHolder information about this entities families.
        for family in families.iter() {
            self.data.set_family_relation(family, self.ent);
        }

        if !self.processor.valid_behaviors_for(families.clone()).is_empty() {
            self.processor.add_processable(self.ent);
        }

        // Give this EntityDataHolder a list of which families this entity has.
        self.data[self.ent].set_families(families);
    }
}

/*
fn main() {
println!("Hello, world!");
let mut manager = EntityManager::new();
let ent = manager.new_entity();
manager.add_component_to(ent, Position{x:1, y:2});

println!("pos: {:?}", manager.data[ent].position.x);
manager.data[ent].position.x += 5;
println!("pos: {:?}", manager.data[ent].position.x);
println!("has glyph? {:?}", manager.data[ent].glyph.has_it());
}
*/
