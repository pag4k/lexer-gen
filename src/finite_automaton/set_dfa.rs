use super::dfa::*;
use super::nfa::*;
use super::set_nfa::get_epsilon_closure;
use alloc::collections::btree_map::BTreeMap;
use alloc::collections::btree_set::BTreeSet;

#[derive(Debug)]
pub struct SetDFA {
    pub states: BTreeSet<usize>,
    pub alphabet: BTreeSet<u8>,
    pub function: BTreeMap<(usize, u8), usize>,
    pub initial_state: usize,
    pub final_states: BTreeSet<usize>,
}

// TODO: Not sure I understand these lifetime.
impl DFA<usize> for SetDFA {
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
        Box::new(self.states.clone().into_iter())
    }

    /// Return the next state based on a state and an input character.
    fn next(&self, state: usize, input: u8) -> Option<usize> {
        self.function.get(&(state, input)).cloned()
    }

    fn alphabet(&self) -> Box<dyn Iterator<Item = u8>> {
        Box::new(self.alphabet.clone().into_iter())
    }
}

impl SetDFA {
    /// Return a DFA from a NFA and map descripting the relation between the NFA and DFA states.
    ///
    /// # Remarks
    ///
    /// Based on the Rabin-Scott powerset construction algorithm.
    ///
    /// Since multiple NFA states correspond to a single DFA state, the NFA states will be sorted
    /// to make sure the comparisons are done properly.
    pub fn from_nfa(nfa: impl NFA<usize>) -> (Self, BTreeMap<BTreeSet<usize>, usize>) {
        let mut nfa_to_dfa_states_map: BTreeMap<Vec<usize>, usize> = BTreeMap::new();
        let mut marked_states: BTreeMap<usize, bool> = BTreeMap::new();
        let mut function: BTreeMap<(usize, u8), usize> = BTreeMap::new();
        let alphabet = nfa.alphabet().collect();
        let epsilon_closure = get_epsilon_closure(&nfa);
        //println!("##### EPSILON CLOSURE DONE");

        // Get the inital states of the nfa and set the corresponding DFA state to 0.
        let mut initial_states = epsilon_closure[&nfa.initial_state()].clone();
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
            for &current_symbol in &alphabet {
                // Get the resulting DFA state.
                let mut states: Vec<usize> = unmarked_states_nfa
                    .iter()
                    .filter_map(|&state| nfa.next(state, Some(current_symbol)))
                    .map(|mut states| {
                        //assert!(states.len() == 1);
                        epsilon_closure[&states.next().unwrap()].clone()
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
            let final_states_count = current_states
                .iter()
                .filter(|&&state| nfa.is_final_state(state))
                .count();

            if final_states_count > 1 {
                println!(
                    "Warning: A DFA final state corresponds to {} distinct tokens.",
                    final_states_count
                );
            }

            // If there is more than one final state, take the first.
            // FIXME: It works, but it is the last token that it kept which is weird.
            if let Some(&state_id) = current_states
                .iter()
                .find(|&&state| nfa.is_final_state(state))
            {
                if nfa.is_final_state(state_id) {
                    final_states.insert(nfa_to_dfa_states_map[current_states]);
                }
            }
        }

        (
            SetDFA {
                states: nfa_to_dfa_states_map.values().cloned().collect(),
                alphabet,
                function,
                initial_state: 0,
                final_states,
            },
            nfa_to_dfa_states_map
                .into_iter()
                .map(|(nfa_states, dfa_state)| {
                    (
                        nfa_states.into_iter().collect::<BTreeSet<usize>>(),
                        dfa_state,
                    )
                })
                .collect(),
        )
    }

    pub fn remove_trap(&mut self) {
        // Assume there is only one trap state.
        let trap_states = get_trap_states(self);
        //assert!(trap_states.len() <= 1);
        for trap_state in trap_states {
            self.states.remove(&trap_state);
            for (pair, to) in self.function.clone() {
                if to == trap_state {
                    self.function.remove(&pair);
                }
            }
        }
    }

    pub fn hopcroft(
        dfa: &impl DFA<usize>,
        mutex_states_sets: &[BTreeSet<usize>],
    ) -> (Self, BTreeMap<BTreeSet<usize>, usize>) {
        let alphabet: BTreeSet<u8> = dfa.alphabet().collect();
        let old_states: Vec<usize> = dfa.states().collect();
        // Check if the mutex states sets intersect.
        for i in 0..mutex_states_sets.len() {
            for j in i + 1..mutex_states_sets.len() {
                if !mutex_states_sets[i].is_disjoint(&mutex_states_sets[j]) {
                    panic!(
                        "State sets {:?} and {:?} are not disjoint.",
                        mutex_states_sets[i], mutex_states_sets[j]
                    );
                }
            }
        }
        // Check if mutex_states mixes non final states and final states.
        for mutex_states in mutex_states_sets {
            if mutex_states
                .iter()
                .map(|&state| if dfa.is_final_state(state) { 1 } else { -1 })
                .sum::<isize>()
                .abs() as usize
                != mutex_states.len()
            {
                panic!(
                    "State set {:?} is mixes non final states with final states.",
                    mutex_states
                );
            }
        }
        // FIXME: I can probably use this to check for duplicates and thus, intersection.
        let mutex_states: BTreeSet<usize> = mutex_states_sets.iter().cloned().flatten().collect();
        /*
        let mutex_states: BTreeSet<usize> = {
            let len = mutex_states.len();
            let mutex_states: BTreeSet<usize> = mutex_states.into_iter().collect();
            if len != mutex_states.len() {
                panic!("Some state sets are no disjoint.");
            }
            mutex_states
        };
        */
        let (leftover_non_final_states, leftover_final_states): (BTreeSet<usize>, BTreeSet<usize>) =
            old_states
                .iter()
                .cloned()
                .filter(|state| !mutex_states.contains(state))
                .partition(|&state| dfa.is_final_state(state));
        // P is the partition.
        let mut p = mutex_states_sets.to_vec();
        p.push(leftover_non_final_states);
        p.push(leftover_final_states);
        // W is the set to try to partition.
        let mut w = p.clone();

        while let Some(a) = w.pop() {
            for &c in &alphabet {
                // let X be the set of states for which a transition on c leads to a state in A
                let x: BTreeSet<usize> = old_states
                    .iter()
                    .filter_map(|&from| {
                        if let Some(to) = dfa.next(from, c) {
                            if a.contains(&to) {
                                Some(from)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .collect();
                let mut new_p: Vec<BTreeSet<usize>> = Default::default();
                for y in p.into_iter() {
                    let intersection: BTreeSet<usize> = x.intersection(&y).cloned().collect();
                    let difference: BTreeSet<usize> = y.difference(&x).cloned().collect();
                    if !intersection.is_empty() && !difference.is_empty() {
                        new_p.push(intersection.clone());
                        new_p.push(difference.clone());
                        if let Some(position) = w.iter().position(|set| *set == y) {
                            w.remove(position);
                            w.push(intersection);
                            w.push(difference);
                        } else if intersection.len() <= difference.len() {
                            w.push(intersection);
                        } else {
                            w.push(difference);
                        }
                    } else {
                        new_p.push(y);
                    }
                }
                p = new_p;
            }
        }

        // Construct the minimal DFA.
        let initial_state: usize = p
            .iter()
            .position(|set| set.contains(&dfa.initial_state()))
            .unwrap();
        let final_states: BTreeSet<usize> = p
            .iter()
            .enumerate()
            .filter_map(|(i, set)| {
                if dfa
                    .final_states()
                    .any(|final_state| set.contains(&final_state))
                {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();

        // Map previous function to new one.
        let mut function: BTreeMap<(usize, u8), usize> = Default::default();

        for &from in &old_states {
            for &input in &alphabet {
                if let Some(to) = dfa.next(from, input) {
                    let new_from: usize = p.iter().position(|set| set.contains(&from)).unwrap();
                    let new_to: usize = p.iter().position(|set| set.contains(&to)).unwrap();
                    function.insert((new_from, input), new_to);
                }
            }
        }

        (
            SetDFA {
                states: (0..p.len()).collect(),
                alphabet,
                function,
                initial_state,
                final_states,
            },
            p.into_iter()
                .enumerate()
                .map(|(hopcroft_state, dfa_states)| {
                    (dfa_states.into_iter().collect(), hopcroft_state)
                })
                .collect(),
        )
    }
}

pub fn get_trap_states<T>(dfa: &impl DFA<T>) -> Vec<T>
where
    T: core::cmp::Eq + core::marker::Copy,
{
    dfa.states()
        .filter(|&from| {
            !dfa.is_final_state(from)
                && dfa.alphabet().all(|input| {
                    if let Some(to) = dfa.next(from, input) {
                        from == to
                    } else {
                        true
                    }
                })
        })
        .collect()
}

pub fn get_backtrack_states<T>(dfa: &impl DFA<T>) -> Vec<T>
where
    T: core::marker::Copy,
{
    dfa.final_states()
        .filter(|&final_state| {
            dfa.alphabet()
                .any(|input| dfa.next(final_state, input).is_some())
        })
        .collect()
}
