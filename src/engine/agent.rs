use crate::engine::state::State;

///Represents an agent of the simulation, encapsulating its behavioural logic.
pub trait Agent {
    /// # Associated Type
    ///The simulation state type. Since the Rust-AB simulation state is implemented by a user-defined struct, this parametrization allows the trait Agent to work with any struct
    /// that implements the traits State, Sync and Send (for safe use in concurrent executions).
    type SimState: State + Sync + Send;

    ///Executes the agent's behavioural logic. In order to allow agents to read and write in the global simulation state, this method takes a reference to the state as argument.
    fn step(&mut self,state: &Self::SimState);

    fn id(&self) -> u128;
   
}