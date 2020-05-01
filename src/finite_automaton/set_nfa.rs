use super::nfa::*;
use alloc::collections::btree_map::BTreeMap;
use alloc::collections::btree_set::BTreeSet;

/// NonDeterministicFiniteAccepter ADusize
#[derive(Debug)]
pub struct SetNFA {
    pub alphabet: BTreeSet<u8>,
    pub function: BTreeMap<(usize, Option<u8>), BTreeSet<usize>>,
    pub initial_state: usize,
    pub last_state: usize,
    pub final_states: BTreeSet<usize>,
}

// TODO: Not sure I understand these lifetime.
impl NFA<usize> for SetNFA {
    fn initial_state(&self) -> usize {
        self.initial_state
    }

    fn final_states(&self) -> Box<dyn Iterator<Item = usize>> {
        Box::new(self.final_states.clone().into_iter())
    }

    fn is_final_state(&self, state: usize) -> bool {
        self.final_states.contains(&state)
    }

    fn states(&self) -> Box<dyn Iterator<Item = usize>> {
        Box::new(0..=self.last_state)
    }

    /// Return the next state based on a state and an input character.
    fn next(&self, state: usize, input: Option<u8>) -> Option<Box<dyn Iterator<Item = usize>>> {
        // FIXME: Why does the map doesn't word?
        if let Some(states) = self.function.get(&(state, input))
        //.map(|states| Box::new(states.clone().into_iter()))
        {
            let s: BTreeSet<usize> = states.clone();
            Some(Box::new(s.into_iter()))
        } else {
            None
        }
    }

    fn alphabet(&self) -> Box<dyn Iterator<Item = u8>> {
        Box::new(self.alphabet.clone().into_iter())
    }
}

impl SetNFA {
    // This function assumes that the regex is in a particular form, as transformed in
    // regular_expression/.
    pub fn from_regex(regex_list: &[Vec<u8>]) -> (impl NFA<usize>, Vec<usize>) {
        // For NFA.
        let alphabet = BTreeSet::new();
        let function: BTreeMap<(usize, Option<u8>), BTreeSet<usize>> = BTreeMap::new();
        let initial_state: usize = 0;
        let last_state: usize = 0;
        let final_states: BTreeSet<usize> = BTreeSet::new();

        let mut nfa = SetNFA {
            alphabet,
            function,
            initial_state,
            last_state,
            final_states,
        };

        // TODO: Do not allow regex that accept empty string!:
        let mut final_states: Vec<usize> = Default::default();

        // Reserve state 0 for first state.
        for regex in regex_list {
            let mut stack: Vec<(usize, usize)> = Vec::new();
            let mut escape_char = false;
            for (i, byte) in regex.iter().enumerate() {
                let c = *byte as char;
                if escape_char {
                    escape_char = false;
                    if crate::regular_expression::ESCAPE_CHAR.contains(&c) {
                        stack.push(nfa.add_symbol_subgraph(*byte));
                        continue;
                    } else {
                        stack.push(nfa.add_symbol_subgraph('\\' as u8));
                    }
                }
                match byte {
                    92 => escape_char = true,
                    42 => {
                        let sub_nfa = stack.pop().unwrap();
                        stack.push(nfa.add_kleene_star_subgraph(sub_nfa));
                    }
                    43 => {
                        let sub_nfa = stack.pop().unwrap();
                        stack.push(nfa.add_kleene_plus_subgraph(sub_nfa));
                    }
                    63 => {
                        let sub_nfa = stack.pop().unwrap();
                        stack.push(nfa.add_optional_subgraph(sub_nfa));
                    }
                    124 => {
                        let right_sub_nfa = stack.pop().unwrap();
                        let left_sub_nfa = stack.pop().unwrap();
                        stack.push(nfa.add_union_subgraph(left_sub_nfa, right_sub_nfa));
                    }
                    197 => {
                        if stack.len() < 2 {
                            panic!(
                                "Stack does not have two elements: {}",
                                regex[0..=i]
                                    .iter()
                                    .map(|&byte| byte as char)
                                    .collect::<String>()
                            );
                        }
                        let right_sub_nfa = stack.pop().unwrap();
                        let left_sub_nfa = stack.pop().unwrap();
                        stack.push(nfa.add_concat_subgraph(left_sub_nfa, right_sub_nfa));
                    }
                    _ => stack.push(nfa.add_symbol_subgraph(*byte)),
                }
            }
            final_states.push(nfa.last_state);
            debug_assert!(!escape_char);
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

        (nfa, final_states)
    }

    fn add_state(&mut self) -> usize {
        self.last_state += 1;
        self.last_state
    }

    fn add_symbol_subgraph(&mut self, byte: u8) -> (usize, usize) {
        self.alphabet.insert(byte);
        let new_start = self.add_state();
        let new_end = self.add_state();
        self.function
            .insert((new_start, Some(byte)), [new_end].iter().cloned().collect());
        (new_start, new_end)
    }

    fn add_concat_subgraph(
        &mut self,
        (start1, end1): (usize, usize),
        (start2, end2): (usize, usize),
    ) -> (usize, usize) {
        self.function
            .insert((end1, None), [start2].iter().cloned().collect());
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
        (new_start, new_end)
    }

    fn add_kleene_star_subgraph(&mut self, (start, end): (usize, usize)) -> (usize, usize) {
        let new_start = self.add_state();
        let new_end = self.add_state();
        self.function.insert(
            (new_start, None),
            [start, new_end].iter().cloned().collect(),
        );
        self.function
            .insert((end, None), [start, new_end].iter().cloned().collect());
        (new_start, new_end)
    }

    fn add_kleene_plus_subgraph(&mut self, (start, end): (usize, usize)) -> (usize, usize) {
        let new_start = self.add_state();
        let new_end = self.add_state();
        self.function
            .insert((new_start, None), [start].iter().cloned().collect());
        self.function
            .insert((end, None), [start, new_end].iter().cloned().collect());
        (new_start, new_end)
    }
}

pub fn get_epsilon_closure(nfa: &impl NFA<usize>) -> BTreeMap<usize, Vec<usize>> {
    let mut epsilon_closure: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    let nfa_states: Vec<usize> = nfa.states().collect();
    for &state in &nfa_states {
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
            if let Some(states_to_add) = nfa.next(unmarked_state, None) {
                for state in states_to_add {
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
