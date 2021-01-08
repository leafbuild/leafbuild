#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Default, Hash)]
pub struct U32Wrap(pub u32);

impl_value_num! {U32Wrap, U32}
