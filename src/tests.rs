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
    let mut interpreter = Interpreter::new("a = b + 1");
    assert_eq!(interpreter.get_next_token(), Some(Token::Variable("a".to_string())));
    assert_eq!(interpreter.get_next_token(), Some(Token::Operator('=')));
    assert_eq!(interpreter.get_next_token(), Some(Token::Variable("b".to_string())));
    assert_eq!(interpreter.get_next_token(), Some(Token::Operator('+')));
    assert_eq!(interpreter.get_next_token(), Some(Token::Integer(1)));
}

