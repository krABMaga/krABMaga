extern crate priority_queue;

use std::sync::Mutex;

use cfg_if::cfg_if;
use clap::{App, Arg};
use lazy_static::*;
use priority_queue::PriorityQueue;
//use rayon::ThreadPool;
use crossbeam::thread;
#[cfg(feature = "parallel")]
use rayon::ThreadPoolBuilder;

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
pub struct Schedule<A: 'static + Agent + Clone + Send> {
    pub step: usize,
    pub time: f32,
    pub events: Mutex<PriorityQueue<AgentImpl<A>, Priority>>,
    pub thread_num:usize,
    //pub pool: Option<ThreadPool>,
    // Mainly used in the visualization to render newly scheduled agents.
    // This is cleared at the start of each step.
    pub newly_scheduled: Mutex<Vec<A>>,
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
            priority: the_priority,
        }
    }
}

impl<A: 'static + Agent + Clone + Send + Sync> Schedule<A> {
    pub fn new() -> Schedule<A> {
        //println!("Using {} thread",*THREAD_NUM);
        cfg_if! {
            if #[cfg(feature ="parallel")]{
                return Schedule {
                    step: 0,
                    time: 0.0,
                    events: Mutex::new(PriorityQueue::new()),
                    thread_num: THREAD_NUM,
                    //pool: Some(ThreadPoolBuilder::new().num_threads(*THREAD_NUM).build().unwrap()),
                    newly_scheduled: Mutex::new(Vec::new())
                }
            }else{
                return Schedule {
                    step: 0,
                    time: 0.0,
                    events: Mutex::new(PriorityQueue::new()),
                    thread_num:1,
                    //pool: None,
                    newly_scheduled: Mutex::new(Vec::new())
                }
            }
        }
    }

    pub fn with_threads(thread_num: usize) -> Schedule<A> {
        //println!("Using {} thread",thread_num);
        cfg_if! {
            if #[cfg(feature ="parallel")]{
                return Schedule {
                    step: 0,
                    time: 0.0,
                    events: Mutex::new(PriorityQueue::new()),
                    thread_num: thread_num,
                    //pool: Some(ThreadPoolBuilder::new().num_threads(thread_num).build().unwrap()),
                    newly_scheduled: Mutex::new(Vec::new())
                }
            }else{
                return Schedule {
                    step: 0,
                    time: 0.0,
                    events: Mutex::new(PriorityQueue::new()),
                    thread_num: 1,
                    //pool: None,
                    newly_scheduled: Mutex::new(Vec::new())
                }
            }
        }
    }

    pub fn schedule_once(&mut self, agent: AgentImpl<A>, the_time: f32, the_ordering: i64) {
        self.events.lock().unwrap().push(
            agent,
            Priority {
                time: the_time,
                ordering: the_ordering,
            },
        );
    }

    pub fn schedule_repeating(&mut self, agent: A, the_time: f32, the_ordering: i64) {
        let mut a = AgentImpl::new(agent);
        a.repeating = true;
        let pr = Priority::new(the_time, the_ordering);
        self.events.lock().unwrap().push(a, pr);
    }

    pub fn simulate<S: State>(&mut self, state: &mut <A as Agent>::SimState, num_step: u128) {
        for _ in 0..num_step {
            self.step(state);
        }
    }

    cfg_if! {
        if #[cfg(feature ="parallel")]{


        pub fn step(&mut self, state: &mut <A as Agent>::SimState){
            self.newly_scheduled.lock().unwrap().clear();
            let thread_num = self.thread_num;

            if self.step == 0{
                state.update(self.step);
            }

            self.step += 1;

            // let start: std::time::Instant = std::time::Instant::now();
            let events = &mut self.events;
            if events.lock().unwrap().is_empty() {
                //println!("coda eventi vuota");
                return
            }

            let thread_division = (events.lock().unwrap().len() as f32 / thread_num as f32).ceil() as usize ;
            let mut cevents: Vec<Vec<Pair<A>>> = vec![Vec::with_capacity(thread_division); thread_num];

           
            let mut index = 0;
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
                        if cevents[index].len() == thread_division{
                            index+=1;
                        }
                        cevents[index].push(Pair::new(agent, priority));
                    },
                    None => panic!("no item"),
                }
            }

            thread::scope( |scope| {
                //N-1 WORKER THREAD
                for _ in 0..thread_num-1{
                    let batch = cevents.pop().unwrap();
                    scope.spawn(|_| {
                        let mut reschedule = Vec::with_capacity(batch.len());
                        let mut newly_scheduled = Vec::with_capacity(batch.len());
                        for mut item in batch {
                            item.agentimpl.agent.step(&state);
                            let should_remove = item.agentimpl.agent.should_remove(&state);
                            let should_reproduce = item.agentimpl.agent.should_reproduce(&state);

                            if item.agentimpl.repeating && !should_remove {
                                reschedule.push( ( item.agentimpl, Priority{ time: item.priority.time+1.0, ordering: item.priority.ordering}) );
                            }

                            if let Some(new_agents) = should_reproduce {
                                for (new_agent, schedule_options) in new_agents {
                                    let ScheduleOptions{ordering, repeating} = schedule_options;
                                    let agent = *new_agent;
                                    let mut new_agent_impl = AgentImpl::new(agent.clone());
                                    new_agent_impl.repeating = repeating;
                                    reschedule.push((new_agent_impl, Priority{time: item.priority.time + 1., ordering}));
                                    newly_scheduled.push(agent);
                                }
                            }
                        }
                        let mut events = self.events.lock().unwrap();
                        for entry in reschedule{
                            events.push(entry.0,entry.1);
                        }
                        drop(events);
                        let mut g_newly_scheduled = self.newly_scheduled.lock().unwrap();
                        for entry in newly_scheduled{
                            g_newly_scheduled.push(entry);
                        }
                    });

                }
                //MAIN THREAD
                if cevents.len() > 0{
                        let batch = cevents.pop().unwrap();
                        let mut reschedule = Vec::with_capacity(batch.len());
                        let mut newly_scheduled = Vec::with_capacity(batch.len());
                        for mut item in batch {
                            item.agentimpl.agent.step(&state);
                            let should_remove = item.agentimpl.agent.should_remove(&state);
                            let should_reproduce = item.agentimpl.agent.should_reproduce(&state);

                            if item.agentimpl.repeating && !should_remove {
                                reschedule.push( ( item.agentimpl, Priority{ time: item.priority.time+1.0, ordering: item.priority.ordering}) );
                            }

                            if let Some(new_agents) = should_reproduce {
                                for (new_agent, schedule_options) in new_agents {
                                    let ScheduleOptions{ordering, repeating} = schedule_options;
                                    let agent = *new_agent;
                                    let mut new_agent_impl = AgentImpl::new(agent.clone());
                                    new_agent_impl.repeating = repeating;
                                    reschedule.push((new_agent_impl, Priority{time: item.priority.time + 1., ordering}));
                                    newly_scheduled.push(agent);
                                }
                            }
                        }
                        let mut events = self.events.lock().unwrap();
                        for entry in reschedule{
                            events.push(entry.0,entry.1);
                        }
                        drop(events);
                        let mut g_newly_scheduled = self.newly_scheduled.lock().unwrap();
                        for entry in newly_scheduled{
                            g_newly_scheduled.push(entry);
                        }
                    }

            });


        state.update(self.step);
        }
    }
    else{
        pub fn step(&mut self,state: &mut <A as Agent>::SimState){
            self.newly_scheduled.lock().unwrap().clear();
            if self.step == 0{
                state.update(self.step);
            }
            self.step += 1;


            // let start: std::time::Instant = std::time::Instant::now();
            let events = &mut self.events;
            if events.lock().unwrap().is_empty() {
                //println!("coda eventi vuota");
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
                        self.newly_scheduled.lock().unwrap().push(agent);
                    }
                }
            }

            state.update(self.step);
            // println!("Time spent calling step method, step {} : {:?}",self.step,start.elapsed());

            }

        }
    }
}

/// A struct used to specify schedule options to pass to an agent's clone when an agent reproduces.
pub struct ScheduleOptions {
    pub ordering: i64,
    pub repeating: bool,
}
