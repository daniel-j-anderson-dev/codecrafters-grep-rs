use std::env;
use std::io;
use std::io::Write;
use std::process;

fn match_pattern(input: &str, pattern: &str) -> bool {
    match pattern {
        pattern if pattern.len() == 1 => {
            return input.contains(pattern);
        },
        r"\d" => {
            for c in input.chars() {
                if c.is_digit(10) {
                    return true;
                }
            }
            return false;
        },
        _ => panic!("Unhandled pattern: {}", pattern),
    }
}

// Usage: echo <input_text> | your_grep.sh -E <pattern>
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().collect::<Vec<_>>();

    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    if args.get(1).ok_or("Missing flag argument")? != "-E" {
        return Err("Expected first argument to be '-E'".into());
    }
    
    let pattern = args.get(2).ok_or("Missing pattern argument")?;

    let input = get_input("")?;

    // Uncomment this block to pass the first stage
    if match_pattern(&input, &pattern) {
        Ok(())
    } else {
        Err("pattern not in input".into())
    }
}

pub fn get_input(prompt: &str) -> Result<String, io::Error> {
    io::stdout().write(prompt.as_bytes())?;
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_owned())
}