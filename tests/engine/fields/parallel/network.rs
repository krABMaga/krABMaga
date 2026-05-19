#[cfg(test)]
#[cfg(any(feature = "parallel"))]
use {krabmaga::engine::fields::field::Field, krabmaga::engine::fields::network::*};

#[cfg(any(feature = "parallel"))]
static NUM_NODES: u16 = 10;

#[cfg(any(feature = "parallel"))]
#[test]
fn network_directed() {
    let mut net: Network<u16, String> = Network::new(true);

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

#[cfg(any(feature = "parallel"))]
#[test]
fn network_undirected() {
    let mut net: Network<u16, String> = Network::new(false);

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

// #[cfg(any(feature = "parallel"))]
// #[test]
// fn network_remove(){
//     let mut net: Network<u16, String> = Network::new(true);

//     for i in 0..NUM_NODES{
//         net.add_node(i);
//     }

//     for i in 0..NUM_NODES{
//         net.add_edge(i, (i+1)%NUM_NODES, EdgeOptions::Simple);
//         net.add_edge((i+1)%NUM_NODES,i,  EdgeOptions::Simple);

//     }
//     net.lazy_update();

//     net.remove_edge(0, 1);
//     net.lazy_update();

//     {
//         let edges = net.get_edges(0).unwrap();
//         assert_eq!(1, edges.len());
//     }
//     assert!(net.remove_node(0));
//     net.lazy_update();

//     assert_eq!(None, net.get_object(0));

// let edges = net.get_edges(1).unwrap();
// for e in edges.clone() {
//     println!("{} -- {} ", e.u, e.v);
// }

// assert_eq!(1, edges.len());

// let edges = net.get_edges(NUM_NODES-1).unwrap();
// assert_eq!(1, edges.len());

// let removed = net.remove_outgoing_edges(3);
// assert_eq!(removed.unwrap().len(), 2);
// net.lazy_update();

// let edges = net.get_edges(3).unwrap();
// assert_eq!(0, edges.len());

// net.remove_all_edges();
// net.lazy_update();

// assert!(net.edges.borrow_mut().is_empty());

// }

// #[cfg(any(feature = "parallel"))]
// #[test]
// fn network_scale_free_1(){
//     let mut net: Network<u16, String> = Network::new(false);
//     let node_set: &[u16] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
//     for i in 0..node_set.len(){
//         net.add_node(node_set[i]);
//     }
//     net.lazy_update();

//     net.preferential_attachment_BA(node_set, INIT_EDGES);

//     for node in node_set{
//         let edges = match net.get_edges(*node) {
//             Some(edges) => edges,
//             None => Vec::new(),
//         };

//         assert!(edges.len() >= INIT_EDGES);
//     }

// }

// #[cfg(any(feature = "parallel"))]
// #[test]
// fn network_scale_free_2(){

//     let mut net: Network<u16, String> = Network::new(false);
//     let node_set: &[u16] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
//     for i in 0..node_set.len(){
//         net.add_node(node_set[i]);
//     }
//     net.lazy_update();

//     net.preferential_attachment_BA_with_seed(node_set, INIT_EDGES, 0);

//     for node in node_set{
//         let edges = match net.get_edges(*node) {
//             Some(edges) => edges,
//             None => Vec::new(),
//         };

//         assert!(edges.len() >= INIT_EDGES);
//     }

//     let mut net2: Network<u16, String> = Network::new(false);
//     for i in 0..node_set.len(){
//         net2.add_node(node_set[i]);
//     }
//     net2.lazy_update();
//     net2.preferential_attachment_BA_with_seed(node_set, INIT_EDGES, 0);

//     for node in node_set{
//         let edges = match net.get_edges(*node) {
//             Some(edges) => edges,
//             None => Vec::new(),
//         };

//         let edges2 = match net2.get_edges(*node) {
//             Some(edges) => edges,
//             None => Vec::new(),
//         };

//         assert_eq!(edges.len(), edges2.len());

//         for i in 0..edges.len(){
//             let e1 = &edges[i];
//             let e2 = &edges2[i];

//             assert_eq!(e1.u, e2.u);
//             assert_eq!(e1.v, e2.v);
//         }

//     }

//     //----

//     let mut net3: Network<u16, String> = Network::new(false);
//     for i in 0..node_set.len(){
//         net3.add_node(node_set[i]);
//     }
//     net3.lazy_update();
//     net3.preferential_attachment_BA_with_seed(node_set, INIT_EDGES, 1);

//     let mut equals = true;
//     for node in node_set{
//         if !equals {break;}
//         let edges = match net.get_edges(*node) {
//             Some(edges) => edges,
//             None => Vec::new(),
//         };

//         let edges2 = match net3.get_edges(*node) {
//             Some(edges) => edges,
//             None => Vec::new(),
//         };

//         if edges.len() != edges2.len(){
//             equals = false;
//             break;
//         }

//         for i in 0..edges.len(){
//             let e1 = &edges[i];
//             let e2 = &edges2[i];

//             if e1.u != e2.u || e1.v != e2.v{
//                 equals = false;
//                 break;
//             }
//         }

//     }

//     assert!(!equals);

// }
