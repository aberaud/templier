use std::env;
use std::io::{self, Read, Write};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    // Reading standard input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read from stdin");

    // Check if input contains {n} tokens and replace them if present
    let mut output = input.clone();
    let mut found_indexed_tokens = false;

    for (i, arg) in args.iter().enumerate() {
        let token = format!("{{{}}}", i);
        let mut last_end = 0;
        while let Some(start) = output[last_end..].find(&token) {
            found_indexed_tokens = true;
            let start = start + last_end;
            let end = start + token.len();
            output.replace_range(start..end, arg);
            last_end = start + arg.len();
        }
    }

    // If no {n} tokens were found but there are {} tokens, replace them in order
    let mut last_end = 0;
    for arg in args.iter() {
        if let Some(start) = output[last_end..].find("{}") {
            if found_indexed_tokens {
                eprintln!("Error: found both indexed and non-indexed tokens");
                std::process::exit(1);
            }        
            let start = start + last_end;
            let end = start + 2;
            output.replace_range(start..end, arg);
            last_end = start + arg.len();
        } else if found_indexed_tokens {
            break;
        } else {
            eprintln!("Error: too many arguments to replace all tokens");
            std::process::exit(1);
        }
    }
    if output[last_end..].contains("{}") {
        eprintln!("Error: not enough arguments to replace all tokens");
        std::process::exit(1);
    }

    // Replace escaped { and } with their unescaped versions as well as \\ with \
    output = output.replace("\\{", "{")
                   .replace("\\}", "}")
                   .replace("\\\\", "\\");

    // Printing the result to standard output
    io::stdout().write_all(output.as_bytes()).unwrap();
}