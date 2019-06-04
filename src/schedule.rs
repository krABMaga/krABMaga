extern crate priority_queue;
extern crate threads_pool;

//use std::sync::{Arc, Mutex};
use std::sync::Arc;
use std::sync::Mutex;

use threads_pool::*;
use priority_queue::PriorityQueue;
use crate::priority::Priority;
use crate::agent::Agent;
use crate::agentimpl::AgentImpl;

pub struct Schedule<A:'static + Agent + Clone + Send>{
    pub step: usize,
    pub time: f64,
    pub events: PriorityQueue<AgentImpl<A>,Priority>,
}

pub struct Pair<A: 'static + Agent + Clone> {
    agentimpl: AgentImpl<A>,
    priority: Priority,
}

impl<A: 'static + Agent + Clone> Pair<A> {
    fn new(agent: AgentImpl<A>, the_priority: Priority) -> Pair<A> {
        Pair {
            agentimpl: agent,
            priority: the_priority
        }
    }
}

impl<A: 'static +  Agent + Clone + Send> Schedule<A> {

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

        let pool = ThreadPool::new(4);


        for mut item in cevents.into_iter() {
            //let data = Arc::clone(&state);

            // pool.execute(move || {
            //     //let mut data = data.lock().unwrap();
                 item.agentimpl.step();
            //     //let agentimpl2 = item.agentimpl.clone();
            //
                 if item.agentimpl.repeating {
                     self.schedule_once(item.agentimpl, item.priority.time + 1.0, item.priority.ordering);
                 }
            // });

        }
    }
}

pub fn simulate<A: 'static +  Agent + Clone + Send>( mut schedule: Arc<Mutex<Schedule<A>>>){

        //let sched: Arc<Mutex<Schedule<A>>> = Arc::new(Mutex::new(*schedule);

//     schedule.step += 1;
//     let mut cevents: Vec<Pair<A>> = Vec::new();
//     println!("----{}----", schedule.step);
// {
//     let events = &mut schedule.events;
//     if events.is_empty() {
//         println!("coda eventi vuota");
//         return
//     }
//
//     match events.peek() {
//         Some(item) => {
//             let (_agent, priority) = item;
//             schedule.time = priority.time;
//         },
//         None => panic!("agente non trovato"),
//     }
//
//     loop {
//         if events.is_empty() {
//             break;
//         }
//
//         match events.peek() {
//             Some(item) => {
//                 let (_agent, priority) = item;
//                 if priority.time > schedule.time {
//                     break;
//                 }
//             },
//             None => panic!("agente non trovato"),
//         }
//
//         let item = events.pop();
//         match item {
//             Some(item) => {
//                 let (agent, priority) = item;
//                 // let x = agent.id.clone();
//                 // println!("{}", x);
//                 cevents.push(Pair::new(agent, priority));
//             },
//             None => panic!("no item"),
//         }
//     }
// }
//     let pool = ThreadPool::new(4);
//
//
//
//     for mut item in cevents.into_iter() {
//         //let data = Arc::clone(&state);
//         let s = sched.clone();
//         pool.execute(move || {
//             //let mut data = data.lock().unwrap();
//             item.agentimpl.step();
//             //let agentimpl2 = item.agentimpl.clone();
//
//             if item.agentimpl.repeating {
//                 s.lock().unwrap().schedule_once(item.agentimpl, item.priority.time + 1.0, item.priority.ordering);
//             }
//         });
//
//     }
}
