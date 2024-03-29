#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ZoidToken<'a> {
    pub kind: ZoidTokenKind,
    pub start: usize,
    pub end: usize,
    pub source: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ZoidTokenKind {
    OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    OpRem,
    OpLogAnd,
    OpLogOr,
    OpLogNot,
    OpBitAnd,
    OpBitOr,
    OpBitNot,
    OpBitXor,
    OpBitShl,
    OpBitShr,
    OpAssign,
    OpEq,
    OpNe,
    OpLt,
    OpLe,
    OpGt,
    OpGe,

    KwIf,
    KwElse,
    KwWhile,
    KwLet,
    KwFn,
    KwReturn,

    Identifier,
    Integer,
    Float,
}
