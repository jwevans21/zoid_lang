use super::*;

#[test]
fn integer() {
    let input = "123";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::Integer);
    assert_eq!(token.source, "123");
}

#[test]
fn basic_float() {
    let input = "123.456";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::Float);
    assert_eq!(token.source, "123.456");
}

#[test]
fn basic_exponent_float() {
    let input = "123e4";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::Float);
    assert_eq!(token.source, "123e4");
}

#[test]
fn basic_uppercase_exponent_float() {
    let input = "123E4";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::Float);
    assert_eq!(token.source, "123E4");
}

#[test]
fn basic_negative_exponent_float() {
    let input = "123e-4";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::Float);
    assert_eq!(token.source, "123e-4");
}

#[test]
fn basic_positive_exponent_float() {
    let input = "123e+4";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::Float);
    assert_eq!(token.source, "123e+4");
}

#[test]
fn float_decimal_exponent() {
    let input = "123.456e4";

    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::Float);
    assert_eq!(token.source, "123.456e4");
}
