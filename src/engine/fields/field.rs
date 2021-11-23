pub trait Field {
    // Swap the state of the field and updates the writing state.
    fn update(&mut self) {}
    // Swap the state of the field and cleans the writing state.
    fn lazy_update(&mut self) {}
}
