use leafbuild_ast::token_data::NumVal;

use crate::env::FileFrame;
use crate::internal::repr::{IrAtom, IrExpr};
use crate::internal::values::{
    BoolWrap, I32Wrap, I64Wrap, LiveVal, U32Wrap, U64Wrap, ValRefMut, Value,
};

pub(super) trait Eval {
    fn eval_in_context<'frame>(&self, frame: &'_ mut FileFrame<'frame, '_>) -> LiveVal<'frame>;

    fn eval_in_context_mut<'frame>(
        &self,
        frame: &'_ mut FileFrame<'frame, '_>,
    ) -> ValRefMut<'frame>;
}

impl Eval for IrExpr {
    fn eval_in_context<'frame>(&self, frame: &'_ mut FileFrame<'frame, '_>) -> LiveVal<'frame> {
        unimplemented!()
    }

    fn eval_in_context_mut<'frame>(
        &self,
        frame: &'_ mut FileFrame<'frame, '_>,
    ) -> ValRefMut<'frame> {
        unimplemented!()
    }
}

impl Eval for IrAtom {
    fn eval_in_context<'frame>(&self, frame: &'_ mut FileFrame<'frame, '_>) -> LiveVal<'frame> {
        unimplemented!()
    }

    fn eval_in_context_mut<'frame>(
        &self,
        frame: &'_ mut FileFrame<'frame, '_>,
    ) -> ValRefMut<'frame> {
        unimplemented!()
    }
}

trait AsBoxedValue {
    fn as_boxed_value<'frame>(&self) -> Box<dyn Value<'frame>>;
}

impl AsBoxedValue for NumVal {
    fn as_boxed_value<'frame>(&self) -> Box<dyn Value<'frame>> {
        match self {
            Self::I32(v) => Box::new(I32Wrap(*v)),
            Self::I64(v) => Box::new(I64Wrap(*v)),
            Self::U32(v) => Box::new(U32Wrap(*v)),
            Self::U64(v) => Box::new(U64Wrap(*v)),
        }
    }
}

impl AsBoxedValue for bool {
    fn as_boxed_value<'frame>(&self) -> Box<dyn Value<'frame>> {
        Box::new(BoolWrap(*self))
    }
}
