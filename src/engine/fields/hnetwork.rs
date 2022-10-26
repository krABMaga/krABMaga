use crate::engine::fields::field::Field;
use hashbrown::HashMap;
use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::Display;
use std::hash::Hash;

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

/// An hyper-edge that can be used inside an `HNetwork`
#[derive(Clone, Debug)]
pub struct HEdge<L: Clone + Hash + Display> {
    /// id nodes of an hyper-edge. Using `HashSet` we have much more control on duplicated nodes/edges
    pub nodes: HashSet<u32>,
    pub label: Option<L>,
    pub weight: Option<f32>,
}

impl<L: Clone + Hash + Display> HEdge<L> {
    /// Create a new hyper-edge
    /// # Arguments
    /// * `list_nodes` - nodes of the hyper-edge
    /// * `edge_options` - Enum to set edge information
    pub fn new(list_nodes: &[u32], edge_options: EdgeOptions<L>) -> HEdge<L> {
        let max_len = list_nodes.len();
        let mut hedge = match edge_options {
            EdgeOptions::Simple => HEdge {
                nodes: HashSet::with_capacity(max_len),
                label: None,
                weight: None,
            },
            EdgeOptions::Labeled(l) => HEdge {
                nodes: HashSet::with_capacity(max_len),
                label: Some(l),
                weight: None,
            },
            EdgeOptions::Weighted(w) => HEdge {
                nodes: HashSet::with_capacity(max_len),
                label: None,
                weight: Some(w),
            },
            EdgeOptions::WeightedLabeled(l, w) => HEdge {
                nodes: HashSet::with_capacity(max_len),
                label: Some(l),
                weight: Some(w),
            },
        };

        for n in list_nodes {
            hedge.nodes.insert(*n);
        }

        hedge
    }
}

impl<L> PartialEq for HEdge<L>
where
    L: Clone + Hash + Display,
{
    /// Two Hyper-edges are equals if the sets of nodes are equal
    fn eq(&self, other: &Self) -> bool {
        self.nodes == other.nodes
    }
}

impl<L: Clone + Hash + Display> Eq for HEdge<L> {}

/// A generalization of a `Network`, to connect with an edge multiple nodes
/// Your Node type can be a simple one like `u32` or a more complex one like a struct.
/// If you want to use a struct as a node, you have to implement several traits.
/// To correctly use the `HNetwork` struct, `Hash` and `PartialEq` traits have
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
pub struct HNetwork<O: Hash + Eq + Clone + Display, L: Clone + Hash + Display> {
    /// Write state of hyper-edges
    pub edges: RefCell<HashMap<u32, Vec<HEdge<L>>>>,
    /// Read state of hyper-edges
    pub redges: RefCell<HashMap<u32, Vec<HEdge<L>>>>,
    /// Write state to manage ids using Nodes
    pub nodes2id: RefCell<HashMap<O, u32>>,
    /// Write state to manage Nodes using ids
    pub id2nodes: RefCell<HashMap<u32, O>>,
    /// Read state to manage Nodes using ids
    pub rid2nodes: RefCell<HashMap<u32, O>>,
}

impl<O: Hash + Eq + Clone + Display, L: Clone + Hash + Display> HNetwork<O, L> {
    /// Create a new HNetwork
    pub fn new() -> HNetwork<O, L> {
        HNetwork {
            edges: RefCell::new(HashMap::new()),
            redges: RefCell::new(HashMap::new()),
            nodes2id: RefCell::new(HashMap::new()),
            id2nodes: RefCell::new(HashMap::new()),
            rid2nodes: RefCell::new(HashMap::new()),
        }
    }

    /// Add a new hyper-edge.
    ///
    /// # Arguments
    /// * `nodes` - nodes of the hyper-edge you want to add
    /// * `edge_options` - Enum to set edge information
    ///
    /// # Returns
    /// * `bool` - `true` if the hyper-edge is added, `false` otherwise
    ///
    /// # Example
    /// ```
    /// let mut net = HNetwork::<u32, String>::new();
    /// net.add_node(1);
    /// net.add_node(2);
    /// net.add_node(3);
    ///
    /// assert!(net.add_edge(&[1, 2, 3], EdgeOptions::Simple));
    ///
    /// // You can't add the same hyper-edge twice
    /// // The order of nodes is not important
    /// assert!(!net.add_edge(&[2, 3, 1], EdgeOptions::Simple));
    ///
    /// ```
    pub fn add_edge(&self, nodes: &[O], edge_options: EdgeOptions<L>) -> bool {
        if nodes.is_empty() {
            return false;
        }

        let nodes2id = self.nodes2id.borrow_mut();

        let mut ids = Vec::with_capacity(nodes.len());
        for n in nodes {
            match nodes2id.get(n) {
                Some(val) => ids.push(*val),
                None => return false,
            }
        }
        let ids = ids.as_slice();

        let mut edges = self.edges.borrow_mut();

        for id in ids {
            match edges.get_mut(id) {
                Some(uedges) => {
                    if uedges.contains(&HEdge::new(ids, edge_options.clone())) {
                        return false;
                    }
                    uedges.push(HEdge::new(ids, edge_options.clone()));
                }
                None => {
                    let vec = vec![HEdge::new(ids, edge_options.clone())];
                    edges.insert(*id, vec);
                }
            }
        }

        true
    }

    /// Add a new node
    /// # Arguments
    /// * `u` - node you want to add
    ///
    /// # Example
    /// ```
    /// let mut net = HNetwork::<u32, String>::new();
    /// net.add_node(0);
    ///
    /// net.update();
    ///
    /// assert!(net.get_node(0).is_some());
    ///
    /// ```
    ///
    pub fn add_node(&self, u: O) {
        let mut nodes2id = self.nodes2id.borrow_mut();
        let mut id2nodes = self.id2nodes.borrow_mut();
        let uid = nodes2id.len() as u32;
        nodes2id.insert(u.clone(), uid);
        id2nodes.insert(uid, u);

        let mut edges = self.edges.borrow_mut();
        match edges.get(&uid) {
            Some(_edges) => {}
            None => {
                let vec: Vec<HEdge<L>> = Vec::new();
                edges.insert(uid, vec);
            }
        }
    }

    /// Get an hyper-edge from a list of nodes. `None` if not found
    ///
    /// # Arguments
    /// * `nodes` - nodes of the hyper-edge you want to get
    ///
    /// # Returns
    /// * `Option<HEdge<L>>` - `Some(HEdge<L>)` if the hyper-edge is found, `None` otherwise
    ///
    /// # Example
    /// ```
    /// let mut net = HNetwork::<u32, String>::new();
    /// net.add_node(1);
    /// net.add_node(2);
    /// net.add_node(3);
    ///
    /// net.add_edge(&[1, 2, 3], EdgeOptions::Simple);
    ///
    /// net.update();
    ///
    /// assert!(net.get_edge(&[1, 2, 3]).is_some());
    /// assert!(net.get_edge(&[3, 1, 2]).is_some());
    ///
    /// assert!(net.get_edge(&[1, 2]).is_none());
    /// ```
    pub fn get_edge(&self, nodes: &[O]) -> Option<HEdge<L>> {
        if nodes.is_empty() {
            return None;
        }

        let nodes2id = self.nodes2id.borrow();

        let mut ids = Vec::with_capacity(nodes.len());
        for n in nodes {
            match nodes2id.get(n) {
                Some(val) => ids.push(*val),
                None => return None,
            }
        }

        let edges = self.redges.borrow();
        match edges.get(&ids[0]) {
            Some(uedges) => {
                let edge: HEdge<L> = HEdge::new(ids.as_slice(), EdgeOptions::Simple);
                for e in uedges {
                    if *e == edge {
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
    /// * `u` - node you want to get the edges
    ///
    /// # Example
    /// ```
    /// let mut net = HNetwork::<u32, String>::new();
    /// net.add_node(1);
    /// net.add_node(2);
    /// net.add_node(3);
    ///
    /// net.add_edge(&[1, 2, 3], EdgeOptions::Simple);
    /// net.add_edge(&[1, 2], EdgeOptions::Simple);
    ///
    /// net.update();
    ///
    /// assert_eq!(net.get_edges(1).unwrap().len(), 2);
    /// assert_eq!(net.get_edges(2).unwrap().len(), 2);
    /// assert_eq!(net.get_edges(3).unwrap().len(), 1);
    /// ```
    ///
    pub fn get_edges(&self, u: O) -> Option<Vec<HEdge<L>>> {
        let nodes2id = self.nodes2id.borrow();
        let uid = match nodes2id.get(&u) {
            Some(u) => u,
            None => return None,
        };
        let edges = self.redges.borrow();
        edges.get(uid).map(|es| (*(es.clone())).to_vec())
    }

    /// Get a node from its id
    ///
    /// # Arguments
    /// * `uid` - id of the node you want to get.
    ///   PS: the id is not the same as the node itself, it is assigned by the network
    ///   when you add a node. You can get the id of a node with `get_id`.
    ///
    /// # Example
    /// ```
    /// let mut net = HNetwork::<u32, String>::new();
    /// net.add_node(10);
    ///
    /// net.update();
    ///
    /// assert_eq!(net.get_node(0).unwrap(), 10);
    /// assert_eq!(net.get_node(10), None);
    /// ```
    pub fn get_object(&self, uid: u32) -> Option<O> {
        self.rid2nodes.borrow_mut().get(&uid).cloned()
    }

    /// Get the id of a node
    ///
    /// # Arguments
    /// * `u` - node you want to get the id
    ///
    /// # Example
    /// ```
    /// let mut net = HNetwork::<u32, String>::new();
    /// net.add_node(10);
    ///
    /// net.update();
    ///
    /// assert_eq!(net.get_id(10).unwrap(), 0);
    /// assert_eq!(net.get_id(0), None);
    /// ```
    ///
    pub fn get_id(&self, u: &O) -> Option<u32> {
        self.nodes2id.borrow().get(u).cloned()
    }
    /// Remove all the edges of the network
    ///
    /// # Example
    /// ```
    /// let mut net = HNetwork::<u32, String>::new();
    /// net.add_node(1);
    /// net.add_node(2);
    /// net.add_node(3);
    ///
    /// net.add_edge(&[1, 2, 3], EdgeOptions::Simple);
    /// net.add_edge(&[1, 2], EdgeOptions::Simple);
    ///
    /// net.update();
    ///
    /// assert_eq!(net.get_edges(1).unwrap().len(), 2);
    /// assert_eq!(net.get_edges(2).unwrap().len(), 2);
    /// assert_eq!(net.get_edges(3).unwrap().len(), 1);
    ///
    /// net.clear_edges();
    /// net.update();
    ///
    /// assert!(net.get_edges(1).is_none());
    /// assert!(net.get_edges(2).is_none());
    /// assert!(net.get_edges(3).is_none());    
    ///
    /// ```
    ///
    pub fn remove_all_edges(&self) {
        let mut edges = self.edges.borrow_mut();
        edges.clear();
    }

    /// Remove a specific edge using a list of nodes
    ///
    /// # Arguments
    /// * `nodes` - nodes of the hyper-edge you want to remove
    ///
    /// # Returns
    /// * `Option<HEdge<L>>` - `Some(HEdge<L>)` if the hyper-edge is found and removed, `None` otherwise
    ///
    /// # Example
    /// ```
    /// let mut net = HNetwork::<u32, String>::new();
    /// net.add_node(1);
    /// net.add_node(2);
    /// net.add_node(3);
    ///
    /// net.add_edge(&[1, 2, 3], EdgeOptions::Simple);
    /// net.add_edge(&[1, 2], EdgeOptions::Simple);
    /// net.remove_edge(&[1, 2, 3]);
    ///
    /// net.update();
    ///
    /// assert!(net.get_edge(&[1, 2, 3]).is_none());
    /// assert!(net.get_edge(&[1, 2]).is_some());
    /// ```
    ///
    pub fn remove_edge(&self, nodes: &[O]) -> Option<HEdge<L>> {
        if nodes.is_empty() {
            return None;
        }
        let nodes2id = self.nodes2id.borrow();

        let mut ids = Vec::with_capacity(nodes.len());
        for n in nodes {
            match nodes2id.get(n) {
                Some(val) => ids.push(*val),
                None => return None,
            }
        }

        let mut removed: Option<HEdge<L>> = None;
        let mut all_edges = self.edges.borrow_mut();
        let to_remove: HEdge<L> = HEdge::new(ids.as_slice(), EdgeOptions::Simple);

        for id in ids {
            let edges = all_edges
                .get_mut(&id)
                .expect("error on get_mut of all_edges");

            let index = match edges.iter().position(|entry| *entry == to_remove) {
                Some(i) => i as i32,
                None => -1,
            };

            if index != -1 {
                removed = Some(edges.remove(index as usize))
            }
        }

        removed
    }

    /// Remove an edge passing an `HEdge` object
    ///
    /// # Arguments
    /// * `to_remove` - `HEdge` you want to remove
    ///
    /// # Returns
    /// * `Option<HEdge<L>>` - `Some(HEdge<L>)` if the hyper-edge is found and removed, `None` otherwise
    ///
    /// # Example
    /// ```
    /// let mut net = HNetwork::<u32, String>::new();
    /// net.add_node(1);
    /// net.add_node(2);
    /// net.add_node(3);
    ///
    /// net.add_edge(&[1, 2, 3], EdgeOptions::Simple);
    /// net.add_edge(&[1, 2], EdgeOptions::Simple);
    ///
    /// net.update();
    ///
    /// let edge = net.get_edge(&[1, 2, 3]).unwrap();
    /// net.remove_edge_obj(&edge);
    ///
    /// net.update();
    /// assert!(net.get_edge(&[1, 2, 3]).is_none());
    ///
    /// ```
    fn remove_edge_with_hedge(&self, to_remove: &HEdge<L>) -> Option<HEdge<L>> {
        let mut removed: Option<HEdge<L>> = None;
        let mut all_edges = self.edges.borrow_mut();

        for id in to_remove.nodes.iter() {
            let edges = all_edges
                .get_mut(id)
                .expect("error on get_mut of all_edges");

            let index = match edges.iter().position(|entry| *entry == *to_remove) {
                Some(i) => i as i32,
                None => -1,
            };

            if index != -1 {
                removed = Some(edges.remove(index as usize))
            }
        }

        removed
    }

    /// Remove a specific node and all the edges that involve it
    ///
    /// # Arguments
    /// * `u` - node you want to remove
    ///
    /// # Returns
    /// * `bool` - `true` if the node is found and removed, `false` otherwise
    ///
    /// # Example
    /// ```
    /// let mut net = HNetwork::<u32, String>::new();
    /// net.add_node(1);
    /// net.add_node(2);
    ///
    /// net.add_edge(&[1, 2], EdgeOptions::Simple);
    ///
    /// net.update();
    ///
    /// assert!(net.get_edge(&[1, 2]).is_some());
    ///
    /// net.remove_node(1);
    /// net.update();
    ///
    /// assert!(net.get_edge(&[1, 2]).is_none());
    /// assert!(net.get_node(1).is_none());
    ///
    /// ```
    ///  
    pub fn remove_object(&self, u: O) -> bool {
        let uid: u32;
        {
            let nodes2id = self.nodes2id.borrow_mut();
            uid = match nodes2id.get(&u) {
                Some(u) => *u,
                None => return false,
            };
        }

        if let Some(to_remove) = self.get_edges(u.clone()) {
            for hedge in to_remove {
                self.remove_edge_with_hedge(&hedge);
            }
        }

        let mut id2nodes = self.id2nodes.borrow_mut();
        let mut nodes2id = self.nodes2id.borrow_mut();

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
    /// let mut net = HNetwork::<Node, String>::new(true);
    /// let n = Node { id: 0, flag: false };
    ///
    /// net.add_node(n.clone());
    /// net.update_node(Node { id: 0, flag: true });
    ///
    /// net.update();
    ///
    /// assert!(net.get_node(0).unwrap().flag);
    /// ```
    pub fn update_node(&self, u: O) {
        let nodes2id = self.nodes2id.borrow_mut();
        let mut id2nodes = self.id2nodes.borrow_mut();
        let uid = match nodes2id.get(&u) {
            Some(u) => u,
            None => return,
        };
        if let Some(value) = id2nodes.get_mut(uid) {
            *value = u
        }
    }
}

impl<O: Hash + Eq + Clone + Display, L: Clone + Hash + Display> Default for HNetwork<O, L> {
    fn default() -> Self {
        Self::new()
    }
}

impl<O: Hash + Eq + Clone + Display, L: Clone + Hash + Display> Field for HNetwork<O, L> {
    fn update(&mut self) {
        let edges = self.edges.borrow();
        let mut redges = self.redges.borrow_mut();
        *redges = edges.clone();

        let id2nodes = self.id2nodes.borrow();
        let mut rid2nodes = self.rid2nodes.borrow_mut();

        *rid2nodes = id2nodes.clone();
    }

    fn lazy_update(&mut self) {
        let edges = self.edges.borrow();
        let mut redges = self.redges.borrow_mut();
        *redges = edges.clone();

        let id2nodes = self.id2nodes.borrow_mut();
        let mut rid2nodes = self.rid2nodes.borrow_mut();

        *rid2nodes = id2nodes.clone();
    }
}
