pub trait NFA<T> {
    fn initial_state(&self) -> T;
    fn final_states(&self) -> Box<dyn Iterator<Item = T>>;
    fn is_final_state(&self, state: T) -> bool;
    fn states(&self) -> Box<dyn Iterator<Item = T>>;
    // FIXME: Does this use dynamic dispatch? Is there an alternative.
    fn next(&self, state: T, input: Option<char>) -> Option<Box<dyn Iterator<Item = T>>>;
    fn alphabet(&self) -> Box<dyn Iterator<Item = char>>;
}
