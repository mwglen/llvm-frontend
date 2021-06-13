mod parser;
mod codegen;
mod lexer;
mod preprocessor;
mod ir_compiler;

use clap::{load_yaml, App, AppSettings};
use std::path::Path;
use parser::parse;
use codegen::generate;
use preprocessor::process;
use lexer::analyze;
use ir_compiler::compile;

fn main() {
    // Parse the cli arguments using clap
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml)
        .setting(AppSettings::ArgRequiredElseHelp);
    let matches = app.get_matches();

    // Get the source code to compile
    let in_file = matches.value_of("cfg").unwrap();
    let code = std::fs::read_to_string(in_file).unwrap();
   
    // Compile the source code
    let code = process(code);
    let tokens = analyze(code);
    let tree = parse(tokens);
    let ir_code = generate(tree);
    let binary = compile(ir_code);

    // Write the binary to a file
    let out_file = in_file.to_owned() + ".out";
    std::fs::write(out_file, binary).unwrap();
}
