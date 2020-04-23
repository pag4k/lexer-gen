extern crate alloc;

mod dot_generator;
mod finite_automaton;
mod regular_expression;
mod test;

use std::fs::File;
use std::io::prelude::*;

use alloc::collections::btree_map::BTreeMap;
use alloc::collections::btree_set::BTreeSet;
use alloc::vec::Vec;

pub const SIGMA: [char; 87] = [
    'Σ', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
    'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A',
    'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T',
    'U', 'V', 'W', 'X', 'Y', 'Z', '!', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=',
    '>', '[', ']', '_', '{', '}', '&', '|', ' ', '\t', '\n',
];

fn main() {
    let language = test::get_language();
    let regex = language
        .iter()
        .map(|&(string, _)| {
            regular_expression::regex(String::from(string).chars().collect::<Vec<char>>())
        })
        .collect::<Vec<Vec<char>>>();
    let (nfa, final_states) = finite_automaton::set_nfa::SetNFA::from_regex(&regex);
    //dbg!(&final_states);
    let dot_graph = dot_generator::DotGraph::from_nfa(&nfa);
    let mut file = File::create("nfa.dot").unwrap();
    file.write_all(&dot_graph.code).unwrap();
    let (dfa, nfa_to_dfa_map) = finite_automaton::set_dfa::SetDFA::from_nfa(nfa, &SIGMA);
    let final_states2: Vec<BTreeSet<usize>> = final_states
        .into_iter()
        .map(|nfa_state| {
            nfa_to_dfa_map
                .clone()
                .into_iter()
                .filter_map(|(nfa_states, dfa_state)| {
                    if nfa_states.contains(&nfa_state) {
                        Some(dfa_state)
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect();
    //dbg!(&final_states2);
    //let dot_graph = dot_generator::DotGraph::from_dfa(&dfa);
    //let mut file = File::create("dfa.dot").unwrap();
    //file.write_all(&dot_graph.code).unwrap();
    let (mut dfa, dfa_to_hopcroft_map) = finite_automaton::set_dfa::SetDFA::hopcroft(&dfa, false);
    let final_states3: Vec<BTreeSet<usize>> = final_states2
        .into_iter()
        .map(|final_dfa_states| {
            dfa_to_hopcroft_map
                .iter()
                .filter_map(|(dfa_states, &hopcroft_state)| {
                    if !final_dfa_states.is_disjoint(dfa_states) {
                        Some(hopcroft_state)
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect();
    //dbg!(&final_states3);
    let (mut dfa, dfa_to_hopcroft_map) =
        finite_automaton::set_dfa::SetDFA::hopcroft_plus(&dfa, &final_states3);
    let final_states4: Vec<BTreeSet<usize>> = final_states3
        .into_iter()
        .map(|final_dfa_states| {
            dfa_to_hopcroft_map
                .iter()
                .filter_map(|(dfa_states, &hopcroft_state)| {
                    if !final_dfa_states.is_disjoint(dfa_states) {
                        Some(hopcroft_state)
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect();
    //  dbg!(&final_states4);
    let mut final_states_to_token: BTreeMap<usize, test::TokenType> = Default::default();
    for (i, states) in final_states4.into_iter().enumerate() {
        for state in states {
            final_states_to_token.insert(state, language[i].1);
        }
    }
    //  let dot_graph = dot_generator::DotGraph::from_dfa(&dfa, &final_states_to_token);
    let mut file = File::create("dfa_hop.dot").unwrap();
    file.write_all(&dot_graph.code).unwrap();
    dfa.remove_trap();
    //    let dfa2 = finite_automaton::DFA2::from_DFA(&dfa);
    //let dot_graph = dot_generator::DotGraph::from_dfa2(&dfa);
    let backtrack_states = finite_automaton::set_dfa::get_backtrack_states(&dfa);
    let dot_graph =
        dot_generator::DotGraph::from_dfa(&dfa, &final_states_to_token, &backtrack_states);
    let mut file = File::create("dfa_no_trap.dot").unwrap();
    file.write_all(&dot_graph.code).unwrap();
}

