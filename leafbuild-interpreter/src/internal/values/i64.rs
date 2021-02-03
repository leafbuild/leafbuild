#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Default, Hash)]
pub struct I64Wrap(pub i64);

impl<'a> Value<'a> for I64Wrap {
    fn get_type(&self) -> Ty {
        Ty::I64
    }
}
