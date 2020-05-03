pub trait NFA<T> {
    fn initial_state(&self) -> T;
    fn final_states(&self) -> Box<dyn Iterator<Item = T>>;
    fn is_final_state(&self, state: T) -> bool;
    fn states(&self) -> &[T];
    // FIXME: Does this use dynamic dispatch? Is there an alternative.
    fn next(&self, state: T, input: Option<u8>) -> Option<&[T]>;
    fn alphabet(&self) -> &[u8];
}
