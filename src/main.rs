extern crate priority_queue;

use priority_queue::PriorityQueue;
use abm::agent::Agent;
use abm::priority::Priority;

//use priority::Priority;
//use agent::Agent;

fn main() {

    let ag1 = Agent {
        id: String::from("Agente1"),
    };

    let pr = Priority {
        time: 10.0,
        priority: 100,
    };

    let ag2 = Agent {
        id: String::from("Agente2"),
    };

    let pr2 = Priority {
        time: 15.0,
        priority: 500,
    };

    let mut pq = PriorityQueue::new();
    pq.push(&ag1, pr);
    pq.push(&ag2, pr2);

    for (item, _) in pq.into_sorted_iter() {
        println!("{}", item);
    }

    println!("Finish");
}
