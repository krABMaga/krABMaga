#[cfg(test)]
#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
use {krabmaga::engine::fields::field::Field, krabmaga::engine::fields::hnetwork::*};

#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
#[test]
fn hnetwork_hedge_types() {
    let mut net: HNetwork<u32, String> = HNetwork::new();
    net.add_node(1);
    net.add_node(2);
    net.update();

    net.add_edge(&[1, 2], EdgeOptions::Labeled("Edge12".to_string()));
    net.update();
    let labeled = net.get_edge(&[1, 2]);
    assert!(labeled.is_some());
    let labeled = labeled.unwrap();
    assert!(labeled.label.is_some());
    assert_eq!(labeled.label.unwrap(), "Edge12");
    let removed = net.remove_edge(&[1, 2]);
    assert!(removed.is_some());
    let removed = removed.unwrap();
    assert_eq!(removed.label.unwrap(), "Edge12");

    //----

    net.add_edge(&[1, 2], EdgeOptions::Weighted(0.123));
    net.update();
    let weighted = net.get_edge(&[1, 2]);
    assert!(weighted.is_some());
    let weighted = weighted.unwrap();
    assert!(weighted.weight.is_some());
    assert_eq!(weighted.weight.unwrap(), 0.123);
    let removed = net.remove_edge(&[1, 2]);
    assert!(removed.is_some());
    let removed = removed.unwrap();
    assert_eq!(removed.weight.unwrap(), 0.123);

    //----
    net.add_edge(
        &[1, 2],
        EdgeOptions::WeightedLabeled("Edge12".to_string(), 0.123),
    );
    net.update();
    let wl = net.get_edge(&[1, 2]);
    assert!(wl.is_some());
    let wl = wl.unwrap();
    assert!(wl.weight.is_some());
    assert!(wl.label.is_some());
    assert_eq!(wl.clone().weight.unwrap(), 0.123);
    assert_eq!(wl.clone().label.unwrap(), "Edge12");
    let removed = net.remove_edge(&[1, 2]);
    assert!(removed.is_some());
    let removed = removed.unwrap();
    assert_eq!(removed.clone().weight.unwrap(), 0.123);
    assert_eq!(removed.clone().label.unwrap(), "Edge12");
}

#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
#[test]
fn hnetwork_update() {
    use crate::utils::mynode::MyNode;

    let mut net: HNetwork<MyNode, String> = HNetwork::new();
    net.update_node(MyNode { id: 0, flag: false });
    net.add_node(MyNode { id: 0, flag: false });
    let node = MyNode { id: 0, flag: true };
    net.update_node(node.clone());
    net.update();
    let get_node = net.get_object(node.id);
    assert_eq!(get_node.unwrap(), node);
    assert!(node.flag);
}

#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
#[test]
fn hnetwork_nodes() {
    // ADD Nod
    let mut net: HNetwork<u32, String> = HNetwork::new();
    net.add_node(1);
    net.add_node(2);
    net.update();
    net.add_edge(&[1, 2], EdgeOptions::Simple);
    net.add_edge(&[1, 2], EdgeOptions::Simple);
    net.add_edge(&[2, 1], EdgeOptions::Simple);
    net.update();

    let edges = net.get_edges(1).unwrap();
    assert_eq!(edges.len(), 1);

    let id = net.get_id(&1).unwrap();
    assert_eq!(id, 0);

    net.add_node(3);
    net.add_node(4);
    net.update();
    net.add_edge(&[3, 1, 2, 4], EdgeOptions::Simple);
    net.add_edge(&[1, 2, 4], EdgeOptions::Simple);
    net.update();
    //Remove Node
    net.update();
    assert!(net.get_edge(&[2, 4, 3, 1]).is_some());
    let n_edges_before = net.get_edges(1).unwrap().len();
    assert!(!net.remove_object(6));
    assert!(net.remove_object(4));
    net.lazy_update();
    assert!(net.get_edge(&[2, 4, 3, 1]).is_none());
    assert!(net.get_object(4).is_none());
    assert!(net.get_edges(4).is_none());
    let n_edges_after = net.get_edges(1).unwrap().len();
    assert!(n_edges_before > n_edges_after);
}

#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
#[test]
fn hnetwork_edges() {
    let edge: HEdge<String> = HEdge::new(&[1, 1, 2, 3, 3, 3], EdgeOptions::Simple);
    assert!([1, 1, 2, 3, 3, 3].len() > edge.nodes.len());
    assert!(edge.weight.is_none());

    assert_eq!(3, edge.nodes.len());

    // ADD Node + Fails Add Edge
    let mut net: HNetwork<u32, String> = HNetwork::new();
    assert!(!net.add_edge(&[1, 2], EdgeOptions::Simple));
    net.add_node(1);
    net.update();
    assert!(!net.add_edge(&[1, 2], EdgeOptions::Simple));
    net.add_node(2);
    net.update();

    // Add Edge + Get Edge
    assert!(net.add_edge(&[1, 2], EdgeOptions::Simple));
    net.update();

    let edge = net.get_edge(&[2, 1, 1]);
    assert!(edge.is_some());
    assert_eq!(edge.unwrap().nodes.len(), 2);
    let edge = net.get_edge(&[2, 1, 3]);
    assert!(edge.is_none());

    net.add_node(3);
    net.add_node(4);
    net.update();
    net.add_edge(&[3, 1, 2, 4], EdgeOptions::Simple);
    net.add_edge(&[1, 2, 4], EdgeOptions::Simple);
    net.update();

    let edge = net.get_edge(&[2, 4, 3, 1]);
    assert!(edge.is_some());
    assert_eq!(edge.unwrap().nodes.len(), 4);
    let n_edges = net.get_edges(1);
    assert!(n_edges.is_some());
    assert_eq!(3, n_edges.as_ref().unwrap().len());

    // Remove Edge
    let edge = net.get_edge(&[2, 4, 3, 1]);
    let removed = net.remove_edge(&[1, 2, 3, 4]);
    net.update();
    assert!(removed.is_some());
    assert_eq!(removed.as_ref().unwrap(), edge.as_ref().unwrap());
    let edge = net.get_edge(&[2, 4, 3, 1]);
    assert!(edge.is_none());

    //Remove all edges
    net.remove_all_edges();
    net.update();
    let n_edges = net.get_edges(1);
    assert!(n_edges.is_none());
    let n_edges = net.get_edges(2);
    assert!(n_edges.is_none());
    let n_edges = net.get_edges(3);
    assert!(n_edges.is_none());
    let n_edges = net.get_edges(4);
    assert!(n_edges.is_none());
}
