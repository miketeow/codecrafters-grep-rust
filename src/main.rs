use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    if pattern == "\\d" {
        for c in input_line.chars() {
            if c.is_digit(10) {
                return true;
            }
        }
        return false;
    } else if pattern == "\\w" {
        for c in input_line.chars() {
            if c.is_alphanumeric() {
                return true;
            }
        }
        return false;
    } else if pattern.starts_with("[") && pattern.ends_with("]") {
        if pattern.starts_with("[^") {
            if pattern.len() > 3 {
                let negative_chars = &pattern[2..pattern.len() - 1];

                if negative_chars.is_empty() {
                    return false;
                }
                for c in input_line.chars() {
                    if negative_chars.contains(c) {
                        return false;
                    }
                }
                return true;
            }
        }
        if pattern.len() > 2 {
            let positive_chars = &pattern[1..pattern.len() - 1];

            if positive_chars.is_empty() {
                return false;
            }
            for c in input_line.chars() {
                if positive_chars.contains(c) {
                    return true;
                }
            }
            return false;
        } else {
            return false;
        }
    } else if pattern.chars().count() == 1 {
        return input_line.contains(pattern);
    } else {
        // panic!("Unhandled pattern: {}", pattern)
        return false;
    }
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // eprintln!("Logs from your program will appear here!");

    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    // Uncomment this block to pass the first stage
    if match_pattern(&input_line, &pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
