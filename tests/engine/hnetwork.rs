
#[cfg(test)]

#[cfg(not(any(feature = "visualization", feature = "visualization_wasm", feature = "parallel")))]
use {
    rust_ab::engine::fields::hnetwork::*,
    rust_ab::engine::fields::field::Field,
};

#[cfg(not(any(feature = "visualization", feature = "visualization_wasm", feature = "parallel")))]
#[test]
fn hnetwork_edges(){
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
    net.add_edge(&[3, 1, 2 , 4], EdgeOptions::Simple);
    net.add_edge(&[1, 2 , 4], EdgeOptions::Simple);
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
    

    //Remove Node
    net.add_edge(&[3, 1, 2 , 4], EdgeOptions::Simple);
    net.update();
    assert!(net.get_edge(&[2, 4, 3, 1]).is_some());
    let n_edges_before = net.get_edges(1).unwrap().len();
    assert!(!net.remove_object(6));
    assert!(net.remove_object(4));
    net.lazy_update();
    assert!(net.get_edge(&[2, 4, 3, 1]).is_none());
    assert!(edge.is_none());
    assert!(net.get_object(4).is_none());
    assert!(net.get_edges(4).is_none());    
    let n_edges_after = net.get_edges(1).unwrap().len();
    assert!(n_edges_before > n_edges_after);

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