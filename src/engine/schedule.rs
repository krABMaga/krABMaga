extern crate priority_queue;

use crate::engine::{agent::Agent, agentimpl::AgentImpl, priority::Priority, state::State};

use cfg_if::cfg_if;
use clap::{App, Arg};
use lazy_static::*;
use priority_queue::PriorityQueue;

cfg_if! {
    if #[cfg(feature ="parallel")]{
        use crossbeam::thread;
        use std::sync::{Arc,Mutex};
    }
}

lazy_static! {
    pub static ref THREAD_NUM: usize = {
        let matches = App::new("Rust-AB")
            .arg(Arg::with_name("bench").long("bench"))
            .arg(
                Arg::with_name("num_thread")
                    .help("sets the number of threads to use")
                    .takes_value(true)
                    .long("nt"),
            )
            .get_matches();
        let n = match matches.value_of("num_thread") {
            Some(nt) => match nt.parse::<usize>() {
                Ok(ris) => ris,
                Err(_) => {
                    eprintln!("error: --nt value is not an integer");
                    num_cpus::get()
                }
            },
            _ => 1,
        };
        n
    };
}
cfg_if! {
    if #[cfg(feature ="parallel")] {
        pub struct Schedule {
            pub step: usize,
            pub time: f32,
            pub events: Arc<Mutex<PriorityQueue<AgentImpl, Priority>>>,
            pub thread_num:usize
        }

        #[derive(Clone)]
        pub struct Pair {
            agentimpl: AgentImpl,
            priority: Priority,
        }

        impl Pair {
            fn new(agent: AgentImpl, the_priority: Priority) -> Pair {
                Pair {
                    agentimpl: agent,
                    priority: the_priority
                }
            }
        }

        impl Schedule {
            pub fn new() -> Schedule {
                Schedule {
                    step: 0,
                    time: 0.0,
                    events: Arc::new(Mutex::new(PriorityQueue::new())),
                    thread_num: *THREAD_NUM
                }
            }

            pub fn with_threads(thread_num: usize) -> Schedule {
                Schedule {
                    step: 0,
                    time: 0.0,
                    events: Arc::new(Mutex::new(PriorityQueue::new())),
                    thread_num,
                }
            }

            pub fn schedule_once(&mut self, agent: AgentImpl, the_time:f32, the_ordering:i32) {
                self.events.lock().unwrap().push(
                    agent,
                    Priority {
                        time: the_time,
                        ordering: the_ordering,
                    },
                );
            }

            pub fn schedule_repeating(&mut self, agent: Box<dyn Agent>, the_time:f32, the_ordering:i32) {
                let mut a = AgentImpl::new(agent);
                a.repeating = true;
                let pr = Priority::new(the_time, the_ordering);
                self.events.lock().unwrap().push(a, pr);
            }

            pub fn get_all_events(&self) -> Vec<Box<dyn Agent>>{
                let mut tor: Vec<Box<dyn Agent>> = Vec::new();
                for e in self.events.lock().unwrap().iter(){
                    tor.push(e.0.agent.clone());
                }
                tor
            }

            pub fn step(&mut self, state: &mut dyn State) {

                let thread_num = self.thread_num;
                
                let mut state = Arc::new(state);
                
                if self.step == 0{
                    Arc::get_mut(&mut state).unwrap().update(self.step.clone() as u64);
                }

                Arc::get_mut(&mut state).unwrap().before_step(self);
                if self.events.lock().unwrap().is_empty() {
                    println!("No agent in the queue to schedule. Terminating.");
                    std::process::exit(0);
                }

                match self.events.lock().unwrap().peek() {
                    Some(item) => {
                        let (_agent, priority) = item;
                        self.time = priority.time;
                    },
                    None => panic!("Agent not found - out loop")
                }

                println!("Parallel step: {}", self.step);
                let _result = thread::scope( |scope| {
                    for tid in 0..thread_num{
                        let events = Arc::clone(&self.events);
                        let mut state = Arc::clone(&state);
                        let schedule_time = self.time.clone();

                        scope.spawn(move |_| {

                            loop {
                            
                                match Arc::get_mut(&mut state) {

                                    Some(state) => {
                                        println!("Thread diocane {}", tid);

                                        let mut q = events.lock().unwrap();

                                        if q.is_empty() {
                                            break;
                                        }

                                        let mut item = q.pop();

                                        std::mem::drop(q);

                                        if item.is_some(){
                                    
                                            let mut item = item.unwrap();
                                            let state = state.as_state_mut();
                                            // let mut state = Arc::get_mut(&mut state).unwrap().as_state_mut();
        
                                            item.0.agent.before_step(state);
        
                                            item.0.agent.step(state, item.0.id);
        
                                            item.0.agent.after_step(state);
        
                                            if item.0.repeating && !item.0.agent.is_stopped(state) {
                                                let mut q = events.lock().unwrap();
                                                q.push(
                                                    item.0,
                                                    Priority {
                                                        time: item.1.time + 1.0,
                                                        ordering: item.1.ordering,
                                                    },
                                                );
                                            }
        
                                        } else {
                                            panic!("Agent not found - inside loop")
                                        }
                                    },
                                    None => {
                                        continue;
                                    }
                                }
                                
                                
                            }
                         });
                    }
                });
                Arc::get_mut(&mut state).unwrap().after_step(self);
                self.step += 1;
                Arc::get_mut(&mut state).unwrap().update(self.step.clone() as u64);
            }
        }
    }
    // SEQUENTIAL IF
    else{
        pub struct Schedule{
            pub step: u64,
            pub time: f32,
            pub events: PriorityQueue<AgentImpl,Priority>
        }

        #[derive(Clone)]
        pub struct Pair{
            agentimpl: AgentImpl,
            priority: Priority,
        }

        impl Pair {
            fn new(agent: AgentImpl, the_priority: Priority) -> Pair {
                Pair {
                    agentimpl: agent,
                    priority: the_priority
                }
            }
        }

        impl Default for Schedule {
            fn default() -> Self {
                Self::new()
            }
        }

        impl Schedule {
            pub fn new() -> Schedule {
                Schedule {
                    step: 0,
                    time: 0.0,
                    events: PriorityQueue::new(),
                }
            }

            pub fn schedule_once(&mut self, agent: AgentImpl,the_time:f32, the_ordering:i32) {
                self.events.push(agent, Priority{time: the_time, ordering: the_ordering});
            }

            pub fn schedule_repeating(&mut self, agent: Box<dyn Agent>, the_time:f32, the_ordering:i32) {
                let mut a = AgentImpl::new(agent);
                //let id = a.id.clone();
                a.repeating = true;
                let pr = Priority::new(the_time, the_ordering);
                self.events.push(a, pr);
            }

            pub fn get_all_events(&self) -> Vec<Box<dyn Agent>>{
                let mut tor: Vec<Box<dyn Agent>> = Vec::new();
                for e in self.events.iter(){
                    tor.push(e.0.agent.clone());
                }
                tor
            }

            // pub fn update_event(&mut self, id: u32, agent: Box<dyn Agent>, repeating: bool ) {

            //     let agent = AgentImpl{id, agent: agent.clone(), repeating};

            //     let event = self.events.get_mut(&agent);

            //     if let Some(mut e) = event {
            //         e.0.agent = agent.agent;
            //     }

            // }

            pub fn step(&mut self, state: &mut dyn State){

                if self.step == 0{
                    state.update(self.step);
                }

                state.before_step(self);

                let events = &mut self.events;

                if events.is_empty() {
                    println!("No agent in the queue to schedule. Terminating.");
                    std::process::exit(0);
                }

                let mut cevents: Vec<Pair> = Vec::new();

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

                    match events.pop() {
                        Some(item) => {
                            let (_agent, priority) = item;
                            if priority.time > self.time {
                                break;
                            }
                            cevents.push(Pair::new(_agent, priority));
                        },
                        None => panic!("Agent not found - inside loop"),
                    }
                }

                for mut item in cevents.into_iter() {

                    item.agentimpl.agent.before_step(state);
                    item.agentimpl.agent.step(state);
                    item.agentimpl.agent.after_step(state);

                    if item.agentimpl.repeating && !item.agentimpl.agent.is_stopped(state) {
                        self.schedule_once(
                            item.agentimpl,
                            item.priority.time + 1.0,
                            item.priority.ordering,
                        );
                    }
                }

                state.after_step(self);
                self.step += 1;
                state.update(self.step);
            }
        }
    }
}

// A struct used to specify schedule options to pass to an agent's clone when an agent reproduces.
pub struct ScheduleOptions {
    pub ordering: i32,
    pub repeating: bool,
}
