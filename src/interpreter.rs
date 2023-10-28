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

    pub fn interpret(&mut self) {
        while let Some(token) = self.get_next_token() {
            match token {
                Token::Operator(_op) => { },
                Token::Closure(_c) => { },
                Token::Integer(_num) => { },
                Token::Variable(_var) => { }
            }
        }
    }

    pub fn advance(&mut self) {
        self.pos += 1;
        self.current_char = self.src.chars().nth(self.pos);
    }

    pub fn retreat(&mut self) {
        if self.pos > 0 {
            self.pos -= 1;
            self.current_char = self.src.chars().nth(self.pos);
        }
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
            Some(';') => Some(Token::Closure(';')),
            Some(d) if d.is_digit(10) => {
                let mut num = 0;
                while let Some(c) = self.current_char {
                    if let Some(dig) = c.to_digit(10) {
                        num = num * 10 + dig as i32;
                        let next_char = self.src.chars().nth(self.pos + 1);
                        if next_char.map_or(false, |c| c.is_digit(10)) { self.advance(); } 
                        else { break; }
                    } else { break; }
                }
                Some(Token::Integer(num))
            },
            Some(c) if c.is_alphabetic() => {
                let mut name = String::new();
                while let Some(c) = self.current_char {
                    if c.is_alphanumeric() {
                        name.push(c);
                        let next_char = self.src.chars().nth(self.pos + 1);
                        if next_char.map_or(false, |c| c.is_alphanumeric()) { self.advance(); } 
                        else { break; }
                    } else { break; }
                }
                Some(Token::Variable(name))
            }
            _ => None,
        };

        if self.current_char.is_some() { self.advance(); }
        token
    }

    pub fn peek(&mut self) -> Option<Token> {
        let saved_pos = self.pos;
        let saved_char = self.current_char;
        let next = self.get_next_token();
        self.pos = saved_pos;
        self.current_char = saved_char;
        next
    }

    pub fn parse_exp(&mut self) -> i32 {
        let mut result = self.parse_term();
        loop {
            handle_op!(self, result, '+', { result += self.parse_term(); });
            handle_op!(self, result, '-', { result -= self.parse_term(); });
            break; // Exit the loop if no more operations
        }
        result
    }

    pub fn parse_term(&mut self) -> i32 {
        let mut result = match self.get_next_token() {
            Some(Token::Integer(n)) => n,
            _ => panic!("Expected an integer: TinyC does not support floating-point numbers.")
        };
        loop {
            handle_op!(self, result, '*', { result *= self.parse_term(); });
            handle_op!(self, result, '/', { result /= self.parse_term(); });
            break; // Exit the loop if no more operations
        }
        result
    }
}