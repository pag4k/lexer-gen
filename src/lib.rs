extern crate alloc;

pub mod dot_generator;
mod finite_automaton;
mod regular_expression;

use alloc::collections::btree_set::BTreeSet;
use alloc::vec::Vec;

pub fn generate_lexer(
    source: Vec<&'static str>,
) -> (
    impl finite_automaton::DFA<usize>,
    Vec<Vec<usize>>,
    Vec<usize>,
) {
    let regex = source
        .iter()
        .map(|&string| regular_expression::regex(string.chars().collect::<Vec<char>>()))
        .collect::<Vec<Vec<char>>>();
    let (nfa, final_states) = finite_automaton::set_nfa::SetNFA::from_regex(&regex);
    let (dfa, nfa_to_dfa_map) = finite_automaton::set_dfa::SetDFA::from_nfa(nfa);
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
    let mut dedup_final_states: Vec<BTreeSet<usize>> = Default::default();
    // FIXME: Very inefficient.
    for (i, final_set) in final_states2.iter().enumerate() {
        let mut new_set = final_set.clone();
        for previous_final_states in final_states2.iter().take(i) {
            new_set = new_set
                .difference(&previous_final_states)
                .cloned()
                .collect();
        }
        dedup_final_states.push(new_set);
    }
    let final_states2 = dedup_final_states;
    //dbg!(&final_states2);
    let (dfa, dfa_to_hopcroft_map) = finite_automaton::set_dfa::SetDFA::hopcroft(&dfa, false);
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
    let (mut dfa, dfa_to_hopcroft_map) =
        finite_automaton::set_dfa::SetDFA::hopcroft_plus(&dfa, &final_states3);
    let final_states: Vec<Vec<usize>> = final_states3
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
    //dbg!(&final_states);
    /*
    let mut final_states_to_token: BTreeMap<usize, usize> = Default::default();
    for (i, states) in final_states4.into_iter().enumerate() {
        for state in states {
            final_states_to_token.insert(state, i);
        }
    }
    */
    // TODO: Check if each token has at least one final state.
    dfa.remove_trap();
    let backtrack_states = finite_automaton::set_dfa::get_backtrack_states(&dfa);

    (dfa, final_states, backtrack_states)
}