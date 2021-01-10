use leafbuild_ast::ast::{Atom, Expr};
use leafbuild_ast::token_data::NumVal;

use crate::env::FileFrame;
use crate::internal::values::types::ValueType;
use crate::internal::values::{BoolWrap, I32Wrap, I64Wrap, U32Wrap, U64Wrap, Value};
use leafbuild_ast::Span;

enum CannotEvaluateError {
    NotImplemented,
}

enum CannotEvaluateMutError {
    NotImplemented,
}

trait Eval {
    fn eval_in_context<'frame>(
        &self,
        frame: &'frame mut FileFrame<'frame>,
    ) -> Result<Box<dyn Value<'frame>>, CannotEvaluateError> {
        Err(CannotEvaluateError::NotImplemented)
    }

    fn eval_in_context_mut<'frame>(
        &self,
        frame: &'frame mut FileFrame<'frame>,
    ) -> Result<&'frame mut dyn Value<'frame>, CannotEvaluateMutError> {
        Err(CannotEvaluateMutError::NotImplemented)
    }
}

impl Eval for Expr {
    fn eval_in_context<'frame>(
        &self,
        frame: &'frame mut FileFrame<'frame>,
    ) -> Result<Box<dyn Value<'frame>>, CannotEvaluateError> {
        // match self {
        //     Expr::Atom(atom) => atom.eval_in_context(frame),
        //     Expr::Op(left, opcode, right) => {
        //         let left = left.eval_in_context(frame)?;
        //         let right = right.eval_in_context(frame)?;
        //         opcode.apply_to(left, right)
        //     }
        //     Expr::UnaryOp(_, _) => {}
        //     Expr::FuncCall(_) => {}
        //     Expr::MethodCall(_) => {}
        //     Expr::PropertyAccess(_) => {}
        //     Expr::Paren { .. } => {}
        //     Expr::Indexed { .. } => {}
        //     Expr::Ternary { .. } => {}
        // }
        unimplemented!()
    }

    fn eval_in_context_mut<'frame>(
        &self,
        frame: &'frame mut FileFrame<'frame>,
    ) -> Result<&'frame mut dyn Value<'frame>, CannotEvaluateMutError> {
        Err(CannotEvaluateMutError::NotImplemented)
    }
}

impl Eval for Atom {
    fn eval_in_context<'frame>(
        &self,
        frame: &'frame mut FileFrame<'frame>,
    ) -> Result<Box<dyn Value<'frame>>, CannotEvaluateError> {
        match self {
            Self::Number(num) => Ok(num.as_boxed_value()),
            Self::Bool(bool) => Ok(bool.as_boxed_value()),
            // Atom::Str(_) => {}
            // Atom::Id(_) => {}
            // Atom::ArrayLit(_, _, _) => {}
            // Atom::MapLit(_, _, _) => {}
            _ => unimplemented!(),
        }
    }

    fn eval_in_context_mut<'frame>(
        &self,
        frame: &'frame mut FileFrame<'frame>,
    ) -> Result<&'frame mut dyn Value<'frame>, CannotEvaluateMutError> {
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

enum BinOpApplyError {
    IncompatibleOperands {
        left: ValueType,
        left_span: Span,
        right: ValueType,
        right_span: Span,
    },
}

trait BinOpApplyTo {
    fn apply_to<'frame>(
        &self,
        frame: &'frame FileFrame,
        left: Box<dyn Value<'frame>>,
        right: Box<dyn Value<'frame>>,
    ) -> Result<Box<dyn Value<'frame>>, BinOpApplyError> {
        unimplemented!()
    }
}
