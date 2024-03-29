use super::*;

#[test]
fn colon() {
    let input = ":";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::Colon);
    assert_eq!(token.source, ":");
}

#[test]
fn semicolon() {
    let input = ";";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::Semicolon);
    assert_eq!(token.source, ";");
}

#[test]
fn comma() {
    let input = ",";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::Comma);
    assert_eq!(token.source, ",");
}
