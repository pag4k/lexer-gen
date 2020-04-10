extern crate alloc;

use alloc::vec::Vec;

const CONCAT_CHAR: char = '⋅';
const NOT_AFTER: [char; 2] = ['(', '|'];
const NOT_BEFORE: [char; 4] = ['*', '?', '|', ')'];

// In increasing order of precedence.
const OPERATORS: [char; 4] = ['|', CONCAT_CHAR, '?', '*'];
const PARENTHESIS: [char; 2] = ['(', ')'];

// TODO: Need some function to check validity.
// TODO: Change + to *.

pub fn regex<T: AsRef<[char]>>(input: T) -> Vec<char> {
    to_postfix(&add_explicit_concat(&replace_range(&Vec::from(
        input.as_ref(),
    ))))
}

fn replace_range<T: AsRef<[char]>>(input: T) -> Vec<char> {
    let mut output = Vec::from(input.as_ref());
    while let Some(left_position) = output.iter().position(|&c| c == '[') {
        if let Some(right_position) = output.iter().position(|&c| c == ']') {
            //assert!((right_position - left_position) % 3 == 0);
            let mut position = left_position + 1;
            let mut chars = Vec::new();
            while position < right_position {
                let start_char = *output.get(position).unwrap();
                let end_char = *output.get(position + 2).unwrap();
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
            output.splice(left_position..=right_position, substring.into_iter());
        //output = output.replace(
        //    output.get(left_position..=right_position).unwrap(),
        //    &String::from_iter(substring),
        //);
        } else {
            //panic!("ERROR: Brackets should some in pairs.");
        }
    }
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
    fn replace_range_letters() {
        let range: Vec<char> = "[a-e]".chars().collect();
        let replaced_range = replace_range(range);
        assert_eq!(replaced_range, "(a|b|c|d|e)".chars().collect::<Vec<char>>());
    }

    #[test]
    fn replace_range_letters_reverse() {
        let range: Vec<char> = "[e-a]".chars().collect();
        let replaced_range = replace_range(range);
        assert_ne!(replaced_range, "(a|b|c|d|e)".chars().collect::<Vec<char>>());
    }

    #[test]
    fn replace_range_numbers() {
        let range: Vec<char> = "[4-9]".chars().collect();
        let replaced_range = replace_range(range);
        assert_eq!(
            replaced_range,
            "(4|5|6|7|8|9)".chars().collect::<Vec<char>>()
        );
    }

    #[test]
    fn both() {
        let regex1 = to_postfix(&add_explicit_concat(&vec![
            '(', 'a', '|', 'b', ')', '*', 'c',
        ]));
        let regex2 = vec!['a', 'b', '|', '*', 'c', CONCAT_CHAR];
        //assert!(equal(&regex1, &regex2));
        //assert_eq!(regex1, regex2);
    }
}
