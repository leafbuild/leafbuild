#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Default, Hash)]
pub struct U64Wrap(pub u64);

impl_value_num! {U64Wrap, U64}
