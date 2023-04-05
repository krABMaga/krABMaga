use crate::engine::fields::field::Field;
use cfg_if::cfg_if;
use core::fmt::Debug;
use hashbrown::HashMap;
use rand::prelude::*;
use std::cell::RefCell;
use std::fmt::Display;
use std::hash::Hash;

use rand::rngs::StdRng;

cfg_if! {
    if #[cfg(any(feature = "parallel", feature = "visualization", feature = "visualization_wasm"))]{
        use crate::utils::dbdashmap::DBDashMap;
    } else {
        use core::fmt::Error;
        use std::fmt::Formatter;
    }
}

/// Available types of an edge/hedge
#[derive(Clone)]
pub enum EdgeOptions<L: Clone + Hash + Display> {
    /// A simple edge, without additional info
    Simple,
    /// An edge with a label
    Labeled(L),
    /// Weighted edge
    Weighted(f32),
    /// Weighted edge with a label
    WeightedLabeled(L, f32),
}

/// An edge of a `Network` struct
#[derive(Clone, Debug)]
pub struct Edge<L: Clone + Hash + Display> {
    /// id of source node
    pub u: u32,
    /// id of destination node
    pub v: u32,
    pub label: Option<L>,
    pub weight: Option<f32>,
}

impl<L: Clone + Hash + Display> Edge<L> {
    /// Create a new edge
    /// # Arguments
    /// * `u_node` - id of source node
    /// * `v_node` - id of destination node
    /// * `edge_options` - edge options (label and/or weight)
    pub fn new(u_node: u32, v_node: u32, edge_options: EdgeOptions<L>) -> Edge<L> {
        match edge_options {
            EdgeOptions::Simple => Edge {
                u: u_node,
                v: v_node,
                label: None,
                weight: None,
            },
            EdgeOptions::Labeled(l) => Edge {
                u: u_node,
                v: v_node,
                label: Some(l),
                weight: None,
            },
            EdgeOptions::Weighted(w) => Edge {
                u: u_node,
                v: v_node,
                label: None,
                weight: Some(w),
            },
            EdgeOptions::WeightedLabeled(l, w) => Edge {
                u: u_node,
                v: v_node,
                label: Some(l),
                weight: Some(w),
            },
        }
    }
}

cfg_if! {
    if #[cfg(any(feature = "parallel", feature = "visualization", feature = "visualization_wasm"))]{

        pub struct Network<O: Hash + Eq + Clone + Display, L: Clone + Hash + Display> {
            pub edges: DBDashMap<u32, Vec<Edge<L>>>,
            pub nodes2id: RefCell<HashMap<O, u32>>,
            pub id2nodes:  DBDashMap<u32, O>,
            pub direct: bool,
        }


        impl<O: Hash + Eq + Clone + Display, L: Clone + Hash + Display> Network<O, L> {
            pub fn new(d: bool) -> Network<O, L> {
                Network {
                    edges: DBDashMap::new(),
                    nodes2id: RefCell::new(HashMap::new()),
                    id2nodes: DBDashMap::new(),
                    direct: d,
                }
            }

            pub fn add_edge(&self, u: O, v: O, edge_options: EdgeOptions<L>) -> (bool, bool) {
                let nodes2id = self.nodes2id.borrow_mut();
                let mut vbool = false;

                let uid = match nodes2id.get(&u){
                    Some(u)=> u,
                    None => return (false,false)
                };

                let vid = match nodes2id.get(&v){
                    Some(v)=> v,
                    None => return (false,false)
                };

                match self.edges.get_write(uid) {
                    Some(mut uedges) => {
                        uedges.push(Edge::new(*uid, *vid, edge_options.clone()));
                    }
                    None => {
                        let mut vec = Vec::new();
                        vec.push(Edge::new(*uid, *vid, edge_options.clone()));
                        self.edges.insert(*uid, vec);
                    }
                }

                let ubool = true;
                if !self.direct {
                    match self.edges.get_write(&vid) {
                        Some(mut vedges) => {
                            vedges.push(Edge::new(*vid, *uid, edge_options.clone()));
                        }
                        None => {
                            let mut vec = Vec::new();
                            vec.push(Edge::new(*vid, *uid, edge_options.clone()));
                            self.edges.insert(*vid, vec);
                        }
                    }
                    vbool = true;
                }
                (ubool,vbool)
            }

            pub fn add_node(&self, u: O) {
                let mut nodes2id = self.nodes2id.borrow_mut();
                let uid = nodes2id.len() as u32;
                nodes2id.insert(u.clone(), uid);
                self.id2nodes.insert(uid, u);

                match self.edges.get_read(&uid){
                    Some(_edges) => {},
                    None => {
                        let vec: Vec<Edge<L>> = Vec::new();
                        self.edges.insert(uid, vec);
                    }
                }
            }

            //part of "preferential attachment" process
            //in which new network members prefer to make a connection to the more popular existing members.
            pub fn add_prob_edge(&self, u: O, n_sample: &usize, my_seed: u64) {
                let id2nodes = &self.id2nodes;
                let mut dist: Vec<(&O, i32)> = Vec::new();
                let edges = &self.edges;

                for k in edges.keys() {
                    match self.get_edges(id2nodes.get_read(k).expect("error on get_read").clone()) {
                        Some(es) => {
                            dist.push((id2nodes.get_read(k).expect("error on get_read"), es.len() as i32));
                        }
                        None => {}
                    }
                }

                //let mut rng = Pcg64::seed_from_u64(my_seed);
                let mut rng = StdRng::seed_from_u64(my_seed);
                let amount: usize = if edges.len() < *n_sample {
                    edges.len()
                } else {
                    *n_sample
                };

                let choices_list = dist
                    .choose_multiple_weighted(&mut rng, amount, |dist| dist.1)
                    .expect("error on choose_multiple_weighted")
                    .collect::<Vec<_>>();

                for choice in choices_list {
                    self.add_edge(u.clone(), choice.0.clone(), EdgeOptions::Simple);
                }
            }

            pub fn get_edge(&self, u: O, v: O) -> Option<Edge<L>> {
                let nodes2id = self.nodes2id.borrow();
                let uid = match nodes2id.get(&u){
                    Some(u)=> u,
                    None => return None
                };

                match self.edges.get_read(uid) {
                    Some(uedges) => {
                        let vid = match nodes2id.get(&v){
                            Some(v)=> v,
                            None => return None
                        };

                        for e in uedges {

                            let vid_edge = nodes2id.get(self.id2nodes.get_read(&e.v).expect("error on get_read"))
                                .expect("error on get");
                            // if self.direct && e.u == *uid && *vid == *vid_edge {
                            //     return Some(e.clone());
                            // } else if !self.direct && ((e.u == *uid && *vid_edge == *vid) || (*vid_edge == *uid && e.u == *vid))
                            // {
                            //     return Some(e.clone());
                            // }
                            if e.u == *uid && *vid_edge == *vid || !self.direct && *vid_edge == *uid && e.u == *vid {
                                return Some(e.clone());
                            }
                        }
                        None
                    }
                    None => None,
                }
            }

            pub fn get_edges(&self, u: O) -> Option<Vec<Edge<L>>> {
                let nodes2id = self.nodes2id.borrow();
                let uid = match nodes2id.get(&u){
                    Some(u)=> u,
                    None => return None
                };
                match self.edges.get_read(uid){
                    Some(es) => {Some((*(es.clone())).to_vec())},
                    None => {None}
                }
            }

            pub fn get_object(&self, uid: u32) -> Option<O>{
                match self.id2nodes.get_read(&uid){
                    Some(u) => Some(u.clone()),
                    None => None
                }
            }

            /**
            Generate an undirected network based on
            Barabási-Albert’s preferential attachment model
            */
            #[allow(non_snake_case)]
            pub fn preferential_attachment_BA(
                &mut self,
                node_set: &[O],
                init_edges: usize
            ) {
                {
                    let n_nodes = self.id2nodes.len();
                    // clear the existing edges
                    self.remove_all_edges();

                    // if there are no nodes return
                    if node_set.len() == 0 || node_set.len() == 1 {
                        return;
                    }

                    // create the first edge between the first two nodes
                    let first_node = node_set[0].clone();
                    let second_node = node_set[1].clone();
                    self.add_edge(first_node.clone(), second_node.clone(), EdgeOptions::Simple);

                    // self.update();

                    let mut rng = rand::thread_rng();
                    let mut dist: Vec<(O, i32, usize)> = Vec::with_capacity(n_nodes);

                    dist.push(((first_node.clone()), 1, 0));
                    dist.push(((second_node.clone()), 1, 1));

                    for i in 2..n_nodes {
                        let node = node_set[i].clone();
                        let mut choice_pos: Vec<usize> = Vec::with_capacity(init_edges);

                        let amount: usize = if dist.len() < init_edges {
                            dist.len()
                        } else {
                            init_edges
                        };

                        let choices_list = dist
                            .choose_multiple_weighted(&mut rng, amount, |choice| choice.1)
                            .expect("error on choose_multiple_weighted")
                            .collect::<Vec<_>>();

                        for choice in choices_list {
                            self.add_edge(node.clone(), choice.0.clone(), EdgeOptions::Simple);
                            choice_pos.push(choice.2);
                        }

                        for i in 0..choice_pos.len() {
                            dist[choice_pos[i]].1 += 1;
                        }

                        dist.push(((node.clone()), amount as i32, i));

                        // self.update();
                    }
                }
                self.update();
            }

            /**
            Generate an undirected network based on
            Barabási-Albert’s preferential attachment model
            with defined seed
            */
            #[allow(non_snake_case)]
            pub fn preferential_attachment_BA_with_seed(
                &mut self,
                node_set: &[O],
                init_edges: usize,
                my_seed: u64,
            ) {
                {
                    let n_nodes = self.id2nodes.len();
                    // clear the existing edges
                    self.remove_all_edges();

                    // if there are no nodes return
                    if node_set.len() == 0 || node_set.len() == 1 {
                        return;
                    }

                    // create the first edge between the first two nodes
                    let first_node = node_set[0].clone();
                    let second_node = node_set[1].clone();
                    self.add_edge(first_node.clone(), second_node.clone(), EdgeOptions::Simple);

                    // self.update();
                    // let mut rng = Pcg64::seed_from_u64(my_seed);
                    let mut rng = StdRng::seed_from_u64(my_seed);
                    let mut dist: Vec<(O, i32, usize)> = Vec::with_capacity(n_nodes);

                    dist.push(((first_node.clone()), 1, 0));
                    dist.push(((second_node.clone()), 1, 1));

                    for i in 2..n_nodes {
                        let node = node_set[i].clone();
                        let mut choice_pos: Vec<usize> = Vec::with_capacity(init_edges);

                        let amount: usize = if dist.len() < init_edges {
                            dist.len()
                        } else {
                            init_edges
                        };

                        let choices_list = dist
                            .choose_multiple_weighted(&mut rng, amount, |choice| choice.1)
                            .expect("error on choose_multiple_weighted")
                            .collect::<Vec<_>>();


                        // let mut choices_list: Vec<(O, i32, usize)> = Vec::new();

                        // choices_list.push(dist[i % 2].clone());


                        for choice in choices_list {
                            self.add_edge(node.clone(), choice.0.clone(), EdgeOptions::Simple);
                            choice_pos.push(choice.2);
                        }

                        for j in 0..choice_pos.len() {
                            dist[choice_pos[j]].1 += 1;
                        }

                        dist.push(((node.clone()), amount as i32, i));

                        // self.update();
                    }
                }
                self.update();
            }


            // pub fn random_attachment(&mut self, node_set: Vec<O>, u: O, direct: bool, init_edges: usize) {
            //     let n_nodes = node_set.len();

            //     self.remove_all_edges();

            //     if n_nodes == 0 {
            //         return;
            //     }
            //     self.add_node(node_set[0]);
            //     self.update();
            //     if n_nodes == 1 {
            //         return;
            //     }
            //     self.add_node(node_set[1]);

            //     self.add_edge(node_set[0], node_set[1], EdgeOptions::Simple);
            //     self.update();

            //     let mut rng = rand::thread_rng();
            //     for i in 0..n_nodes {
            //         let node = node_set[i] as O;

            //         let mut choices_listget_ed = node_set
            //             .choose_multiple(&mut rng, init_edges)
            //             .collect::<Vec<_>>();

            //         for choice in choices_list {
            //             self.add_edge(node, *choice, EdgeOptions::Simple);
            //         }
            //     }
            //     self.update();
            // }


            pub fn remove_all_edges(&self) {
                self.edges.clear();
            }

            pub fn remove_edge(&self, u: O, v: O) -> Option<Edge< L>> {
                let nodes2id = self.nodes2id.borrow();

                let uid = match nodes2id.get(&u){
                    Some(u)=> u,
                    None => return None
                };

                let vid = match nodes2id.get(&v){
                    Some(v)=> v,
                    None => return None
                };

                let mut u_edges = self.edges.get_write(uid).expect("error on get_write");

                let index = match u_edges
                    .iter()
                    .position(|entry| (entry.u == *uid && entry.v == *vid) ||
                    (entry.u == *vid && entry.v == *uid)){
                        Some(i) => i,
                        None => return None,
                };

                let u_edge = u_edges.remove(index);

                std::mem::drop(u_edges);

                if self.direct {
                    return Some(u_edge.clone());
                } else {
                    let mut v_edges = self.edges.get_write(vid).expect("error on get_write");
                    v_edges.retain(|entry| {
                        !((entry.u == *uid && entry.v == *vid) ||
                        (entry.u == *vid && entry.v == *uid))
                    });
                    return Some(u_edge.clone());
                }
            }

            pub fn remove_incoming_edges(&self, u: O) -> Option<Vec<Edge<L>>> {
                let nodes = self.edges.keys();
                let mut ris = vec![];
                let nodes2id = self.nodes2id.borrow();

                let uid = match nodes2id.get(&u){
                    Some(u)=> u,
                    None => return None
                };

                for v in nodes {
                    if v != uid {
                        let vnode = self.id2nodes.get_read(v).expect("error on get_read");
                        match self.remove_edge(vnode.clone(), u.clone()) {
                            Some(e) => ris.push(e),
                            None => (),
                        }
                    }
                }
                Some(ris)
            }


            pub fn remove_outgoing_edges(&self, u: O) -> Option<Vec<Edge<L>>> {
                let nodes = self.edges.keys();
                let mut ris = vec![];
                let nodes2id = self.nodes2id.borrow();

                let uid = match nodes2id.get(&u){
                    Some(u)=> u,
                    None => return None
                };

                for v in nodes {
                    if v != uid {
                        let vnode = self.id2nodes.get_read(v).expect("error on get_read");
                        match self.remove_edge(u.clone(), vnode.clone()) {
                            Some(e) => ris.push(e),
                            None => (),
                        }
                    }
                }
                Some(ris)
            }

            pub fn remove_node(&self, u: O) -> bool {

                let uid: u32;
                {
                    let nodes2id = self.nodes2id.borrow_mut();

                    uid = match nodes2id.get(&u) {
                    Some(u) => u.clone(),
                    None => return false,
                    };
                }


                match self.remove_outgoing_edges(u.clone()) {
                    Some(_) => {
                        self.edges.remove(&uid);
                    }
                    None => return false,
                };

                match self.remove_incoming_edges(u.clone()) {
                    Some(_) => {
                        self.edges.remove(&uid);
                    }
                    None => return false,
                };

                let mut nodes2id = self.nodes2id.borrow_mut();
                self.id2nodes.remove(&uid);
                nodes2id.remove(&u);
                true
            }

            pub fn update_node(&self, u: O){
                let uid: u32;
                {
                    let nodes2id = self.nodes2id.borrow();
                    uid = match nodes2id.get(&u){
                        Some(u)=> *u,
                        None => return
                    };
                }
                let mut nodes2id = self.nodes2id.borrow_mut();

                match self.id2nodes.get_write(&uid){
                    Some(mut value) => {

                        if let Some(_) = nodes2id.remove(&value){
                            nodes2id.insert(u.clone(), uid.clone());
                        }
                        *value = u
                    },
                    None => return
                };
            }
        }

        impl<O: Hash + Eq + Clone + Display, L: Clone + Hash + Display> Field for Network<O, L> {
            fn update(&mut self) {
                self.edges.update();
                self.id2nodes.update();
            }
            fn lazy_update(&mut self) {
                self.edges.update();
                self.id2nodes.update();
            }
        }

    } else { // not for visualization or parallel feature

        /// A network is a collection of nodes and edges.
        /// Your Node type can be a simple one like `u32` or a more complex one like a struct.
        /// If you want to use a struct as a node, you have to implement several traits.
        /// To correctly use the `Network` struct, `Hash` and `PartialEq` traits have
        /// to work with ID of the node:
        ///
        /// # Example
        /// ```
        /// #[derive(Clone, Debug, Eq)]
        /// struct Node {
        ///    id: u32,
        ///    flag: bool,
        /// }
        ///
        /// // implement Hash and PartialEq traits
        /// impl Hash for Node {
        ///    fn hash<H: Hasher>(&self, state: &mut H) {
        ///       self.id.hash(state);
        ///   }
        /// }
        ///
        /// impl PartialEq for Node {
        ///   fn eq(&self, other: &Self) -> bool {
        ///      self.id == other.id
        ///     }
        /// }
        ///
        /// impl Display for Node {
        ///    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        ///       write!(f, "Node: {}", self.id)
        ///   }
        /// }
        ///
        /// ```
        ///
        pub struct Network<O: Hash + Eq + Clone + Display, L: Clone + Hash + Display> {
            /// Write state of edges
            // pub edges: RefCell<HashMap<u32, Vec<Edge<L>>>>,
            /// Read state of edges
            // pub redges: RefCell<HashMap<u32, Vec<Edge<L>>>>,
            pub edges: Vec<RefCell<HashMap<u32, Vec<Edge<L>>>>>,
            pub read: usize,
            pub write: usize,
            /// Map from nodes to their id
            // pub nodes2id: RefCell<HashMap<O, u32>>,
            pub nodes2id: Vec<RefCell<HashMap<O, u32>>>,
            /// Map from id to nodes
            // pub id2nodes: RefCell<HashMap<u32, O>>,
            /// Map from id to nodes. Used as a read state
            // pub rid2nodes: RefCell<HashMap<u32, O>>,
            pub id2nodes: Vec<RefCell<HashMap<u32, O>>>,
            /// directed graph or not
            pub direct: bool,
        }

        impl<O: Hash + Eq + Clone + Display, L: Clone + Hash + Display + Debug> Display for Network<O, L> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
        {
            let id2nodes = self.id2nodes[self.read].borrow();
            let nodes = id2nodes.keys();
            let mut formatter = String::new();

            for i in 0..nodes.len(){
               // formatter.push_str(format!("{} ", i.to_string()).as_str() );
                for j in 0..nodes.len(){


                    let id1 = id2nodes.get(&(i as u32)).expect("error on get");
                    let id2 = id2nodes.get(&(j as u32)).expect("error on get");

                    match self.get_edge(id1.clone(), id2.clone()) {
                        Some(_) => formatter.push('1'),
                        None => formatter.push('0'),
                    }
                }
               formatter.push('-');
            }
            // for neighbor in matrix.clone() {
            //     for edge in neighbor {
            //         formatter.push_str(format!("{},{}     ", edge.u.to_string(),edge.v.to_string()).as_str());
            //     }
            //     formatter.push_str(" - ");
            // }
            write!(f, "MATRIX:\n{:?}", formatter)
        }

        }
        impl<O: Hash + Eq + Clone + Display, L: Clone + Hash + Display> Network<O, L> {
            /// Create a new Network. Network can directed or not.
            /// # Arguments
            /// * `d` - true if the network is directed
            ///
            /// # Generic Arguments
            /// * `O` - type of nodes.
            /// * `L` - type of labels.
            pub fn new(d: bool) -> Network<O, L> {
                Network {
                    // edges: RefCell::new(HashMap::new()),
                    // redges: RefCell::new(HashMap::new()),
                    edges: vec![RefCell::new(HashMap::new()), RefCell::new(HashMap::new())],
                    read: 0,
                    write: 1,
                    // nodes2id: RefCell::new(HashMap::new()),
                    nodes2id: vec![RefCell::new(HashMap::new()), RefCell::new(HashMap::new())],
                    // id2nodes: RefCell::new(HashMap::new()),
                    // rid2nodes: RefCell::new(HashMap::new()),
                    id2nodes: vec![RefCell::new(HashMap::new()), RefCell::new(HashMap::new())],
                    direct: d,
                }
            }

            /// Add new edge. Add also reverse edge if `direct` is true
            ///
            /// # Arguments
            /// * `u` - source node
            /// * `v` - target node
            /// * `edge_options` - edge options enum (label and/or weight)
            ///
            /// # Returns
            /// * `(bool, bool)` - first bool is true if the edge is added, false otherwise.
            /// Second bool is true if the reverse edge is added, false otherwise.
            /// If the network is directed, the second bool is always false.
            ///
            /// # Example: Directed network
            /// ```
            /// let mut net = Network::<u32, String>::new(true);
            /// let (added, _) = net.add_edge(1, 2, EdgeOptions::Simple);
            ///
            /// // nodes aren't added automatically
            /// assert!(!added);
            ///
            /// // add nodes
            /// net.add_node(1);
            /// net.add_node(2);
            ///
            /// // add edge
            /// let (added, _) = net.add_edge(1, 2, EdgeOptions::Simple);
            /// assert!(added);
            ///
            /// // add reverse edge manually
            /// let (added, _) = net.add_edge(2, 1, EdgeOptions::Simple);
            /// assert!(added);
            ///
            /// ```
            ///
            /// # Example: Undirected network
            /// ```
            /// let mut net = Network::<u32, String>::new(false);
            /// let (added1, added2) = net.add_edge(1, 2, EdgeOptions::Simple);
            ///
            /// // nodes aren't added automatically
            /// assert!(!added1);
            /// assert!(!added2);
            ///
            /// // add nodes
            /// net.add_node(1);
            /// net.add_node(2);
            ///
            /// // add edge
            /// let (added1, added2) = net.add_edge(1, 2, EdgeOptions::Simple);
            /// assert!(added1);
            /// assert!(added2);
            /// ```
            ///
            pub fn add_edge(&self, u: O, v: O, edge_options: EdgeOptions<L>) -> (bool, bool) {
                let nodes2id = self.nodes2id[self.write].borrow_mut();
                let mut vbool = false;

                let uid = match nodes2id.get(&u) {
                    Some(u) => u,
                    None => return (false, false),
                };

                let vid = match nodes2id.get(&v) {
                    Some(v) => v,
                    None => return (false, false),
                };

                let mut edges = self.edges[self.write].borrow_mut();

                match edges.get_mut(uid) {
                    Some(uedges) => {
                        uedges.push(Edge::new(*uid, *vid, edge_options.clone()));
                    }
                    None => {
                        let vec = vec![Edge::new(*uid, *vid, edge_options.clone())];
                        edges.insert(*uid, vec);
                    }
                }

                let ubool = true;
                if !self.direct {
                    match edges.get_mut(vid) {
                        Some(vedges) => {
                            vedges.push(Edge::new(*vid, *uid, edge_options));
                        }
                        None => {
                            let vec = vec![Edge::new(*vid, *uid, edge_options)];
                            edges.insert(*vid, vec);
                        }
                    }
                    vbool = true;
                }
                (ubool, vbool)
            }

            /// Add a new node to the network.
            ///
            /// # Arguments
            /// * `u` - node to add
            ///
            /// # Example
            /// ```
            /// let mut net = Network::<u32, String>::new(true);
            /// net.add_node(0);
            ///
            /// net.update();
            ///
            /// let node = net.get_object(0);
            /// assert_eq!(node, Some(&0));
            ///
            /// ```
            ///
            pub fn add_node(&self, u: O) {
                let mut nodes2id = self.nodes2id[self.write].borrow_mut();
                let mut id2nodes = self.id2nodes[self.write].borrow_mut();
                let uid = nodes2id.len() as u32;
                nodes2id.insert(u.clone(), uid);
                id2nodes.insert(uid, u);

                let mut edges = self.edges[self.write].borrow_mut();
                match edges.get(&uid) {
                    Some(_edges) => {}
                    None => {
                        let vec: Vec<Edge<L>> = Vec::new();
                        edges.insert(uid, vec);
                    }
                }
            }


            /// Get an `Edge` from the network
            ///
            /// # Arguments
            /// * `u` - source node
            /// * `v` - target node
            ///
            /// If the network is directed, the edge is searched from `u` to `v`.
            ///
            /// If the network is undirected, the edge is searched from `u` to `v` and from `v` to `u`.
            /// The first edge found is returned.
            ///
            /// # Returns
            /// * `Option<Edge<L>>` - `Edge` if it exists, `None` otherwise
            ///
            /// # Example: Directed network
            /// ```
            /// let mut net = Network::<u32, String>::new(true);
            ///
            /// net.add_node(0);
            /// net.add_node(1);
            /// net.add_edge(0, 1, EdgeOptions::Simple);
            ///
            /// net.update();
            ///
            /// let edge = net.get_edge(0, 1);
            /// assert!(edge.is_some());
            ///
            /// let edge = net.get_edge(1, 0);
            /// assert!(edge.is_none());
            ///
            /// ```
            ///
            /// # Example: Undirected network
            /// ```
            /// let mut net = Network::<u32, String>::new(false);
            ///
            /// net.add_node(0);
            /// net.add_node(1);
            /// net.add_edge(0, 1, EdgeOptions::Simple);
            ///
            /// let none = net.get_edge(1, 0);
            /// assert!(none.is_none());
            ///
            /// net.update();
            ///
            /// let edge = net.get_edge(0, 1);
            /// assert!(edge.is_some());
            ///
            /// let edge = net.get_edge(1, 0);
            /// assert!(edge.is_some());
            ///
            /// ```
            pub fn get_edge(&self, u: O, v: O) -> Option<Edge<L>> {
                let nodes2id = self.nodes2id[self.read].borrow();

                let id2nodes = self.id2nodes[self.read].borrow();

                let uid = match nodes2id.get(&u) {
                    Some(u) => u,
                    None => return None,
                };

                let edges = self.edges[self.read].borrow();
                match edges.get(uid) {
                    Some(uedges) => {
                        let vid = match nodes2id.get(&v) {
                            Some(v) => v,
                            None => return None,
                        };

                        for e in uedges {
                            let vid_edge = nodes2id.get(id2nodes.get(&e.v).expect("error on get")).expect("error on get");
                            if e.u == *uid && *vid_edge == *vid || !self.direct && *vid_edge == *uid && e.u == *vid {
                                return Some(e.clone());
                            }
                        }
                        None
                    }
                    None => None,
                }
            }

            /// Get all edges of a node
            ///
            /// # Arguments
            /// * `u` - node
            ///
            /// # Returns
            /// * `Option<Vec<Edge<L>>>` - `Vec` of `Edge`s if the node exists, `None` otherwise.
            ///
            /// # Example: Directed network
            /// ```
            /// let mut net = Network::<u32, String>::new(true);
            ///
            /// net.add_node(0);
            /// net.add_node(1);
            /// net.add_node(2);
            /// net.add_node(3);
            /// net.add_edge(0, 1, EdgeOptions::Simple);
            /// net.add_edge(0, 2, EdgeOptions::Simple);
            ///
            /// net.update();
            ///
            /// let edges = net.get_edges(0);
            /// assert_eq!(edges.unwrap().len(), 2);
            ///
            /// let edges = net.get_edges(1);
            /// assert_eq!(edges.unwrap().len(), 1);
            ///
            /// let edges = net.get_edges(3);
            /// assert!(edges.unwrap().is_empty());
            ///
            /// let edges = net.get_edges(4);
            /// assert!(edges.is_none());
            ///
            /// ```
            ///
            /// # Example: Undirected network
            /// ```
            /// let mut net = Network::<u32, String>::new(false);
            ///
            /// net.add_node(0);
            /// net.add_node(1);
            /// net.add_node(2);
            /// net.add_node(3);
            /// net.add_edge(0, 1, EdgeOptions::Simple);
            /// net.add_edge(0, 2, EdgeOptions::Simple);
            ///
            /// net.update();
            ///
            /// let edges = net.get_edges(0);
            /// // 2 edges from 0 to 1 and 2, and 2 edges from 1 and 2 to 0
            /// assert_eq!(edges.unwrap().len(), 4);
            ///
            /// let edges = net.get_edges(1);
            /// // 1 edge from 1 to 0, and 1 edge from 0 to 1
            /// assert_eq!(edges.unwrap().len(), 2);
            ///
            /// let edges = net.get_edges(3);
            /// assert!(edges.unwrap().is_empty());
            ///
            /// let edges = net.get_edges(4);
            /// assert!(edges.is_none());
            ///
            /// ```
            pub fn get_edges(&self, u: O) -> Option<Vec<Edge<L>>> {
                let nodes2id = self.nodes2id[self.read].borrow();
                let uid = match nodes2id.get(&u) {
                    Some(u) => u,
                    None => return None,
                };
                let edges = self.edges[self.read].borrow();
                edges.get(uid).map(|es| (*(es.clone())).to_vec())
            }


            /// Get a node from an id. Returns `None` if the id is not found
            ///
            /// # Arguments
            /// * `uid` - id of the node
            ///
            /// # Returns
            /// * `Option<O>` - node if the id exists, `None` otherwise.
            ///
            /// # Example
            /// ```
            /// id: u32,
            ///
            /// let mut net = Network::<u32, String>::new(true);
            ///
            /// net.add_node(1);
            ///
            /// net.update();
            ///
            /// // PS: the id is not the same as the node, it is an internal id
            /// // that is used to store the node in the network
            /// let node = net.get_object(0);
            /// assert_eq!(node.unwrap(), 1);
            ///
            /// let none = net.get_object(1);
            /// assert!(none.is_none());
            ///
            pub fn get_object(&self, uid: u32) -> Option<O> {
                // self.id2nodes[self.read].borrow_mut().get(&uid).cloned()
                self.id2nodes[self.read].borrow().get(&uid).cloned()
            }

            /// Get the id of a node. Returns `None` if the node is not found
            ///
            /// # Arguments
            /// * `u` - node
            ///
            /// # Returns
            /// * `Option<u32>` - id of the node if the node exists, `None` otherwise.
            ///
            /// # Example
            /// ```
            /// let mut net = Network::<u32, String>::new(true);
            ///
            /// net.add_node(1);
            ///
            /// net.update();
            ///
            /// // PS: the id is not the same as the node, it is an internal id
            /// // that is used to store the node in the network
            /// let id = net.get_id(1);
            /// assert_eq!(id.unwrap(), 0);
            ///
            /// let none = net.get_id(0);
            /// assert!(none.is_none());
            ///
            /// ```
            ///
            pub fn get_id(&self, u: O) -> Option<u32> {
                self.nodes2id[self.read].borrow().get(&u).cloned()
            }

            /// Generate an undirected network based on
            /// Barabási-Albert’s preferential attachment model.
            /// Create the edges, but not the nodes.
            ///
            ///
            /// # Arguments
            /// * `node_set` - nodes of the network
            /// * `init_edges` - initial edges for each node
            ///
            /// # Example
            /// ```
            /// let mut net = Network::<u32, String>::new(false);
            /// let node_set: &[u32] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
            ///
            /// for node in node_set {
            ///    net.add_node(*node);
            /// }
            ///
            /// net.update();
            ///
            /// // 1 edge for each node
            /// net.preferential_attachment_BA(node_set, 1);
            ///
            /// // Network is ready to be used
            /// ```
            ///
            #[allow(non_snake_case)]
            pub fn preferential_attachment_BA(
                &mut self,
                node_set: &[O],
                init_edges: usize
            ) {
                {
                    let id2nodes = self.id2nodes[self.write].borrow_mut();
                    let n_nodes = id2nodes.len();
                    // clear the existing edges
                    self.remove_all_edges();

                    // if there are no nodes return
                    if node_set.is_empty() || node_set.len() == 1 {
                        return;
                    }

                    // create the first edge between the first two nodes
                    let first_node = node_set[0].clone();
                    let second_node = node_set[1].clone();
                    self.add_edge(first_node.clone(), second_node.clone(), EdgeOptions::Simple);
                    // self.update();

                    let mut rng = rand::thread_rng();
                    let mut dist: Vec<(O, i32, usize)> = Vec::with_capacity(n_nodes);

                    // if self.direct {
                    //     dist.push((first_node, 0, 0));
                    // }
                    // else {
                    //     dist.push((first_node, 1, 0));
                    // }

                    dist.push((first_node, 1, 0));
                    dist.push((second_node, 1, 1));

                    // iterates on the node_set skipping the first two nodes
                    for i in 2..n_nodes {
                        let mut choice_pos: Vec<usize> = Vec::with_capacity(init_edges);

                        let node = node_set[i].clone();

                        let amount: usize = if dist.len() < init_edges {
                            dist.len()
                        } else {
                            init_edges
                        };

                        let choices_list = dist
                            .choose_multiple_weighted(&mut rng, amount, |choice| choice.1)
                            .expect("error onchoose_multiple_weighted")
                            .collect::<Vec<_>>();

                        for choice in choices_list {
                            self.add_edge(node.clone(), choice.0.clone(), EdgeOptions::Simple);
                            choice_pos.push(choice.2);
                        }

                        for i in 0..choice_pos.len() {
                            dist[choice_pos[i]].1 += 1;
                        }

                        dist.push(((node.clone()), amount as i32, i));
                    }
                }
                self.update();
            }

            /// Generate an undirected network based on
            /// Barabási-Albert’s preferential attachment model
            /// with defined seed.
            /// Create the edges, but not the nodes.
            ///
            /// # Arguments
            /// * `node_set` - nodes of the network
            /// * `init_edges` - initial edges for each node
            /// * `my_seed` - seed for the random number generator
            ///
            /// # Example
            /// ```
            /// let mut net = Network::<u32, String>::new(false);
            /// let mut net2 = Network::<u32, String>::new(false);
            ///
            /// let node_set: &[u32] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
            ///
            /// for node in node_set {
            ///   net.add_node(*node);
            ///   net2.add_node(*node);
            /// }
            ///
            /// net.update();
            /// net2.update();
            ///
            /// // 1 edge for each node
            /// net.preferential_attachment_BA_with_seed(node_set, 1, 5);
            /// net2.preferential_attachment_BA_with_seed(node_set, 1, 5);
            ///
            /// // networks are the same
            /// for node in node_set {
            ///    let edges = match net.get_edges(*node) {
            ///        Some(edges) => edges,
            ///        None => Vec::new(),
            ///    };
            ///    let edges2 = match net2.get_edges(*node) {
            ///        Some(edges) => edges,
            ///        None => Vec::new(),
            ///    };
            ///    assert_eq!(edges.len(), edges2.len());
            ///    for i in 0..edges.len() {
            ///        let e1 = &edges[i];
            ///        let e2 = &edges2[i];
            ///        assert_eq!(e1.u, e2.u);
            ///        assert_eq!(e1.v, e2.v);
            ///    }
            /// }
            ///
            /// ```
            #[allow(non_snake_case)]
            pub fn preferential_attachment_BA_with_seed(
                &mut self,
                node_set: &[O],
                init_edges: usize,
                my_seed: u64,
            ) {
                {
                    let id2nodes = self.id2nodes[self.write].borrow_mut();
                    let n_nodes = id2nodes.len();
                    // clear the existing edges
                    self.remove_all_edges();

                    // if there are no nodes return
                    if node_set.is_empty() || node_set.len() == 1 {
                        return;
                    }

                    // create the first edge between the first two nodes
                    let first_node = node_set[0].clone();
                    let second_node = node_set[1].clone();
                    self.add_edge(first_node.clone(), second_node.clone(), EdgeOptions::Simple);

                    // let mut rng = Pcg64::seed_from_u64(my_seed);
                    let mut rng = StdRng::seed_from_u64(my_seed);
                    let mut dist: Vec<(O, i32, usize)> = Vec::with_capacity(n_nodes);

                    dist.push((first_node, 1, 0));
                    dist.push((second_node, 1, 1));


                    // iterates on the node_set skipping the first two nodes
                    for i in 2..n_nodes {
                        let mut choice_pos: Vec<usize> = Vec::with_capacity(init_edges);

                        let node = node_set[i].clone();

                        let amount: usize = if dist.len() < init_edges {
                            dist.len()
                        } else {
                            init_edges
                        };

                        let choices_list = dist
                            .choose_multiple_weighted(&mut rng, amount, |choice| choice.1)
                            .expect("error on choose_multiple_weighted")
                            .collect::<Vec<_>>();

                        for choice in choices_list {
                            self.add_edge(node.clone(), choice.0.clone(), EdgeOptions::Simple);
                            choice_pos.push(choice.2);
                        }

                        for i in 0..choice_pos.len() {
                            dist[choice_pos[i]].1 += 1;
                        }

                        dist.push(((node.clone()), amount as i32, i));


                    }
                }
                self.update();
            }


            /// Remove all Network edges
            ///
            /// # Example
            /// ```
            /// let mut net = Network::<u32, String>::new(false);
            ///
            /// net.add_node(0);
            /// net.add_node(1);
            /// net.add_node(2);
            ///
            /// net.add_edge(0, 1, EdgeOptions::Simple);
            /// net.add_edge(0, 2, EdgeOptions::Simple);
            /// net.add_edge(1, 2, EdgeOptions::Simple);
            ///
            /// net.remove_all_edges();
            /// net.update();
            ///
            /// assert!(net.get_edges(0).unwrap().is_empty());
            /// assert!(net.get_edges(1).unwrap().is_empty());
            /// assert!(net.get_edges(2).unwrap().is_empty());
            /// ```
            ///
            pub fn remove_all_edges(&self) {
                let mut edges = self.edges[self.write].borrow_mut();
                edges.clear();
            }

            /// Remove a specific edge. Remove also reverse edge if `direct` is true
            ///
            /// # Arguments
            /// * `u` - instance of the first node
            /// * `v` - instance of the second node
            ///
            /// # Returns
            /// * `Option<Edge<L>` - the removed edge, if it existed.
            ///
            /// # Example: Directed Network
            /// ```
            /// let mut net = Network::<u32, String>::new(true);
            ///
            /// net.add_node(0);
            /// net.add_node(1);
            ///
            /// net.add_edge(0, 1, EdgeOptions::Simple);
            /// net.add_edge(1, 0, EdgeOptions::Simple);
            ///
            /// net.remove_edge(0, 1);
            /// net.update();
            ///
            /// assert!(net.get_edges(0).unwrap().is_empty());
            /// assert_eq!(net.get_edges(1).unwrap().len(), 1);
            ///
            /// ```
            ///
            /// # Example: Undirected Network
            /// ```
            /// let mut net = Network::<u32, String>::new(false);
            ///
            /// net.add_node(0);
            /// net.add_node(1);
            ///
            /// net.add_edge(0, 1, EdgeOptions::Simple);
            ///
            /// net.remove_edge(0, 1);
            /// net.update();
            ///
            /// assert!(net.get_edges(0).unwrap().is_empty());
            /// assert!(net.get_edges(1).unwrap().is_empty());
            ///
            /// ```
            pub fn remove_edge(&self, u: O, v: O) -> Option<Edge<L>> {
                let nodes2id = self.nodes2id[self.read].borrow();

                let uid = match nodes2id.get(&u) {
                    Some(u) => u,
                    None => return None,
                };

                let vid = match nodes2id.get(&v) {
                    Some(v) => v,
                    None => return None,
                };


                let mut edges = self.edges[self.write].borrow_mut();
                let u_edges = edges.get_mut(uid).expect("error on get_mut");

                let index = match u_edges.iter().position(|entry| {
                    (entry.u == *uid && entry.v == *vid) || (entry.u == *vid && entry.v == *uid)
                }) {
                    Some(i) => i,
                    None => return None,
                };

                let u_edge = u_edges.remove(index);

                if !self.direct {
                    let v_edges = edges.get_mut(vid).expect("error on get_mut");
                    println!(" HELLO ");
                    v_edges.retain(|entry| {
                        !((entry.u == *uid && entry.v == *vid) || (entry.u == *vid && entry.v == *uid))
                    });
                }
                Some(u_edge)

            }


            /// Remove all incoming edges of a node
            ///
            /// # Arguments
            /// * `u` - instance of the node
            ///
            /// # Returns
            /// * `Vec<Edge<L>>` - the removed edges, if they existed.
            ///
            /// # Example: Directed Network
            /// ```
            /// let mut net = Network::<u32, String>::new(true);
            ///
            /// net.add_node(0);
            /// net.add_node(1);
            /// net.add_node(2);
            ///
            /// net.add_edge(1, 0, EdgeOptions::Simple);
            /// net.add_edge(2, 0, EdgeOptions::Simple);
            /// net.add_edge(0, 1, EdgeOptions::Simple);
            ///
            /// net.remove_incoming_edges(0);
            /// net.update();
            ///
            /// assert_eq!(net.get_edges(0).unwrap().len(), 1);
            /// assert!(net.get_edges(1).unwrap().is_empty());
            /// assert!(net.get_edges(2).unwrap().is_empty());
            ///
            /// ```
            ///
            /// # Example: Undirected Network
            /// Removing incoming edges of a node in an undirected network is equivalent
            /// to removing all edges of the node
            ///
            /// ```
            /// let mut net = Network::<u32, String>::new(false);
            ///
            /// net.add_node(0);
            /// net.add_node(1);
            /// net.add_node(2);
            ///
            /// net.add_edge(1, 0, EdgeOptions::Simple);
            /// net.add_edge(2, 0, EdgeOptions::Simple);
            /// net.add_edge(1, 2, EdgeOptions::Simple);
            ///
            /// net.remove_incoming_edges(0);
            /// net.update();
            ///
            /// assert!(net.get_edges(0).unwrap().is_empty());
            /// assert_eq!(net.get_edges(1).unwrap().len(), 1);
            /// assert_eq!(net.get_edges(2).unwrap().len(), 1);
            ///
            /// ```
            pub fn remove_incoming_edges(&self, u: O) -> Option<Vec<Edge<L>>> {

                let mut ris = vec![];
                let id2nodes = self.id2nodes[self.write].borrow();
                let nodes2id = self.nodes2id[self.read].borrow();

                let uid = match nodes2id.get(&u) {
                    Some(u) => u,
                    None => return None,
                };

                for v in id2nodes.keys(){
                    if v != uid {
                            let vnode = id2nodes.get(v).expect("error on get");
                            if let Some(e) = self.remove_edge(vnode.clone(), u.clone()) {
                                ris.push(e)
                        }
                    }
                }

                Some(ris)
            }

            /// Remove all outgoing edges of a node
            ///
            /// # Arguments
            /// * `u` - instance of the node
            ///
            /// # Returns
            /// * `Option<Vec<Edge<L>>>` - the removed edges, if they existed.
            ///
            /// # Example: Directed Network
            /// ```
            /// let mut net = Network::<u32, String>::new(true);
            ///
            /// net.add_node(0);
            /// net.add_node(1);
            /// net.add_node(2);
            ///
            /// net.add_edge(0, 1, EdgeOptions::Simple);
            /// net.add_edge(0, 2, EdgeOptions::Simple);
            /// net.add_edge(2, 0, EdgeOptions::Simple);
            ///
            /// net.remove_outgoing_edges(0);
            /// net.update();
            ///
            /// assert!(net.get_edges(0).unwrap().is_empty());
            /// assert_eq!(net.get_edges(2).unwrap()len(), 1);
            ///
            /// ```
            ///
            /// # Example: Undirected Network
            /// Removing outgoing edges of a node in an undirected network is equivalent
            /// to removing all edges of the node
            /// ```
            /// let mut net = Network::<u32, String>::new(false);
            ///
            /// net.add_node(0);
            /// net.add_node(1);
            /// net.add_node(2);
            ///
            /// net.add_edge(0, 1, EdgeOptions::Simple);
            /// net.add_edge(0, 2, EdgeOptions::Simple);
            /// net.add_edge(1, 2, EdgeOptions::Simple);
            ///
            /// net.remove_outgoing_edges(0);
            /// net.update();
            ///
            /// assert!(net.get_edges(0).unwrap().is_empty());
            /// assert_eq!(net.get_edges(1).unwrap().len(), 1);
            /// assert_eq!(net.get_edges(2).unwrap().len(), 1);
            /// ```
            ///
            pub fn remove_outgoing_edges(&self, u: O) -> Option<Vec<Edge<L>>> {

                let mut ris = vec![];
                let id2nodes = self.id2nodes[self.write].borrow();
                let nodes2id = self.nodes2id[self.read].borrow();

                let uid = match nodes2id.get(&u) {
                    Some(u) => u,
                    None => return None,
                };

                for v in id2nodes.keys(){
                    if v != uid {
                            let vnode = id2nodes.get(v).expect("error on get");
                            if let Some(e) = self.remove_edge(u.clone(), vnode.clone()) {
                                ris.push(e)
                        }
                    }
                }

                Some(ris)
            }


            /// Remove a specific node.
            /// All edges connected to the node are removed.
            /// # Arguments
            /// * `u` - instance of the node
            ///
            /// # Returns
            /// * `bool` - true if the node was removed, false otherwise
            ///
            /// # Example: Directed Network
            /// ```
            /// let mut net = Network::<u32, String>::new(true);
            ///
            /// net.add_node(0);
            /// net.add_node(1);
            /// net.add_node(2);
            ///
            /// net.add_edge(0, 1, EdgeOptions::Simple);
            /// net.add_edge(0, 2, EdgeOptions::Simple);
            /// net.add_edge(2, 0, EdgeOptions::Simple);
            ///
            /// net.remove_node(0);
            /// net.update();
            ///
            /// assert!(net.get_edges(0).is_none());
            /// assert!(net.get_edges(2).unwrap().is_empty());
            ///
            /// ```
            ///
            /// # Example: Undirected Network
            /// ```
            /// let mut net = Network::<u32, String>::new(false);
            ///
            /// net.add_node(0);
            /// net.add_node(1);
            /// net.add_node(2);
            ///
            /// net.add_edge(0, 1, EdgeOptions::Simple);
            /// net.add_edge(0, 2, EdgeOptions::Simple);
            ///
            /// net.remove_node(0);
            /// net.update();
            ///
            /// assert!(net.get_edges(0).is_none());
            /// assert_eq!(net.get_edges(1).unwrap().is_empty());
            /// assert_eq!(net.get_edges(2).unwrap().is_empty());
            ///
            /// ```
            pub fn remove_node(&self, u: O) -> bool {
                let uid: u32;
                {
                    let nodes2id = self.nodes2id[self.write].borrow_mut();

                    uid = match nodes2id.get(&u) {
                    Some(u) => *u,
                    None => return false,
                    };
                }


                match self.remove_outgoing_edges(u.clone()) {
                    Some(_) => {
                        let mut edges = self.edges[self.write].borrow_mut();
                        edges.remove(&uid);
                    }
                    None => return false,
                };

                match self.remove_incoming_edges(u.clone()) {
                    Some(_) => {
                        let mut edges = self.edges[self.write].borrow_mut();
                        edges.remove(&uid);
                    }
                    None => return false,
                };

                let mut id2nodes = self.id2nodes[self.write].borrow_mut();
                let mut nodes2id = self.nodes2id[self.write].borrow_mut();

                id2nodes.remove(&uid);
                nodes2id.remove(&u);
                true
            }

            /// Update node info.
            /// This method worsk if nodes are `Struct` with `Hash`, `Eq` and `PartialEq`
            /// traits implemented. All the methods has to consider only di id of the node.
            ///
            /// Primitive types are not supported with this method, because their update changes
            /// the hash value, and the node is not found in the network.
            ///
            /// # Arguments
            /// * `u` - instance of the node to update
            ///
            /// # Example
            /// ```
            /// #[derive(Clone, Debug, Eq)]
            /// struct Node {
            ///    id: u32,
            ///    flag: bool,
            /// }
            ///
            /// // implement Hash and PartialEq traits for Node
            ///
            /// let mut net = Network::<Node, String>::new(true);
            /// let n = Node { id: 0, flag: false };
            ///
            /// net.add_node(n.clone());
            ///
            /// net.update_node(Node { id: 0, flag: true });
            /// net.update();
            ///
            /// assert!(net.get_node(0).unwrap().flag);
            /// ```
            pub fn update_node(&self, u: O) {

                let nodes2id = self.nodes2id[self.write].borrow_mut();
                let mut id2nodes = self.id2nodes[self.write].borrow_mut();


                let uid = match nodes2id.get(&u) {
                    Some(u) => u,
                    None => return,
                };
                if let Some(value) = id2nodes.get_mut(uid) { *value = u }
            }
        }

        impl<O: Hash + Eq + Clone + Display, L: Clone + Hash + Display> Field
            for Network<O, L>
        {
            fn update(&mut self) {
                let edges = self.edges[self.write].borrow_mut();
                let mut redges = self.edges[self.read].borrow_mut();
                *redges = edges.clone();

                let id2nodes = self.id2nodes[self.write].borrow_mut();
                let mut rid2nodes = self.id2nodes[self.read].borrow_mut();
                *rid2nodes = id2nodes.clone();

                let nodes2id = self.nodes2id[self.write].borrow_mut();
                let mut rnodes2id = self.nodes2id[self.read].borrow_mut();
                *rnodes2id = nodes2id.clone();
            }

            fn lazy_update(&mut self) {

                let edges = self.edges[self.write].borrow_mut();
                let mut redges = self.edges[self.read].borrow_mut();
                *redges = edges.clone();

                let id2nodes = self.id2nodes[self.write].borrow_mut();
                let mut rid2nodes = self.id2nodes[self.read].borrow_mut();
                *rid2nodes = id2nodes.clone();

                let nodes2id = self.nodes2id[self.write].borrow_mut();
                let mut rnodes2id = self.nodes2id[self.read].borrow_mut();
                *rnodes2id = nodes2id.clone();
            }
        }
    }
}
