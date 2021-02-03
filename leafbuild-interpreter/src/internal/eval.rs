use leafbuild_ast::ast::{Atom, Expr};
use leafbuild_ast::token_data::NumVal;

use crate::env::FileFrame;
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

impl Eval for Expr {
    fn eval_in_context<'frame>(&self, frame: &'_ mut FileFrame<'frame, '_>) -> LiveVal<'frame> {
        match self {
            Self::Atom(atom) => atom.eval_in_context(frame),
            Self::Op(left, opcode, right) => {
                let left = left.eval_in_context(frame);
                let right = right.eval_in_context(frame);
                // opcode.apply_to(frame, left, right)
                unimplemented!()
            }
            Self::UnaryOp(opcode, expr) => {
                let v = expr.eval_in_context(frame);
                // opcode.apply_to(frame, v)
                unimplemented!()
            }
            Self::FnCall(call) => {
                unimplemented!()
            }
            Self::MethodCall(_) => {
                unimplemented!()
            }
            Self::PropertyAccess(_) => {
                unimplemented!()
            }
            Self::Paren { .. } => {
                unimplemented!()
            }
            Self::Indexed { .. } => {
                unimplemented!()
            }
            Self::Ternary { .. } => {
                unimplemented!()
            }
        }
    }

    fn eval_in_context_mut<'frame>(
        &self,
        frame: &'_ mut FileFrame<'frame, '_>,
    ) -> &'frame mut dyn Value<'frame> {
        unimplemented!()
    }
}

impl Eval for Atom {
    fn eval_in_context<'frame>(
        &self,
        frame: &'_ mut FileFrame<'frame, '_>,
    ) -> Box<dyn Value<'frame>> {
        match self {
            Self::Number(num) => num.as_boxed_value(),
            Self::Bool(bool) => bool.as_boxed_value(),
            // Atom::Str(_) => {}
            // Atom::Id(_) => {}
            // Atom::ArrayLit(_, _, _) => {}
            // Atom::MapLit(_, _, _) => {}
            _ => unimplemented!(),
        }
    }

    fn eval_in_context_mut<'frame>(
        &self,
        frame: &'_ mut FileFrame<'frame, '_>,
    ) -> &'frame mut dyn Value<'frame> {
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
