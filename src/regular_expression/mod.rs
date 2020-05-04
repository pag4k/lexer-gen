extern crate alloc;
pub mod character_class;

use alloc::vec::Vec;
use character_class::*;

// Potential other ESCAPE_CHAR: {}$.\
pub const ESCAPE_CHAR: [char; 11] = ['*', '+', '?', '|', '^', '⋅', '(', ')', '[', ']', '\\'];

const CONCAT_CHAR: char = '⋅';
const NOT_AFTER: [char; 2] = ['(', '|'];
const NOT_BEFORE: [char; 5] = ['*', '+', '?', '|', ')'];

// In increasing order of precedence.
const OPERATORS: [char; 4] = ['|', CONCAT_CHAR, '?', '*'];
const PARENTHESIS: [char; 2] = ['(', ')'];

// TODO: Need some function to check validity.

pub fn regex<T: AsRef<[char]>>(input: T) -> Vec<char> {
    to_postfix(&add_explicit_concat(&replace_classes(&Vec::from(
        input.as_ref(),
    ))))
}

// FIXME: Assume there are no escape char in classes.
fn replace_classes<T: AsRef<[char]>>(input: T) -> Vec<char> {
    let mut output = Vec::from(input.as_ref());
    //dbg!(&output);
    let mut last_position = 0;
    while let Some(left_position) = output.iter().skip(last_position).position(|&c| c == '[') {
        let left_position = last_position + left_position;
        //dbg!(&left_position);
        // FIXME: Probably a better way to handle this increase.
        last_position = left_position + 1;
        if left_position != 0 && output[left_position - 1] == '\\' {
            continue;
        }
        while let Some(right_position) = output.iter().skip(left_position).position(|&c| c == ']') {
            let right_position = left_position + right_position;
            //dbg!(&right_position);
            if right_position != 0 && output[right_position - 1] == '\\' {
                continue;
            }
            let mut class: AsciiCharacterClass = Default::default();
            let mut position = left_position + 1;
            let negated = output[position] == '^';
            position += if negated { 1 } else { 0 };
            while position < right_position {
                //println!("Char {} at {}", output[position], position);

                let (start_char, end_char) = if position + 4 < right_position
                    && output[position] == '\\'
                    && output[position + 2] == '-'
                    && output[position + 3] == '\\'
                {
                    position += 5;
                    (output[position - 5 + 1], output[position - 5 + 4])
                } else if position + 3 < right_position
                    && output[position] == '\\'
                    && output[position + 2] == '-'
                    && output[position + 3] != '\\'
                {
                    position += 4;
                    (output[position - 4 + 1], output[position - 4 + 3])
                } else if position + 3 < right_position
                    && output[position] != '\\'
                    && output[position + 1] == '-'
                    && output[position + 2] == '\\'
                {
                    position += 4;
                    (output[position - 4], output[position - 4 + 3])
                } else if position + 2 < right_position
                    && output[position] != '\\'
                    && output[position + 1] == '-'
                    && output[position + 2] != '\\'
                {
                    position += 3;
                    (output[position - 3], output[position - 3 + 2])
                } else if position + 1 < right_position && output[position] == '\\' {
                    class.add(output[position + 1]);
                    position += 2;
                    continue;
                } else if position < right_position && output[position] != '\\' {
                    class.add(output[position]);
                    position += 1;
                    continue;
                } else {
                    panic!(
                        "Unsupported pattern starting at: {:?}.",
                        &output[position..]
                    );
                };

                if (end_char as u8) < (start_char as u8) {
                    // FIXME: Find a better way to propagate this error.
                    println!("ERROR '{}-{}': Range values reversed. Start char code is greater than end char code.", start_char, end_char);
                    // TODO: Not sure if I just want to take the last char.
                    class.add(end_char);
                } else {
                    let chars: Vec<char> = ((start_char as u8)..=(end_char as u8))
                        .map(|char| char as char)
                        .collect();
                    class.add_slice(&chars);
                };
            }
            let mut substring: Vec<char> = Vec::new();
            if negated {
                class.negate()
            };
            if !class.is_empty() {
                substring.push('(');
                //let zip: Vec<char> = class
                //    .to_array()
                //   .into_iter()
                //  .zip(['|'].into_iter().cycle().copied())
                // .flatten()
                //.collect();
                for c in class.to_array() {
                    if ESCAPE_CHAR.contains(&c) {
                        substring.push('\\');
                    }
                    substring.push(c);
                    substring.push('|');
                }
                substring.pop();
                substring.push(')');
            }
            output.splice(left_position..=right_position, substring.into_iter());
            //last_position = left_position + substring.len();
            //output = output.replace(
            //    output.get(left_position..=right_position).unwrap(),
            //    &String::from_iter(substring),
            //);
            //} else {
            //panic!("ERROR: Brackets should some in pairs.");
            break;
        }
    }
    //dbg!(output.clone().into_iter().collect::<String>());
    output
}

fn add_explicit_concat<T: AsRef<[char]>>(input: T) -> Vec<char> {
    let mut new_regex: Vec<char> = Vec::new();

    for (i, &c) in input.as_ref().iter().enumerate() {
        new_regex.push(c);

        // If next char is an ESCAPE_CHAR, continue to avoid adding a CONCAT_CHAR.
        if c == '\\' {
            if let Some(next_c) = input.as_ref().get(i + 1) {
                if ESCAPE_CHAR.contains(next_c) {
                    continue;
                }
            }
        }
        // Skip if current char is in NOT_AFTER.
        if NOT_AFTER.contains(&c) {
            continue;
        }
        // Get next char and skip if it is in NOT BEFORE.
        // If not, add the CONCAT_CHAR.
        if let Some(next_c) = input.as_ref().get(i + 1) {
            if NOT_BEFORE.contains(next_c) {
                continue;
            }
            new_regex.push(CONCAT_CHAR);
        }
    }

    //dbg!(new_regex.clone().into_iter().collect::<String>());
    new_regex
}

fn greater_precedence(first: char, second: char) -> bool {
    let mut found_first = false;

    // Same chars return true.
    for c in OPERATORS.iter() {
        if *c == first {
            found_first = true;
        }
        if *c == second {
            return found_first;
        }
    }

    false
}

// This function assumes that there are explicit concat?
fn to_postfix<T: AsRef<[char]>>(input: T) -> Vec<char> {
    let mut new_regex: Vec<char> = Vec::new();
    let mut operator_stack: Vec<char> = Vec::new();

    let mut escape_char = false;
    for &c in input.as_ref() {
        if escape_char && ESCAPE_CHAR.contains(&c) {
            escape_char = false;
            new_regex.push(c);
            continue;
        }
        if c == '\\' {
            escape_char = true;
        }
        if OPERATORS.contains(&c) {
            while !operator_stack.is_empty()
                && *operator_stack.last().unwrap() != '('
                && greater_precedence(c, *operator_stack.last().unwrap())
            {
                new_regex.push(operator_stack.pop().unwrap());
            }
            operator_stack.push(c);
        } else if PARENTHESIS.contains(&c) {
            if c == '(' {
                operator_stack.push(c);
            } else {
                while !operator_stack.is_empty() && *operator_stack.last().unwrap() != '(' {
                    new_regex.push(operator_stack.pop().unwrap());
                }
                operator_stack.pop();
            }
        } else {
            new_regex.push(c)
        }
    }

    // If there are operators left on the stack, add them in order of precedence.
    operator_stack.sort_by_cached_key(|a| OPERATORS.iter().find(|c| a == *c).unwrap());
    new_regex.append(&mut operator_stack);

    //dbg!(new_regex.clone().into_iter().collect::<String>());
    new_regex
}

#[cfg(test)]
mod tests {

    use crate::regular_expression::*;

    #[test]
    fn add_concat() {
        let regex1 = add_explicit_concat(&vec!['(', 'a', '|', 'b', ')', '*', 'c']);
        let regex2 = vec!['(', 'a', '|', 'b', ')', '*', CONCAT_CHAR, 'c'];
        assert_eq!(regex1, regex2);
    }

    #[test]
    fn replace_classes_letters() {
        let range: Vec<char> = "[a-e]".chars().collect();
        let replaced_range = replace_classes(range);
        assert_eq!(replaced_range, "(a|b|c|d|e)".chars().collect::<Vec<char>>());
    }

    #[test]
    fn replace_classes_lowercase_letters_reverse() {
        let range: Vec<char> = "[e-a]".chars().collect();
        let replaced_range = replace_classes(range);
        assert_ne!(replaced_range, "(a|b|c|d|e)".chars().collect::<Vec<char>>());
    }

    #[test]
    fn replace_classes_uppercase_letters() {
        let range: Vec<char> = "[A-E]".chars().collect();
        let replaced_range = replace_classes(range);
        assert_eq!(replaced_range, "(A|B|C|D|E)".chars().collect::<Vec<char>>());
    }

    #[test]
    fn replace_classes_numbers() {
        let range: Vec<char> = "[4-9]".chars().collect();
        let replaced_range = replace_classes(range);
        assert_eq!(
            replaced_range,
            "(4|5|6|7|8|9)".chars().collect::<Vec<char>>()
        );
    }

    #[test]
    fn postfix() {
        let before: Vec<char> = "(a|b)*⋅c".chars().collect();
        let after: Vec<char> = "ab|*c⋅".chars().collect();
        assert_eq!(to_postfix(before), after);
    }

    #[test]
    fn both() {
        let regex1 = to_postfix(&add_explicit_concat(&vec![
            '(', 'a', '|', 'b', ')', '*', 'c',
        ]));
        let regex2 = vec!['a', 'b', '|', '*', 'c', CONCAT_CHAR];
        assert_eq!(regex1, regex2);
    }

    #[test]
    fn replace_classes_mixed() {
        let range: Vec<char> = "[c-e8a4-5Z]".chars().collect();
        let replaced_range = replace_classes(range);
        assert_eq!(
            replaced_range,
            "(4|5|8|Z|a|c|d|e)".chars().collect::<Vec<char>>()
        );
    }

    // FIXME: Fix the code to make this test work.
    #[test]
    fn negate() {
        let range: Vec<char> = "[^\t-\\|]".chars().collect();
        let after = to_postfix(&add_explicit_concat(&replace_classes(&range)));
        assert_eq!(after, "}~|".chars().collect::<Vec<char>>());
    }
}
