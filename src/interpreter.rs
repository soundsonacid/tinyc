#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Token {
    Operator(char),
    Closure(char),
    Integer(i32),
    Variable(String),
}

pub struct Interpreter<'a> {
    src: &'a str, 
    pos: usize,
    pub current_char: Option<char>,
    pub var_table: std::collections::HashMap<String, i32>,
}

impl<'a> Interpreter<'a> {
    pub fn new(src: &'a str) -> Self {
        Self { src, pos: 0, current_char: src.chars().next() , var_table: std::collections::HashMap::default() }
    }

    pub fn advance(&mut self) {
        self.pos += 1;
        self.current_char = self.src.chars().nth(self.pos);
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() { self.advance(); }
            else { break; }
        }
    }

    pub fn get_next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        let token = match self.current_char {
            Some('+') => Some(Token::Operator('+')),
            Some('-') => Some(Token::Operator('-')),
            Some('*') => Some(Token::Operator('*')),
            Some('/') => Some(Token::Operator('/')),
            Some('=') => Some(Token::Operator('=')),
            Some('>') => Some(Token::Operator('>')),
            Some('<') => Some(Token::Operator('<')),
            Some('(') => Some(Token::Closure('(')),
            Some(')') => Some(Token::Closure(')')),
            Some('{') => Some(Token::Closure('{')),
            Some('}') => Some(Token::Closure('}')),
            Some(d) if d.is_digit(10) => {
                let mut num = 0;
                while let Some(c) = self.current_char {
                    if let Some(dig) = c.to_digit(10) {
                        num = num * 10 + dig as i32;
                        self.advance();
                    } else { break; }
                }
                Some(Token::Integer(num))
            },
            Some(c) if c.is_alphabetic() => {
                let mut name = String::new();
                while let Some(c) = self.current_char {
                    if c.is_alphanumeric() {
                        name.push(c);
                        self.advance();
                    } else { break; }
                };
                Some(Token::Variable(name))
            }
            _ => None,
        };

        if self.current_char.is_some() { self.advance(); }
        token
    }
}