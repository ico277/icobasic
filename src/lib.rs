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
        if !object.is_empty()  {
            return match object.to_lowercase().as_str() {
                "if" | "else" | "for" | "while" | "break" | "let" => Token::Keyword(object.to_lowercase()),
                _ => Token::Identifier(object)
            }
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
        //TODO FIX bug that ignores empty strings
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
    use std::iter::Peekable;

    #[derive(Debug)]
    pub enum VariableType {
        Number(i32),
        String(String),
    }
    #[derive(Debug)]
    pub struct Variable {
        pub identifier: String,
        pub value: Option<VariableType>,
    }

    #[derive(Debug)]
     pub enum FunctionArgument {
        Variable(String),
        String(String),
        Number(i32),
    }
    #[derive(Debug)]
    pub struct FunctionCall {
        pub func_name: String,
        pub arguments: Vec<FunctionArgument>,
    }

    #[derive(Debug)]
    pub enum Instruction {
        VariableCreate(Variable),
        VariableAssign(Variable),
        FunctionCall(FunctionCall),        
    }

    pub fn parser<T>(mut tokens: Peekable<T>) -> Vec<Instruction>
    where
        T: Iterator<Item = Token>
    {
        let mut instructions = vec![];

        loop {
            let mut full_inst = vec![];
            while let Some(t) = tokens.peek() {
                if let Token::ExpressionEnd(_) = t {
                    tokens.next();
                    break
                } else {
                    full_inst.push(tokens.next().unwrap());
                }       
            }

            //instruction matching
            match full_inst.as_slice() {
                [Token::Keyword(key), Token::Identifier(ident), Token::Operator(op), value] 
                    if key == "let" && op == "=" =>
                {
                    match value {
                        Token::Number(n) => {
                            instructions.push(Instruction::VariableCreate(Variable {
                                identifier: ident.clone(),
                                value: Some(VariableType::Number(n.parse().unwrap())),
                            }));
                        },
                        Token::String(s) => {
                            instructions.push(Instruction::VariableCreate(Variable {
                                identifier: ident.clone(),
                                value: Some(VariableType::String(s.clone())),
                            }));
                        },
                        Token::Identifier(i) => {
                            instructions.push(Instruction::VariableCreate(Variable {
                                identifier: i.clone(),
                                value: None,
                            }));
                        },
                        _ => {
                            eprintln!("Error: Invalid syntax for variable assign operation!");
                        }
                    }
                },
                
                [Token::Identifier(ident), Token::Operator(op), value] 
                    if op == "=" =>
                {
                    match value {
                        Token::Number(n) => {
                            instructions.push(Instruction::VariableAssign(Variable {
                                identifier: ident.clone(),
                                value: Some(VariableType::Number(n.parse().unwrap())),
                            }));
                        },
                        Token::String(s) => {
                            instructions.push(Instruction::VariableAssign(Variable {
                                identifier: ident.clone(),
                                value: Some(VariableType::String(s.clone())),
                            }));
                        },
                        Token::Identifier(i) => {
                            instructions.push(Instruction::VariableAssign(Variable {
                                identifier: i.clone(),
                                value: None,
                            }));
                        },
                        _ => {
                            eprintln!("Error: Invalid syntax for variable assign operation!");
                        }
                    }
                },

                [Token::Identifier(func), value] => match value {
                    Token::Number(num) => {
                        instructions.push(Instruction::FunctionCall(FunctionCall {
                            func_name: func.clone(),
                            arguments: vec![FunctionArgument::Number(num.parse().unwrap())],
                        }));  
                    },
                    Token::String(string) => {
                        instructions.push(Instruction::FunctionCall(FunctionCall {
                            func_name: func.clone(),
                            arguments: vec![FunctionArgument::String(string.clone())],
                        }));  
                    },
                    Token::Identifier(i) => {
                        instructions.push(Instruction::FunctionCall(FunctionCall {
                            func_name: func.clone(),
                            arguments: vec![FunctionArgument::Variable(i.clone())],
                        }))                        
                    },
                    _ => {
                        eprintln!("Error: Invalid syntax for function call!")
                    },
                }, 

                [Token::Identifier(func), first_arg, Token::Seperator(_), rest_args @ ..] => {
                    let mut func_call = FunctionCall {
                        func_name: func.clone(),
                        arguments: Vec::new(),
                    };
                    match first_arg {
                        Token::Number(num) => {
                            func_call.arguments.push(FunctionArgument::Number(num.parse().unwrap()));
                        },
                        Token::String(string) => {
                            func_call.arguments.push(FunctionArgument::String(string.clone()));
                        },
                        Token::Identifier(i) => {   
                            func_call.arguments.push(FunctionArgument::Variable(i.clone()));                    
                        },
                        _ => {
                            eprintln!("Error: Invalid syntax for function call!")
                        },
                    }

                    let mut last_sep = true;
                    for arg in rest_args {
                        match (arg, last_sep) {
                            (Token::Number(num), true) => {
                                func_call.arguments.push(FunctionArgument::Number(num.parse().unwrap()));
                                last_sep = false;    
                            },
                            (Token::String(string), true) => {
                                func_call.arguments.push(FunctionArgument::String(string.clone()));
                                last_sep = false;
                            },
                            (Token::Identifier(i), true) => {
                                func_call.arguments.push(FunctionArgument::Variable(i.clone()));
                                last_sep = false;
                            },
                            (Token::Seperator(_), false) => {
                                last_sep = true
                            },
                            _ => {
                                eprintln!("Error: Invalid syntax for function call argument seperation!");
                            }
                        }
                    }

                    instructions.push(Instruction::FunctionCall(func_call));
                },
   
                _ => {
                    eprintln!("Error: Invalid call!\n-> {:#?}", full_inst);
                },
            }

            if let None = tokens.peek() {
                break
            }
        }

        instructions    
    }
}


/*
x86_64-linux-c99
x86_64-win32-c99
i686-linux-c99
i686-win32-c99
*/

pub mod compiler {
    use std::iter::Peekable;
    use std::collections::HashMap;	
    use crate::parser::Instruction;
    use crate::parser::Variable;

    enum ConstantValue {
        String(String),
        Number(i32),
    }

    

    struct ProgramContext {
        extern_section: Vec<String>,                        // list of imports
        data_section: HashMap<usize, ConstantValue>,        // map of index -> value
        global_vars: HashMap<String, (usize, Variable)>,    // map of variable -> index
    }
    
    #[cfg(feature = "x86_64-linux-c99")]
    const TEMPLATE_X86_64_LINUX_C99: &str = include_str!("./templates/x86_64-linux-c99.asm");
    #[cfg(feature = "x86_64-win32-c99")]
    const TEMPLATE_X86_64_WIN32_C99: &str = include_str!("./templates/x86_64-win32-c99.asm");
    #[cfg(feature = "i686-linux-c99")]
    const TEMPLATE_I686_LINUX_C99: &str = include_str!("./templates/i686-linux-c99.asm");
    #[cfg(feature = "i686-win32-c99")]
    const TEMPLATE_I686_WIN32_C99: &str = include_str!("./templates/i686-win32-c99.asm");

    pub fn compile<I>(instructions: Peekable<I>, target: String) -> String
    where
        I: Iterator<Item = Instruction>
    {
        return match target.as_str() {
            #[cfg(feature = "x86_64-linux-c99")]
            "x86_64-linux-c99" => x86_64_linux_c99_compile(instructions),
            #[cfg(feature = "x86_64-win32-c99")]
            "x86_64-win32-c99" => x86_64_win32_c99_compile(instructions),
            #[cfg(feature = "i686-linux-c99")]
            "i686-linux-c99" => i686_linux_c99_compile(instructions),
            #[cfg(feature = "i686-win32-c99")]
            "i686-win32-c99" => i686_win32_c99_compile(instructions),
            t => panic!("Invalid target '{}'! (Be sure you enabled it as a feature!)", t)
        }
    }

    #[cfg(feature = "x86_64-linux-c99")]
    fn x86_64_linux_c99_compile<I>(instructions: Peekable<I>) -> String
    where
        I: Iterator<Item = Instruction>
    {
        String::new()
    }

    #[cfg(feature = "x86_64-win32-c99")]
    fn x86_64_win32_c99_compile<I>(instructions: Peekable<I>) -> String
    where
        I: Iterator<Item = Instruction>
    {
        String::new()
    }

    #[cfg(feature = "i686-linux-c99")]
    fn i686_linux_c99_compile<I>(instructions: Peekable<I>) -> String
    where
        I: Iterator<Item = Instruction>
    {
        String::new()
    }

    #[cfg(feature = "i686-win32-c99")]
    fn i686_win32_c99_compile<I>(instructions: Peekable<I>) -> String
    where
        I: Iterator<Item = Instruction>
    {
        String::new()
    }
}
