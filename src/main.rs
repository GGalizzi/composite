use std::collections::HashMap;
use std::ops::{Index, IndexMut};

type Entity = u32;

macro_rules! components {
    (
        $([$access:ident, $access_mut:ident, $ty:ty]),+
            ) => {
        struct ComponentData {
            components: HashMap<Entity, EntityData>,
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

        struct EntityData {
            $(
                pub $access: Option<$ty>,
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

            $(
                pub fn $access(&self) -> &$ty {
                    self.$access.as_ref().unwrap()
                }

                pub fn $access_mut(&mut self) -> &mut $ty {
                    self.$access.as_mut().unwrap()
                }
                )+
        }
    }
}

components!([position, position_mut, Position],
            [glyph, glyph_mut, Glyph]);

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

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

struct Glyph {
    ch: char,
}

trait Component {
    fn add_to(self, ent: Entity, data: &mut ComponentData);
}

impl Component for Position {
    fn add_to(self, ent: Entity, data: &mut ComponentData) {
        let ent_data = data.components.get_mut(&ent).expect("no entity");
        ent_data.position = Some(self);
    }
}

impl Component for Glyph {
    fn add_to(self, ent: Entity, data: &mut ComponentData) {
        let ent_data = data.components.get_mut(&ent).expect("no entity");
        ent_data.glyph = Some(self)
    }
}

struct EntityManager {
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

fn main() {
    println!("Hello, world!");
    let mut manager = EntityManager::new();
    let ent = manager.new_entity();
    manager.add_component_to(ent, Position{x:1, y:2});

    println!("pos: {:?}", manager.data[ent].position());
    manager.data[ent].position_mut().x += 5;
    println!("pos: {:?}", manager.data[ent].position());
}
