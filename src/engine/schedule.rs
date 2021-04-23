extern crate priority_queue;


use priority_queue::PriorityQueue;
use crate::engine::priority::Priority;
use crate::engine::agent::Agent;
use crate::engine::agentimpl::AgentImpl;
use crate::engine::state::State;
use std::sync::Mutex;
use rayon::{ThreadPoolBuilder,ThreadPool};
use lazy_static::*;
use clap::{App, Arg};
use cfg_if::cfg_if;
use crossbeam::queue::SegQueue;
use std::hash::Hash;

lazy_static!{
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

///A scheduler providing functionalities to manage the simulation according to event-based scheduling.
///Schedule works with a FIFO priority queue that sorts agents based on scheduling time and an integer priority value.
pub struct Schedule<A:'static + Agent + Clone + Send + Hash + Eq>{
    pub step: Mutex<usize>,
    pub time: Mutex<f64>,
    pub events: Mutex<PriorityQueue<AgentImpl<A>,Priority>>,
    pub events_buf: SegQueue<Pair<A>>,
    pub pool: Option<ThreadPool>,
    pub stepping: Mutex<bool>,
    removed: Mutex<Vec<AgentImpl<A>>>
}

#[derive(Clone)]
pub struct Pair<A: 'static + Agent + Clone + Hash + Eq > {
    agentimpl: AgentImpl<A>,
    priority: Priority,
}

impl<A: 'static + Agent + Clone + Hash + Eq > Pair<A> {
    fn new(agent: AgentImpl<A>, the_priority: Priority) -> Pair<A> {
        Pair {
            agentimpl: agent,
            priority: the_priority
        }
    }
}

impl<A: 'static +  Agent + Clone + Send + Sync + Hash + Eq > Schedule<A> {
    ///Instantiates a new scheduler.
    ///When the "parallel" feature is specified, the new Schedule object will hold a ThreadPool for use in parallel executions.
    pub fn new() -> Schedule<A> {
        //println!("Using {} thread",*THREAD_NUM);
        cfg_if!{
            if #[cfg(feature ="parallel")]{
                return Schedule {
                    step: Mutex::new(0),
                    time: Mutex::new(0.0),
                    events: Mutex::new(PriorityQueue::new()),
                    events_buf: SegQueue::new(),
                    pool: Some(ThreadPoolBuilder::new().num_threads(*THREAD_NUM).build().unwrap()),
                    stepping:Mutex::new(false),
                    removed: Mutex::new(Vec::new()),
                }
            }else{
                return Schedule {
                    step: Mutex::new(0),
                    time: Mutex::new(0.0),
                    events: Mutex::new(PriorityQueue::new()),
                    events_buf: SegQueue::new(),
                    pool: None,
                    stepping:Mutex::new(false),
                    removed: Mutex::new(Vec::new()),
                }
            }
        }
    }

    ///Instantiates a new scheduler, specifying the number of threads to use.
    pub fn with_threads(_thread_num:usize) -> Schedule<A> {
        //println!("Using {} thread",thread_num);
        cfg_if!{
            if #[cfg(feature ="parallel")]{
                return Schedule {
                    step: Mutex::new(0),
                    time: Mutex::new(0.0),
                    events: Mutex::new(PriorityQueue::new()),
                    events_buf: SegQueue::new(),
                    pool: Some(ThreadPoolBuilder::new().num_threads(_thread_num).build().unwrap()),
                    stepping: Mutex::new(false),
                    removed: Mutex::new(Vec::new()),
                }
            }else{
                return Schedule {
                    step: Mutex::new(0),
                    time: Mutex::new(0.0),
                    events: Mutex::new(PriorityQueue::new()),
                    events_buf: SegQueue::new(),
                    pool: None,
                    stepping:Mutex::new(false),
                    removed: Mutex::new(Vec::new()),
                }
            }
        }
    }

    ///Schedule an agent for a specific simulation step at time the_time, with priority value the_ordering.
    pub fn schedule_once(&self, agent: AgentImpl<A>,the_time:f64, the_ordering:i64) {
        if *self.stepping.lock().unwrap(){
            self.events_buf.push(Pair::new(agent,Priority{time: the_time, ordering: the_ordering}));    
        }else{
            self.events.lock().unwrap().push(agent, Priority{time: the_time, ordering: the_ordering});
        }
    }
    ///Schedule an agent for a specific simulation step at time the_time, with priority value the_ordering, re-scheduling it for every subsequent step.
    pub fn schedule_repeating(&self, agent: A, the_time:f64, the_ordering:i64) {
        let mut a = AgentImpl::new(agent);
        a.repeating = true;
        let pr = Priority::new(the_time, the_ordering);
        if *self.stepping.lock().unwrap(){
            self.events_buf.push(Pair::new(a,pr));
        }else{
            self.events.lock().unwrap().push(a, pr);
        }
    }

    pub fn remove(&self, agent: A){
        let a = AgentImpl::new(agent);
        match (*self.events.lock().unwrap()).remove(&a){
            Some(_)=>{ },
            None=>{ self.removed.lock().unwrap().push(a);},
        }
    }

    pub fn step_count(&self) -> usize{
        *self.step.lock().unwrap()
    }


    ///Executes num_step simulation steps.
    pub fn simulate<S: State>(&mut self, state: &mut <A as Agent>::SimState, num_step:u128){
        for _ in 0..num_step{
            self.step(state);
        }
    }

    
    cfg_if!{
        if #[cfg(feature ="parallel")]{
        ///Executes one simulation step.
        ///When the feature "parallel" is specified, this method executes the agents' behaviour concurrently, using the instantiated ThreadPool.
        ///At the end of the step, the simulation state is updated.
        pub fn step(&self, state: &<A as Agent>::SimState){
            let thread_num = self.pool.as_ref().unwrap().current_num_threads();
            *self.stepping.lock().unwrap() = true;
            if *self.step.lock().unwrap() == 0{
                state.update();      
            }
            
            *self.step.lock().unwrap() += 1;
        
            // let start: std::time::Instant = std::time::Instant::now();
            let events = &self.events;
            if events.lock().unwrap().is_empty() {
                println!("coda eventi vuota");
                return
            }
            
            
            for x in self.removed.lock().unwrap().iter(){
                events.lock().unwrap().remove(x);
            }
            
            self.removed.lock().unwrap().clear();
            
            let thread_division = (events.lock().unwrap().len() as f64 / thread_num as f64).ceil() as usize ;
            let mut cevents = Vec::with_capacity(thread_num);
            for i in 0..thread_num{
                cevents.push(Vec::with_capacity(thread_division));
            }
            
            
            let mut i = 0;

            match events.lock().unwrap().peek() {
                Some(item) => {
                    let (_agent, priority) = item;
                    *self.time.lock().unwrap() = priority.time;
                },
                None => panic!("agente non trovato"),
            }

            loop {
                if events.lock().unwrap().is_empty() {
                    break;
                }

                match events.lock().unwrap().peek() {
                    Some(item) => {
                        let (_agent, priority) = item;
                        if priority.time > *self.time.lock().unwrap() {
                            break;
                        }
                    },
                    None => panic!("agente non trovato"),
                }

                let item = events.lock().unwrap().pop();
                
                match item {
                    Some(item) => {
                        let (agent, priority) = item;
                        // let x = agent.id.clone();
                        // println!("{}", x);
                      
                        let index = match thread_num{
                            0 => 0,
                            _ => i%thread_num
                        };
                        
                        cevents[index].push(Pair::new(agent, priority));
                        
                        i+=1;
                    },
                    None => panic!("no item"),
                }
            }
            
            self.pool.as_ref().unwrap().scope( |scope| {
                for _ in 0..thread_num{            
                    let batch = cevents.pop().unwrap();            
                    scope.spawn(|_| {             
                        let mut reschedule = Vec::with_capacity(batch.len());               
                        for mut item in batch{              
                            item.agentimpl.agent.step(&state);              
                            if item.agentimpl.repeating{   
                                reschedule.push( ( item.agentimpl, Priority{ time: item.priority.time+1.0, ordering: item.priority.ordering}) );   
                            }
                        }
                        let mut events = self.events.lock().unwrap();
                        for entry in reschedule{
                            events.push(entry.0,entry.1);
                        } 
                    });
                    
                }
            });
            
            while !self.events_buf.is_empty(){
                let pair = self.events_buf.pop().unwrap();
                self.events.lock().unwrap().push(pair.agentimpl,pair.priority);
            }

        state.update();
        *self.stepping.lock().unwrap() = false;
       
        }
    }
    else{
        ///Executes one simulation step.
        ///When the feature "parallel" is specified, this method executes the agents' behaviour concurrently, using the instantiated ThreadPool.
        ///At the end of the step, the simulation state is updated.
        pub fn step(&self,state: &<A as Agent>::SimState){
            *self.stepping.lock().unwrap() = true;
            if *self.step.lock().unwrap() == 0{
                state.update();
            }
            *self.step.lock().unwrap() += 1;


            // let start: std::time::Instant = std::time::Instant::now();
            let events = &self.events;
            if events.lock().unwrap().is_empty() {
                println!("coda eventi vuota");
                return;
            }

           
            for x in self.removed.lock().unwrap().iter(){
                events.lock().unwrap().remove(x);
            }
            self.removed.lock().unwrap().clear();

            let mut cevents: Vec<Pair<A>> = Vec::new();

            match events.lock().unwrap().peek() {
                Some(item) => {
                    let (_agent, priority) = item;
                    *self.time.lock().unwrap() = priority.time;
                }
                None => panic!("agente non trovato"),
            }

            loop {
                if events.lock().unwrap().is_empty() {
                    break;
                }

                match events.lock().unwrap().peek() {
                    Some(item) => {
                        let (_agent, priority) = item;
                        if priority.time > *self.time.lock().unwrap() {
                            break;
                        }
                    }
                    None => panic!("agente non trovato"),
                }

                let item = events.lock().unwrap().pop();
                match item {
                    Some(item) => {
                        let (agent, priority) = item;
                        // let x = agent.id.clone();
                        // println!("{}", x);
                        cevents.push(Pair::new(agent, priority));
                    }
                    None => panic!("no item"),
                }
            }
        
            for mut item in cevents.into_iter() {
                // println!("agentimpl{} steps",item.agentimpl.agent.id());
                item.agentimpl.agent.step(state);
                if item.agentimpl.repeating {
                    self.schedule_once(
                        item.agentimpl,
                        item.priority.time + 1.0,
                        item.priority.ordering,
                    );
                }
            }
            
           
            while !self.events_buf.is_empty(){
                let pair = self.events_buf.pop().unwrap();
                self.events.lock().unwrap().push(pair.agentimpl,pair.priority);
            }

            state.update();
            // println!("Time spent calling step method, step {} : {:?}",self.step,start.elapsed());
            *self.stepping.lock().unwrap() = false;
            }

        }
    }

}



