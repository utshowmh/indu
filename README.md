<p align="center">
   <img src="https://i.ibb.co/9YJk4bP/indu.png" alt="indu" border="0" width="50%">
</p>

<h1 align="center">
The Indu Programming Language
</h1>

## What is it

Indu (means moon in bangla) is a dynamically typed scripting language that is cleaner in syntax and simple to write program in.

## How is it implimented

It has a [scanner](https://en.wikipedia.org/wiki/Lexical_analysis) (see the source [here](https://github.com/utshowmh/indu/blob/main/src/frontend/scanner.rs)) that takes source code as string and converts it into a vector (dynamic array) of tokens (see the source [here](https://github.com/utshowmh/indu/blob/main/src/common/token.rs)). The [parser](https://en.wikipedia.org/wiki/Parsing) (see the source [here](https://github.com/utshowmh/indu/blob/main/src/frontend/parser.rs)) takes the vector of token and turns it into an [AST](https://en.wikipedia.org/wiki/Abstract_syntax_tree). Then the [compiler](https://en.wikipedia.org/wiki/Compiler) (see the source [here](https://github.com/utshowmh/indu/blob/main/src/frontent/compiler.rs)) walks the given AST and emmits bytecode for a [virtual machine](https://en.wikipedia.org/wiki/Virtual_machine#Process_virtual_machines) (see the source [here](https://github.com/utshowmh/indu/blob/main/src/backend/vm.rs)). And finally then the virtual machine executes generated instructions.

## Using It

After building the project, run `indu` for a repl or provide source_path (like `indu ./source_path.in`) to run a script.

## Why in Rust

Why not?

## MSRV (Minimum supported Rust version)

1.67.0

## Building from source

To build Indu from source you need to haver Rust installed in your system. You can find instruction about installing Rust in [here](https://www.rust-lang.org/tools/install). Once you have Rust, you can build indu with cargo by and running: `cargo build --release`
