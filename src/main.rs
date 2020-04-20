extern crate alloc;

mod dot_generator;
mod finite_automaton;
mod regular_expression;

use std::fs::File;
use std::io::prelude::*;

//use alloc::collections::btree_map::BTreeMap;
use alloc::vec::Vec;

pub const SIGMA: [char; 87] = [
    'Σ', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
    'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A',
    'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T',
    'U', 'V', 'W', 'X', 'Y', 'Z', '!', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=',
    '>', '[', ']', '_', '{', '}', '&', '|', ' ', '\t', '\n',
];

fn main() {
    //let alphabet = vec!['a', 'b', 'c'];
    //let regex1 = regular_expression::regex(vec!['a', 'b']);
    //let regex2 = regular_expression::regex(vec!['a', 'b', 'b']);
    let integer_string = String::from("(([1-2][0-2]*)|0)");
    let float_string =
        String::from("(([1-2][0-2]*)|0).(([0-2]*[1-2])|0)((e(a|b)?)(([1-2][0-2]*)|0))?");
    //"(.(({digit}*{nonzero})|0))".to_string(),
    //let float_string = String::from("{integer}{fraction}(e(+|-){integer})");
    let strings = vec![integer_string, float_string];
    //replace_symbolic_name(&mut strings, get_symbolic_names());
    //dbg!(&strings[0]);
    let integer_regex = regular_expression::regex(strings[0].chars().collect::<Vec<char>>());
    let float_regex = regular_expression::regex(strings[1].chars().collect::<Vec<char>>());
    let regex_list = vec![integer_regex, float_regex];
    //dbg!(regex_list[0].clone().into_iter().collect::<String>());
    let nfa = finite_automaton::set_nfa::SetNFA::from_regex(&regex_list);
    let (dfa, _) = finite_automaton::set_dfa::SetDFA::from_nfa(nfa, &SIGMA);
    let dot_graph = dot_generator::DotGraph::from_dfa(&dfa);
    let mut file = File::create("test.dot").unwrap();
    file.write_all(&dot_graph.code).unwrap();
    let mut dfa = finite_automaton::set_dfa::SetDFA::hopcroft(&dfa);
    dfa.remove_trap();
    //    let dfa2 = finite_automaton::DFA2::from_DFA(&dfa);
    //let dot_graph = dot_generator::DotGraph::from_dfa2(&dfa);
    //dbg!(&dfa2);
    let mut file = File::create("test_no_trap.dot").unwrap();
    file.write_all(&dot_graph.code).unwrap();
}
// To check backtrack:
// - Find final states.
// - If all alphabet goes to trap, no backtrack.
// - If not, need one more symbol to verify if final.
// - Two option, leave dfa like this and find a way to deal with backtrack.
// - Or, add a new final state that will replace and backtrack.
// let regex1: Regex = to_postfix(&add_explicit_concat(&vec![
//     '(', 'a', '|', 'b', ')', '*', 'c',
// ]));

// let alphanum = String::from("[a-zA-Z0-9]");
// let digit = String::from("[0-9]");

// let nonzero = String::from("[1-9]");
// let letter = String::from("[a-zA-Z]");
// // FIXME: For some reason, I need way too many parenthesis.
// let mut s = vec![String::from("(a|b)*c")];
// replace_symbolic_name(&mut s, get_symbolic_names());
// dbg!(replace_range(&s[0]));
// let regex2: Vec<Regex> = vec![Regex(&replace_range(&s[0]))];
// dbg!(&regex2[0].iter().collect::<String>());
// dbg!(add_explicit_concat(&replace_range(&s[0]).chars().collect())
//     .iter()
//     .collect::<String>());
// let nfa = NFA::from_regex(&regex2);
// // let graph = DotGraph::from_nfa(&nfa, (0, nfa.last_state));
// let (dfa, _) = DFA::from_nfa(nfa, &SIGMA);
// let graph = DotGraph::from_dfa(&dfa, (0, dfa.states.len() - 1));
// let mut code = String::from_utf8(graph.code).unwrap();
// code = code.replace("\n", "");
// code = code.replace("\t", "");
// code = code.chars().filter(|&c| c != '\"').collect();
// code = code.replace("\"", "'");
// print!("{:#?}", code);

/*
fn get_symbolic_names() -> BTreeMap<String, String> {
    let mut symbolic_names_map = BTreeMap::new();
    symbolic_names_map.insert("{alphanum}".to_string(), "[a-zA-Z0-9]".to_string());
    symbolic_names_map.insert(
        "{integer}".to_string(),
        "(({nonzero}{digit}*)|0)".to_string(),
    );
    symbolic_names_map.insert(
        "{fraction}".to_string(),
        "(.(({digit}*{nonzero})|0))".to_string(),
    );
    // symbolic_names_map.insert(
    //     "{float}".to_string(),
    //     "{integer}{fraction}(e(+|-)?{integer})".to_string(),
    // );

    symbolic_names_map.insert("{letter}".to_string(), "[a-zA-Z]".to_string());
    symbolic_names_map.insert("{digit}".to_string(), "[0-9]".to_string());
    symbolic_names_map.insert("{nonzero}".to_string(), "[1-9]".to_string());
    symbolic_names_map
}

fn replace_symbolic_name(strings: &mut Vec<String>, symbolic_names_map: BTreeMap<String, String>) {
    'main: loop {
        for (i, string) in strings.clone().iter().enumerate() {
            for (symbolic_name, expression) in symbolic_names_map.iter() {
                if string.find(symbolic_name).is_some() {
                    let new_string = string.replace(symbolic_name, expression);
                    strings.remove(i);
                    strings.push(new_string);
                    continue 'main;
                }
            }
        }
        break;
    }
}
*/
