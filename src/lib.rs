//! An Entity Component System for game development..
//!
#![allow(dead_code)]
use std::collections::HashMap;
use std::ops::{Index, IndexMut};

pub mod component_presence;
use component_presence::ComponentPresence;
use component_presence::ComponentPresence::*;

/// Type Entity is simply an ID used as indexes.
pub type Entity = u32;


/// The components macro defines all the structs and traits that manage
/// the component part of the ECS.
#[macro_export]
macro_rules! components {
    (
        $([$access:ident, $ty:ty]),+
            ) => {

        /// ComponentData 
        pub struct ComponentData {
            components: HashMap<Entity, EntityData>,
        }

        /// This trait marks a struct as a component.
        ///
        /// It should implement the `add_to` function, which is automatically generated
        /// by the `components` macro.
        pub trait Component {
            fn add_to(self, ent: Entity, data: &mut ComponentData);
        }

        impl ComponentData {
            pub fn new() -> ComponentData {
                ComponentData {
                    components: HashMap::new(),
                }
            }

            pub fn get(&self, ent: &Entity) -> &EntityData {
                self.components.get(ent).expect("no entity")
            }

            pub fn get_mut(&mut self, ent: &Entity) -> &mut EntityData {
                self.components.get_mut(ent).expect("No entity")
            }
        }

        /// EntityData
        pub struct EntityData {
            $(
                pub $access: ComponentPresence<$ty>,
                )+
        }

        impl EntityData {
            pub fn new() -> EntityData {
                EntityData {
                    $(
                        $access: None,
                        )+
                }
            }
        }

        $(
            impl Component for $ty {
                fn add_to(self, ent: Entity, data: &mut ComponentData) {
                    let ent_data = data.components.get_mut(&ent).expect("no entity");
                    ent_data.$access = Comp(self);
                }
            }
            )+

            
            impl Index<Entity> for ComponentData {
                type Output = EntityData;

                fn index(&self, index: Entity) -> &EntityData {
                    &self.get(&index)
                }
            }

        impl IndexMut<Entity> for ComponentData {
            fn index_mut(&mut self, index: Entity) -> &mut EntityData {
                self.get_mut(&index)
            }
        }


        /// The `EntityManager` type manages all the entities.
        ///
        /// It is in charge of creating and destroying entities.
        /// It also takes care of adding or removing components, through the `ComponentData` it contains.
        pub struct EntityManager {
            entities: Vec<Entity>,
            data: ComponentData,
        }

        impl EntityManager {
            pub fn new() -> EntityManager {
                EntityManager{
                    entities: vec!(),
                    data: ComponentData::new(),
                }
            }

            pub fn new_entity(&mut self) -> Entity {
                let ent = self.entities.len() as Entity;
                self.entities.push(ent);
                self.data.components.insert(ent, EntityData::new());
                ent
            }

            pub fn add_component_to<C: Component>(&mut self, e: Entity, c: C) {
                c.add_to(e, &mut self.data);
            }
        }
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
