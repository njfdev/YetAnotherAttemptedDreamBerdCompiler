use std::env;

mod preprocessor;
mod lexer;
mod parser;
mod interpreter;

fn main() {
    let args: Vec<String> = env::args().collect();

    // There should only be two arguments, the first one is the name of the program and the second one is the file name
    if args.len() != 2 {
        eprintln!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];

    let contents = std::fs::read_to_string(filename).expect("Something went wrong reading the file");

    let preprocessed = preprocessor::preprocess(&contents);

    let s_lexer = lexer::Lexer::new(preprocessed);
    
    let mut s_parser = parser::Parser::new(s_lexer);
    let ast = s_parser.parse();

    let interpreter = interpreter::Interpreter::new();
    interpreter.interpret(ast);
}
