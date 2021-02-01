#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Default, Hash)]
pub struct U32Wrap(pub u32);

impl<'a> Value<'a> for U32Wrap {
    fn get_type(&self) -> Ty {
        Ty::U32
    }
}
