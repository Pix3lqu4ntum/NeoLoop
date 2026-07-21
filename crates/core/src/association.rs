/// The four standard Merise cardinalities carried by each end of an
/// association: (0,1), (1,1), (0,n) and (1,n).
pub enum Cardinality {
    ZeroOne,
    OneOne,
    ZeroMany,
    OneMany,
}
