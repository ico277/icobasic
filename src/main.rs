use std::{fs::File, io::BufReader};
use std::env::args;

use icobasic::lexar::lexar;
use icobasic::parser::parser;

fn main() {
    let mut args = args().into_iter();
    let reader = BufReader::new(File::open(args.nth(1).unwrap()).unwrap());
    let tokens = lexar(reader);

    println!("Lexer Result:");
    for token in &tokens {
        println!("{:?}", token);
    }
    println!();

    let instructions = parser(tokens.into_iter().peekable());

    println!("Parser Result:");
    for instr in &instructions {    
        println!("{:?}", instr);
    }
    println!();

    
}
