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
      println!("The input line now is: {}", input_line);
        return input_line.is_empty();
    };

    if input_line.is_empty() {
        return false;
    }

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

    if pattern.len() > 2 && pattern.chars().nth(1) == Some('+'){
      let token_chars = pattern.chars().next().unwrap(); // The X from X+
      let pattern_after_plus = &pattern[2..]; // pattern after X+

      // must match at least one. a == a, in the case of aaabc against a+bc
      if input_line.chars().next() != Some(token_chars) {
        return false;
      }

      // example aabc
      let input_after_one = &input_line[token_chars.len_utf8()..];

      return match_pattern_recursive(input_after_one, pattern_after_plus) || match_pattern_recursive(input_after_one, pattern);
    }

    if pattern.starts_with("["){
      if let Some(end_bracket_idx) = pattern.find("]") {
        let group_classes = &pattern[1..end_bracket_idx];
        let other_pattern = &pattern[end_bracket_idx + 1..];

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

    println!("Check if reach normal compare here");

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

// helper function

fn match_leading_token<'a>(pattern_token_segment: &str, input_line: &'a str) -> Option<&'a str> {
  if pattern_token_segment.starts_with("\\") {
    if pattern_token_segment.len() < 2 || input_line.is_empty() {
      return None
    }

    let class_char = pattern_token_segment.chars().nth(1).unwrap();
    let input_char = input_line.chars().next().unwrap();
    let matched_class = match class_char {
      'd' => input_char.is_ascii_digit(),
      'w' => input_char.is_ascii_alphanumeric() || input_char == '_',
      _ => false,
    };
    if matched_class {
      return Some(&input_line[input_char.len_utf8()..]);
    }
  } else if pattern_token_segment.starts_with("[") {
    if let Some(end_bracket_idx) = pattern_token_segment.find(']') {
      if end_bracket_idx + 1 != pattern_token_segment.len() {
        return None;
      }

      let group_classes = &pattern_token_segment[1..end_bracket_idx];
      if input_line.is_empty() {
        return None;
      }
      let current_char = input_line.chars().next().unwrap();
      let is_negative_group = group_classes.starts_with('^');
      let chars_in_group = if is_negative_group {
        &group_classes[1..]
      } else {group_classes};

      // Guard against empty char group like "[]" ot "[^]"
      if chars_in_group.is_empty() && pattern_token_segment.len() > 2 {

      }

      let matched_char_flag = chars_in_group.contains(current_char);
      let group_matches_current_char = if is_negative_group {!matched_char_flag} else {matched_char_flag};

      if group_matches_current_char {
        return Some(&input_line[current_char.len_utf8()..]);
      }
    }
  } else if !pattern_token_segment.is_empty() && !input_line.is_empty() {
    let p_char = pattern_token_segment.chars().next().unwrap();
    let i_char = input_line.chars().next().unwrap();

    if p_char == i_char {
      return Some(&input_line[i_char.len_utf8()..]);
    }
  }
  None
}

fn handle_star_quantifier<'a>(token_x_definition: &str, pattern_after_quantifier: &str, current_input: &'a str) -> bool {
  // Option 1, X is matched zero time, proceed to match the rest of the pattern.
  if match_pattern_recursive(current_input, pattern_after_quantifier){
    return true;
  }

  // Option 2, X is matched one or more times, then recurse.
  if let Some(input_after_one_more_x) = match_leading_token(token_x_definition, current_input){
    // If X is matched, recursively call this function for the * part
    if handle_star_quantifier(token_x_definition, pattern_after_quantifier, input_after_one_more_x){
      return true;
    }
  }
  false
}
// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    if match_pattern(&input_line, &pattern) {
        println!("pattern matched");
        process::exit(0)
    } else {
        println!("pattern not matched");
        process::exit(1)
    }
}
