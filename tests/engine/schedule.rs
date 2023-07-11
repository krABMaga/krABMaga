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

    schedule.schedule_repeating(Box::new(node1), 0., 0);
    schedule.schedule_repeating(Box::new(node2), 0., 0);

    let agents = schedule.get_all_events();
    assert_eq!(agents.len(), 2);

    for (i, a) in agents.iter().enumerate() {
        assert_eq!(
            *a.downcast_ref::<MyNode>().unwrap(),
            if i == 0 { node1 } else { node2 }
        );
    }

    assert!(schedule.dequeue(Box::new(node1), node1.id));
    let agents = schedule.get_all_events();
    assert_eq!(agents.len(), 1);
    let a = agents[0].downcast_ref::<MyNode>().unwrap();
    assert_eq!(*a, node2);

    assert!(schedule.dequeue(Box::new(node2), node2.id));
    let agents = schedule.get_all_events();
    assert_eq!(agents.len(), 0);
}

#[cfg(test)]
#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
#[test]
fn distributed_schedule_operations() {
    use krabmaga::engine::schedule::Schedule;

    use crate::utils::mynode::MyNode;

    let mut schedule = Schedule::new();
    let node1 = MyNode { id: 0, flag: false };
    let node2 = MyNode { id: 1, flag: false };

    let (id1, opt1) = schedule.distributed_schedule_repeating(Box::new(node1), 0., 0);
    let (id2, opt2) = schedule.distributed_schedule_repeating(Box::new(node2), 0., 0);

    assert_eq!(id1, 0);
    assert!(opt1);

    assert_eq!(id2, 1);
    assert!(opt2);

    let agents = schedule.get_all_events();
    assert_eq!(agents.len(), 2);

    for (i, a) in agents.iter().enumerate() {
        assert_eq!(
            *a.downcast_ref::<MyNode>().unwrap(),
            if i == 0 { node1 } else { node2 }
        );
    }

    assert!(schedule.dequeue(Box::new(node1), node1.id));
    let agents = schedule.get_all_events();
    assert_eq!(agents.len(), 1);
    let a = agents[0].downcast_ref::<MyNode>().unwrap();
    assert_eq!(*a, node2);

    assert!(schedule.dequeue(Box::new(node2), node2.id));
    let agents = schedule.get_all_events();
    assert_eq!(agents.len(), 0);
}
