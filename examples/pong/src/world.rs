use composite::EntityManager;
use composite::Entity;
use composite::behavior::Behavior;

use ::components::EntityData;
use super::FamilyData;

pub struct World {
    pub manager: EntityManager<EntityData, FamilyData>
}

impl World {
    pub fn new() -> World {
        World { manager: EntityManager::new(), }
    }
}
