//! An Entity Component System for game development..
//!
#![allow(dead_code)]
use std::collections::HashMap;
use std::ops::{Index, IndexMut};

pub mod component_presence;

/// Type Entity is simply an ID used as indexes.
pub type Entity = u32;


/// The components macro defines all the structs and traits that manage
/// the component part of the ECS.
#[macro_export]
macro_rules! components {
    (
        $([$access:ident, $ty:ty]),+
            ) => {

        use $crate::component_presence::ComponentPresence;
        use $crate::component_presence::ComponentPresence::*;
        use $crate::{EntityDataHolder, Component, Entity, ComponentData};
        
        pub struct EntityData {
            $(
                pub $access: ComponentPresence<$ty>,
                )+
        }

        impl EntityData {
            pub fn new_empty() -> EntityData {
                EntityData {
                    $(
                        $access: None,
                        )+
                }
            }
        }

        impl EntityDataHolder for EntityData {
            fn new() -> Self {
                EntityData::new_empty()
            }
        }

        $(
            impl Component<EntityData> for $ty {
                fn add_to(self, ent: Entity, data: &mut ComponentData<EntityData>) {
                    let ent_data = data.components.get_mut(&ent).expect("no entity");
                    ent_data.$access = Comp(self);
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
}

/// ComponentData 
pub struct ComponentData<D: EntityDataHolder> {
    pub components: HashMap<Entity, D>,
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
        }
    }

    pub fn get(&self, ent: &Entity) -> &D {
        self.components.get(ent).expect("no entity")
    }

    pub fn get_mut(&mut self, ent: &Entity) -> &mut D {
        self.components.get_mut(ent).expect("No entity")
    }

    pub fn create_component_data_for(&mut self, ent: Entity) {
        self.components.insert(ent, D::new());
    }
}

impl<D: EntityDataHolder> Index<Entity> for ComponentData<D> {
    type Output = D;

    fn index(&self, index: Entity) -> &D {
        &self.get(&index)
    }
}

impl<D: EntityDataHolder> IndexMut<Entity> for ComponentData<D> {
    fn index_mut(&mut self, index: Entity) -> &mut D {
        self.get_mut(&index)
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
pub struct EntityManager<D: EntityDataHolder> {
    entities: Vec<Entity>,
    pub data: ComponentData<D>,
}

impl<D: EntityDataHolder> EntityManager<D> {

    /// Creates a new EntityManager
    pub fn new() -> EntityManager<D> {
        EntityManager{
            entities: vec!(),
            data: ComponentData::new(),
        }
    }

    /// Creates a new entity, assigning it an unused ID, returning that ID for further use.
    pub fn new_entity(&mut self) -> Entity {
        let ent = self.entities.len() as Entity;
        self.entities.push(ent);

        self.data.create_component_data_for(ent);
        ent
    }

    /// Adds the specified component to the entity.
    pub fn add_component_to<C: Component<D>>(&mut self, e: Entity, c: C) {
        c.add_to(e, &mut self.data);
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
