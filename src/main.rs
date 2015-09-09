use std::collections::HashMap;
use std::ops::Index;

type Entity = u32;

macro_rules! components {
    (
        $([$access:ident, $mutaccess:ident, $ty:ty]),+
            ) => {
        struct ComponentData {
            $(
                $access: HashMap<Entity, $ty>,
                )+
        }

        impl ComponentData {
            pub fn new() -> ComponentData {
                ComponentData {
                    $(
                        $access: HashMap::new(),
                        )+
                }
            }

            pub fn get(&self, ent: &Entity) -> EntityData {
                EntityData::new(ent, self)
            }

            pub fn get_mut(&mut self, ent: &Entity) -> EntityDataMut {
                EntityDataMut::new(ent, self)
            }
        }

        struct EntityData<'a> {
            $(
                $access: Option<&'a $ty>,
                )+
        }

        impl <'a> EntityData<'a> {
            pub fn new(idx: &Entity, data: &'a ComponentData) -> EntityData<'a> {
                EntityData {
                    $(
                        $access: data.$access.get(idx),
                        )+
                }
            }

            $(
                pub fn $access(&self) -> &$ty {
                    self.$access.expect("no component")
                }
                )+
        }

        struct EntityDataMut<'a> {
            $(
                $access: Option<&'a mut $ty>,
                )+
        }

        impl<'a> EntityDataMut<'a> {
            pub fn new(idx: &Entity, data: &'a mut ComponentData) -> EntityDataMut<'a> {
                EntityDataMut {
                    $(
                        $access: data.$access.get_mut(idx),
                        )+
                }
            }

            $(
                pub fn $access(self) -> &'a mut $ty {
                    self.$access.expect("no component")
                }
                )+
        }
    }
}

components!([position, position_mut, Position],
            [glyph, glyph_mut, Glyph]);

/*impl<'a> Index<Entity> for ComponentData {
    type Output = EntityData<'a>;

    fn index(&self, index: Entity) -> &'a EntityData<'a> {
        &self.get(index)
    }
}*/

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

struct Glyph {
    ch: char,
}

trait Component {
    fn add_to(self, Entity, &mut ComponentData);
}

impl Component for Position {
    fn add_to(self, ent: Entity, data: &mut ComponentData) {
        data.position.insert(ent, self);
    }
}

impl Component for Glyph {
    fn add_to(self, ent: Entity, data: &mut ComponentData) {
        data.glyph.insert(ent, self);
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

    //println!("id: {:?}, pos: {:?}", ent, manager.data.position());
    println!("pos: {:?}", manager.data.get(&ent).position());
    manager.data.get_mut(&ent).position().x += 5;
    println!("pos: {:?}", manager.data.get(&ent).position());
}
