mod ast;
mod loop_helper;
mod parser;
mod opt;
mod interp;

#[cfg(test)]
mod buffer;
#[cfg(test)]
mod parser_tests;
#[cfg(test)]
mod interp_tests;
#[cfg(test)]
mod opt_test;

use ast::{Instruction, Program};

use std::fs::File;
use std::io::{Read, Result};

fn load_program(fname: String) -> Result<Program> {
    let mut file = File::open(fname)?;
    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents)?;
    let text = String::from_utf8(contents).unwrap();

    Ok(parser::parse(text))
}

fn main() {
    match load_program("examples/mandelbrot.bf".to_string()) {
        Ok(ref mut p) => {
            let p = opt::optimize(p);
            //println!("{:?}", p);
            interp::interp(&p, &mut std::io::stdin(), &mut std::io::stdout());
        }
        Err(err) => panic!("Cannot read file because {:?}", err),
    }
}
