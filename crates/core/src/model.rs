use slotmap::SlotMap;

use crate::{Attribute, AttributeId, Entity, EntityId};

pub struct Model {
    pub entities: SlotMap<EntityId, Entity>,
    pub attributes: SlotMap<AttributeId, Attribute>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            entities: SlotMap::with_key(),
            attributes: SlotMap::with_key(),
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

    /// Remove an entity and the attributes it owns, returning the entity if
    /// it existed.
    pub fn remove_entity(&mut self, id: EntityId) -> Option<Entity> {
        let entity = self.entities.remove(id)?;
        for attribute_id in &entity.attributes {
            self.attributes.remove(*attribute_id);
        }
        Some(entity)
    }

    // ---- Attributes -----------------------------------------------------

    /// Attach a new attribute to an entity, preserving insertion order in
    /// the entity's attribute list. Returns `None` if the entity does not
    /// exist.
    pub fn add_attribute(&mut self, entity: EntityId, attribute: Attribute) -> Option<AttributeId> {
        if !self.entities.contains_key(entity) {
            return None;
        }
        let id = self.attributes.insert(attribute);
        self.entities[entity].attributes.push(id);
        Some(id)
    }

    /// Rename an attribute's conceptual name. Returns `false` if the id no
    /// longer exists.
    pub fn rename_attribute(&mut self, id: AttributeId, name: impl Into<String>) -> bool {
        match self.attributes.get_mut(id) {
            Some(attribute) => {
                attribute.conceptual_name = name.into();
                true
            }
            None => false,
        }
    }

    /// Remove an attribute, detaching it from its owning entity's list.
    pub fn remove_attribute(&mut self, id: AttributeId) -> Option<Attribute> {
        let attribute = self.attributes.remove(id)?;
        for entity in self.entities.values_mut() {
            entity.attributes.retain(|attribute_id| *attribute_id != id);
        }
        Some(attribute)
    }
}

impl Default for Model {
    fn default() -> Self {
        Self::new()
    }
}
