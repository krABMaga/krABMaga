use rust_ab::engine::field::field::Field;
use rust_ab::engine::field::network::{EdgeOptions::*, Network};

#[test]
fn network_test_undirect() {
    let mut network: Network<String, String> = Network::new(false);

    let node1 = String::from("node1");
    let node2 = String::from("node2");
    let node3 = String::from("node3");
    let node4 = String::from("node4");
    let node5 = String::from("node5");

    network.add_node(&node1);
    network.add_node(&node2);
    network.add_node(&node3);
    network.add_node(&node4);
    network.add_node(&node5);

    network.lazy_update();

    network.add_edge(&node1, &node2, WeightedLabeled(String::from("friend"), 2.0));
    network.add_edge(&node1, &node3, WeightedLabeled(String::from("friend"), 2.0));
    network.add_edge(&node3, &node4, WeightedLabeled(String::from("friend"), 2.0));
    network.add_edge(&node4, &node1, WeightedLabeled(String::from("friend"), 2.0));

    network.lazy_update();

    match network.get_edge(&node2, &node1) {
        Some(_) => assert!(true),
        None => assert!(false),
    };

    match network.get_edge(&node1, &node4) {
        Some(_) => assert!(true),
        None => assert!(false),
    };

    match network.get_edge(&node2, &node4) {
        Some(_) => assert!(false),
        None => assert!(true),
    };

    match network.get_edge(&node1, &node5) {
        Some(_) => assert!(false),
        None => assert!(true),
    };
    network.update_edge(
        &node1,
        &node2,
        WeightedLabeled(String::from("friend2"), 4.0),
    );

    network.update();

    match network.get_edge(&node1, &node2) {
        Some(edge) => {
            assert_eq!(edge.label.unwrap(), "friend2");
        }
        None => assert!(false),
    };
}

#[test]
fn network_test_direct() {
    let mut network: Network<String, String> = Network::new(true);

    let node1 = String::from("node1");
    let node2 = String::from("node2");
    let node3 = String::from("node3");
    let node4 = String::from("node4");
    let node5 = String::from("node5");

    network.add_node(&node1);
    network.add_node(&node2);
    network.add_node(&node3);
    network.add_node(&node4);
    network.add_node(&node5);

    network.lazy_update();

    network.add_edge(&node1, &node2, WeightedLabeled(String::from("friend"), 2.0));
    network.add_edge(&node1, &node3, WeightedLabeled(String::from("friend"), 2.0));
    network.add_edge(&node3, &node4, WeightedLabeled(String::from("friend"), 2.0));
    network.add_edge(&node4, &node1, WeightedLabeled(String::from("friend"), 2.0));

    network.lazy_update();

    match network.get_edge(&node1, &node2) {
        Some(_) => assert!(true),
        None => assert!(false),
    };

    match network.get_edge(&node3, &node4) {
        Some(_) => assert!(true),
        None => assert!(false),
    };

    match network.get_edge(&node2, &node1) {
        Some(_) => assert!(false),
        None => assert!(true),
    };

    match network.get_edge(&node1, &node4) {
        Some(_) => assert!(false),
        None => assert!(true),
    };
}
