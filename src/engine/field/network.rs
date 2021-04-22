use crate::engine::field::field::Field;
use crate::utils::dbdashmap::DBDashMap;
use std::fmt::Display;
use std::hash::Hash;

pub enum EdgeOptions<L: Clone + Hash + Display> {
    Simple,
    Labeled(L),
    Weighted(f64),
    WeightedLabeled(L, f64),
}
//use EdgeOptions::{Simple, Labeled, Weighted, WeightedLabeled};

#[derive(Clone)]
pub struct Edge<O: Hash + Eq + Clone + Display, L: Clone + Hash + Display> {
    pub u: O,
    pub v: O,
    pub label: Option<L>,
    pub weight: Option<f64>,
}


impl<O: Hash + Eq + Clone + Display, L: Clone + Hash + Display> Edge<O, L> {
    pub fn new(u_node: O, v_node: O, edgeOptions: EdgeOptions<L>) -> Edge<O, L> {
        match edgeOptions {
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

pub struct Network<O: Hash + Eq + Clone + Display, L: Clone + Hash + Display> {
    pub edges: DBDashMap<O, Vec<Edge<O, L>>>,
    pub direct: bool,
}

impl<O: Hash + Eq + Clone + Display, L: Clone + Hash + Display> Network<O, L> {
    pub fn new(d: bool) -> Network<O, L> {
        Network {
            edges: DBDashMap::new(),
            direct: d,
        }
    }

    pub fn addNode(&self, u: &O) {
        let mut vec: Vec<Edge<O, L>> = Vec::new();
        self.edges.insert(u.clone(), vec);
    }

    pub fn addEdge(&self, u: &O, v: &O, edgeOptions: EdgeOptions<L>) -> Option<Edge<O, L>> {
        //println!("addEdge");
        let e = Edge::new(u.clone(), v.clone(), edgeOptions);
        match self.edges.get_mut(u) {
            Some(mut uedges) => {
                uedges.push(e.clone());
            }
            None => {
                let mut vec = Vec::new();
                vec.push(e.clone());
                self.edges.insert(u.clone(), vec);
            }
        }
        if !self.direct {
            match self.edges.get_mut(v) {
                Some(mut vedges) => {
                    vedges.push(e.clone());
                }
                None => {
                    let mut vec = Vec::new();
                    vec.push(e.clone());
                    self.edges.insert(v.clone(), vec);
                }
            }
        }
        Some(e)
    }

    pub fn updateEdge(&self, u: &O, v: &O, edgeOptions: EdgeOptions<L>) -> Option<Edge<O, L>> {
        let e = Edge::new(u.clone(), v.clone(), edgeOptions);
        let ris = match self.edges.get_mut(u) {
            Some(mut uedges) => {
                //TODO search the edge and change it
                uedges.retain( |entry| !((entry.u == e.u && entry.v == e.v) || (entry.v == e.u && entry.u == e.v)) );
                uedges.push(e.clone());
                Some(e.clone())
            }
            None => None,
        };

        if !self.direct{
            match self.edges.get_mut(v) {
                Some(mut uedges) => {
                    //TODO search the edge and change it
                    uedges.retain( |entry| !((entry.u == e.u && entry.v == e.v) || (entry.v == e.u && entry.u == e.v)) );
                    uedges.push(e.clone());
                    //TODO
                }
                None => panic!("Error! undirected edge not found"),
            }
        }
        ris
    }

    pub fn getNodes(&self) -> Vec<&O> {
        self.edges.keys()
    }

    pub fn getEdges(&self, u: &O) -> Option<&Vec<Edge<O, L>>> {
        self.edges.get(&u)
    }

    pub fn getEdge(&self, u: &O, v: &O) -> Option<Edge<O, L>> {
        match self.edges.get(u) {
            Some(uedges) => {
                for e in uedges {
                    if (self.direct && e.u == *u && e.v == *v) {
                        return Some(e.clone());
                    } else if (!self.direct
                        && ((e.u == *u && e.v == *v) || (e.v == *u && e.u == *v)))
                    {
                        return Some(e.clone());
                    }
                }
                None
            }
            None => None,
        }
    }

 

    pub fn removeEdge(&self, u: &O, v: &O) -> Option<Edge<O, L>> {
        //TODO
        let mut u_edges = self.edges.get_mut(u).unwrap();
        let index =  match u_edges.iter().position( |entry|  ((entry.u == *u && entry.v == *v) || (entry.u == *v && entry.v == *u)) ){
            Some(i) => i,
            None => return None,
        };

        let u_edge = u_edges.remove(index);
        std::mem::drop(u_edges);

        if self.direct {
            return  Some(u_edge.clone());
        }else{
            let mut v_edges = self.edges.get_mut(v).unwrap();
            v_edges.retain( |entry|  !((entry.u == *u && entry.v == *v) || (entry.u == *v && entry.v == *u)));
            return  Some(u_edge.clone());
        }
    }

    pub fn removeEdges(&self, u: &O) -> Option<Vec<Edge<O, L>>> {
        //TODO remove vector for u and all edges for v nodes
        let nodes = self.edges.w_keys();
        let mut ris = vec![];

        for v in nodes.iter(){
            if v != u{
                match self.removeEdge(v,u){
                    Some(e) => ris.push(e),
                    None => (),
                }
            }
        }

        Some(ris)
    }

    pub fn removeAllEdges(&self){
        self.edges.clear();
    }
    pub fn removeNode(&self, u: &O) -> bool {
        match self.removeEdges(u){
            Some(_) => {self.edges.remove(u); true},
            None => false,
        }
        
    }
}

impl<O: Hash + Eq + Clone + Display, L: Clone + Hash + Display > Field for Network<O,L>{
    fn lazy_update(&self){
        self.edges.update();
    }
    fn update(&self){
        self.edges.update();
    }
}
