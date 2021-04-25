use std::marker::PhantomData;

use leafbuild_syntax::ast::visitor::ThreadSafeVisitor;
use leafbuild_syntax::ast::{AstNode, AstToken, BoolLit, Expr, PrimaryExpr, StrLit};

pub struct ExprEvaluator<'a>(PhantomData<&'a ()>);

impl<'a> ExprEvaluator<'a> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),

    Bool(bool),

    String(String),

    Error,
}

impl<'a> ThreadSafeVisitor for ExprEvaluator<'a> {
    type Output = Value;

    fn visit_bool_lit(&self, bool_lit: BoolLit) -> Self::Output {
        let v = bool_lit.get_true_opt().is_some();
        Value::Bool(v)
    }

    fn visit_str_lit(&self, str_lit: StrLit) -> Self::Output {
        if let Some(s) = str_lit.get_single_line_string_opt() {
            self.visit_str(s)
        } else {
            self.visit_multiline_str(str_lit.get_multiline_string_opt().unwrap())
        }
    }

    fn visit_num_lit(&self, num_lit: NumLit) -> Self::Output {
        use std::num::ParseIntError;
        use std::str::FromStr;
        #[derive(Copy, Clone, Debug, PartialOrd, Eq, PartialEq)]
        pub enum NumVal {
            /// i32 number
            I32(i32),
            /// i64 number
            I64(i64),
            /// u32 number
            U32(u32),
            /// u64 number
            U64(u64),
        }

        #[derive(Copy, Clone)]
        enum Tp {
            I32,
            I64,
            U32,
            U64,
        }
        impl Tp {
            const fn into_unsigned(self) -> Self {
                match self {
                    Self::I32 => Self::U32,
                    Self::I64 => Self::U64,
                    x => x,
                }
            }

            const fn into_long(self) -> Self {
                match self {
                    Self::I32 => Self::I64,
                    Self::U32 => Self::U64,
                    x => x,
                }
            }

            const fn zero(self) -> NumVal {
                match self {
                    Self::I32 => NumVal::I32(0),
                    Self::I64 => NumVal::I64(0),
                    Self::U32 => NumVal::U32(0),
                    Self::U64 => NumVal::U64(0),
                }
            }

            fn create_from_str(self, s: &str, radix: u32) -> Result<NumVal, ParseIntError> {
                match self {
                    Self::I32 => i32::from_str_radix(s, radix).map(NumVal::I32),
                    Self::U32 => u32::from_str_radix(s, radix).map(NumVal::U32),
                    Self::I64 => i64::from_str_radix(s, radix).map(NumVal::I64),
                    Self::U64 => u64::from_str_radix(s, radix).map(NumVal::U64),
                }
            }
        }

        impl FromStr for NumVal {
            type Err = ParseIntError;
            /// parse a number from a number literal string
            fn from_str(s: &str) -> Result<Self, ParseIntError> {
                let tp = s
                    .chars()
                    .rev()
                    .take_while(|chr| Self::is_suffix(*chr))
                    .fold(Tp::I32, |tp, chr| match chr {
                        'u' | 'U' => tp.into_unsigned(),
                        'l' | 'L' => tp.into_long(),
                        _ => tp,
                    });
                if s.starts_with("0x") {
                    Self::parse_hex(s, tp)
                } else if s.starts_with("0b") {
                    Self::parse_bin(s, tp)
                } else if s.starts_with('0') {
                    Self::parse_oct(s, tp)
                } else {
                    Self::parse_dec(s, tp)
                }
            }
        }

        impl NumVal {
            fn parse_hex(s: &str, tp: Tp) -> Result<Self, ParseIntError> {
                // s = "0x.."
                tp.create_from_str(&s["0x".len()..].trim_end_matches(Self::is_suffix), 16)
            }

            fn parse_bin(s: &str, tp: Tp) -> Result<Self, ParseIntError> {
                // s = "0b..."
                tp.create_from_str(&s["0b".len()..].trim_end_matches(Self::is_suffix), 2)
            }

            fn parse_oct(s: &str, tp: Tp) -> Result<Self, ParseIntError> {
                // s = "0..."
                let s = &s["0".len()..].trim_end_matches(Self::is_suffix);
                if s.is_empty() {
                    return Ok(tp.zero());
                }

                tp.create_from_str(s, 8)
            }

            fn parse_dec(s: &str, tp: Tp) -> Result<Self, ParseIntError> {
                // s = "..."
                tp.create_from_str(s.trim_end_matches(Self::is_suffix), 10)
            }

            const fn is_suffix(chr: char) -> bool {
                matches!(chr, 'u' | 'U' | 'l' | 'L')
            }
        }

        let t = num_lit.get_text();
        let num = t.parse().unwrap();
        match num {
            NumVal::I32(v) => Value::I32(v),
            NumVal::U32(v) => Value::U32(v),
            NumVal::I64(v) => Value::I64(v),
            NumVal::U64(v) => Value::U64(v),
        }
    }

    // fn visit_infix_binary_op_expr(&self, infix_binary_op_expr: InfixBinaryOpExpr) -> Self::Output {}

    fn visit_expr(&self, expr: Expr) -> Self::Output {
        self.visit(expr.syntax().first_child().unwrap())
    }

    fn visit_primary_expr(&self, primary_expr: PrimaryExpr) -> Self::Output {
        self.visit_element(primary_expr.syntax().first_child_or_token().unwrap())
    }

    fn visit_infix_binary_op_expr(&self, infix_binary_op_expr: InfixBinaryOpExpr) -> Self::Output {
        let lhs = infix_binary_op_expr.get_left_operand();
        let rhs = infix_binary_op_expr.get_right_operand();

        let op = infix_binary_op_expr.op_details();

        Value::I32(
            match self.visit_expr(lhs) {
                Value::I32(v) => v,
                _ => panic!("Test"),
            } + match self.visit_expr(rhs) {
                Value::I32(v) => v,
                _ => panic!("Test"),
            },
        )
    }
}
