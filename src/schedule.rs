extern crate priority_queue;

use priority_queue::PriorityQueue;
use crate::priority::Priority;
use crate::agent::Agent;

pub struct Schedule {
    step: usize,
    time: f64,
    queue:PriorityQueue<Agent,Priority>
}

impl Schedule {
    pub fn new() -> Schedule {
        Schedule {
            step: 0,
            time: 0.0,
            queue: PriorityQueue::new(),
        }
    }
    pub fn schedule_once(&mut self, agent:Agent, time:f64, ordering:i64){
        self.queue.push(agent, Priority{time: time, priority: ordering});
    }
    pub fn schedule_repeating(&mut self, agent:Agent, time:f64, ordering:i64){
        self.queue.push(agent, Priority{time: time, priority: ordering});
    }
    pub fn step(&mut self){
        
    }

}
