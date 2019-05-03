extern crate priority_queue;

use priority_queue::PriorityQueue;
use crate::priority::Priority;
use crate::agent::Agent;
use crate::agentimpl::AgentImpl;

pub struct Schedule<A: Agent + Clone>{
    pub step: usize,
    pub time: f64,
    pub events: PriorityQueue<AgentImpl<A>,Priority>,
}

pub struct Pair<A: Agent + Clone> {
    agentimpl: AgentImpl<A>,
    priority: Priority,
}

impl<A: Agent + Clone> Pair<A> {
    fn new(agent: AgentImpl<A>, the_priority: Priority) -> Pair<A> {
        Pair {
            agentimpl: agent,
            priority: the_priority
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

    pub fn schedule_once(&mut self, agent: AgentImpl<A>,the_time:f64, the_ordering:i64) {
        self.events.push(agent, Priority{time: the_time, ordering: the_ordering});
    }

    pub fn schedule_repeating(&mut self, agent: A, the_time:f64, the_ordering:i64) {
        let mut a = AgentImpl::new(agent);
        a.repeating = true;
        let pr = Priority::new(the_time, the_ordering);
        self.events.push(a, pr);
    }

    pub fn step(&mut self){
        self.step += 1;
        println!("----{}----", self.step);
        let events = &mut self.events;
        if events.is_empty() {
            println!("coda eventi vuota");
            return
        }

        let mut cevents: Vec<Pair<A>> = Vec::new();

        match events.peek() {
            Some(item) => {
                let (_agent, priority) = item;
                self.time = priority.time;
            },
            None => panic!("agente non trovato"),
        }

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
                    // let x = agent.id.clone();
                    // println!("{}", x);
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

            item.agentimpl.step();
        }
    }
}
