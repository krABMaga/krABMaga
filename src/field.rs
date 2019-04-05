use std::collections::HashMap;
use crate::agent::Agent;
use crate::agentimpl::AgentImpl;

#[derive(Default)]
pub struct Field<A: Agent + Clone> {
    pub hash_map : HashMap<u32, AgentImpl<A>>,
}
