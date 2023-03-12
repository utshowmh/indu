mod common;
mod frontend;
mod runtime;

use std::{
    env::args,
    fs::read_to_string,
    io::{stdin, stdout, Write},
};

use common::error::{Error, ErrorKind};
use runtime::environment::Environment;

use crate::{
    frontend::{parser::Parser, scanner::Scanner},
    runtime::interpreter::Interpreter,
};

const COMMANDS: &str = "\
#cmd        ->  prints available commands.
#env        ->  shows environment (variable bindings).
";

pub fn start() {
    run().unwrap_or_else(|error| error.report());
}

fn run() -> Result<(), Error> {
    let args: Vec<String> = args().collect();

    match args.len() {
        1 => run_repl(),
        2 => run_file(&args[1]),
        _ => {
            eprintln!("Usage: indu [path_to_source]");
            Ok(())
        }
    }
}

fn run_file(source_path: &str) -> Result<(), Error> {
    if let Ok(source) = read_to_string(source_path) {
        let mut scanner = Scanner::new(&source);
        let tokens = scanner.scan()?;

        let mut parser = Parser::new(tokens);
        let expression = parser.parse()?;

        let mut interpreter = Interpreter::new(Environment::new());
        interpreter.interpret(expression)?;

        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::SystemError,
            format!("Could not read file `{source_path}`"),
            None,
        ))
    }
}

fn run_repl() -> Result<(), Error> {
    println!("Welcome to Indu REPL. Type  `#cmd` to see available commands.\n");

    let mut environment = Environment::new();

    loop {
        print!("Indu :> ");
        stdout().flush().or(Err(Error::new(
            ErrorKind::SystemError,
            "Could not flush stdout".to_string(),
            None,
        )))?;
        let mut line = String::new();
        stdin().read_line(&mut line).or(Err(Error::new(
            ErrorKind::SystemError,
            "Could not read from stdin".to_string(),
            None,
        )))?;
        let line = line.trim();

        if line.starts_with('#') {
            match line {
                "#cmd" => print!("{COMMANDS}"),
                "#env" => print!("{environment}"),
                "#exit" => {
                    println!("Exiting Indu REPL.");
                    break;
                }
                _ => {
                    Error::new(ErrorKind::SystemError, "Unknown command".to_string(), None).report()
                }
            }

            continue;
        }

        let mut scanner = Scanner::new(line);
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
        });

        environment = interpreter.environment.clone();
    }

    Ok(())
}
