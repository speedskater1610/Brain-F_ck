# Brainf*ck Interpreter in Rust

This is a simple yet robust BF interpreter written in [Rust](https://www.rust-lang.org/). It reads `.bf` files, interprets their contents, and executes the Brainfuck code directly in your terminal.

## Features

- Fully supports the Brainfuck language (`>`, `<`, `+`, `-`, `.`, `,`, `[`, `]`)
- bounds checking with clear error messages
- handles unmatched brackets
- uses a jump table for efficient loop execution
- automatically expands the memory tape as needed (why do you need more than 30,000 registors?)
- uses with stdin and stdout

## Usage

### 1. clone

make sure to clone the repo before using or just download the only file (main.bf)

```git
git clone https://github.com/speedskater1610/Brain-F_ck/
```

### 2. Compile

Make sure you have Rust installed. If not, get it from [https://rustup.rs](https://rustup.rs).

```bash
cargo build --release
