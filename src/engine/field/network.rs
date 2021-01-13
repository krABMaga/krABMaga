use std::fmt::Display;
use std::hash::Hash;
use crate::engine::field::field::Field;
use crate::utils::dbdashmap::DBDashMap;

pub enum EdgeOptions<L: Clone + Hash> {
    Simple,
    Labeled(L),
    Weighted(f64),
    WeightedLabeled(L,f64)
}
use EdgeOptions::{Simple, Labeled, Weighted, WeightedLabeled};

#[derive(Clone)]
pub struct Edge<O: Hash + Eq + Clone , L: Clone + Hash >{
    pub u: O,
    pub v: O,
    pub label: Option<L>,
    pub weight: Option<f64>
}

impl<O: Hash + Eq + Clone, L: Clone + Hash> Edge<O,L>  {
    pub fn new(u_node: O, v_node:O, edgeOptions: EdgeOptions<L>) -> Edge<O,L> {
        match edgeOptions {
            EdgeOptions::Simple =>  Edge{u: u_node, v: v_node, label: None, weight: None},
            EdgeOptions::Labeled(l) => Edge{u: u_node, v: v_node, label: Some(l), weight: None},
            EdgeOptions::Weighted(w) => Edge{u: u_node, v: v_node, label: None, weight: Some(w)},
            EdgeOptions::WeightedLabeled(l, w) => Edge{u: u_node, v: v_node, label: Some(l), weight: Some(w)},
        }
    }
}

pub struct Network<O: Hash + Eq + Clone, L: Clone + Hash > {
    pub edges: DBDashMap<O, Vec<Edge<O,L>>>,
    pub direct: bool,
}

impl<O: Hash + Eq + Clone, L: Clone + Hash > Network<O,L>  {
    pub fn new(d: bool) -> Network<O,L> {
        Network {
            edges: DBDashMap::new(),
            direct: d
        }
    }
    
    pub fn addEdge(&self,  u: O,  v: O, edgeOptions:EdgeOptions<L>) -> Option<Edge<O,L>>{
        let e = Edge::new(u.clone(), v.clone(), edgeOptions);
        match self.edges.get(&u){
            Some(uedges) => { 
             
                let mut vec = uedges.to_vec();
                vec.push(e.clone());
                self.edges.insert(u, vec);
                Some(e)
            }
            None => None
        }
    }
    
    pub fn getNodes(&self) ->  Option<Vec<&O>>{
        None
    }

    pub fn getEdges(&self, u: O) ->  Option<&Vec<Edge<O,L>>>{
        None
    }
    
    pub fn getEdge(&self, u: O, v: O) ->  Option<&Edge<O,L>>{
        None
    }

    pub fn removeEdge(&self,  u: O,  v: O) -> Option<Edge<O,L>>{
        None
    }

    pub fn removeEdges(&self, u: O) ->  Option<Vec<Edge<O,L>>>{
        None
    }

    pub fn removeAllEdges(&self, u: O) ->  bool{
        false
    }
}