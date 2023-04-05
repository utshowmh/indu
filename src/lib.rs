mod backend;
mod common;
mod frontend;

use std::{
    env::args,
    fs::read_to_string,
    io::{stdin, stdout, Write},
};

use backend::vm::VirtualMachine;
use common::error::{Error, ErrorKind};
use frontend::compiler::Compiler;

use crate::{
    backend::chunk::Chunk,
    common::ast::Program,
    frontend::{compiler::SymbolTable, parser::Parser, scanner::Scanner},
};

const COMMANDS: &str = "\
@cmd                    : prints available commands.
@exit, @e               : exit from the REPL.
";
const USAGE: &str = "\
Usage:
indu                        : run the REPL.
indu [file_path]    : execute given file.
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
            eprintln!("{USAGE}");
            Ok(())
        }
    }
}

fn run_file(source_path: &str) -> Result<(), Error> {
    if let Ok(source) = read_to_string(source_path) {
        run_source(&source).unwrap_or_else(|error| error.report());
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::System,
            format!("Could not read file from '{source_path}'."),
            None,
        ))
    }
}

fn run_source(source: &str) -> Result<(), Error> {
    let mut scanner = Scanner::new(&source);
    let tokens = scanner.scan()?;

    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;

    let mut compiler = Compiler::new(None);
    let chunk = compiler.compile(program)?;

    let mut vm = VirtualMachine::new();
    vm.interpret(chunk)?;

    Ok(())
}

fn run_repl() -> Result<(), Error> {
    println!("Welcome to Indu REPL.\nType '@cmd' to see available commands.\n");

    let mut line = String::new();
    let mut bindings = vec![SymbolTable::default()];

    loop {
        print!("|> ");
        stdout().flush().or(Err(Error::new(
            ErrorKind::System,
            "Could not flush 'stdout'.".to_string(),
            None,
        )))?;
        stdin().read_line(&mut line).or(Err(Error::new(
            ErrorKind::System,
            "Could not read line from 'stdin'.".to_string(),
            None,
        )))?;

        if line.starts_with('@') {
            match line.trim() {
                "@cmd" => print!("{COMMANDS}"),
                "@exit" | "@e" => {
                    println!("Exiting Indu REPL.");
                    break;
                }
                command => Error::new(
                    ErrorKind::System,
                    format!("Invalid command. '{command}' is not a known command."),
                    None,
                )
                .report(),
            }

            continue;
        }

        let mut scanner = Scanner::new(line.trim());
        let tokens = scanner.scan().unwrap_or_else(|error| {
            error.report();
            Vec::new()
        });

        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap_or_else(|error| {
            error.report();
            Program::new()
        });

        let mut compiler = Compiler::new(Some(bindings.clone()));
        let chunk = compiler.compile(program).unwrap_or_else(|error| {
            error.report();
            Chunk::new()
        });

        let mut vm = VirtualMachine::new();
        vm.interpret(chunk).unwrap_or_else(|error| {
            error.report();
        });

        line.clear();
        bindings = compiler.get_bindings();
    }

    Ok(())
}
