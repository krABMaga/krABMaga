extern crate priority_queue;

use priority_queue::PriorityQueue;
use crate::priority::Priority;
use crate::agent::Agent;
use crate::agentimpl::AgentImpl;
use crate::simstate::SimState;

pub struct Schedule<A: Agent + Clone>{
    step: usize,
    time: f64,
    events: PriorityQueue<AgentImpl<A>,Priority>
}

struct Pair<A: Agent + Clone> {
    agentimpl: AgentImpl<A>,
    priority: Priority,
}

impl<A: Agent + Clone> Pair<A> {
    fn new(agent: AgentImpl<A>, priority: Priority) -> Pair<A> {
        Pair {
            agentimpl: agent,
            priority: priority
        }
    }
}

impl<A: Agent + Clone> Schedule<A> {
    pub fn new() -> Schedule<A> {
        Schedule {
            step: 0,
            time: 0.0,
            events: PriorityQueue::new(),
        }
    }
    pub fn schedule_once(&mut self, agent: AgentImpl<A>, time:f64, ordering:i64) {
        self.events.push(agent, Priority{time: time, ordering: ordering});
    }

    pub fn schedule_repeating(&mut self, mut agent: AgentImpl<A>, time:f64, ordering:i64) {
        agent.repeating = true;
        self.events.push(agent, Priority{time: time, ordering: ordering});
    }

    pub fn step(&mut self, simstate: &SimState<A>){
        self.step += 1;
        let mut events = &mut self.events;
        if events.is_empty()
            { return; }

        match events.peek() {
            Some(item) => {
                let (_agent, priority) = item;
                self.time = priority.time;
            },
            None => panic!("agente non trovato"),
        }


        let mut cevents: Vec<Pair<A>> = Vec::new();

        loop {
            if events.is_empty() {
                break;
            }

            match events.peek() {
                Some(item) => {
                    let (_agent, priority) = item;
                    if priority.time > self.time {
                        break;
                    }
                },
                None => panic!("agente non trovato"),
            }

            let item = events.pop();
            match item {
                Some(item) => {
                    let (agent, priority) = item;
                    cevents.push(Pair::new(agent, priority));
                },
                None => panic!("no item"),
            }

        }

        for item in cevents.into_iter() {

            let agentimpl2 = item.agentimpl.clone();
            if item.agentimpl.repeating {
                self.schedule_once(agentimpl2, item.priority.time + 1.0, item.priority.ordering);
            }

            item.agentimpl.step(simstate);

        }

    }

}
