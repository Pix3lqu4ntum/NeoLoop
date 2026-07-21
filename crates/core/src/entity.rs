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
