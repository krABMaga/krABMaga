extern crate priority_queue;

use std::time::Duration;

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
                                        long("n")
                                    ).
                                    get_matches();
                                let n = match matches.value_of("num_thread"){
                                    Some(nt) => match nt.parse::<usize>(){
                                                Ok(ris) => ris,
                                                Err(_) => {
                                                    eprintln!("error: --n value is not an integer");
                                                    num_cpus::get()
                                                }
                                    },
                                    _ => 1
                                };
                                // println!("Using {} threads",n);
                                n
                                };
    
}
cfg_if! {
    if #[cfg(feature ="parallel")]{
        pub struct Schedule<A: 'static + Agent + Clone + Send> {
            pub step: usize,
            pub time: f32,
            pub events: Arc<Mutex<PriorityQueue<AgentImpl<A>, Priority>>>,
            pub thread_num:usize
        }
        
        #[derive(Clone)]
        pub struct Pair<A: 'static + Agent + Clone> {
            agentimpl: AgentImpl<A>,
            priority: Priority,
        }
        
        impl<A: 'static +  Agent + Clone + Send> Schedule<A> {
            pub fn new() -> Schedule<A> {
                Schedule {
                    step: 0,
                    time: 0.0,
                    events: Arc::new(Mutex::new(PriorityQueue::new())),
                    thread_num: *THREAD_NUM
                }
            }

            pub fn with_threads(thread_num: usize) -> Schedule<A> {
                Schedule {
                    step: 0,
                    time: 0.0,
                    events: Arc::new(Mutex::new(PriorityQueue::new())),
                    thread_num,
                }
            }
        
            pub fn schedule_once(&mut self, agent: AgentImpl<A>,the_time:f32, the_ordering:i32) {
              
                self.events.lock().unwrap().push(
                    agent,
                    Priority {
                        time: the_time,
                        ordering: the_ordering,
                    },
                );
            }
        
            pub fn schedule_repeating(&mut self, agent: A, the_time:f32, the_ordering:i32) {
                let mut a = AgentImpl::new(agent);
                a.repeating = true;
                let pr = Priority::new(the_time, the_ordering);
                self.events.lock().unwrap().push(a, pr);
            }
        
            pub fn step(&mut self, state: &mut <A as Agent>::SimState) -> (Duration, Duration, Duration){
                let thread_num = self.thread_num;
                let mut state = Arc::new(state);
                
                if self.step == 0{
                    Arc::get_mut(&mut state).unwrap().update(self.step.clone());
                }
                let fetch_time = std::time::Instant::now();
                self.step += 1;
                
                let fetch_time = fetch_time.elapsed();
               
                let step_time = std::time::Instant::now();

                // println!("Size BEFORE {:?} {:?} {:?}", self.events[0].lock().unwrap().len(), self.events[1].lock().unwrap().len(), self.events[2].lock().unwrap().len());

                match self.events.lock().unwrap().peek() {
                    Some(item) => {
                        let (_agent, priority) = item;
                        self.time = priority.time;
                    },
                    None => panic!("Agent not found - out loop")
                }

                let _result = thread::scope( |scope| {
                    
                    for _ in 0..thread_num{
                        let events = Arc::clone(&self.events);
                        let state = Arc::clone(&state); 
                        let schedule_time = self.time.clone();
                        
                        scope.spawn(move |_| {
                            let mut q = events.lock().unwrap();
                            loop {
                                if q.is_empty() {
                                    break;
                                }
                
                                match q.pop() {
                                    Some(item) => {
                                        let (mut agent, priority) = item;
                                        if priority.time > schedule_time {
                                            q.push(agent, Priority{ time: priority.time, ordering: priority.ordering} );
                                            break;
                                        }
                                       
                                        agent.agent.step(&state);
                                        if agent.repeating {
                                            q.push(agent, Priority{ time: priority.time+1.0, ordering: priority.ordering} );
                                        }
                                    },
                                    None => panic!("Agent not found - inside loop"),
                                }
                            }
                         });
                    }
                });
                // println!("Size AFTER {:?} {:?} {:?}", self.events[0].lock().unwrap().len(), self.events[1].lock().unwrap().len(), self.events[2].lock().unwrap().len());

                let step_time = step_time.elapsed();
                let update_time = std::time::Instant::now();

                Arc::get_mut(&mut state).unwrap().update(self.step.clone());

                let update_time = update_time.elapsed();

                (fetch_time,step_time,update_time)
            }
        }
    }
    else{ // if sequential
        pub struct Schedule<A:'static + Agent + Clone + Send>{
        pub step: usize,
        pub time: f32,
        pub events: PriorityQueue<AgentImpl<A>,Priority>,
        pub newly_scheduled: Vec<A>
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
                    newly_scheduled: Vec::new()
                }
            }
        
            pub fn schedule_once(&mut self, agent: AgentImpl<A>,the_time:f32, the_ordering:i32) {
                self.events.push(agent, Priority{time: the_time, ordering: the_ordering});
            }
        
            pub fn schedule_repeating(&mut self, agent: A, the_time:f32, the_ordering:i32) {
                let mut a = AgentImpl::new(agent);
                a.repeating = true;
                let pr = Priority::new(the_time, the_ordering);
                self.events.push(a, pr);
            }
        
            pub fn step(&mut self, state: &mut <A as Agent>::SimState) -> (Duration, Duration, Duration){
                self.newly_scheduled.clear();
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
        
                let fetch_time = std::time::Instant::now();
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
                let fetch_time = fetch_time.elapsed();
                
                let step_time = std::time::Instant::now();
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
                                self.newly_scheduled.push(agent);
                            }
                        }    
                    }
                let step_time = step_time.elapsed();

                let update_time = std::time::Instant::now();
                state.update(self.step);
                let update_time = update_time.elapsed();

                (fetch_time,step_time,update_time)
        
            }
        }
    }
}

/// A struct used to specify schedule options to pass to an agent's clone when an agent reproduces.
pub struct ScheduleOptions {
    pub ordering: i32,
    pub repeating: bool,
}