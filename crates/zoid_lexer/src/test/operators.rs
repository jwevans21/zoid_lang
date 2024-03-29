use super::*;

#[test]
fn operator_add() {
    let input = "+";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::OpAdd);
    assert_eq!(token.source, "+");
}

#[test]
fn operator_sub() {
    let input = "-";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::OpSub);
    assert_eq!(token.source, "-");
}

#[test]
fn operator_mul() {
    let input = "*";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::OpMul);
    assert_eq!(token.source, "*");
}

#[test]
fn operator_div() {
    let input = "/";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::OpDiv);
    assert_eq!(token.source, "/");
}

#[test]
fn operator_rem() {
    let input = "%";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::OpRem);
    assert_eq!(token.source, "%");
}

#[test]
fn operator_log_and() {
    let input = "&&";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::OpLogAnd);
    assert_eq!(token.source, "&&");
}

#[test]
fn operator_log_or() {
    let input = "||";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::OpLogOr);
    assert_eq!(token.source, "||");
}

#[test]
fn operator_log_not() {
    let input = "!";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::OpLogNot);
    assert_eq!(token.source, "!");
}

#[test]
fn operator_bit_and() {
    let input = "&";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::OpBitAnd);
    assert_eq!(token.source, "&");
}

#[test]
fn operator_bit_or() {
    let input = "|";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::OpBitOr);
    assert_eq!(token.source, "|");
}

#[test]
fn operator_bit_not() {
    let input = "~";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::OpBitNot);
    assert_eq!(token.source, "~");
}

#[test]
fn operator_bit_xor() {
    let input = "^";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::OpBitXor);
    assert_eq!(token.source, "^");
}

#[test]
fn operator_bit_shl() {
    let input = "<<";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::OpBitShl);
    assert_eq!(token.source, "<<");
}

#[test]
fn operator_bit_shr() {
    let input = ">>";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::OpBitShr);
    assert_eq!(token.source, ">>");
}

#[test]
fn operator_assign() {
    let input = "=";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::OpAssign);
    assert_eq!(token.source, "=");
}

#[test]
fn operator_eq() {
    let input = "==";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::OpEq);
    assert_eq!(token.source, "==");
}

#[test]
fn operator_ne() {
    let input = "!=";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::OpNe);
    assert_eq!(token.source, "!=");
}

#[test]
fn operator_lt() {
    let input = "<";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::OpLt);
    assert_eq!(token.source, "<");
}

#[test]
fn operator_le() {
    let input = "<=";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::OpLe);
    assert_eq!(token.source, "<=");
}

#[test]
fn operator_gt() {
    let input = ">";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::OpGt);
    assert_eq!(token.source, ">");
}

#[test]
fn operator_ge() {
    let input = ">=";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::OpGe);
    assert_eq!(token.source, ">=");
}
