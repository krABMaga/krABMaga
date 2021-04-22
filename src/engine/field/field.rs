///This trait encapsulates the simulation field's update logic, allowing two possibilities: lazy update and update.
pub trait Field {
    /// Swap the state of the field and updates the writing state.
    fn update(&self) {}
    /// Swap the state of the field and cleans the writing state.
    fn lazy_update(&self) {}
}
