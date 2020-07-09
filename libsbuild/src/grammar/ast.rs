pub enum AtomT {
    Number(i32),
    Str(String),
    Id(String),
}

pub enum Expr {
    Atom(AtomT),
    Op(Box<Expr>, Opcode, Box<Expr>),
}

pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}