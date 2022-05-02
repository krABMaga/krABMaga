/// Module to define Agent methods
pub mod agent;

#[doc(hidden)]
pub mod agentimpl;

/// Folder containing all the fields available on the engine
pub mod fields;
/// File to define the basic structs for locations of the agents used for Fields
pub mod location;
#[doc(hidden)]
pub mod priority;

///File to define the Schedule structure for managing all the agents in the running simulation
pub mod schedule;

/// Module to define State methods
pub mod state;
