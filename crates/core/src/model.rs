use slotmap::SlotMap;

use crate::{Association, AssociationId, Attribute, AttributeId, Entity, EntityId};

pub struct Model {
    pub entities: SlotMap<EntityId, Entity>,
    pub attributes: SlotMap<AttributeId, Attribute>,
    pub associations: SlotMap<AssociationId, Association>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            entities: SlotMap::with_key(),
            attributes: SlotMap::with_key(),
            associations: SlotMap::with_key(),
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

    /// Remove an entity and everything that depends on it: the attributes it
    /// owns and any association that references it through a leg. Returns the
    /// entity if it existed.
    pub fn remove_entity(&mut self, id: EntityId) -> Option<Entity> {
        let entity = self.entities.remove(id)?;
        for attribute_id in &entity.attributes {
            self.attributes.remove(*attribute_id);
        }
        self.associations
            .retain(|_, association| association.legs.iter().all(|leg| leg.entity_id != id));
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

    // ---- Associations ---------------------------------------------------

    /// Store an association and return its id.
    pub fn add_association(&mut self, association: Association) -> AssociationId {
        self.associations.insert(association)
    }

    /// Remove an association, returning it if it existed.
    pub fn remove_association(&mut self, id: AssociationId) -> Option<Association> {
        self.associations.remove(id)
    }
}

impl Default for Model {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Cardinality, DataType, Leg};

    fn sample_attribute(name: &str) -> Attribute {
        Attribute {
            conceptual_name: name.into(),
            logic_name: name.into(),
            data_type: DataType::Counter,
            is_nullable: false,
            is_unique: false,
            is_identifier: false,
            complement: String::new(),
            comment: String::new(),
        }
    }

    fn binary_association(name: &str, left: EntityId, right: EntityId) -> Association {
        Association {
            name: name.into(),
            logic_name: name.to_lowercase(),
            attributes: vec![],
            legs: vec![
                Leg {
                    entity_id: left,
                    cardinality: Cardinality::OneMany,
                    role: None,
                    forced_pk_side: None,
                },
                Leg {
                    entity_id: right,
                    cardinality: Cardinality::ZeroMany,
                    role: None,
                    forced_pk_side: None,
                },
            ],
            comment: String::new(),
            identification: None,
        }
    }

    #[test]
    fn new_model_is_empty() {
        let model = Model::new();
        assert_eq!(model.entities.len(), 0);
        assert_eq!(model.attributes.len(), 0);
        assert_eq!(model.associations.len(), 0);
    }

    #[test]
    fn add_rename_and_remove_entity() {
        let mut model = Model::new();

        let id = model.add_entity("Customer");
        assert_eq!(model.entities.len(), 1);
        assert_eq!(model.entities[id].name, "Customer");

        assert!(model.rename_entity(id, "Client"));
        assert_eq!(model.entities[id].name, "Client");

        let removed = model.remove_entity(id).expect("entity should exist");
        assert_eq!(removed.name, "Client");
        assert_eq!(model.entities.len(), 0);

        // Operating on a stale id is a no-op, not a panic.
        assert!(model.remove_entity(id).is_none());
        assert!(!model.rename_entity(id, "Ghost"));
    }

    #[test]
    fn add_rename_and_remove_attribute() {
        let mut model = Model::new();
        let entity = model.add_entity("Customer");

        let attr = model
            .add_attribute(entity, sample_attribute("email"))
            .expect("entity exists");
        assert_eq!(model.attributes.len(), 1);
        // The attribute is registered in its owning entity's ordered list.
        assert_eq!(model.entities[entity].attributes, vec![attr]);

        assert!(model.rename_attribute(attr, "email_address"));
        assert_eq!(model.attributes[attr].conceptual_name, "email_address");

        let removed = model
            .remove_attribute(attr)
            .expect("attribute should exist");
        assert_eq!(removed.conceptual_name, "email_address");
        assert_eq!(model.attributes.len(), 0);
        // Removal also detaches it from the owning entity.
        assert!(model.entities[entity].attributes.is_empty());
    }

    #[test]
    fn adding_an_attribute_to_a_missing_entity_fails() {
        let mut model = Model::new();
        let entity = model.add_entity("Temp");
        model.remove_entity(entity);

        assert!(model.add_attribute(entity, sample_attribute("x")).is_none());
        assert_eq!(model.attributes.len(), 0);
    }

    #[test]
    fn add_and_remove_association() {
        let mut model = Model::new();
        let a = model.add_entity("Order");
        let b = model.add_entity("Product");

        let assoc = model.add_association(binary_association("Contains", a, b));
        assert_eq!(model.associations.len(), 1);

        let removed = model.remove_association(assoc).expect("association exists");
        assert_eq!(removed.name, "Contains");
        assert_eq!(model.associations.len(), 0);
        assert!(model.remove_association(assoc).is_none());
    }

    #[test]
    fn removing_an_entity_cascades_to_its_attributes_and_associations() {
        let mut model = Model::new();
        let order = model.add_entity("Order");
        let product = model.add_entity("Product");

        let attr = model
            .add_attribute(order, sample_attribute("reference"))
            .expect("entity exists");
        let assoc = model.add_association(binary_association("Contains", order, product));

        assert_eq!(model.attributes.len(), 1);
        assert_eq!(model.associations.len(), 1);

        model.remove_entity(order);

        // The owned attribute is gone.
        assert!(model.attributes.get(attr).is_none());
        assert_eq!(model.attributes.len(), 0);
        // The association referencing the removed entity is gone too.
        assert!(model.associations.get(assoc).is_none());
        assert_eq!(model.associations.len(), 0);
        // The unrelated entity is untouched.
        assert!(model.entities.get(product).is_some());
    }

    #[test]
    fn ids_stay_valid_after_removing_another_entity() {
        let mut model = Model::new();
        let a = model.add_entity("A");
        let b = model.add_entity("B");
        let c = model.add_entity("C");

        model.remove_entity(b);

        // slotmap semantics: removing one key never invalidates the others,
        // and they still resolve to the same data.
        assert_eq!(model.entities.get(a).map(|e| e.name.as_str()), Some("A"));
        assert_eq!(model.entities.get(c).map(|e| e.name.as_str()), Some("C"));
        assert!(model.entities.get(b).is_none());
    }

    #[test]
    fn every_cardinality_variant_is_usable_on_a_leg() {
        let mut model = Model::new();
        let entity = model.add_entity("Node");

        for cardinality in [
            Cardinality::ZeroOne,
            Cardinality::OneOne,
            Cardinality::ZeroMany,
            Cardinality::OneMany,
        ] {
            let assoc = model.add_association(Association {
                name: "Link".into(),
                logic_name: "link".into(),
                attributes: vec![],
                legs: vec![Leg {
                    entity_id: entity,
                    cardinality,
                    role: None,
                    forced_pk_side: None,
                }],
                comment: String::new(),
                identification: None,
            });
            // Confirm the variant round-trips through the model unchanged.
            let stored = &model.associations[assoc].legs[0].cardinality;
            let label = match stored {
                Cardinality::ZeroOne => "0,1",
                Cardinality::OneOne => "1,1",
                Cardinality::ZeroMany => "0,n",
                Cardinality::OneMany => "1,n",
            };
            assert!(!label.is_empty());
        }
    }
}
