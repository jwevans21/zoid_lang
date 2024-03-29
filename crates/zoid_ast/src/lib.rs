#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ZoidProgram {
    pub functions: Vec<ZoidFunction>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ZoidFunction {
    pub name: String,
    pub parameters: Vec<(String, ZoidType)>,
    pub return_type: ZoidType,
    pub body: Vec<ZoidStatement>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ZoidType {
    I32,
    F32,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ZoidStatement {
    Block(Vec<ZoidStatement>),
    If(ZoidExpression, Box<ZoidStatement>, Box<ZoidStatement>),
    While(ZoidExpression, Box<ZoidStatement>),
    VariableDeclaration(String, ZoidType, ZoidExpression),
    Return(ZoidExpression),
    Expression(ZoidExpression),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ZoidExpression {
    Identifier(String),
    LiteralInteger(i64),
    LiteralFloat(f64),

    Add(Box<ZoidExpression>, Box<ZoidExpression>),
    Subtract(Box<ZoidExpression>, Box<ZoidExpression>),
    Multiply(Box<ZoidExpression>, Box<ZoidExpression>),
    Divide(Box<ZoidExpression>, Box<ZoidExpression>),
    Remainder(Box<ZoidExpression>, Box<ZoidExpression>),
    LogicalAnd(Box<ZoidExpression>, Box<ZoidExpression>),
    LogicalOr(Box<ZoidExpression>, Box<ZoidExpression>),
    LogicalNot(Box<ZoidExpression>),
    BitwiseAnd(Box<ZoidExpression>, Box<ZoidExpression>),
    BitwiseOr(Box<ZoidExpression>, Box<ZoidExpression>),
    BitwiseNot(Box<ZoidExpression>),
    BitwiseXor(Box<ZoidExpression>, Box<ZoidExpression>),
    BitwiseShiftLeft(Box<ZoidExpression>, Box<ZoidExpression>),
    BitwiseShiftRight(Box<ZoidExpression>, Box<ZoidExpression>),
    Equality(Box<ZoidExpression>, Box<ZoidExpression>),
    Inequality(Box<ZoidExpression>, Box<ZoidExpression>),
    LessThan(Box<ZoidExpression>, Box<ZoidExpression>),
    LessThanOrEqual(Box<ZoidExpression>, Box<ZoidExpression>),
    GreaterThan(Box<ZoidExpression>, Box<ZoidExpression>),
    GreaterThanOrEqual(Box<ZoidExpression>, Box<ZoidExpression>),
}
