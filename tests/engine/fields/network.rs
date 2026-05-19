#[cfg(test)]
#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
use {krabmaga::engine::fields::field::Field, krabmaga::engine::fields::network::*};

#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
static NUM_NODES: u16 = 10;
#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
static INIT_EDGES: usize = 1;

#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
#[test]
fn network_edge_types() {
    let mut net: Network<u32, String> = Network::new(false);
    net.add_node(1);
    net.add_node(2);
    net.update();

    net.add_edge(1, 2, EdgeOptions::Labeled("Edge12".to_string()));
    net.update();
    let labeled = net.get_edge(1, 2);
    assert!(labeled.is_some());
    let labeled = labeled.unwrap();
    assert!(labeled.label.is_some());
    assert_eq!(labeled.label.unwrap(), "Edge12");
    let removed = net.remove_edge(1, 2);
    assert!(removed.is_some());
    let removed = removed.unwrap();
    assert_eq!(removed.label.unwrap(), "Edge12");

    //----

    net.add_edge(1, 2, EdgeOptions::Weighted(0.123));
    net.update();
    let weighted = net.get_edge(1, 2);
    assert!(weighted.is_some());
    let weighted = weighted.unwrap();
    assert!(weighted.weight.is_some());
    assert_eq!(weighted.weight.unwrap(), 0.123);
    let removed = net.remove_edge(1, 2);
    assert!(removed.is_some());
    let removed = removed.unwrap();
    assert_eq!(removed.weight.unwrap(), 0.123);

    //----
    net.add_edge(
        1,
        2,
        EdgeOptions::WeightedLabeled("Edge12".to_string(), 0.123),
    );
    net.update();
    let wl = net.get_edge(1, 2);
    assert!(wl.is_some());
    let wl = wl.unwrap();
    assert!(wl.weight.is_some());
    assert!(wl.label.is_some());
    assert_eq!(wl.clone().weight.unwrap(), 0.123);
    assert_eq!(wl.clone().label.unwrap(), "Edge12");
    let removed = net.remove_edge(1, 2);
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
fn network_gets_fault() {
    let mut net: Network<u16, String> = Network::new(true);

    assert!(!net.get_edge(1, 2).is_some());
    assert!(!net.get_object(1).is_some());

    net.add_node(1);
    net.update();
    assert!(!net.get_edge(1, 2).is_some());
    net.add_node(2);
    assert!(!net.get_edge(1, 2).is_some());
}

#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
#[test]
fn network_update_nodes() {
    #[derive(Clone, Debug, Eq)]
    struct TestNode {
        id: u16,
        flag: bool,
    }

    impl PartialEq for TestNode {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    impl std::hash::Hash for TestNode {
        fn hash<H>(&self, state: &mut H)
        where
            H: std::hash::Hasher,
        {
            self.id.hash(state);
        }
    }

    impl std::fmt::Display for TestNode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "TestNode({})", self.id)
        }
    }

    let mut net: Network<TestNode, String> = Network::new(false);

    net.add_node(TestNode { id: 1, flag: false });
    net.update_node(TestNode { id: 1, flag: true });

    net.update();
    let node = net.get_object(0);
    assert!(node.is_some());
    let node = node.unwrap();
    assert_eq!(node.id, 1);
    assert_eq!(node.flag, true);
}

#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
#[test]
fn network_directed() {
    let mut net: Network<u16, String> = Network::new(true);

    let (added, _) = net.add_edge(1, 2, EdgeOptions::Simple);
    assert!(!added);

    for i in 0..NUM_NODES {
        net.add_node(i);
    }

    for i in 0..NUM_NODES {
        net.add_edge(i, (i + 1) % NUM_NODES, EdgeOptions::Simple);
    }
    net.update();

    for i in 0..NUM_NODES {
        let node = net.get_object(i as u32).unwrap();
        assert_eq!(node, i);

        let edges = net.get_edges(i).unwrap();
        assert_eq!(1, edges.len());

        let e = net.get_edge(i, (i + 1) % NUM_NODES).unwrap();
        assert_eq!(e.u as u16, i);
        assert_eq!(e.v as u16, (i + 1) % NUM_NODES);
    }

    for i in 0..NUM_NODES {
        net.add_edge((i + 1) % NUM_NODES, i, EdgeOptions::Simple);
    }

    net.lazy_update();

    for i in 0..NUM_NODES {
        let node = net.get_object(i as u32).unwrap();
        assert_eq!(node, i);

        let edges = net.get_edges(i).unwrap();
        assert_eq!(2, edges.len());
    }
}

#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
#[test]
fn network_undirected() {
    let mut net: Network<u16, String> = Network::new(false);

    let (added1, added2) = net.add_edge(1, 2, EdgeOptions::Simple);
    assert!(!added1);
    assert!(!added2);

    for i in 0..NUM_NODES {
        net.add_node(i);
    }

    for i in 0..NUM_NODES {
        net.add_edge(i, (i + 1) % NUM_NODES, EdgeOptions::Simple);
    }
    net.lazy_update();

    for i in 0..NUM_NODES {
        let node = net.get_object(i as u32).unwrap();
        assert_eq!(node, i);

        let edges = net.get_edges(i).unwrap();
        assert_eq!(2, edges.len());

        let e = net.get_edge(i, (i + 1) % NUM_NODES).unwrap();
        assert_eq!(e.u as u16, i);
        assert_eq!(e.v as u16, (i + 1) % NUM_NODES);
    }
}

#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
#[test]
fn network_remove_directed() {
    let mut net: Network<u16, String> = Network::new(true);

    for i in 0..NUM_NODES {
        net.add_node(i);
    }

    for i in 0..NUM_NODES {
        net.add_edge(i, (i + 1) % NUM_NODES, EdgeOptions::Simple);
        net.add_edge((i + 1) % NUM_NODES, i, EdgeOptions::Simple);
    }
    net.lazy_update();

    net.remove_edge(0, 1);
    net.lazy_update();

    {
        let edges = net.get_edges(0).unwrap();
        assert_eq!(1, edges.len());
    }
    assert!(net.remove_node(0));
    net.lazy_update();

    assert_eq!(None, net.get_object(0));

    let edges = net.get_edges(1).unwrap();
    for e in edges.clone() {
        println!("{} -- {} ", e.u, e.v);
    }

    assert_eq!(1, edges.len());

    let edges = net.get_edges(NUM_NODES - 1).unwrap();
    assert_eq!(1, edges.len());

    let removed = net.remove_outgoing_edges(3);
    assert_eq!(removed.unwrap().len(), 2);
    net.lazy_update();

    let edges = net.get_edges(3).unwrap();
    assert_eq!(0, edges.len());

    net.remove_all_edges();
    net.lazy_update();

    // assert!(net.edges.borrow_mut().is_empty());
}

#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
#[test]
fn network_remove_undirected() {
    let mut net: Network<u16, String> = Network::new(false);

    for i in 0..NUM_NODES {
        net.add_node(i);
    }

    for i in 0..NUM_NODES {
        net.add_edge(i, (i + 1) % NUM_NODES, EdgeOptions::Simple);
    }
    net.lazy_update();

    net.remove_edge(0, 1);
    net.lazy_update();

    {
        let edges = net.get_edges(0).unwrap();
        assert_eq!(1, edges.len());
    }
    assert!(net.remove_node(0));
    net.lazy_update();

    assert_eq!(None, net.get_object(0));

    let edges = net.get_edges(1).unwrap();
    for e in edges.clone() {
        println!("{} -- {} ", e.u, e.v);
    }

    assert_eq!(1, edges.len());

    let edges = net.get_edges(NUM_NODES - 1).unwrap();
    assert_eq!(1, edges.len());

    let removed = net.remove_outgoing_edges(3);
    assert_eq!(removed.unwrap().len(), 2);
    net.lazy_update();

    let edges = net.get_edges(3).unwrap();
    assert_eq!(0, edges.len());

    net.remove_all_edges();
    net.lazy_update();

    // assert!(net.edges.borrow_mut().is_empty());
}

#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
#[test]
fn network_scale_free_1() {
    let mut net: Network<u16, String> = Network::new(false);
    let node_set: &[u16] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    for i in 0..node_set.len() {
        net.add_node(node_set[i]);
    }
    net.lazy_update();

    net.preferential_attachment_BA(node_set, INIT_EDGES);

    for node in node_set {
        let edges = match net.get_edges(*node) {
            Some(edges) => edges,
            None => Vec::new(),
        };

        assert!(edges.len() >= INIT_EDGES);
    }
}

#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
#[test]
fn network_scale_free_2() {
    let mut net: Network<u16, String> = Network::new(false);
    let node_set: &[u16] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    for i in 0..node_set.len() {
        net.add_node(node_set[i]);
    }
    net.lazy_update();

    net.preferential_attachment_BA_with_seed(node_set, INIT_EDGES, 0);

    for node in node_set {
        let edges = match net.get_edges(*node) {
            Some(edges) => edges,
            None => Vec::new(),
        };

        assert!(edges.len() >= INIT_EDGES);
    }

    let mut net2: Network<u16, String> = Network::new(false);
    for i in 0..node_set.len() {
        net2.add_node(node_set[i]);
    }
    net2.lazy_update();
    net2.preferential_attachment_BA_with_seed(node_set, INIT_EDGES, 0);

    for node in node_set {
        let edges = match net.get_edges(*node) {
            Some(edges) => edges,
            None => Vec::new(),
        };

        let edges2 = match net2.get_edges(*node) {
            Some(edges) => edges,
            None => Vec::new(),
        };

        assert_eq!(edges.len(), edges2.len());

        for i in 0..edges.len() {
            let e1 = &edges[i];
            let e2 = &edges2[i];

            assert_eq!(e1.u, e2.u);
            assert_eq!(e1.v, e2.v);
        }
    }

    //----

    let mut net3: Network<u16, String> = Network::new(false);
    for i in 0..node_set.len() {
        net3.add_node(node_set[i]);
    }
    net3.lazy_update();
    net3.preferential_attachment_BA_with_seed(node_set, INIT_EDGES, 1);

    let mut equals = true;
    for node in node_set {
        if !equals {
            break;
        }
        let edges = match net.get_edges(*node) {
            Some(edges) => edges,
            None => Vec::new(),
        };

        let edges2 = match net3.get_edges(*node) {
            Some(edges) => edges,
            None => Vec::new(),
        };

        if edges.len() != edges2.len() {
            equals = false;
            break;
        }

        for i in 0..edges.len() {
            let e1 = &edges[i];
            let e2 = &edges2[i];

            if e1.u != e2.u || e1.v != e2.v {
                equals = false;
                break;
            }
        }
    }

    assert!(!equals);
}
