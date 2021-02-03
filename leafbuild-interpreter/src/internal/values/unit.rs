#[derive(Debug)]
pub struct UnitWrap;

impl<'a> Value<'a> for UnitWrap {
    fn get_type(&self) -> Ty {
        Ty::Tuple(vec![])
    }
}
