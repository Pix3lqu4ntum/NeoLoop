use crate::AttributeId;

pub struct Entity {
    pub name: String,
    pub logic_name: String,
    pub attributes: Vec<AttributeId>,
    pub comment: String,
    pub is_fictitious: bool,
}

pub struct Attribute {
    pub conceptual_name: String,
    pub logic_name: String,
    pub data_type: DataType,
    pub is_nullable: bool,
    pub is_unique: bool,
    pub is_identifier: bool,
    pub complement: String,
    pub comment: String,
}

pub struct TextProperties {
    pub length: Option<u32>,
    pub collation: Option<String>,
}

pub enum IntegerWidth {
    Bits8,
    Bits16,
    Bits32,
    Bits64,
}

pub enum DataType {
    // Looping Equivalent : variable
    Variable(TextProperties),

    // Looping Equivalent : Fixe
    Fixed(TextProperties),

    // Looping Equivalent : Volumineux
    LongText,

    // Looping Equivalent : Entier
    Integer(IntegerWidth),

    // Looping Equivalent : Décimal
    Decimal {
        precision: u8, // Default 15, minimum at 1, max at 65
        scale: u8,     // Default 2, minimum at 2, max at 14 (must be <= precision)
    },

    // Looping Equivalent : Réel
    Real,

    // Looping Equivalent : Monétaire
    Monetary,

    // Looping Equivalent : Compteur (Auto-increment)
    Counter,

    // Looping Equivalent : Booléen
    Boolean,

    // Looping Equivalent : Date
    Date,

    // Looping Equivalent : Heure
    Time,

    // Looping Equivalent : Date/Heure
    DateTime,

    // Looping Equivalent : Tableau
    Array(Box<DataType>),

    // Looping Equivalent : JSON
    JSON,

    // Looping Equivalent : Géometrique
    Geometry,

    // Looping Equivalent : Géographique
    Geography,

    // Looping Equivalent : Libre
    Free(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entity_stores_attribute_ids_in_order() {
        use crate::AttributeId;
        use slotmap::SlotMap;

        let mut attributes: SlotMap<AttributeId, Attribute> = SlotMap::with_key();
        let id_a = attributes.insert(Attribute {
            conceptual_name: "id".into(),
            logic_name: "id".into(),
            data_type: DataType::Counter,
            is_nullable: false,
            is_unique: true,
            is_identifier: true,
            complement: String::new(),
            comment: String::new(),
        });
        let id_b = attributes.insert(Attribute {
            conceptual_name: "name".into(),
            logic_name: "name".into(),
            data_type: DataType::Variable(TextProperties {
                length: Some(255),
                collation: None,
            }),
            is_nullable: false,
            is_unique: false,
            is_identifier: false,
            complement: String::new(),
            comment: String::new(),
        });

        let entity = Entity {
            name: "Employee".into(),
            logic_name: "employee".into(),
            attributes: vec![id_a, id_b],
            comment: String::new(),
            is_fictitious: false,
        };

        // Order must be preserved — this is what the GUI will rely on
        // to display attributes in the same order the user arranged them.
        assert_eq!(entity.attributes, vec![id_a, id_b]);
    }

    #[test]
    fn array_data_type_can_nest_recursively() {
        let nested = DataType::Array(Box::new(DataType::Array(Box::new(DataType::Integer(
            IntegerWidth::Bits32,
        )))));

        match nested {
            DataType::Array(inner) => match *inner {
                DataType::Array(_) => {} // ok, one level of nesting confirmed
                _ => panic!("expected nested Array"),
            },
            _ => panic!("expected Array"),
        }
    }
}
