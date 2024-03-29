use super::*;

#[test]
fn alpha() {
    let input = "identifier";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::Identifier);
    assert_eq!(token.source, "identifier");
}

#[test]
fn alpha_numeric() {
    let input = "identifier123";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::Identifier);
    assert_eq!(token.source, "identifier123");
}

#[test]
fn underscore_start() {
    let input = "_identifier";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::Identifier);
    assert_eq!(token.source, "_identifier");
}

#[test]
fn underscore_middle() {
    let input = "id_entifier";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::Identifier);
    assert_eq!(token.source, "id_entifier");
}
