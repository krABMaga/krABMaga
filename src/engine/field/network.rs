use std::fmt::Display;
use std::hash::Hash;
use crate::engine::field::field::Field;
use crate::utils::dbdashmap::DBDashMap;

pub enum EdgeOptions<L: Clone + Hash + Display> {
    Simple,
    Labeled(L),
    Weighted(f64),
    WeightedLabeled(L,f64)
}
//use EdgeOptions::{Simple, Labeled, Weighted, WeightedLabeled};

#[derive(Clone)]
pub struct Edge<O: Hash + Eq + Clone + Display, L: Clone + Hash + Display>{
    pub u: O,
    pub v: O,
    pub label: Option<L>,
    pub weight: Option<f64>
}

impl<O: Hash + Eq + Clone + Display, L: Clone + Hash + Display> Edge<O,L>  {
    pub fn new(u_node: O, v_node:O, edgeOptions: EdgeOptions<L>) -> Edge<O,L> {
        match edgeOptions {
            EdgeOptions::Simple =>  Edge{u: u_node, v: v_node, label: None, weight: None},
            EdgeOptions::Labeled(l) => Edge{u: u_node, v: v_node, label: Some(l), weight: None},
            EdgeOptions::Weighted(w) => Edge{u: u_node, v: v_node, label: None, weight: Some(w)},
            EdgeOptions::WeightedLabeled(l, w) => Edge{u: u_node, v: v_node, label: Some(l), weight: Some(w)},
        }
    }
}

pub struct Network<O: Hash + Eq + Clone + Display, L: Clone + Hash + Display > {
    pub edges: DBDashMap<O, Vec<Edge<O,L>>>,
    pub direct: bool,
}

impl<O: Hash + Eq + Clone + Display, L: Clone + Hash + Display > Network<O,L>  {
    pub fn new(d: bool) -> Network<O,L> {
        Network {
            edges: DBDashMap::new(),
            direct: d
        }
    }

    pub fn addNode(&self,  u: &O){
        let mut vec: Vec<Edge<O,L>> = Vec::new();
        self.edges.insert(u.clone(), vec);
    }

  
    pub fn addEdge(&self,  u: &O,  v: &O, edgeOptions:EdgeOptions<L>) -> Option<Edge<O,L>>{
        //println!("addEdge");
        let e = Edge::new(u.clone(), v.clone(), edgeOptions);
        match self.edges.get_mut(u){
            Some(mut uedges) => { 
                uedges.push(e.clone());
                if self.direct {
                    Some(e)
                }else{
                    match self.edges.get_mut(v){
                        Some(mut vedges) => { 
                            vedges.push(e.clone());
                            Some(e)
                        }
                        None => {
                            let mut vec = Vec::new();
                            vec.push(e.clone());
                            self.edges.insert(v.clone(), vec);
                            Some(e)
                        }
                    }
                }
            }
            None => {
                let mut vec = Vec::new();
                vec.push(e.clone());
                self.edges.insert(u.clone(), vec);
                if self.direct {
                    Some(e)
                }else{
                    match self.edges.get_mut(v){
                        Some(mut vedges) => { 
                            vedges.push(e.clone());
                            Some(e)
                        }
                        None => {
                            let mut vec = Vec::new();
                            vec.push(e.clone());
                            self.edges.insert(v.clone(), vec);
                            Some(e)
                        }
                    }
                }
            }
        }
    }
    
    pub fn updateEdge(&self,  u: &O,  v: &O, edgeOptions:EdgeOptions<L>) -> Option<Edge<O,L>>{
        let e = Edge::new(u.clone(), v.clone(), edgeOptions);
        match self.edges.get_mut(u){
            Some(uedges) => { 
                //TODO search the edge and change it
                let mut vec = uedges.to_vec();
                vec.push(e.clone());
                self.edges.insert(u.clone(), vec);
                Some(e)
                //TODO
            }
            None => None
        }
    }
    
    pub fn getNodes(&self) ->  Vec<&O>{
        self.edges.keys()
    }

    pub fn getEdges(&self, u: &O) ->  Option<&Vec<Edge<O,L>>>{
        self.edges.get(&u)
    }
    
    pub fn getEdge(&self, u: &O, v: &O) ->  Option<Edge<O,L>>{
        match self.edges.get(u){
            Some(uedges) => { 
                for e in uedges {
                    if (self.direct && e.u == *u && e.v == *v){
                        return Some(e.clone())
                    }else if (!self.direct && ( (e.u == *u && e.v == *v) || (e.v == *u && e.u == *v) ) ){
                        return Some(e.clone())
                    } 
                }
                None
            }
            None => None
        }
    }

    pub fn removeEdge(&self,  u: &O,  v: &O) -> Option<Edge<O,L>>{
        //TODO
       /*  let u_edge = self.getEdge(u.clone(),v.clone()).unwrap();
        let u_edges = self.getEdges(u.clone()).unwrap();
        let index = u_edges.iter().position(|x| (x.u == u_edge.u && x.v == u_edge.v)).unwrap();
        let mut vec_u = u_edges.to_vec();
        vec_u.remove(index);
        self.edges.insert(u, vec_u);

        if self.direct {
            return  Some(u_edge.clone());
        }else{
            
            let v_edges = self.getEdges(v.clone()).unwrap();
            let index = v_edges.iter().position(|x| (x.u == u_edge.u && x.v == u_edge.v)).unwrap();
            let mut vec_v = v_edges.to_vec();
            vec_v.remove(index);
            self.edges.insert(v, vec_v);
            return  Some(u_edge.clone());
        } */
        None
    }

    pub fn removeEdges(&self, u: &O) ->  Option<Vec<Edge<O,L>>>{
        //TODO remove vector for u and all edges for v nodes
        None
    }

    pub fn removeAllEdges(&self, u: &O) ->  bool{
        false
    }
}

impl<O: Hash + Eq + Clone + Display, L: Clone + Hash + Display > Field for Network<O,L>{
    fn update(&mut self){
        self.edges.update();
    }
}