//use crate::finite_accepters::*;
/*
use core::str;

extern crate alloc;
use alloc::vec::Vec;

/// DotGraph ADT
pub struct DotGraph {
    pub code: Vec<u8>,
}

impl DotGraph {
    /// Generate a .gv file based on the section of NFA specified in arguments.
    // pub fn from_nfa(
    //     nfa: &NFA,
    //     //transtions: &HashMap<(usize, char), usize>,
    //     //tokens: &HashMap<usize, TokenType>,
    //     //backtrack: &HashSet<usize>,
    //     (first_state, last_state): (usize, usize),
    // ) -> DotGraph {
    //     let mut dot_graph = DotGraph { code: Vec::new() };
    //     dot_graph.add_line("digraph finite_state_machine {");
    //     dot_graph.add_line("\trankdir=LR;");
    //     dot_graph.add_line("\tsize=\"8,5\"");

    //     dot_graph.add_line("");

    //     for state in (first_state..=last_state)
    //         .clone()
    //         .rev()
    //         .filter(|state| nfa.final_states.contains(&state))
    //     {
    //         let line = format!(
    //             "\tnode [shape = rectangle, label=\"{} -> {}\", fontsize=12] token{};",
    //             state, state, state
    //         );
    //         dot_graph.add_line(&line);
    //     }

    //     dot_graph.add_line("");

    //     for state in first_state..=last_state {
    //         let node = if nfa.final_states.contains(&state) {
    //             "doublecircle"
    //         } else {
    //             "circle"
    //         };
    //         let color = "black";
    //         //  if backtrack.contains(&state) {
    //         //     "red"
    //         // } else {
    //         //     "black"
    //         // };
    //         let line = format!(
    //             "\tnode [shape = {}, label=\"{}\", fontsize=12, color={}] {};",
    //             node, state, color, state
    //         );
    //         dot_graph.add_line(&line);
    //     }

    //     dot_graph.add_line("");

    //     let line = "\tnode [shape = point, color=black] q0;";
    //     dot_graph.add_line(&line);
    //     let line = format!("\tq0\t->\t{};", first_state);
    //     dot_graph.add_line(&line);

    //     dot_graph.add_line("");

    //     for (from, to) in nfa
    //         .function
    //         .iter()
    //         .filter(|(from, _)| first_state <= from.0 && from.0 <= last_state)
    //     {
    //         for n in to {
    //             let line = format!(
    //                 "\t{}\t->\t{}\t[ label = \"{}\" ];",
    //                 from.0,
    //                 n,
    //                 match from.1 {
    //                     Some(c) => c,
    //                     None => 'ε',
    //                 }
    //             );
    //             dot_graph.add_line(&line);
    //         }
    //     }
    //     dot_graph.add_line("}");

    //     dot_graph
    // }
    /*
        pub fn from_dfa(
            dfa: &DFA,
            //transtions: &HashMap<(usize, char), usize>,
            //tokens: &HashMap<usize, TokenType>,
            //backtrack: &HashSet<usize>,
            (first_state, last_state): (usize, usize),
        ) -> DotGraph {
            let mut dot_graph = DotGraph { code: Vec::new() };
            dot_graph.add_line("digraph finite_state_machine {");
            dot_graph.add_line("\trankdir=LR;");
            dot_graph.add_line("\tsize=\"8,5\"");

            dot_graph.add_line("");

            for state in (first_state..=last_state)
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

            for state in first_state..=last_state {
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
            let line = format!("\tq0\t->\t{};", first_state);
            dot_graph.add_line(&line);

            dot_graph.add_line("");

            for (from, to) in dfa
                .function
                .iter()
                .filter(|(from, _)| first_state <= from.0 && from.0 <= last_state)
            {
                let line = format!("\t{}\t->\t{}\t[ label = \"{}\" ];", from.0, to, from.1);
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
*/
