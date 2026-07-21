use slotmap::SlotMap;

use crate::{Entity, EntityId};

pub struct Model {
    pub entities: SlotMap<EntityId, Entity>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            entities: SlotMap::with_key(),
        }
    }

    // ---- Entities -------------------------------------------------------

    /// Create a new entity with the given conceptual name and return its id.
    pub fn add_entity(&mut self, name: impl Into<String>) -> EntityId {
        self.entities.insert(Entity {
            name: name.into(),
            logic_name: String::new(),
            attributes: Vec::new(),
            comment: String::new(),
            is_fictitious: false,
        })
    }

    /// Rename an entity's conceptual name. Returns `false` if the id no
    /// longer exists.
    pub fn rename_entity(&mut self, id: EntityId, name: impl Into<String>) -> bool {
        match self.entities.get_mut(id) {
            Some(entity) => {
                entity.name = name.into();
                true
            }
            None => false,
        }
    }

    /// Remove an entity, returning it if it existed.
    pub fn remove_entity(&mut self, id: EntityId) -> Option<Entity> {
        self.entities.remove(id)
    }
}

impl Default for Model {
    fn default() -> Self {
        Self::new()
    }
}
