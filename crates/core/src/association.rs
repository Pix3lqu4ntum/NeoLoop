use crate::{AttributeId, EntityId};

pub struct Association {
    pub name: String,
    pub logic_name: String,
    pub attributes: Vec<AttributeId>,
    pub legs: Vec<Leg>,
    pub comment: String,
    /// Presence indicates this association is identifying (identification
    /// relative). `weak_entity` designates which of the legs' entities is
    /// the dependent (weak) side. `None` means a regular association.
    pub identification: Option<Identification>,
}

pub struct Identification {
    pub weak_entity: EntityId,
}

pub struct Leg {
    pub entity_id: EntityId,
    pub cardinality: Cardinality,
    pub role: Option<String>,
    /// Only meaningful when both legs of the association have a max
    /// cardinality of One. Ignored otherwise.
    pub forced_pk_side: Option<bool>,
}

/// The four standard Merise cardinalities carried by each end of an
/// association: (0,1), (1,1), (0,n) and (1,n).
pub enum Cardinality {
    ZeroOne,
    OneOne,
    ZeroMany,
    OneMany,
}

#[cfg(test)]
mod tests {
    use super::*;
    use slotmap::SlotMap;

    fn dummy_entity_ids() -> (EntityId, EntityId) {
        let mut entities: SlotMap<EntityId, ()> = SlotMap::with_key();
        let a = entities.insert(());
        let b = entities.insert(());
        (a, b)
    }

    #[test]
    fn regular_association_has_no_identification() {
        let (a, b) = dummy_entity_ids();

        let assoc = Association {
            name: "Owns".into(),
            logic_name: "owns".into(),
            attributes: vec![],
            legs: vec![
                Leg {
                    entity_id: a,
                    cardinality: Cardinality::OneMany,
                    role: None,
                    forced_pk_side: None,
                },
                Leg {
                    entity_id: b,
                    cardinality: Cardinality::ZeroOne,
                    role: None,
                    forced_pk_side: None,
                },
            ],
            comment: String::new(),
            identification: None,
        };

        assert!(assoc.identification.is_none());
    }

    #[test]
    fn identifying_association_designates_exactly_one_weak_entity() {
        let (owner, dependent) = dummy_entity_ids();

        let assoc = Association {
            name: "HasLine".into(),
            logic_name: "has_line".into(),
            attributes: vec![],
            legs: vec![
                Leg {
                    entity_id: owner,
                    cardinality: Cardinality::OneOne,
                    role: None,
                    forced_pk_side: None,
                },
                Leg {
                    entity_id: dependent,
                    cardinality: Cardinality::ZeroMany,
                    role: None,
                    forced_pk_side: None,
                },
            ],
            comment: String::new(),
            identification: Some(Identification {
                weak_entity: dependent,
            }),
        };

        // The type guarantees at most one weak entity can ever be designated —
        // this test just confirms it points to the intended leg, not the owner.
        let weak = assoc.identification.as_ref().unwrap().weak_entity;
        assert_eq!(weak, dependent);
        assert_ne!(weak, owner);
    }

    #[test]
    fn reflexive_association_uses_roles_to_disambiguate_legs() {
        let mut entities: SlotMap<EntityId, ()> = SlotMap::with_key();
        let employee = entities.insert(());

        let assoc = Association {
            name: "Manages".into(),
            logic_name: "manages".into(),
            attributes: vec![],
            legs: vec![
                Leg {
                    entity_id: employee,
                    cardinality: Cardinality::ZeroOne,
                    role: Some("manager".into()),
                    forced_pk_side: None,
                },
                Leg {
                    entity_id: employee,
                    cardinality: Cardinality::ZeroMany,
                    role: Some("subordinate".into()),
                    forced_pk_side: None,
                },
            ],
            comment: String::new(),
            identification: None,
        };

        // Both legs reference the same entity (reflexive) — roles are what
        // will let `transform` generate distinct FK column names later.
        assert_eq!(assoc.legs[0].entity_id, assoc.legs[1].entity_id);
        assert_ne!(assoc.legs[0].role, assoc.legs[1].role);
    }
}
