#[cfg(test)]
#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
#[test]
fn schedule_operations() {
    use krabmaga::engine::schedule::Schedule;

    use crate::utils::mynode::MyNode;

    let mut schedule = Schedule::new();
    let node1 = MyNode { id: 0, flag: false };
    let node2 = MyNode { id: 1, flag: false };
    let node3 = MyNode { id: 2, flag: false };

    schedule.schedule_repeating(Box::new(node1), 0., 0);
    schedule.schedule_repeating(Box::new(node2), 0., 0);
    let (id, opt) = schedule.distributed_schedule_repeating(Box::new(node3), 0., 0);

    let agents = schedule.get_all_events();
    assert_eq!(agents.len(), 3);
    assert_eq!(id, 2);
    assert_eq!(opt, true);

    for (i, a) in agents.iter().enumerate() {
        assert_eq!(
            *a.downcast_ref::<MyNode>().unwrap(),
            if i == 0 { node1 } else { node2 }
        );
    }

    assert!(schedule.dequeue(Box::new(node1), node1.id));
    let agents = schedule.get_all_events();
    assert_eq!(agents.len(), 2);
    let a = agents[0].downcast_ref::<MyNode>().unwrap();
    assert_eq!(*a, node2);

    assert!(schedule.dequeue(Box::new(node2), node2.id));
    let agents = schedule.get_all_events();
    assert_eq!(agents.len(), 1);
}
