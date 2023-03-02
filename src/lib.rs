mod common;
mod frontend;

use std::{
    env::args,
    fs::read_to_string,
    io::{stdin, stdout, Write},
    process::exit,
};

use crate::frontend::scanner::Scanner;

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
        error.report(&source);
        exit(65);
    });
    println!("{:#?}", tokens);
}

fn run_repl() {
    println!("Indu REPL. Press [Ctrl] + [c] to exit.\n");

    loop {
        print!("indu :> ");
        stdout().flush().expect("ERROR: Could not flush stdout.");
        let mut line = String::new();
        stdin()
            .read_line(&mut line)
            .expect("ERROR: Could not read line from stdin.");
        let mut scanner = Scanner::new(line.trim());
        let tokens = scanner.scan().unwrap_or_else(|error| {
            error.report(&line);
            Vec::new()
        });
        println!("{:#?}", tokens);
    }
}
