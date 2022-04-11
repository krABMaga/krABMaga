use crate::engine::fields::field::Field;
use hashbrown::HashMap;
use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::Display;
use std::hash::Hash;

#[derive(Clone)]
pub enum EdgeOptions<L: Clone + Hash + Display> {
    Simple,
    Labeled(L),
    Weighted(f32),
    WeightedLabeled(L, f32),
}

#[derive(Clone, Debug)]
pub struct HEdge<L: Clone + Hash + Display> {
    pub nodes: HashSet<u32>,
    pub label: Option<L>,
    pub weight: Option<f32>,
}

impl<L: Clone + Hash + Display> HEdge<L> {
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
    fn eq(&self, other: &Self) -> bool {
        self.nodes == other.nodes
    }
}

impl<L: Clone + Hash + Display> Eq for HEdge<L> {}

pub struct HNetwork<O: Hash + Eq + Clone + Display, L: Clone + Hash + Display> {
    pub edges: RefCell<HashMap<u32, Vec<HEdge<L>>>>,
    pub redges: RefCell<HashMap<u32, Vec<HEdge<L>>>>,
    pub nodes2id: RefCell<HashMap<O, u32>>,
    pub id2nodes: RefCell<HashMap<u32, O>>,
    pub rid2nodes: RefCell<HashMap<u32, O>>,
}

impl<O: Hash + Eq + Clone + Display, L: Clone + Hash + Display> HNetwork<O, L> {
    pub fn new() -> HNetwork<O, L> {
        HNetwork {
            edges: RefCell::new(HashMap::new()),
            redges: RefCell::new(HashMap::new()),
            nodes2id: RefCell::new(HashMap::new()),
            id2nodes: RefCell::new(HashMap::new()),
            rid2nodes: RefCell::new(HashMap::new()),
        }
    }

    // fn default() -> Self {
    //     Self::new()
    // }

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

    pub fn get_edges(&self, u: O) -> Option<Vec<HEdge<L>>> {
        let nodes2id = self.nodes2id.borrow();
        let uid = match nodes2id.get(&u) {
            Some(u) => u,
            None => return None,
        };
        let edges = self.redges.borrow();
        match edges.get(uid) {
            Some(es) => Some((*(es.clone())).to_vec()),
            None => None,
        }
    }

    pub fn get_object(&self, uid: u32) -> Option<O> {
        match self.rid2nodes.borrow_mut().get(&uid) {
            Some(u) => Some(u.clone()),
            None => None,
        }
    }

    pub fn remove_all_edges(&self) {
        let mut edges = self.edges.borrow_mut();
        edges.clear();
    }

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
            let edges = all_edges.get_mut(&id).unwrap();

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

    fn remove_edge_with_hedge(&self, to_remove: &HEdge<L>) -> Option<HEdge<L>> {
        let mut removed: Option<HEdge<L>> = None;
        let mut all_edges = self.edges.borrow_mut();

        for id in to_remove.nodes.iter() {
            let edges = all_edges.get_mut(id).unwrap();

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
