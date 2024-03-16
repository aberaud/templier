use std::env;
use std::io::{self, Write};

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().skip(1).collect();

    let reader = io::stdin();
    let stdout = io::stdout();
    let mut writer = stdout.lock();
    let mut found_indexed_tokens = false;
    let mut last_arg = 0; // index of the last argument used if there are no indexed tokens

    for line in reader.lines() {
        let mut output = line.expect("Failed to read line from stdin");
        let mut last_end = 0;
        while let Some(start) = output[last_end..].find('{') {
            let start = start + last_end;
            let end = start + 1;
            if start > 0 && &output[start - 1..start] == "\\" {
                last_end = end;
                continue;
            }
            let end = output[start..].find('}').expect("Error: found unescaped '{' but no matching '}'") + start;
            let token = &output[start + 1..end];
            if token.is_empty() {
                if found_indexed_tokens {
                    return Err("Found both indexed and non-indexed tokens");
                }
                if last_arg >= args.len() {
                    return Err("Not enough arguments to replace all tokens");
                }
                output.replace_range(start..end + 1, &args[last_arg]);
                last_end = start + args[last_arg].len();
                last_arg += 1;
            }
            else if token.parse::<usize>().is_ok() {
                if last_arg != 0 {
                    return Err("Found both indexed and non-indexed tokens");
                }
                found_indexed_tokens = true;
                let index = token.parse::<usize>().unwrap();
                if index >= args.len() {
                    return Err("Index out of range");
                }
                output.replace_range(start..end + 1, &args[index]);
                last_end = start + args[index].len();
            } else {
                return Err("Invalid token");
            }
        }
        
        output = output.replace("\\{", "{")
                        .replace("\\}", "}")
                        .replace("\\\\", "\\");
        writer.write_all(output.as_bytes()).expect("Failed to write to stdout");
        writer.write_all(b"\n").expect("Failed to write to stdout");
    }
    if last_arg != 0 && last_arg < args.len() - 1 {
        return Err("Too few tokens to replace all arguments");
    }
    Ok(())
}
