use std::env;
use std::io;
use std::process;

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

    if pattern.starts_with("["){
      if let Some(end_bracket_idx) = pattern.find("]") {
        let group_classes = &pattern[1..end_bracket_idx];
        let other_pattern = &pattern[end_bracket_idx + 1..];

        if input_line.is_empty() {
          return false;
        }

        let current_char = input_line.chars().next().unwrap();
        // handle negative group
        let is_negative_group = group_classes.starts_with("^");
        let chars_in_group = if is_negative_group {
           &group_classes[1..]
        } else {
           group_classes
        };

        let matched_chars = chars_in_group.contains(current_char);

        let group_matched_chars = if is_negative_group {
          !matched_chars
        } else {
          matched_chars
        };

        if group_matched_chars {
          return match_pattern_recursive(&input_line[current_char.len_utf8()..], other_pattern);
        } else {
          return false;
        }
      } else {
        // Unclosed bracket
        return false;
      }
    }
    if input_line.is_empty() {
        return false;
    }

    let pattern_chars = pattern.chars().next().unwrap();
    let input_chars = input_line.chars().next().unwrap();
    let pattern_chars_len = pattern_chars.len_utf8();
    let input_chars_len = input_chars.len_utf8();

    if pattern_chars == input_chars {
        return match_pattern_recursive(
            &input_line[input_chars_len..],
            &pattern[pattern_chars_len..],
        );
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
