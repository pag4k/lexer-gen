//use crate::finite_accepters::*;

extern crate alloc;

use crate::finite_automaton::*;
use alloc::vec::Vec;
/// DotGraph ADT
pub struct DotGraph {
    pub code: Vec<u8>,
}

impl DotGraph {
    /// Generate a .gv file based on the section of NFA specified in arguments.
    pub fn from_nfa<'a>(
        nfa: &impl NFA<'a, usize>,
        //tokens: &HashMap<usize, TokenType>,
        //backtrack: &HashSet<usize>,
        //(first_state, last_state): (usize, usize),
    ) -> DotGraph {
        let states: Vec<usize> = nfa.states().collect();
        let alphabet: Vec<char> = nfa.alphabet().collect();

        let mut dot_graph = DotGraph { code: Vec::new() };
        dot_graph.add_line("digraph finite_state_machine {");
        dot_graph.add_line("\trankdir=LR;");
        dot_graph.add_line("\tsize=\"8,5\"");

        dot_graph.add_line("");

        for state in states.iter().filter(|&&state| nfa.is_final_state(state)) {
            let line = format!(
                "\tnode [shape = rectangle, label=\"{} -> {}\", fontsize=12] token{};",
                state, state, state
            );
            dot_graph.add_line(&line);
        }

        dot_graph.add_line("");

        for &state in &states {
            let node = if nfa.is_final_state(state) {
                "doublecircle"
            } else {
                "circle"
            };
            let color = "black";
            //  if backtrack.contains(&state) {
            //     "red"
            // } else {
            //     "black"
            // };
            let line = format!(
                "\tnode [shape = {}, label=\"{}\", fontsize=12, color={}] {};",
                node, state, color, state
            );
            dot_graph.add_line(&line);
        }

        dot_graph.add_line("");

        let line = "\tnode [shape = point, color=black] q0;";
        dot_graph.add_line(&line);
        let line = format!("\tq0\t->\t{};", nfa.initial_state());
        dot_graph.add_line(&line);

        dot_graph.add_line("");

        for &from in &states {
            if let Some(to_iter) = nfa.next(from, None) {
                for to in to_iter {
                    let line = format!("\t{}\t->\t{}\t[ label = \"{}\" ];", from, to, 'ε');
                    dot_graph.add_line(&line);
                }
            }
            for &input in &alphabet {
                if let Some(to_iter) = nfa.next(from, Some(input)) {
                    for to in to_iter {
                        let line = format!("\t{}\t->\t{}\t[ label = \"{}\" ];", from, to, input);
                        dot_graph.add_line(&line);
                    }
                }
            }
        }

        //for (from, to) in dfa.function.iter()
        //.filter(|(from, _)| first_state <= from.0 && from.0 <= last_state)
        //{
        //   let line = format!("\t{}\t->\t{}\t[ label = \"{}\" ];", from.0, to, from.1);
        //dot_graph.add_line(&line);
        //}
        dot_graph.add_line("}");

        dot_graph
    }
    pub fn from_dfa<'a>(
        dfa: &impl DFA<'a, usize>,
        //tokens: &HashMap<usize, TokenType>,
        //backtrack: &HashSet<usize>,
        //(first_state, last_state): (usize, usize),
    ) -> DotGraph {
        let states: Vec<usize> = dfa.states().collect();
        let alphabet: Vec<char> = dfa.alphabet().collect();

        let mut dot_graph = DotGraph { code: Vec::new() };
        dot_graph.add_line("digraph finite_state_machine {");
        dot_graph.add_line("\trankdir=LR;");
        dot_graph.add_line("\tsize=\"8,5\"");

        dot_graph.add_line("");

        for state in states.iter().filter(|&&state| dfa.is_final_state(state)) {
            let line = format!(
                "\tnode [shape = rectangle, label=\"{} -> {}\", fontsize=12] token{};",
                state, state, state
            );
            dot_graph.add_line(&line);
        }

        dot_graph.add_line("");

        for &state in &states {
            let node = if dfa.is_final_state(state) {
                "doublecircle"
            } else {
                "circle"
            };
            let color = "black";
            //  if backtrack.contains(&state) {
            //     "red"
            // } else {
            //     "black"
            // };
            let line = format!(
                "\tnode [shape = {}, label=\"{}\", fontsize=12, color={}] {};",
                node, state, color, state
            );
            dot_graph.add_line(&line);
        }

        dot_graph.add_line("");

        let line = "\tnode [shape = point, color=black] q0;";
        dot_graph.add_line(&line);
        let line = format!("\tq0\t->\t{};", dfa.initial_state());
        dot_graph.add_line(&line);

        dot_graph.add_line("");

        for &from in &states {
            for &input in &alphabet {
                if let Some(to) = dfa.next(from, input) {
                    let line = format!("\t{}\t->\t{}\t[ label = \"{}\" ];", from, to, input);
                    dot_graph.add_line(&line);
                }
            }
        }

        //for (from, to) in dfa.function.iter()
        //.filter(|(from, _)| first_state <= from.0 && from.0 <= last_state)
        //{
        //   let line = format!("\t{}\t->\t{}\t[ label = \"{}\" ];", from.0, to, from.1);
        //dot_graph.add_line(&line);
        //}
        dot_graph.add_line("}");

        dot_graph
    }
    /*
        pub fn from_dfa2(dfa: &DFA2) -> DotGraph {
            let mut dot_graph = DotGraph { code: Vec::new() };
            dot_graph.add_line("digraph finite_state_machine {");
            dot_graph.add_line("\trankdir=LR;");
            dot_graph.add_line("\tsize=\"8,5\"");

            dot_graph.add_line("");

            for state in dfa
                .states
                .iter()
                .rev()
                .filter(|state| dfa.final_states.contains(&state))
            {
                let line = format!(
                    "\tnode [shape = rectangle, label=\"{} -> {}\", fontsize=12] token{};",
                    state, state, state
                );
                dot_graph.add_line(&line);
            }

            dot_graph.add_line("");

            for state in dfa.states.iter() {
                let node = if dfa.final_states.contains(&state) {
                    "doublecircle"
                } else {
                    "circle"
                };
                let color = "black";
                //  if backtrack.contains(&state) {
                //     "red"
                // } else {
                //     "black"
                // };
                let line = format!(
                    "\tnode [shape = {}, label=\"{}\", fontsize=12, color={}] {};",
                    node, state, color, state
                );
                dot_graph.add_line(&line);
            }

            dot_graph.add_line("");

            let line = "\tnode [shape = point, color=black] q0;";
            dot_graph.add_line(&line);
            let line = format!("\tq0\t->\t{};", dfa.initial_state);
            dot_graph.add_line(&line);

            dot_graph.add_line("");

            for ((from, to), set) in dfa.function.iter() {
                let line = format!(
                    "\t{}\t->\t{}\t[ label = \"{}\" ];",
                    from,
                    to,
                    to_character_class(set.clone().into_iter().collect::<Vec<char>>())
                );
                //dbg!(&line);
                dot_graph.add_line(&line);
            }
            dot_graph.add_line("}");

            dot_graph
        }

    */
    /// Helper function to add a line to the output String.
    fn add_line(&mut self, line: &str) {
        self.code.extend(line.as_bytes());
        self.code.push(b'\n');
    }
}
