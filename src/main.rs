mod ast_builder;
mod builder;
mod tokenizer;
mod utils;

use ast_builder::build_ast;
use builder::generate_rust_code;
use tokenizer::tokenize;
use utils::{read_file, write_file};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = "test/test.vrt";
    let input = read_file(filename)?;

    let tokens = tokenize(&input)?;
    // dbg!(&tokens);
    let ast = build_ast(&tokens);
    // dbg!(&ast);

    let rust_code = generate_rust_code(&ast);
    write_file(rust_code.as_str())?;

    Ok(())
}
