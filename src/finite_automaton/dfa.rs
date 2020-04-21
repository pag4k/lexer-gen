pub trait DFA<'a, T> {
    fn initial_state(&self) -> T;
    fn final_states(&self) -> Box<dyn Iterator<Item = T>>;
    fn is_final_state(&self, state: T) -> bool;
    fn states(&self) -> Box<dyn Iterator<Item = T>>;
    fn next(&self, state: T, input: char) -> Option<T>;
    fn alphabet(&self) -> Box<dyn Iterator<Item = char>>;
}