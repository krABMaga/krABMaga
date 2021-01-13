use std::fmt::Display;
use std::hash::Hash;
use crate::engine::field::field::Field;
use crate::utils::dbdashmap::DBDashMap;

pub struct Edge<O: Hash + Eq , L: Clone + Hash >{
    pub u: O,
    pub v: O,
    pub label: L,
    pub weight: f64
}
pub struct Network<O: Hash + Eq , L: Clone + Hash > {
    pub edges: DBDashMap<O, Vec<Edge<O,L>>>,
    pub direct: bool,
}

impl<O: Hash + Eq , L: Clone + Hash > Network<O,L>  {
    pub fn new(d: bool) -> Network<O,L> {
        Network {
            edges: DBDashMap::new(),
            direct: d
        }
    }
    
    pub fn addEdge(&self,  u: O,  v: O, l: L, w: f64) -> Option<&Edge<O,L>>{
        None
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