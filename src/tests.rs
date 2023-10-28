// these tests do NOT count towards the 1,000 line goal

use crate::interpreter::{Interpreter, Token};

#[test]
fn test_advance() {
    let mut interpreter = Interpreter::new("abc");
    interpreter.advance();
    assert_eq!(interpreter.current_char, Some('b'));
}

#[test]
fn test_skip_whitespace() {
    let mut interpreter = Interpreter::new("   abc");
    interpreter.skip_whitespace();
    assert_eq!(interpreter.current_char, Some('a'));
}

#[test]
fn test_get_next_token() {
    let mut interpreter = Interpreter::new("a = b + 1;");
    assert_eq!(interpreter.get_next_token(), Some(Token::Variable("a".to_string())));
    assert_eq!(interpreter.get_next_token(), Some(Token::Operator('=')));
    assert_eq!(interpreter.get_next_token(), Some(Token::Variable("b".to_string())));
    assert_eq!(interpreter.get_next_token(), Some(Token::Operator('+')));
    assert_eq!(interpreter.get_next_token(), Some(Token::Integer(1)));
    assert_eq!(interpreter.get_next_token(), Some(Token::Closure(';')));
}

#[test]
fn test_peek() {
    let mut interpreter = Interpreter::new("a = b;");
    assert_eq!(interpreter.peek(), Some(Token::Variable("a".to_string())));
    assert_eq!(interpreter.get_next_token(), Some(Token::Variable("a".to_string())));
    assert_eq!(interpreter.peek(), Some(Token::Operator('=')));
    assert_eq!(interpreter.get_next_token(), Some(Token::Operator('=')));
    assert_eq!(interpreter.peek(), Some(Token::Variable("b".to_string())));
    assert_eq!(interpreter.get_next_token(), Some(Token::Variable("b".to_string())));
    assert_eq!(interpreter.peek(), Some(Token::Closure(';')));
    assert_eq!(interpreter.get_next_token(), Some(Token::Closure(';')));
}

#[test]
fn test_parse_term() {
    let mut interpreter = Interpreter::new("3 * 5;");
    assert_eq!(interpreter.parse_term(), 15);

    let mut interpreter = Interpreter::new("10 / 2 / 1;");
    assert_eq!(interpreter.parse_term(), 5);

    let mut interpreter = Interpreter::new("7;");
    assert_eq!(interpreter.parse_term(), 7);
}

#[test]
fn test_parse_exp() {
    let mut interpreter = Interpreter::new("3 * 2 + 1;");
    assert_eq!(interpreter.parse_exp(), 7);

    let mut interpreter = Interpreter::new("10 / 2 - 3;");
    assert_eq!(interpreter.parse_exp(), 2);

    let mut interpreter = Interpreter::new("3 + 2;");
    assert_eq!(interpreter.parse_exp(), 5);

    let mut interpreter = Interpreter::new("3 - 2;");
    assert_eq!(interpreter.parse_exp(), 1);
}