mod common;
mod frontend;
mod runtime;

use std::{
    env::args,
    fs::read_to_string,
    io::{stdin, stdout, Write},
    process::exit,
};

use runtime::environment::Environment;

use crate::{
    frontend::{parser::Parser, scanner::Scanner},
    runtime::interpreter::Interpreter,
};

pub fn run() {
    let args: Vec<String> = args().collect();

    if args.len() > 2 {
        eprintln!("Usage: indu [script]");
        exit(64);
    } else if args.len() == 2 {
        let source_path = &args[1];
        run_file(source_path);
    } else {
        run_repl();
    }
}

fn run_file(source_path: &str) {
    let source =
        read_to_string(source_path).expect(&format!("ERROR: Could not read file {}.", source_path));

    let mut scanner = Scanner::new(&source);
    let tokens = scanner.scan().unwrap_or_else(|error| {
        error.report();
        exit(65);
    });

    let mut parser = Parser::new(tokens);
    let expression = parser.parse().unwrap_or_else(|error| {
        error.report();
        exit(65);
    });

    let mut interpreter = Interpreter::new(Environment::new());
    interpreter.interpret(expression).unwrap_or_else(|error| {
        error.report();
        exit(65);
    });
}

fn run_repl() {
    println!("Welcome to Indu REPL. Press Ctrl-C to exit.\n");

    let mut environment = Environment::new();

    loop {
        print!("indu :> ");
        stdout().flush().expect("ERROR: Could not flush stdout.");
        let mut line = String::new();
        stdin()
            .read_line(&mut line)
            .expect("ERROR: Could not read line from stdin.");

        let mut scanner = Scanner::new(line.trim());
        let tokens = scanner.scan().unwrap_or_else(|error| {
            error.report();
            Vec::new()
        });

        let mut parser = Parser::new(tokens);
        let expression = parser.parse().unwrap_or_else(|error| {
            error.report();
            Vec::new()
        });

        let mut interpreter = Interpreter::new(environment.clone());
        interpreter.interpret(expression).unwrap_or_else(|error| {
            error.report();
            ()
        });

        environment = interpreter.environment.clone();
    }
}
