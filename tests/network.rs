use rust_ab::engine::field::network::{Network, EdgeOptions::*};
use rust_ab::engine::field::field::Field;

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

    network.update();

    network.addEdge(&node1, &node2, WeightedLabeled(String::from("friend"), 2.0));
    network.addEdge(&node1, &node3, WeightedLabeled(String::from("friend"), 2.0));
    network.addEdge(&node3, &node4, WeightedLabeled(String::from("friend"), 2.0));
    network.addEdge(&node4, &node1, WeightedLabeled(String::from("friend"), 2.0)); 

    network.update();

    match network.getEdge(&node2, &node1) {
        Some(edge) => edge,
        None => panic!("edge node2-node1 not found"),
    };
    
    match network.getEdge(&node1, &node4) {
        Some(edge) => edge,
        None => panic!("edge node4-node1 not found"),
    };

    match network.getEdge(&node2, &node4) {
        Some(edge) => panic!("edge node2-node4 found"),
        None => {},
    };

    match network.getEdge(&node1, &node5) {
        Some(edge) => panic!("edge node1-node5 found"),
        None => {},
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

    network.update();

    network.addEdge(&node1, &node2, WeightedLabeled(String::from("friend"), 2.0));
    network.addEdge(&node1, &node3, WeightedLabeled(String::from("friend"), 2.0));
    network.addEdge(&node3, &node4, WeightedLabeled(String::from("friend"), 2.0));
    network.addEdge(&node4, &node1, WeightedLabeled(String::from("friend"), 2.0)); 

    network.update();

    match network.getEdge(&node1, &node2) {
        Some(edge) => edge,
        None => panic!("edge node2-node1 not found"),
    };
    
    match network.getEdge(&node3, &node4) {
        Some(edge) => edge,
        None => panic!("edge node4-node1 not found"),
    };

    match network.getEdge(&node2, &node1) {
        Some(edge) => panic!("edge node2-node1 found"),
        None => {},
    };

    match network.getEdge(&node1, &node4) {
        Some(edge) => panic!("edge node1-node4 found"),
        None => {},
    };
}
