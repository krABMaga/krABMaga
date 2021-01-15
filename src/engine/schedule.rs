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
pub struct Schedule<A:'static + Agent + Clone + Send>{
    pub step: usize,
    pub time: f64,
    pub events: Mutex<PriorityQueue<AgentImpl<A>,Priority>>,
    pub pool: Option<ThreadPool>
}

#[derive(Clone)]
pub struct Pair<A: 'static + Agent + Clone > {
    agentimpl: AgentImpl<A>,
    priority: Priority,
}

impl<A: 'static + Agent + Clone  > Pair<A> {
    fn new(agent: AgentImpl<A>, the_priority: Priority) -> Pair<A> {
        Pair {
            agentimpl: agent,
            priority: the_priority
        }
    }
}

impl<A: 'static +  Agent + Clone + Send + Sync > Schedule<A> {

    pub fn new() -> Schedule<A> {
        //println!("Using {} thread",*THREAD_NUM);
        cfg_if!{
            if #[cfg(feature ="parallel")]{
                return Schedule {
                    step: 0,
                    time: 0.0,
                    events: Mutex::new(PriorityQueue::new()),
                    pool: Some(ThreadPoolBuilder::new().num_threads(*THREAD_NUM).build().unwrap()),
                }
            }else{
                return Schedule {
                    step: 0,
                    time: 0.0,
                    events: Mutex::new(PriorityQueue::new()),
                    pool: None,
                }
            }
        }
    }

    pub fn with_threads(thread_num:usize) -> Schedule<A> {
        //println!("Using {} thread",thread_num);
        cfg_if!{
            if #[cfg(feature ="parallel")]{
                return Schedule {
                    step: 0,
                    time: 0.0,
                    events: Mutex::new(PriorityQueue::new()),
                    pool: Some(ThreadPoolBuilder::new().num_threads(thread_num).build().unwrap()),
                }
            }else{
                return Schedule {
                    step: 0,
                    time: 0.0,
                    events: Mutex::new(PriorityQueue::new()),
                    pool: None,
                }
            }
        }
    }

    pub fn schedule_once(&mut self, agent: AgentImpl<A>,the_time:f64, the_ordering:i64) {
        self.events.lock().unwrap().push(agent, Priority{time: the_time, ordering: the_ordering});
    }

    pub fn schedule_repeating(&mut self, agent: A, the_time:f64, the_ordering:i64) {
        let mut a = AgentImpl::new(agent);
        a.repeating = true;
        let pr = Priority::new(the_time, the_ordering);
        self.events.lock().unwrap().push(a, pr);
    }

    pub fn simulate<S: State>(&mut self, state: &mut <A as Agent>::SimState, num_step:u128){
        for _ in 0..num_step{
            self.step(state);
        }
    }


    cfg_if!{
        if #[cfg(feature ="parallel")]{


        pub fn step(&mut self, state: &mut <A as Agent>::SimState){
            let thread_num = self.pool.as_ref().unwrap().current_num_threads();

            if self.step == 0{
                state.update();      
            }
            
            self.step += 1;
        
            // let start: std::time::Instant = std::time::Instant::now();
            let events = &mut self.events;
            if events.lock().unwrap().is_empty() {
                println!("coda eventi vuota");
                return
            }
            
            let thread_division = (events.lock().unwrap().len() as f64 / thread_num as f64).ceil() as usize ;
            let mut cevents: Vec<Vec<Pair<A>>> = vec![Vec::with_capacity(thread_division); thread_num];
            
            let mut i = 0;

            match events.lock().unwrap().peek() {
                Some(item) => {
                    let (_agent, priority) = item;
                    self.time = priority.time;
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
                        if priority.time > self.time {
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
                
        
        state.update();
        }
    }
    else{
        pub fn step(&mut self,state: &mut <A as Agent>::SimState){
            if self.step == 0{
                state.update();
            }
            self.step += 1;


            // let start: std::time::Instant = std::time::Instant::now();
            let events = &mut self.events;
            if events.lock().unwrap().is_empty() {
                println!("coda eventi vuota");
                return;
            }

            let mut cevents: Vec<Pair<A>> = Vec::new();

            match events.lock().unwrap().peek() {
                Some(item) => {
                    let (_agent, priority) = item;
                    self.time = priority.time;
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
                        if priority.time > self.time {
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
                item.agentimpl.agent.step(state);
                if item.agentimpl.repeating {
                    self.schedule_once(
                        item.agentimpl,
                        item.priority.time + 1.0,
                        item.priority.ordering,
                    );
                }
            }
            
            state.update();
            // println!("Time spent calling step method, step {} : {:?}",self.step,start.elapsed());

            }

        }
    }

}



