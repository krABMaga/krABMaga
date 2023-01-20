/// Trait for implement a generic Field in the engine
///
/// update and lazy update allow to update the field whenever is requested
pub trait Field {
    /// Swap the state of the field and updates the writing state.
    fn update(&mut self) {}
    /// Swap the state of the field and cleans the writing state.
    fn lazy_update(&mut self) {}
}
