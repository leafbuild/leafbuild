#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
pub struct BoolWrap(pub bool);

impl<'frame> Value<'frame> for BoolWrap {
    fn get_type(&self) -> Ty {
        Ty::Bool
    }
}
