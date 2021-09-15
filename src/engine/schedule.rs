extern crate priority_queue;


use std::sync::Mutex;

use cfg_if::cfg_if;
use clap::{App, Arg};
use lazy_static::*;
use priority_queue::PriorityQueue;
use std::time::Duration;

use crate::engine::agent::Agent;
use crate::engine::agentimpl::AgentImpl;
use crate::engine::priority::Priority;
use crate::engine::state::State;


lazy_static! {
    pub static ref THREAD_NUM: usize =
                                {
                                let matches = App::new("Rust-AB").
                                arg(
                                    Arg::with_name("bench").
                                    long("bench")
                                ).
                                    arg(
                                        Arg::with_name("num_thread").
                                        help("sets the number of threads to use")
                                        .takes_value(true).
                                        long("nt")
                                    ).
                                    get_matches();
                                let n = match matches.value_of("num_thread"){
                                    Some(nt) => match nt.parse::<usize>(){
                                                Ok(ris) => ris,
                                                Err(_) => {
                                                    eprintln!("error: --nt value is not an integer");
                                                    num_cpus::get()
                                                }
                                    },
                                    _ => num_cpus::get()
                                };
                                //println!("Using {} threads",n);
                                n
                                };
}
cfg_if! {
    if #[cfg(feature ="parallel")]{
        pub struct Schedule<A:'static + Agent + Clone + Send>{
            pub step: usize,
            pub time: f32,
            pub events: PriorityQueue<AgentImpl<A>,Priority>,
        }
        
        #[derive(Clone)]
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
        
            pub fn schedule_once(&mut self, agent: AgentImpl<A>,the_time:f32, the_ordering:i64) {
                self.events.push(agent, Priority{time: the_time, ordering: the_ordering});
            }
        
            pub fn schedule_repeating(&mut self, agent: A, the_time:f32, the_ordering:i64) {
                let mut a = AgentImpl::new(agent);
                a.repeating = true;
                let pr = Priority::new(the_time, the_ordering);
                self.events.push(a, pr);
            }
        
            pub fn step(&mut self, state: &mut <A as Agent>::SimState) -> (Duration, Duration, Duration){
                if self.step == 0{
                    state.update(self.step);
                }
        
                self.step += 1;
        
                let events = &mut self.events;
                if events.is_empty() {
                    println!("Empty Queue");
                    ()
                }
        
                let mut cevents: Vec<Pair<A>> = Vec::new();
        
        
                match events.peek() {
                    Some(item) => {
                        let (_agent, priority) = item;
                        self.time = priority.time;
                    },
                    None => panic!("Agent not found - out loop"),
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
                        None => panic!("Agent not found - inside loop"),
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
                //println!("STEP,{},SCHEDULE,{}", self.step,cevents.len());
                for mut item in cevents.into_iter() {
        
                        item.agentimpl.agent.step(state);
        
                        let should_remove = item.agentimpl.agent.should_remove(&state);
                        let should_reproduce = item.agentimpl.agent.should_reproduce(&state);
        
                        if item.agentimpl.repeating && !should_remove {
                            self.schedule_once(
                                item.agentimpl,
                                item.priority.time + 1.0,
                                item.priority.ordering,
                            );
                        }
        
                        if let Some(new_agents) = should_reproduce {
                            for (new_agent, schedule_options) in new_agents {
                                let ScheduleOptions{ordering, repeating} = schedule_options;
                                let agent = *new_agent;
                                let mut new_agent_impl = AgentImpl::new(agent.clone());
                                new_agent_impl.repeating = repeating;
                                self.schedule_once(new_agent_impl, item.priority.time + 1., ordering);
                            }
                        }
                }
                state.update(self.step);
                (Duration::new(5, 0), Duration::new(5, 0), Duration::new(5, 0))
        
            }
        
        }
    }

    // if sequential
    else{
        pub struct Schedule<A:'static + Agent + Clone + Send>{
            pub step: usize,
            pub time: f32,
            pub events: PriorityQueue<AgentImpl<A>,Priority>,
        }
        
        #[derive(Clone)]
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
        
            pub fn schedule_once(&mut self, agent: AgentImpl<A>,the_time:f32, the_ordering:i64) {
                self.events.push(agent, Priority{time: the_time, ordering: the_ordering});
            }
        
            pub fn schedule_repeating(&mut self, agent: A, the_time:f32, the_ordering:i64) {
                let mut a = AgentImpl::new(agent);
                a.repeating = true;
                let pr = Priority::new(the_time, the_ordering);
                self.events.push(a, pr);
            }
        
            pub fn step(&mut self, state: &mut <A as Agent>::SimState) -> (Duration, Duration, Duration){
                if self.step == 0{
                    state.update(self.step);
                }
        
                self.step += 1;
        
                let events = &mut self.events;
                if events.is_empty() {
                    println!("Empty Queue");
                    ()
                }
        
                let mut cevents: Vec<Pair<A>> = Vec::new();
        
        
                match events.peek() {
                    Some(item) => {
                        let (_agent, priority) = item;
                        self.time = priority.time;
                    },
                    None => panic!("Agent not found - out loop"),
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
                        None => panic!("Agent not found - inside loop"),
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
                //println!("STEP,{},SCHEDULE,{}", self.step,cevents.len());
                for mut item in cevents.into_iter() {
        
                        item.agentimpl.agent.step(state);
        
                        let should_remove = item.agentimpl.agent.should_remove(&state);
                        let should_reproduce = item.agentimpl.agent.should_reproduce(&state);
        
                        if item.agentimpl.repeating && !should_remove {
                            self.schedule_once(
                                item.agentimpl,
                                item.priority.time + 1.0,
                                item.priority.ordering,
                            );
                        }
        
                        if let Some(new_agents) = should_reproduce {
                            for (new_agent, schedule_options) in new_agents {
                                let ScheduleOptions{ordering, repeating} = schedule_options;
                                let agent = *new_agent;
                                let mut new_agent_impl = AgentImpl::new(agent.clone());
                                new_agent_impl.repeating = repeating;
                                self.schedule_once(new_agent_impl, item.priority.time + 1., ordering);
                            }
                        }
                }
                state.update(self.step);
                (Duration::new(5, 0), Duration::new(5, 0), Duration::new(5, 0))
        
            }
        
        }
    }
}

/// A struct used to specify schedule options to pass to an agent's clone when an agent reproduces.
pub struct ScheduleOptions {
    pub ordering: i64,
    pub repeating: bool,
}