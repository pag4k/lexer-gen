extern crate alloc;

use alloc::collections::btree_map::BTreeMap;
use alloc::collections::btree_set::BTreeSet;
use alloc::vec::Vec;

//use crate::regular_expression::Regex;

// The NFA defined here is limited since it assumes it will be created using
// Thompson's construction.regular_expression.
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

/// NonDeterministicFiniteAccepter ADT
#[derive(Debug)]
pub struct NFA {
    pub alphabet: BTreeSet<char>,
    pub function: BTreeMap<(usize, Option<char>), BTreeSet<usize>>,
    pub initial_state: usize,
    pub last_state: usize,
    pub final_states: BTreeSet<usize>,
}

impl NFA {
    pub fn from_regex(regex_list: &[Vec<char>]) -> NFA {
        // For NFA.
        let mut alphabet = BTreeSet::new();
        let mut function: BTreeMap<(usize, Option<char>), BTreeSet<usize>> = BTreeMap::new();
        let initial_state: usize = 0;
        let mut last_state: usize = 0;
        let mut final_states: BTreeSet<usize> = BTreeSet::new();

        let mut nfa = NFA {
            alphabet,
            function,
            initial_state,
            last_state,
            final_states,
        };
        // Reserve state 0 for first state.
        for regex in regex_list {
            let mut stack: Vec<(usize, usize)> = Vec::new();
            for c in regex {
                match c {
                    '*' => {
                        let sub_nfa = stack.pop().unwrap();
                        stack.push(nfa.add_closure_subgraph(sub_nfa));
                    }
                    '?' => {
                        let sub_nfa = stack.pop().unwrap();
                        stack.push(nfa.add_optional_subgraph(sub_nfa));
                    }
                    '|' => {
                        let right_sub_nfa = stack.pop().unwrap();
                        let left_sub_nfa = stack.pop().unwrap();
                        stack.push(nfa.add_union_subgraph(left_sub_nfa, right_sub_nfa));
                    }
                    '⋅' => {
                        let right_sub_nfa = stack.pop().unwrap();
                        let left_sub_nfa = stack.pop().unwrap();
                        stack.push(nfa.add_concat_subgraph(left_sub_nfa, right_sub_nfa));
                    }
                    _ => stack.push(nfa.add_symbol_subgraph(*c)),
                }
            }
            // For the case where the regex was empty.
            if !stack.is_empty() {
                let (final_start, final_end) = stack.pop().unwrap();
                if let Some(to) = nfa.function.get_mut(&(initial_state, None)) {
                    to.insert(final_start);
                } else {
                    nfa.function.insert(
                        (initial_state, None),
                        [final_start].iter().cloned().collect(),
                    );
                }

                nfa.final_states.insert(final_end);
            }
        }

        nfa
    }

    fn add_state(&mut self) -> usize {
        self.last_state += 1;
        self.last_state
    }

    fn add_symbol_subgraph(&mut self, c: char) -> (usize, usize) {
        self.alphabet.insert(c);
        let new_start = self.add_state();
        let new_end = self.add_state();
        self.function
            .insert((new_start, Some(c)), [new_end].iter().cloned().collect());
        // self.final_states.insert(new_end);
        (new_start, new_end)
    }

    fn add_concat_subgraph(
        &mut self,
        (start1, end1): (usize, usize),
        (start2, end2): (usize, usize),
    ) -> (usize, usize) {
        self.function
            .insert((end1, None), [start2].iter().cloned().collect());
        // self.final_states.remove(&end1);
        (start1, end2)
    }

    fn add_union_subgraph(
        &mut self,
        (start1, end1): (usize, usize),
        (start2, end2): (usize, usize),
    ) -> (usize, usize) {
        let new_start = self.add_state();
        let new_end = self.add_state();
        self.function.insert(
            (new_start, None),
            [start1, start2].iter().cloned().collect(),
        );
        self.function
            .insert((end1, None), [new_end].iter().cloned().collect());
        self.function
            .insert((end2, None), [new_end].iter().cloned().collect());
        // self.final_states.remove(&end1);
        // self.final_states.remove(&end2);
        // self.final_states.insert(new_end);
        (new_start, new_end)
    }

    fn add_optional_subgraph(&mut self, (start, end): (usize, usize)) -> (usize, usize) {
        let new_start = self.add_state();
        let new_end = self.add_state();
        self.function.insert(
            (new_start, None),
            [start, new_end].iter().cloned().collect(),
        );
        self.function
            .insert((end, None), [new_end].iter().cloned().collect());
        // self.final_states.remove(&end);
        // self.final_states.insert(new_end);
        (new_start, new_end)
    }

    fn add_closure_subgraph(&mut self, (start, end): (usize, usize)) -> (usize, usize) {
        let new_start = self.add_state();
        let new_end = self.add_state();
        self.function.insert(
            (new_start, None),
            [start, new_end].iter().cloned().collect(),
        );
        self.function
            .insert((end, None), [start, new_end].iter().cloned().collect());
        // self.final_states.remove(&end);
        // self.final_states.insert(new_end);
        (new_start, new_end)
    }

    fn get_epsilon_closure(&self) -> BTreeMap<usize, Vec<usize>> {
        let mut epsilon_closure: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
        for state in 0..=self.last_state {
            let mut marked_states: BTreeMap<usize, bool> = BTreeMap::new();

            // Add current state as an unmarked state.
            marked_states.insert(state, false);

            // Main loop of the algorithm. On each iteraton, an unmarked state is selected.
            while let Some(unmarked_state) = marked_states
                .clone()
                .into_iter()
                .find(|(_, marked)| !marked)
            {
                // dbg!(unmarked_state);
                // Get and mark the selected state.
                let (unmarked_state, _) = unmarked_state;
                marked_states.insert(unmarked_state, true);

                // Add all states reachable with an epsilon transition as unmarked.
                if let Some(states_to_add) = self.function.get(&(unmarked_state, None)) {
                    for &state in states_to_add {
                        // Make sure it is only inserted once.
                        marked_states.entry(state).or_insert(false);
                    }
                }
            }
            // Add all marked states to epsilon closure.
            epsilon_closure.insert(
                state,
                marked_states.into_iter().map(|(state, _)| state).collect(),
            );
        }
        epsilon_closure
    }
}

#[derive(Debug)]
pub struct DFA {
    pub states: BTreeSet<usize>,
    alphabet: BTreeSet<char>,
    pub function: BTreeMap<(usize, char), usize>,
    initial_state: usize,
    pub final_states: BTreeSet<usize>,
}

impl DFA {
    /// Return the initial state of the DFA.
    pub fn get_initial_state(&self) -> usize {
        self.initial_state
    }
    /// Return the next state based on a state and an input character.
    pub fn next(&self, state: usize, input: char) -> Option<usize> {
        self.function.get(&(state, input)).cloned()
    }
    /// Return if a state is final.
    pub fn is_final_state(&self, state: usize) -> bool {
        self.final_states.get(&state).is_some()
    }
    /// Return a DFA from a NFA and map descripting the relation between the NFA and DFA states.
    ///
    /// # Remarks
    ///
    /// Based on the Rabin-Scott powerset construction algorithm.
    ///
    /// Since multiple NFA states correspond to a single DFA state, the NFA states will be sorted
    /// to make sure the comparisons are done properly.
    pub fn from_nfa(nfa: NFA, alphabet: &[char]) -> (Self, BTreeMap<Vec<usize>, usize>) {
        let mut nfa_to_dfa_states_map: BTreeMap<Vec<usize>, usize> = BTreeMap::new();
        let mut marked_states: BTreeMap<usize, bool> = BTreeMap::new();
        let mut function: BTreeMap<(usize, char), usize> = BTreeMap::new();

        let epsilon_closure = nfa.get_epsilon_closure();
        //println!("##### EPSILON CLOSURE DONE");

        // Get the inital states of the nfa and set the corresponding DFA state to 0.
        let mut initial_states = epsilon_closure[&nfa.initial_state].clone();
        initial_states.sort_unstable();
        initial_states.dedup();
        nfa_to_dfa_states_map.insert(initial_states, 0);
        marked_states.insert(0, false);

        // Main loop of the algorithm. On each iteraton, an unmarked state is selected.
        while let Some(unmarked_state) = nfa_to_dfa_states_map
            .clone()
            .into_iter()
            .find(|(_, state)| !marked_states[state])
        {
            // Get and mark the selected state.
            let (unmarked_states_nfa, unmarked_state_dfa) = unmarked_state;
            marked_states.insert(unmarked_state_dfa, true);

            // Iterator over all symbol in the alphabet.
            for &current_symbol in nfa.alphabet.iter() {
                // Get the resulting DFA state.
                let mut states: Vec<usize> = unmarked_states_nfa
                    .iter()
                    .filter_map(|&state| nfa.function.get(&(state, Some(current_symbol))))
                    .map(|states| {
                        //assert!(states.len() == 1);
                        epsilon_closure[states.iter().next().unwrap()].clone()
                    })
                    .flatten()
                    .collect();
                states.sort_unstable();
                states.dedup();

                // Check if the DFA state already exist.
                let next_id = match nfa_to_dfa_states_map.get(&states) {
                    // If so, return its id.
                    Some(next_id) => *next_id,
                    // If not, get a new id and add it.
                    None => {
                        let next_id = nfa_to_dfa_states_map.len();
                        nfa_to_dfa_states_map.insert(states, next_id);
                        marked_states.insert(next_id, false);
                        next_id
                    }
                };
                // Add to transition function.
                function.insert((unmarked_state_dfa, current_symbol), next_id);
            }
        }

        // It is assumed that there is only one NFA final state for each DFA state.
        let mut final_states: BTreeSet<usize> = BTreeSet::new();
        for current_states in nfa_to_dfa_states_map.keys() {
            if current_states
                .iter()
                .filter(|&state| nfa.final_states.get(state).is_some())
                .count()
                > 1
            {
                unreachable!("A DFA state corresponds to more than one NFA final states.");
            }
            if let Some(state_id) = current_states
                .iter()
                .find(|&state| nfa.final_states.get(state).is_some())
            {
                if nfa.final_states.get(state_id).is_some() {
                    final_states.insert(nfa_to_dfa_states_map[current_states]);
                }
            }
        }

        (
            DFA {
                states: nfa_to_dfa_states_map.values().cloned().collect(),
                alphabet: BTreeSet::new(),
                function,
                initial_state: 0,
                final_states,
            },
            nfa_to_dfa_states_map,
        )
    }
}
