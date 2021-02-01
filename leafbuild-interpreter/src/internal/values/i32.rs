#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Default, Hash)]
pub struct I32Wrap(pub i32);

impl<'a> Value<'a> for I32Wrap {
    fn get_type(&self) -> Ty {
        Ty::I32
    }
}
