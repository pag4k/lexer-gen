extern crate alloc;
mod character_class;

use alloc::vec::Vec;
use character_class::*;

const CONCAT_CHAR: char = '⋅';
const NOT_AFTER: [char; 2] = ['(', '|'];
const NOT_BEFORE: [char; 4] = ['*', '?', '|', ')'];

// In increasing order of precedence.
const OPERATORS: [char; 4] = ['|', CONCAT_CHAR, '?', '*'];
const PARENTHESIS: [char; 2] = ['(', ')'];

// TODO: Need some function to check validity.
// TODO: Change + to *.
// TODO: Replace ?.

pub fn regex<T: AsRef<[char]>>(input: T) -> Vec<char> {
    to_postfix(&add_explicit_concat(&replace_classes(&Vec::from(
        input.as_ref(),
    ))))
}

fn replace_classes<T: AsRef<[char]>>(input: T) -> Vec<char> {
    let mut output = Vec::from(input.as_ref());
    while let Some(left_position) = output.iter().position(|&c| c == '[') {
        if let Some(right_position) = output.iter().position(|&c| c == ']') {
            let mut class: CharacterClass = Default::default();
            let mut position = left_position + 1;
            let negated = output[position] == '^';
            position += if negated { 1 } else { 0 };
            while position < right_position {
                //println!("Char {}", output[position]);
                if position + 2 < right_position && output[position + 1] == '-' {
                    let start_char = output[position];
                    let end_char = output[position + 2];
                    if (end_char as u8) < (start_char as u8) {
                        // FIXME: Find a better way to propagate this error.
                        println!("ERROR '{}-{}': Range values reversed. Start char code is greater than end char code.", start_char, end_char);
                        // TODO: Not sure if I just want to take the last char.
                        class.add(end_char);
                    } else if (start_char.is_numeric() && end_char.is_numeric())
                        || (start_char.is_lowercase() && end_char.is_lowercase())
                        || (start_char.is_uppercase() && end_char.is_uppercase())
                    {
                        let chars: Vec<char> = ((start_char as u8)..=(end_char as u8))
                            .map(|char| char as char)
                            .collect();
                        class.add_slice(&chars);
                    } else {
                        // FIXME: Not sure if I want this to be an error.
                        println!(
                            "ERROR '{}-{}': Characters of different type..",
                            start_char, end_char
                        );
                    };
                    position += 3;
                } else {
                    class.add(output[position]);
                    position += 1;
                }
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
                    substring.push(c);
                    substring.push('|');
                }
                substring.pop();
                substring.push(')');
            }
            output.splice(left_position..=right_position, substring.into_iter());
        //output = output.replace(
        //    output.get(left_position..=right_position).unwrap(),
        //    &String::from_iter(substring),
        //);
        } else {
            //panic!("ERROR: Brackets should some in pairs.");
        }
    }
    dbg!(output.clone().into_iter().collect::<String>());
    output
}

fn add_explicit_concat<T: AsRef<[char]>>(input: T) -> Vec<char> {
    let mut new_regex: Vec<char> = Vec::new();

    for (i, c) in input.as_ref().iter().enumerate() {
        new_regex.push(*c);

        // Skip if current char is in NOT_AFTER.
        if NOT_AFTER.contains(c) {
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

    dbg!(new_regex.clone().into_iter().collect::<String>());
    new_regex
}

/*
fn replace_question_mark<T: AsRef<[char]>>(input: T) -> Vec<char> {
    let mut new_regex: Vec<char> = Vec::new();

    let mut parenthesis_stack = Vec::new();
    let mut last_parenthesis = None;

    for (i, c) in input.as_ref().iter().enumerate() {
        new_regex.push(*c);

        match c {
            '(' => parenthesis_stack.push(i),
            ')' => {
                if parenthesis_stack.is_empty() {
                    // FIXME: Find better way to handle this.
                    panic!("Unclosed parenthesis.");
                }
                last_parenthesis = parenthesis_stack.pop();
            }
            '?' => match new_regex.get(i - 1) {
                Some(previous_char) => {
                    if *previous_char == ')' {
                        new_regex.insert(last_parenthesis.unwrap(), '(');
                    }
                }
                None => {
                    // FIXME: Handle this.
                    panic!("First char is +.")
                }
            },
            _ => {}
        }
    }

    new_regex
}
*/

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

    for c in input.as_ref() {
        if OPERATORS.contains(c) {
            while !operator_stack.is_empty()
                && *operator_stack.last().unwrap() != '('
                && greater_precedence(*c, *operator_stack.last().unwrap())
            {
                new_regex.push(operator_stack.pop().unwrap());
            }
            operator_stack.push(*c);
        } else if PARENTHESIS.contains(c) {
            if *c == '(' {
                operator_stack.push(*c);
            } else {
                while !operator_stack.is_empty() && *operator_stack.last().unwrap() != '(' {
                    new_regex.push(operator_stack.pop().unwrap());
                }
                operator_stack.pop();
            }
        } else {
            new_regex.push(*c)
        }
    }

    // If there are operators left on the stack, add them in order of precedence.
    operator_stack.sort_by_cached_key(|a| OPERATORS.iter().find(|c| a == *c).unwrap());
    new_regex.append(&mut operator_stack);

    dbg!(new_regex.clone().into_iter().collect::<String>());
    new_regex
}

#[cfg(test)]
mod tests {

    use crate::regular_expression::*;

    #[test]
    fn add_concat() {
        let regex1 = add_explicit_concat(&vec!['(', 'a', '|', 'b', ')', '*', 'c']);
        let regex2 = vec!['(', 'a', '|', 'b', ')', '*', CONCAT_CHAR, 'c'];
        //assert!(equal(&regex1, &regex2));
        //assert_eq!(regex1, regex2);
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
}
