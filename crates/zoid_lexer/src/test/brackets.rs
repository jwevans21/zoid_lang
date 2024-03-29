use super::*;

#[test]
fn left_paren() {
    let input = "(";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::LParen);
    assert_eq!(token.source, "(");
}

#[test]
fn right_paren() {
    let input = ")";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::RParen);
    assert_eq!(token.source, ")");
}

#[test]
fn left_brace() {
    let input = "{";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::LBrace);
    assert_eq!(token.source, "{");
}

#[test]
fn right_brace() {
    let input = "}";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::RBrace);
    assert_eq!(token.source, "}");
}

#[test]
fn left_bracket() {
    let input = "[";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::LBracket);
    assert_eq!(token.source, "[");
}

#[test]
fn right_bracket() {
    let input = "]";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::RBracket);
    assert_eq!(token.source, "]");
}

#[test]
fn multi_bracket() {
    let input = "({)})][";
    let mut lexer = ZoidLexer::new(input, "test");

    assert_eq!(lexer.lex().unwrap().unwrap().kind, ZoidTokenKind::LParen);
    assert_eq!(lexer.lex().unwrap().unwrap().kind, ZoidTokenKind::LBrace);
    assert_eq!(lexer.lex().unwrap().unwrap().kind, ZoidTokenKind::RParen);
    assert_eq!(lexer.lex().unwrap().unwrap().kind, ZoidTokenKind::RBrace);
    assert_eq!(lexer.lex().unwrap().unwrap().kind, ZoidTokenKind::RParen);
    assert_eq!(lexer.lex().unwrap().unwrap().kind, ZoidTokenKind::RBracket);
    assert_eq!(lexer.lex().unwrap().unwrap().kind, ZoidTokenKind::LBracket);
}
