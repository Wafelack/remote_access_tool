mod lexer;
mod parser;
mod tests;

use lexer::{Lexer, TType, Token};

fn main() {
    println!("Hello, world!");
}

pub type VLispResult<T> = std::result::Result<T, Vec<String>>;
