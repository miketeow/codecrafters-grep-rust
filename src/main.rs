use std::env;
use std::io;
use std::process;

// fn match_pattern(input_line: &str, pattern: &str) -> bool {
//     if pattern == "\\d" {
//         for c in input_line.chars() {
//             if c.is_digit(10) {
//                 return true;
//             }
//         }
//         return false;
//     } else if pattern == "\\w" {
//         for c in input_line.chars() {
//             if c.is_alphanumeric() {
//                 return true;
//             }
//         }
//         return false;
//     } else if pattern.starts_with("[") && pattern.ends_with("]") {
//         if pattern.starts_with("[^") {
//             if pattern.len() > 3 {
//                 let negative_chars = &pattern[2..pattern.len() - 1];

//                 if negative_chars.is_empty() {
//                     return false;
//                 }

//                 for c in input_line.chars() {
//                     if !negative_chars.contains(c) {
//                         return true;
//                     }
//                 }
//                 return false;
//             }
//         }
//         if pattern.len() > 2 {
//             let positive_chars = &pattern[1..pattern.len() - 1];

//             if positive_chars.is_empty() {
//                 return false;
//             }
//             for c in input_line.chars() {
//                 if positive_chars.contains(c) {
//                     return true;
//                 }
//             }
//             return false;
//         } else {
//             return false;
//         }
//     } else if pattern.chars().count() == 1 {
//         return input_line.contains(pattern);
//     } else {
//         // panic!("Unhandled pattern: {}", pattern)
//         return false;
//     }
// }

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    if pattern.starts_with("^") {
        return match_pattern_recursive(input_line, &pattern[1..]);
    } else {
        for i in 0..=input_line.len() {
            if match_pattern_recursive(&input_line[i..], pattern) {
                return true;
            }
        }
        return false;
    }
}

fn match_pattern_recursive(input_line: &str, pattern: &str) -> bool {
    if pattern.is_empty() {
        return true;
    }

    if pattern == "$" {
        return input_line.is_empty();
    };

    if pattern.starts_with("\\") {
        if pattern.len() < 2 {
            return false;
        }

        let class_char = pattern.chars().nth(1).unwrap();

        if input_line.is_empty() {
            return false;
        }

        let input_char = input_line.chars().next().unwrap();
        let input_char_len = input_char.len_utf8();

        let matched_class = match class_char {
            'd' => input_char.is_ascii_digit(),
            'w' => input_char.is_ascii_alphanumeric(),
            _ => return false,
        };

        if matched_class {
            return match_pattern_recursive(&input_line[input_char_len..], &pattern[2..]);
        } else {
            return false;
        }
    }

    if input_line.is_empty() {
        return false;
    }

    let p_chars = pattern.chars().next().unwrap();
    let i_chars = input_line.chars().next().unwrap();
    let p_chars_len = p_chars.len_utf8();
    let i_chars_len = i_chars.len_utf8();

    if p_chars == i_chars {
        return match_pattern_recursive(&input_line[i_chars_len..], &pattern[p_chars_len..]);
    }
    return false;
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
        println!("pattern matched");
        process::exit(0)
    } else {
        println!("pattern not matched");
        process::exit(1)
    }
}
