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
