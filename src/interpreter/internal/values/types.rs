#[derive(Debug)]
pub enum ValueType {
    I32,
    I64,
    U32,
    U64,

    String,

    Vector(Box<ValueType>),
    Map(Box<ValueType>, Box<ValueType>),

    Object,
}
