#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Default, Hash)]
pub struct I64Wrap(pub i64);

impl_value_num! {I64Wrap, I64}
