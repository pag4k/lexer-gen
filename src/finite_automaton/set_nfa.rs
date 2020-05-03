use super::nfa::*;
use alloc::collections::btree_map::{BTreeMap, Entry};
use alloc::collections::btree_set::BTreeSet;

/// NonDeterministicFiniteAccepter ADusize
#[derive(Debug)]
pub struct SetNFA {
    pub alphabet: Vec<u8>,
    pub function: BTreeMap<(usize, Option<u8>), Vec<usize>>,
    pub initial_state: usize,
    pub states: Vec<usize>,
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

    fn states(&self) -> &[usize] {
        &self.states
    }

    /// Return the next state based on a state and an input character.
    fn next(&self, state: usize, input: Option<u8>) -> Option<&[usize]> {
        // FIXME: Why does the map doesn't word?
        self.function
            .get(&(state, input))
            .map(|states| states.as_slice())
    }

    fn alphabet(&self) -> &[u8] {
        &self.alphabet
    }
}

impl SetNFA {
    // This function assumes that the regex is in a particular form, as transformed in
    // regular_expression/.
    pub fn from_regex(regex_list: &[Vec<u8>]) -> (impl NFA<usize>, Vec<usize>) {
        // For NFA.
        let alphabet: Vec<u8> = Default::default();
        let function: BTreeMap<(usize, Option<u8>), Vec<usize>> = BTreeMap::new();
        let initial_state: usize = 0;
        let states: Vec<usize> = Default::default();
        let final_states: BTreeSet<usize> = BTreeSet::new();

        let mut nfa = SetNFA {
            alphabet,
            function,
            initial_state,
            states,
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
                        stack.push(nfa.add_symbol_subgraph(b'\\'));
                    }
                }
                match byte {
                    b'\\' => escape_char = true,
                    b'*' => {
                        let sub_nfa = stack.pop().unwrap();
                        stack.push(nfa.add_kleene_star_subgraph(sub_nfa));
                    }
                    b'+' => {
                        let sub_nfa = stack.pop().unwrap();
                        stack.push(nfa.add_kleene_plus_subgraph(sub_nfa));
                    }
                    b'?' => {
                        let sub_nfa = stack.pop().unwrap();
                        stack.push(nfa.add_optional_subgraph(sub_nfa));
                    }
                    b'|' => {
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
            // FIXME: Better way to han
            final_states.push(nfa.states().last().unwrap().to_owned());
            debug_assert!(!escape_char);

            // Add regex NFA to final NFA.
            assert!(!stack.is_empty());
            let (regex_start, regex_end) = stack.pop().unwrap();
            match nfa.function.entry((initial_state, None)) {
                Entry::Occupied(mut entry) => entry.get_mut().push(regex_start),
                Entry::Vacant(entry) => {
                    entry.insert(vec![regex_start]);
                }
            }
            nfa.final_states.insert(regex_end);
        }

        (nfa, final_states)
    }

    fn add_state(&mut self) -> usize {
        self.states.push(self.states.len());
        self.states.last().unwrap().to_owned()
    }

    fn add_symbol_subgraph(&mut self, byte: u8) -> (usize, usize) {
        if !self.alphabet.contains(&byte) {
            self.alphabet.push(byte);
        }
        let new_start = self.add_state();
        let new_end = self.add_state();
        self.function.insert((new_start, Some(byte)), vec![new_end]);
        (new_start, new_end)
    }

    fn add_concat_subgraph(
        &mut self,
        (start1, end1): (usize, usize),
        (start2, end2): (usize, usize),
    ) -> (usize, usize) {
        self.function.insert((end1, None), vec![start2]);
        (start1, end2)
    }

    fn add_union_subgraph(
        &mut self,
        (start1, end1): (usize, usize),
        (start2, end2): (usize, usize),
    ) -> (usize, usize) {
        let new_start = self.add_state();
        let new_end = self.add_state();
        self.function
            .insert((new_start, None), vec![start1, start2]);
        self.function.insert((end1, None), vec![new_end]);
        self.function.insert((end2, None), vec![new_end]);
        (new_start, new_end)
    }

    fn add_optional_subgraph(&mut self, (start, end): (usize, usize)) -> (usize, usize) {
        let new_start = self.add_state();
        let new_end = self.add_state();
        self.function
            .insert((new_start, None), vec![start, new_end]);
        self.function.insert((end, None), vec![new_end]);
        (new_start, new_end)
    }

    fn add_kleene_star_subgraph(&mut self, (start, end): (usize, usize)) -> (usize, usize) {
        let new_start = self.add_state();
        let new_end = self.add_state();
        self.function
            .insert((new_start, None), vec![start, new_end]);
        self.function.insert((end, None), vec![start, new_end]);
        (new_start, new_end)
    }

    fn add_kleene_plus_subgraph(&mut self, (start, end): (usize, usize)) -> (usize, usize) {
        let new_start = self.add_state();
        let new_end = self.add_state();
        self.function.insert((new_start, None), vec![start]);
        self.function.insert((end, None), vec![start, new_end]);
        (new_start, new_end)
    }
}

pub fn get_epsilon_closure(nfa: &impl NFA<usize>) -> Vec<Vec<usize>> {
    //let mut epsilon_closure: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    let nfa_states_count = nfa.states().len();
    let mut epsilon_closure: Vec<Vec<usize>> = vec![Default::default(); nfa_states_count];
    //for &state in &nfa_states {
    for (state, ref mut list) in epsilon_closure.iter_mut().enumerate() {
        let mut marked_states: BTreeMap<usize, bool> = BTreeMap::new();

        // Add current state as an unmarked state.
        marked_states.insert(state, false);

        // Main loop of the algorithm. On each iteraton, an unmarked state is selected.
        while let Some(unmarked_state) = marked_states.iter().find(|(_, &marked)| !marked) {
            // Get and mark the selected state.
            let (&unmarked_state, _) = unmarked_state;
            marked_states.insert(unmarked_state, true);

            // Add all states reachable with an epsilon transition as unmarked.
            if let Some(states_to_add) = nfa.next(unmarked_state, None) {
                for &state in states_to_add {
                    // Make sure it is only inserted once.
                    marked_states.entry(state).or_insert(false);
                }
            }
        }
        // Add all marked states to epsilon closure.
        let mut new_list = marked_states.into_iter().map(|(state, _)| state).collect();
        list.append(&mut new_list);
    }
    epsilon_closure
}
