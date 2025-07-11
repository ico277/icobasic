use std::{fs::File, io::BufReader};
use std::env::args;

use icobasic::lexar::lexar;
use icobasic::parser;

fn main() {
    let mut args = args().into_iter();
    let reader = BufReader::new(File::open(args.nth(1).unwrap()).unwrap());
    let tokens = lexar(reader);
    for token in tokens {
        println!("{:?}", token);
    }
    
}
