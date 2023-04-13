
#![allow(warnings)]
use crate::engine::location::Real2D;
use crate::engine::fields::field::Field;
use crate::mpi::topology::Communicator;
use crate::mpi::point_to_point::Destination;
use crate::lazy_static;
use crate::universe;
use mpi::datatype::UserDatatype;
use mpi::traits::*;
use mpi::Address;
use mpi::topology::SystemCommunicator;
use mpi::point_to_point::Source;
use mpi::Threading;
use mpi::ffi::MPI_Finalize;
use core::mem::size_of;


#[derive(Clone, Equivalence)]
pub struct Block {
    id: u32,
    x: f32,
    y: f32,
    width: f32,
    height: f32
}

/* unsafe impl Equivalence for Block {
    type Out = UserDatatype;
    fn equivalent_datatype() -> Self::Out {
        UserDatatype::structured(
            &[1, 1, 1, 1, 1],
            &[
                (size_of::<f32>() * 8) as mpi::Address,
                (size_of::<f32>() * 4) as mpi::Address,
                (size_of::<f32>() * 2) as mpi::Address,
                size_of::<f32>() as mpi::Address,
                size_of::<u32>() as mpi::Address,
            ],
            &[u32::equivalent_datatype()],
            &[f32::equivalent_datatype(); 4],
            
        )
    }
} */

#[derive(Debug, PartialEq)]
enum Axis {
    Vertical,
    Horizontal
}

#[derive(Clone)]
pub struct Kdtree<O: Clone + Copy + PartialEq + std::fmt::Display> {
    pub id: u32,
    pub pos_x: f32,
    pub pos_y: f32,
    width: f32,
    height: f32,
    pub locs: Vec<(O, f32, f32)>,
    pub rlocs: Vec<(O, f32, f32)>,
    subtrees: Vec<Block>,
    processors: u32,
    is_leaf: bool,
}

/* lazy_static!{
    pub static ref universe:Universe = mpi::initialize().expect("Error initialing mpi environment");
    static ref root_rank:u32 = 0;
}  */

/* static universe:Universe = mpi::initialize().expect("Error initialing mpi environment");
static world:SystemCommunicator = universe.world();
static root_rank:u32 = 0; */


impl<O: Clone + Copy + PartialEq + std::fmt::Display> Kdtree<O> {
    pub fn new(
        id: u32,
        pos_x: f32,
        pos_y: f32,
        width: f32,
        height: f32,
    ) -> Self {
       Kdtree {
            id,
            pos_x,
            pos_y,
            locs: Vec::new(),
            rlocs: Vec::new(),
            subtrees: Vec::new(),
            width,
            height,
            processors: 0,
            is_leaf: true,
        }
    }

    pub fn create_tree(id:u32, x:f32, y:f32, width: f32, height:f32) -> Self{
        let mut tree = Kdtree::new(id, x, y, width, height);
        //let (_universe, threading) = mpi::initialize_with_threading(mpi::Threading::Multiple).unwrap();
        tree.first_subdivision();
        tree
    }


    
    pub fn first_subdivision(&mut self){
        let world = universe.world();
        
        const FIRST_SUB_DIMENSION:usize=4;
        let mut count = 0;
        let mut temp_subtrees: Vec<Kdtree<O>> = vec![];
        
        
        if self.processors == 0{
            self.processors = world.size() as u32;
        }
        
        if world.rank() == 0 {
            println!("Running distributed (MPI) Kdtree with {} processors", self.processors);
        

        //Root subdivision
        let nodes = self.split(&Axis::Vertical);
        self.is_leaf=false;
        temp_subtrees.push(nodes.0);
        temp_subtrees.push(nodes.1);

        for i in 0..FIRST_SUB_DIMENSION/2{
            let mut id = self.id.clone();
            let x = temp_subtrees[i].split(&Axis::Horizontal);
            temp_subtrees[i]=x.0;
            temp_subtrees.push(x.1);

        }
     
        count+=FIRST_SUB_DIMENSION as u32;
        let mut axis = Axis::Vertical;

        //Progressive subdivision
        while count<self.processors{
            for n in 0..temp_subtrees.len(){
                if count >= temp_subtrees[n*2].processors{break;}
                let nodes=temp_subtrees[n*2].split(&axis);
                temp_subtrees[n*2] = nodes.0;
                temp_subtrees.insert((n*2)+1, nodes.1);
                count+=1;
            }
            if axis == Axis::Vertical {axis=Axis::Horizontal;}
            else {axis=Axis::Vertical;}
        }
        let mut subtree_id = self.id;
        for subtree in temp_subtrees.iter_mut(){
            let mut block = Block{id: subtree_id, x: subtree.pos_x, y: subtree.pos_y, width: subtree.width, height: subtree.height};
            self.subtrees.push(block);
            println!("generato id {} per {};{} w:{} h:{}", subtree_id, subtree.pos_x, subtree.pos_y, subtree.width, subtree.height);
            

            subtree_id+=1;
            //world.process_at_rank(subtree_id as i32).send(&mut block);

            /* world.process_at_rank(0 as i32).broadcast_into(&mut subtree_id);
            println!("Process {} received value {}",world.rank(), &subtree_id);
            world.process_at_rank(0 as i32).broadcast_into(&mut block);
            println!("Process {} received value {};{}",world.rank(), &block.x, &block.y); */
            
        }

        for i in 1.. world.size(){
            world.process_at_rank(i).send(&self.subtrees);
        }
        
    }
    else {
        let (subtrees, _) = world.process_at_rank(0).receive_vec::<Block>();
        self.subtrees=subtrees;
        /* println!("{} Ricevo elemento...", world.rank());
        let id = world.any_process().receive::<u32>();
        println!("Process {} received value {}", world.rank(), id.0);
        self.id = id.0; */
    }
    } 

    
    fn split(&mut self, direction:&Axis) -> (Kdtree<O>, Kdtree<O>){

        let mut id = self.id.clone();
        let mut node_x = self.pos_x;
        let mut node_y = self.pos_y;
        let mut node_w = self.width;
        let mut node_h = self.height;


        let mut n1 = self.clone();
        n1.locs.clear();

        match direction {
            Axis::Vertical => {
                n1.width=n1.width/2.0;
                node_x = self.pos_x + self.width/2.0;
                node_w = self.width/2.0;
            },
            Axis::Horizontal => {
                self.height=self.height/2.0;
                n1.height=self.height;
                node_y = self.pos_y + self.height;
                node_h = self.height;
            },
        }
        let agents: Vec<(O,f32,f32)>=Vec::new();
        let p = self.processors;
        let mut n2 = Kdtree::new(id, node_x, node_y, node_w, node_h);
        self.is_leaf=false;

        return (n1,n2);
    }

    fn split_on_median(&mut self, median:f32, direction:bool, id_node:&str) -> (Kdtree<O>, Kdtree<O>){

        let mut id = self.id.clone();
        let mut node_x = self.pos_x;
        let mut node_y = self.pos_y;
        let mut node_w = self.width;
        let mut node_h = self.height;
        let mut dir = Axis::Vertical;

        if direction{ dir = Axis::Horizontal}
        else {dir = Axis::Vertical}


        //println!("Axis è {:?}",dir);
        let mut n1 = self.clone();
        self.is_leaf=false;
        n1.locs.clear();

        match dir{
            Axis::Vertical => {
                n1.width=median-self.pos_x;
                node_x = median;
                println!("self.pos_x {}, median vale {}, self.width {}, n1.width {}",self.pos_x,  median, self.width,n1.width);
                node_w=self.width-n1.width;
            },

            Axis::Horizontal => {
                n1.height=median-self.pos_y;
                node_y = median;
                println!("self.pos_y {}, median vale {}, self.height {}, n1.height {}",self.pos_y,  median, self.height,n1.height);
                node_h = self.height-n1.height;
            },
        }
        let agents: Vec<(O,f32,f32)>=Vec::new();
        let p = self.processors;
        let mut n2 = Kdtree::new(id, node_x, node_y, node_w, node_h);

        println!("La divisione ha generato l'albero {} in {};{} con w: {} e h:{} e is_leaf: {}", n1.id, n1.pos_x,n1.pos_y,n1.width,n1.height, n1.is_leaf);
        println!("La divisione ha generato l'albero {} in {};{} con w: {} e h:{} e is_leaf: {}", n2.id, n2.pos_x,n2.pos_y,n2.width,n2.height, n2.is_leaf);
        return (n1,n2);
    }



    pub fn get_block_by_location (&self, x: f32, y: f32) -> u32{
        for block in self.subtrees.iter(){
            if block.x <= x
            && x < block.x + block.width
            && block.y <= y
            && y < block.y + block.height {
                return block.id;
            }
        }
        //println!("Restituisco 0");
        0
    }

    pub fn ismaster(&self)->bool{
        let world = universe.world();
        if world.rank() == 0{
            return true;
        }
        false
    }

    pub fn insert(&mut self, agent: O, x: f32, y: f32) {
        /* let mut block_id:u32;
        let world = universe.world();

        block_id = self.get_block_by_location(x, y);

        if world.rank() == block_id as i32{
            //println!("Inserisco agente {};{} nel Vec del processo {} con id blocco {} ", x, y, world.rank(), block_id);
            self.locs.push((agent,x,y));
        } */
        //let world = universe.world();

        //println!("Inserisco agente {};{} nel Vec del processo {} con id {} ", x, y, world.rank(), self.id);
        self.locs.push((agent,x,y));
        
    } 

    fn contains(&self, x: f32, y: f32) -> bool {
        self.pos_x <= x
            && x < self.pos_x + self.width
            && self.pos_y <= y
            && y < self.pos_y + self.height
    }
    

    fn calculate_median(&self, agents: &Vec<(O,i32,i32)>) -> i32{
        let len = agents.len();
        
        if len >=1 && len%2==1{
            return agents[len/2].1;
        }

        else if len >=1{
            return (agents[len/2-1].1 + agents[(len/2)].1) / 2
        }
        else{
            return 0;
        }
    }

    fn calculate_median_on_y(&self, agents: &Vec<(O,i32,i32)>) -> i32{
        let len = agents.len();
        
        if len >1 && len%2==1{
            return agents[len/2].2;
        }

        else if len>1{
            return (agents[len/2-1].2 + agents[(len/2)].2) / 2
        }
        else{
            return 0;
        }
    }

    pub fn query_by_location(&mut self, x: f32, y:f32) -> Option<O>{
        let mut option:Option<O> = None;
        let world = universe.world();

        let mut block_id = self.get_block_by_location(x, y);
        world.process_at_rank(0 as i32).broadcast_into(&mut block_id);
        if world.rank() == block_id as i32 {
            for r in self.rlocs.iter(){
                if r.1==x && r.2==y{
                    option=Some(r.0);
                    return option;
                }
            }
        }

        option      
    }
    
    pub fn get_neighbors_within_distance(&self, loc:Real2D, distance:f32) -> Vec<&(O,f32,f32)>{
        let mut neighbors:Vec<&(O, f32, f32)> = Vec::new(); 
        //println!("rlocs: {}", self.rlocs.len());
        /* let world = universe.world();

        let mut block_id = self.get_block_by_location(loc.x, loc.y);
        //world.process_at_rank(0 as i32).broadcast_into(&mut block_id);
        if world.rank() == block_id as i32 {
            for r in self.rlocs.iter(){
                if f32::abs(r.1 - loc.x)<distance && f32::abs(r.2 - loc.y)<distance{
                    neighbors.push(r.clone());
                }
            }
        } */

        for r in self.rlocs.iter(){
            if f32::abs(r.1 - loc.x)<distance && f32::abs(r.2 - loc.y)<distance{
                neighbors.push(r);
            }
        }

        neighbors 

        /* let mut neighbors: Vec<O>;

                neighbors = Vec::new();

                if distance <= 0.0 {
                    return neighbors;
                }

                if distance <= 0.0 {
                    return neighbors;
                }

                let disc_dist = (distance/self.discretization).floor() as i32;
                let disc_loc = self.discretize(&loc);
                let max_x = (self.width/self.discretization).ceil() as i32;
                let max_y =  (self.height/self.discretization).ceil() as i32;

                let mut min_i = disc_loc.x - disc_dist;
                let mut max_i = disc_loc.x + disc_dist;
                let mut min_j = disc_loc.y - disc_dist;
                let mut max_j = disc_loc.y + disc_dist;

                if self.toroidal {
                    min_i = cmp::max(0, min_i);
                    max_i = cmp::min(max_i, max_x-1);
                    min_j = cmp::max(0, min_j);
                    max_j = cmp::min(max_j, max_y-1);
                }

                for i in min_i..max_i+1 {
                    for j in min_j..max_j+1 {
                        let bag_id = Int2D {
                            x: t_transform(i, max_x),
                            y: t_transform(j, max_y),
                        };

                        let check = check_circle(&bag_id, self.discretization, self.width, self.height, &loc, dist, self.toroidal);

                        let index = ((bag_id.x * self.dh) + bag_id.y) as usize;
                        // let bags = self.rbags.borrow();
                        let bags = self.bags[self.read].borrow();

                        for elem in &bags[index]{
                            if (check == 0 && distance(&loc, &(elem.get_location()), self.width, self.height, self.toroidal) <= dist) || check == 1 {
                                neighbors.push(*elem);
                            }
                        }

                    }
                }
                neighbors */
    }

    /* pub fn get_all_agents(&mut self) -> Vec<(O, f32,f32)>{
        let mut all_agents: Vec<(O, f32, f32)> = Vec::new();
        let world = universe.world();

        if world.rank() == 0 {
            world.process_at_rank(0).gather_into_root(&self.rlocs, &mut all_agents[..]);
            println!("Root gathered sequence: {:?}.", all_agents);
        } else {
            world.process_at_rank(0).gather_into(&self.rlocs);
        }

        world.barrier(); //?
    
        all_agents
    } */

    

    fn balance(&self, agents: &mut Vec<(O,i32,i32)>, direction:bool) -> (Vec<Vec<(O,i32,i32)>>, Vec<i32>){
        let len = agents.len();
        let mut medians: Vec<i32> = Vec::new();
        /*for i in 0..len{
            println!("Mi preparo a bilanciare {};{}", agents[i].1, agents[i].2)
        }*/
        let mut vec_right = agents.split_off(len/2);
        let mut median_right=0;
        let mut median_left=0;

        if direction{
            vec_right.sort_by(|a,b| a.1.cmp(&b.1));
            agents.sort_by(|a,b| a.1.cmp(&b.1));
            median_right = self.calculate_median(&vec_right);
            median_left = self.calculate_median(agents);
            println!("mediana x sinistra {} ",median_left);
            println!("mediana x destra {} ",median_right);
        }
        else{
            vec_right.sort_by(|a,b| a.2.cmp(&b.2));
            agents.sort_by(|a,b| a.2.cmp(&b.2));
            median_right = self.calculate_median_on_y(&vec_right);
            median_left = self.calculate_median_on_y(agents);
            println!("mediana y sinistra {} ",median_left);
            println!("mediana y destra {} ",median_right);
        }
        medians.push(median_left);
        medians.push(median_right);
        let mut vec: Vec<Vec<(O,i32,i32)>> = Vec::new();
        vec.push(agents.to_vec());
        vec.push(vec_right);
        return (vec,medians);
    }

}

impl<O: Clone + Copy + PartialEq + std::fmt::Display> Drop for Kdtree<O>{
    fn drop(&mut self) {
    }
}

impl<O: Eq + Clone + Copy + std::fmt::Display> Field for Kdtree<O> {
    fn lazy_update(&mut self){
        unsafe {
            std::ptr::swap(
                &mut self.locs,
                &mut self.rlocs,
            )
        }
        self.locs.clear();
    }

    fn update(&mut self){
        let mut rlocs_clone=self.rlocs.clone();
        self.locs.append(&mut rlocs_clone);
        
    }
}