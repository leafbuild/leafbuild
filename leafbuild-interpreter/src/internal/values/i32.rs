#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Default, Hash)]
pub struct I32Wrap(pub i32);

impl_value_num! {I32Wrap, I32}
