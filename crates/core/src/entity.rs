use crate::AttributeId;

pub struct Entity {
    pub name: String,
    pub logic_name: String,
    pub attributes: Vec<AttributeId>,
    pub comment: String,
    pub is_fictitious: bool,
}
