use super::*;

#[test]
fn keyword_if() {
    let input = "if";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::KwIf);
    assert_eq!(token.source, "if");
}

#[test]
fn keyword_else() {
    let input = "else";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::KwElse);
    assert_eq!(token.source, "else");
}

#[test]
fn keyword_while() {
    let input = "while";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::KwWhile);
    assert_eq!(token.source, "while");
}

#[test]
fn keyword_let() {
    let input = "let";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::KwLet);
    assert_eq!(token.source, "let");
}

#[test]
fn keyword_fn() {
    let input = "fn";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::KwFn);
    assert_eq!(token.source, "fn");
}

#[test]
fn keyword_return() {
    let input = "return";
    let mut lexer = ZoidLexer::new(input, "test");

    let token = lexer.lex();

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(token.is_some());
    let token = token.unwrap();
    assert_eq!(token.kind, ZoidTokenKind::KwReturn);
    assert_eq!(token.source, "return");
}
