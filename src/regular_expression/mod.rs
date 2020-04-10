extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use core::iter::FromIterator;
use core::str;

pub type Regex = Vec<char>;

const CONCAT_CHAR: char = '⋅';
const NOT_AFTER: [char; 2] = ['(', '|'];
const NOT_BEFORE: [char; 4] = ['*', '?', '|', ')'];

// In increasing order of precedence.
const OPERATORS: [char; 4] = ['|', CONCAT_CHAR, '?', '*'];
const PARENTHESIS: [char; 2] = ['(', ')'];

// TODO: Need some function to check validity.
// TODO: Change + to *.

pub fn Regex(string: &str) -> Regex {
    to_postfix(&add_explicit_concat(&Vec::from_iter(string.chars())))
}

pub fn replace_range(string: &str) -> String {
    let mut new_string = String::from(string);
    while let Some(left_position) = new_string.find('[') {
        if let Some(right_position) = new_string.find(']') {
            //assert!((right_position - left_position) % 3 == 0);
            let mut position = left_position + 1;
            let mut chars = Vec::new();
            while position < right_position {
                let start_char = new_string.chars().nth(position).unwrap();
                let end_char = new_string.chars().nth(position + 2).unwrap();
                let mut new_chars = if (start_char.is_numeric() && end_char.is_numeric())
                    || (start_char.is_lowercase() && end_char.is_lowercase())
                    || (start_char.is_uppercase() && end_char.is_uppercase())
                {
                    //assert!(start_char as usize <= end_char as usize);
                    ((start_char as u8)..=(end_char as u8))
                        .map(|char| char as char)
                        .collect()
                } else {
                    Vec::new()
                };
                chars.append(&mut new_chars);
                position += 3;
            }
            let mut substring: Vec<char> = ['('].to_vec();
            for c in chars {
                substring.push(c);
                substring.push('|');
            }
            substring.pop();
            substring.push(')');
            new_string = new_string.replace(
                new_string.get(left_position..=right_position).unwrap(),
                &String::from_iter(substring),
            );
        } else {
            //panic!("ERROR: Brackets should some in pairs.");
        }
    }
    new_string
}

pub fn add_explicit_concat(regex: &Regex) -> Regex {
    let mut new_regex: Regex = Vec::new();

    for (i, c) in regex.iter().enumerate() {
        new_regex.push(*c);

        // Skip if current char is in NOT_AFTER.
        if NOT_AFTER.contains(c) {
            continue;
        }
        // Get next char and skip if it is in NOT BEFORE.
        // If not, add the CONCAT_CHAR.
        if let Some(next_c) = regex.get(i + 1) {
            if NOT_BEFORE.contains(next_c) {
                continue;
            }
            new_regex.push(CONCAT_CHAR);
        }
    }

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

fn to_postfix(regex: &Regex) -> Regex {
    let mut new_regex: Regex = Vec::new();
    let mut operator_stack: Vec<char> = Vec::new();

    for c in regex {
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

    new_regex
}

#[cfg(test)]
mod tests {

    use crate::regular_expression::*;

    #[test]
    fn add_concat() {
        let regex1: Regex = add_explicit_concat(&vec!['(', 'a', '|', 'b', ')', '*', 'c']);
        let regex2: Regex = vec!['(', 'a', '|', 'b', ')', '*', CONCAT_CHAR, 'c'];
        //assert!(equal(&regex1, &regex2));
        //assert_eq!(regex1, regex2);
    }

    #[test]
    fn both() {
        let regex1: Regex = to_postfix(&add_explicit_concat(&vec![
            '(', 'a', '|', 'b', ')', '*', 'c',
        ]));
        let regex2 = vec!['a', 'b', '|', '*', 'c', CONCAT_CHAR];
        //assert!(equal(&regex1, &regex2));
        //assert_eq!(regex1, regex2);
    }
}
