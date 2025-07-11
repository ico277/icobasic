pub mod lexar {
    pub mod token {
        #[derive(Debug)]
        pub enum Token {
            Identifier(String),
            Keyword(String),
            Number(String),
            String(String),
            Operator(String),
            Seperator(String),
            LineEnd(String),
            ExpressionEnd(String),
            Unknown(String),
        }
    }

    use std::io::BufRead;
    use std::iter::Peekable;	

    use token::Token;

    fn next_token<I>(chars: &mut Peekable<I>) -> Token
    where
        I: Iterator<Item = char> + Clone,
    {
        // skip whitespace
        while let Some(c) = chars.peek() {
            if c.is_whitespace() {
                chars.next();
            } else {
                break;
            }
        }

        let mut object = String::new();

        // numbers
        while let Some(c) = chars.peek() {
            if c.is_digit(10) {
                object.push(chars.next().unwrap());
            } else {
                break;
            }
        }
        if !object.is_empty() {
            return Token::Number(object);
        }

        // identifiers
        while let Some(c) = chars.peek() {
            if c.is_alphanumeric() || *c == '$' {
                object.push(chars.next().unwrap());
            } else {
                break;
            }
        }
        if !object.is_empty() {
            return Token::Identifier(object);
        }

        // Operators
        match chars.peek() {
            Some('=') | Some('+') | Some('-') | Some('/') | Some('*') => {
                object.push(chars.next().unwrap());
            }
            _ => (),
        }
        if !object.is_empty() {
            return Token::Operator(object);
        }

        // String
        if let Some('"') = chars.peek() {
            chars.next().unwrap();
            loop {
                match chars.next() {
                    Some('"') => break,
                    Some('\\') => {
                        match chars.next() {
                            Some('n') => object.push('\n'),
                            Some('r') => object.push('\r'),
                            Some('t') => object.push('\t'),
                            Some('\\') => object.push('\\'),
                            Some('"') => object.push('"'),
                            Some('x') => {
                                match (chars.next(), chars.next()) {
                                    (Some(s1), Some(s2)) => {
                                        let num = format!("{}{}", s1, s2);
                                        object.push(
                                            u8::from_str_radix(num.as_str(), 16).unwrap() as char,
                                        );
                                    }
                                    _ => (),//{ return Token::Unknown(String::from("Unexpected end of string")) },
                                }
                            }
                            Some(s) => {
                                eprintln!("Warning: Invalid escape code '{}'!", s);
                                object.push(s);
                            }
                            None => continue,
                        }
                    }
                    Some(s) => object.push(s),
                    None => break,
                }
            }
        }
        if !object.is_empty() {
            return Token::String(object);
        }

        // comma seperator
        if let Some(',') = chars.peek() {
            return Token::Seperator(String::from(chars.next().unwrap()));
        }

        // semicolon seperator
        if let Some(';') = chars.peek() {
            return Token::ExpressionEnd(String::from(chars.next().unwrap()));
        }

        // check if end of line is reached
        match chars.peek() {
            Some('\n') => {
                return Token::LineEnd(String::from(chars.next().unwrap()));
            }
            None => return Token::LineEnd(String::new()),
            _ => (),
        }

        // unknown token from pos
        Token::Unknown(String::from(chars.next().unwrap_or('\0')))
    }

    pub fn lexar<F: BufRead>(input: F) -> Vec<Token> {
        let mut tokens = vec![];
        let lines = input.lines().map(|l| l.unwrap());
        for line in lines {
            let mut iter = line.chars().peekable();
            let mut last_token = next_token(&mut iter);
            loop {
                match last_token {
                    Token::LineEnd(s) => {
                        tokens.push(Token::ExpressionEnd(s));
                        break;
                    }
                    t => tokens.push(t),
                }

                last_token = next_token(&mut iter);
            }
        }

        tokens
    }
}

pub mod parser {
    use crate::lexar::token::Token;

    pub enum VariableType {
        Number(i32),
        String(String),
    }
    pub struct Variable {
        pub identifier: String,
        pub value: Option<VariableType>,
    }

    pub enum FunctionArgument {
        Variable(String),
        String(String),
        Number(i32),
    }
    pub struct FunctionCall {
        pub func_name: String,
        pub arguments: Vec<FunctionArgument>,
    }

    pub enum Instruction {
        VariableCreate(Variable),
        VariableAssign(Variable),
        FunctionCall(FunctionCall),        
    }

    pub fn parser(tokens: Vec<Token>) -> Vec<Instruction> {
        let mut instructions = vec![];

        let mut full_inst;

        instructions    
    }
}
