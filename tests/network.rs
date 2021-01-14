use rust_ab::engine::field::network::{Network, EdgeOptions::*};

#[test]
fn network_test_direct() {
    
    let network: Network<String, String> = Network::new(true);

    let node1 = String::from("node1");
    let node2 = String::from("node2");
    let node3 = String::from("node3");
    let node4 = String::from("node4");

    network.addEdge(node1.clone(), node2.clone(), WeightedLabeled(String::from("friend"), 2.0));
   
    match network.getEdge(node1.clone(), node2.clone()) {
        Some(edge) => edge,
        None => panic!("edge node1-node2 not found"),
    };
}
