#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Default, Hash)]
pub struct U64Wrap(pub u64);

impl<'a> Value<'a> for U64Wrap {
    fn get_type(&self) -> Ty {
        Ty::U64
    }
}
