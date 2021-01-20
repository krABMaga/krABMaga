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

    network.addNode(&node1);
    network.addNode(&node2);
    network.addNode(&node3);
    network.addNode(&node4);
    network.addNode(&node5);

    network.lazy_update();

    network.addEdge(&node1, &node2, WeightedLabeled(String::from("friend"), 2.0));
    network.addEdge(&node1, &node3, WeightedLabeled(String::from("friend"), 2.0));
    network.addEdge(&node3, &node4, WeightedLabeled(String::from("friend"), 2.0));
    network.addEdge(&node4, &node1, WeightedLabeled(String::from("friend"), 2.0));

    network.lazy_update();

    match network.getEdge(&node2, &node1) {
        Some(edge) => assert!(true),
        None => assert!(false),
    };

    match network.getEdge(&node1, &node4) {
        Some(edge) => assert!(true),
        None => assert!(false),
    };

    match network.getEdge(&node2, &node4) {
        Some(edge) => assert!(false),
        None => assert!(true),
    };

    match network.getEdge(&node1, &node5) {
        Some(edge) => assert!(false),
        None => assert!(true),
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

    network.addNode(&node1);
    network.addNode(&node2);
    network.addNode(&node3);
    network.addNode(&node4);
    network.addNode(&node5);

    network.lazy_update();

    network.addEdge(&node1, &node2, WeightedLabeled(String::from("friend"), 2.0));
    network.addEdge(&node1, &node3, WeightedLabeled(String::from("friend"), 2.0));
    network.addEdge(&node3, &node4, WeightedLabeled(String::from("friend"), 2.0));
    network.addEdge(&node4, &node1, WeightedLabeled(String::from("friend"), 2.0));

    network.lazy_update();

    match network.getEdge(&node1, &node2) {
        Some(edge) => assert!(true),
        None => assert!(false),
    };

    match network.getEdge(&node3, &node4) {
        Some(edge) => assert!(true),
        None => assert!(false),
    };

    match network.getEdge(&node2, &node1) {
        Some(edge) => assert!(false),
        None => assert!(true),
    };

    match network.getEdge(&node1, &node4) {
        Some(edge) => assert!(false),
        None => assert!(true),
    };
}
