use std::{
    env,
    fs,
    io::{self, Read, Write},
};

fn interpret(code: &str) -> io::Result<()> {
    let mut cells = vec![0u8; 30000];
    let mut ptr = 0usize;
    let mut pc = 0usize;
    let code_chars: Vec<char> = code.chars().collect();

    // precompute jump table for loops
    let mut jump_forward = std::collections::HashMap::new();
    let mut jump_backward = std::collections::HashMap::new();
    let mut stack = Vec::new();

    for (i, &c) in code_chars.iter().enumerate() {
        if c == '[' {
            stack.push(i);
        } else if c == ']' {
            if let Some(start) = stack.pop() {
                jump_forward.insert(start, i);
                jump_backward.insert(i, start);
            } else {
                eprintln!("ERROR - Unmatched closing bracket at position {}", i);
                return Ok(());
            }
        }
    }

    if !stack.is_empty() {
        eprintln!("ERROR - Unmatched opening bracket at position {}", stack.pop().unwrap());
        return Ok(());
    }

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = stdin.lock();
    let mut input_buffer = [0u8; 1];

    while pc < code_chars.len() {
        match code_chars[pc] {
            '>' => {
                ptr += 1;
                if ptr >= cells.len() {
                    cells.push(0);
                }
            }
            '<' => {
                if ptr == 0 {
                    eprintln!("ERROR - Pointer moved left at zero position");
                    return Ok(());
                }
                ptr -= 1;
            }
            '+' => cells[ptr] = cells[ptr].wrapping_add(1),
            '-' => cells[ptr] = cells[ptr].wrapping_sub(1),
            '.' => {
                stdout.write_all(&[cells[ptr]])?;
                stdout.flush()?;
            }
            ',' => {
                if input.read(&mut input_buffer)? == 0 {
                    cells[ptr] = 0; // EOF
                } else {
                    cells[ptr] = input_buffer[0];
                }
            }
            '[' => {
                if cells[ptr] == 0 {
                    pc = *jump_forward.get(&pc).unwrap();
                }
            }
            ']' => {
                if cells[ptr] != 0 {
                    pc = *jump_backward.get(&pc).unwrap();
                }
            }
            _ => {} // ignore other chars
        }
        pc += 1;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file.bf>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    if !filename.ends_with(".bf") {
        eprintln!("Input file must have .bf extension");
        std::process::exit(1);
    }

    let code = fs::read_to_string(filename)?;
    interpret(&code)?;

    Ok(())
}
