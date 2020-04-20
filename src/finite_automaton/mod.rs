extern crate alloc;

pub mod dfa;
pub use dfa::*;
pub mod nfa;
pub use nfa::*;
pub mod set_dfa;
pub use set_dfa::*;
pub mod set_nfa;
pub use set_nfa::*;

//use crate::regular_expression::Regex;

// The NFA defined here is limited since it assumes it will be created using
// Thompson's construction.
// We can assume that each state will only have either one normal transition
// or up to 2 epsilon transitions (except the initial state that goes to all regex line).

// #[derive(Debug)]
// enum Transition {
//     SymbolTransition { transition: (char, usize) },
//     EpsiloonTransition { transitions: Vec<usize> },
// }

// #[derive(Debug)]
// struct State {
//     id: usize,
//     last_state: bool,
//     transition: Transition,
// }

/*
#[derive(Debug)]
pub struct DFA2 {
    pub states: BTreeSet<usize>,
    pub alphabet: BTreeSet<char>,
    pub function: BTreeMap<(usize, usize), BTreeSet<char>>,
    pub initial_state: usize,
    pub final_states: BTreeSet<usize>,
}

impl DFA2 {
    pub fn from_DFA(dfa: &DFA) -> Self {
        let mut function: BTreeMap<(usize, usize), BTreeSet<char>> = Default::default();
        //     dbg!(&dfa.function);
        for (&(from, char), &to) in dfa.function.iter() {
            match function.entry((from, to)) {
                Entry::Vacant(entry) => {
                    let mut new_entry: BTreeSet<char> = Default::default();
                    new_entry.insert(char);
                    entry.insert(new_entry);
                }
                Entry::Occupied(mut entry) => {
                    entry.get_mut().insert(char);
                }
            };
        }

        DFA2 {
            states: dfa.states.clone(),
            alphabet: dfa.alphabet.clone(),
            function,
            initial_state: dfa.initial_state,
            final_states: dfa.final_states.clone(),
        }
    }
}
*/
