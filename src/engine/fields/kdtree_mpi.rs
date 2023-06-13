use crate::engine::fields::field::Field;
use crate::engine::location::{Int2D, Real2D};
use crate::lazy_static;
use crate::mpi::point_to_point::Destination;
use crate::mpi::request::WaitGuard;
use crate::mpi::topology::Communicator;
use crate::mpi::Rank;
use crate::universe;
use core::mem::size_of;
use hashbrown::HashMap;
use hashbrown::HashSet;
use mpi::datatype::UserDatatype;
use mpi::ffi::MPI_Finalize;
use mpi::point_to_point::Source;
use mpi::topology::SystemCommunicator;
use mpi::traits::*;
use mpi::Address;
use mpi::Threading;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::cmp;
use std::sync::Arc;
use std::sync::Mutex;

pub trait Location2D<Real2D> {
    fn get_location(self) -> Real2D;
    fn set_location(&mut self, loc: Real2D);
}

#[derive(Clone, Equivalence)]
pub struct Block {
    id: u32,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[derive(Debug, PartialEq)]
enum Axis {
    Vertical,
    Horizontal,
}

#[derive(Clone)]
pub struct Kdtree<O: Location2D<Real2D> + Clone + Copy + PartialEq + std::fmt::Display + 'static> {
    pub id: u32,
    pub pos_x: f32,
    pub pos_y: f32,
    width: f32,
    height: f32,
    original_width: f32,
    original_height: f32,
    pub locs: Vec<RefCell<Vec<Vec<O>>>>,
    pub nagents: RefCell<usize>,
    read: usize,
    write: usize,
    pub dh: i32,
    pub dw: i32,
    discretization: f32,
    subtrees: Vec<Block>,
    pub neighbor_trees: Vec<i32>,
    pub prec_neighbors: Vec<Vec<O>>,
    neighbors: Vec<Vec<O>>,
    pub received_neighbors: Vec<O>,
    halo_regions: Vec<Block>,
    neighbors_halo_regions: Vec<Vec<(i32, i32)>>,
    pub agents_to_send: Vec<Vec<O>>,
    pub scheduled_agent: HashMap<u32, u32>,
    distance: f32,
    processors: u32,
    pub density_estimation: usize,
    pub density_estimation_check: bool,
}

impl<
        O: Location2D<Real2D>
            + Clone
            + Copy
            + PartialEq
            + std::fmt::Display
            + mpi::datatype::Equivalence,
    > Kdtree<O>
{
    pub fn new(
        id: u32,
        pos_x: f32,
        pos_y: f32,
        width: f32,
        height: f32,
        discretization: f32,
        distance: f32,
    ) -> Self {
        Kdtree {
            id,
            pos_x,
            pos_y,
            locs: vec![
                RefCell::new(
                    std::iter::repeat_with(Vec::new)
                        .take(
                            (((width / discretization).ceil() + 1.0)
                                * ((height / discretization).ceil() + 1.0))
                                as usize,
                        )
                        .collect(),
                ),
                RefCell::new(
                    std::iter::repeat_with(Vec::new)
                        .take(
                            (((width / discretization).ceil() + 1.0)
                                * ((height / discretization).ceil() + 1.0))
                                as usize,
                        )
                        .collect(),
                ),
            ],
            subtrees: Vec::new(),
            neighbor_trees: Vec::new(),
            nagents: RefCell::new(0),
            original_width: 0.,
            original_height: 0.,
            read: 0,
            write: 1,
            dh: ((height / discretization).ceil() as i32 + 1),
            dw: ((width / discretization).ceil() as i32 + 1),
            discretization,
            width,
            height,
            processors: 0,
            prec_neighbors: Vec::new(),
            neighbors: vec![vec![]; 4],
            received_neighbors: Vec::new(),
            halo_regions: Vec::new(),
            neighbors_halo_regions: Vec::new(),
            agents_to_send: Vec::new(),
            scheduled_agent: HashMap::new(),
            distance,
            density_estimation: 0,
            density_estimation_check: false,
        }
    }

    pub fn create_tree(
        id: u32,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        discretization: f32,
        distance: f32,
    ) -> Self {
        let world = universe.world();
        let mut tree = Kdtree::new(id, x, y, width, height, discretization, distance);
        tree.original_height = height;
        tree.original_width = width;
        tree.first_subdivision();
        tree
    }

    pub fn first_subdivision(&mut self) {
        let world = universe.world();
        let mut count = 0;
        let mut temp_subtrees: Vec<Kdtree<O>> = vec![];

        if self.processors == 0 {
            self.processors = world.size() as u32;
        }

        if world.size() == 1 {
            println!("Running distributed (MPI) Kdtree with a single processor");
            println!(
                "Generated id{} for {};{} w:{} h:{}",
                self.id, self.pos_x, self.pos_y, self.width, self.height
            );
        }

        if world.size() != 1 {
            if world.rank() == 0 {
                println!(
                    "Running distributed (MPI) Kdtree with {} processors",
                    self.processors
                );

                if (self.processors != 1)
                //Root subdivision
                {
                    //nodes in subtrees
                    let nodes = self.split(&Axis::Vertical);
                    temp_subtrees.push(nodes.0);
                    temp_subtrees.push(nodes.1);

                    let mut count = 2;
                    let mut axis = Axis::Horizontal;

                    while count < self.processors {
                        for n in 0..temp_subtrees.len() {
                            if count >= self.processors {
                                break;
                            }
                            let nodes = temp_subtrees[n * 2].split(&axis);
                            temp_subtrees[n * 2] = nodes.0;
                            temp_subtrees.insert((n * 2) + 1, nodes.1);
                            count += 1;
                        }
                        if axis == Axis::Vertical {
                            axis = Axis::Horizontal;
                        } else {
                            axis = Axis::Vertical;
                        }
                    }
                }

                let mut subtree_id = self.id;
                for subtree in temp_subtrees.iter_mut() {
                    let mut block = Block {
                        id: subtree_id,
                        x: subtree.pos_x,
                        y: subtree.pos_y,
                        width: subtree.width,
                        height: subtree.height,
                    };
                    self.subtrees.push(block);
                    println!(
                        "Generated id {} for {};{} w:{} h:{}",
                        subtree_id, subtree.pos_x, subtree.pos_y, subtree.width, subtree.height
                    );
                    subtree_id += 1;
                }

                for sub in self.subtrees.iter() {
                    if sub.id as i32 == world.rank() {
                        self.width = sub.width;
                        self.height = sub.height;
                    }
                }

                //calculates neighbors id
                let self_points = self.get_boundary_points(true);
                self.agents_to_send = vec![vec![]; universe.world().size() as usize];
                for sub in self.subtrees.iter() {
                    if sub.id as i32 != world.rank() {
                        let other_points = self.get_block_boundary_points(sub, true);
                        if self.get_boundary_points(true).iter().any(|&self_point| {
                            other_points
                                .iter()
                                .any(|&other_point| self_point == other_point)
                        }) {
                            self.neighbor_trees.push(sub.id as i32);
                        }
                    }
                }

                for i in 1..world.size() {
                    world.process_at_rank(i).send(&self.subtrees);
                }
            } else {
                let (subtrees, _) = world.process_at_rank(0).receive_vec::<Block>();
                self.agents_to_send = vec![vec![]; universe.world().size() as usize];
                self.subtrees = subtrees;
                for subtree in self.subtrees.iter() {
                    if subtree.id as i32 == world.rank() {
                        self.width = subtree.width;
                        self.height = subtree.height;
                        self.pos_x = subtree.x;
                        self.pos_y = subtree.y;
                        for sub in self.subtrees.iter() {
                            if sub.id as i32 != world.rank() {
                                let self_points = self.get_boundary_points(true);
                                let other_points = self.get_block_boundary_points(sub, true);
                                if self.get_boundary_points(true).iter().any(|&self_point| {
                                    other_points
                                        .iter()
                                        .any(|&other_point| self_point == other_point)
                                }) {
                                    self.neighbor_trees.push(sub.id as i32);
                                }
                            }
                        }
                    }
                }
            }
        }

        self.calculate_regions();

        self.calculate_neighbor_regions();
    }

    fn get_boundary_points(&self, toroidal: bool) -> Vec<(f32, f32)> {
        let (x, y) = (self.pos_x, self.pos_y);
        let (width, height) = (self.width, self.height);
        let o_w = self.original_width;
        let o_h = self.original_height;

        let mut points = vec![
            (x, y),
            (x + width, y),
            (x, y + height),
            (x + width, y + height),
        ];
        if toroidal {
            let toroidal_points = vec![
                (x % o_w, y % o_h),
                ((x + width) % o_w, y % o_h),
                (x % o_w, (y + height) % o_h),
                ((x + width) % o_w, (y + height) % o_h),
            ];

            points.extend(toroidal_points.iter().cloned());
        }

        points
    }

    fn get_block_boundary_points(&self, block: &Block, toroidal: bool) -> Vec<(f32, f32)> {
        let (x, y) = (block.x, block.y);
        let (width, height) = (block.width, block.height);
        let o_w = self.original_width;
        let o_h = self.original_height;

        let mut points = vec![
            (x, y),
            (x + width, y),
            (x, y + height),
            (x + width, y + height),
        ];
        if toroidal {
            let toroidal_points = vec![
                (x % o_w, y % o_h),
                ((x + width) % o_w, y % o_h),
                (x % o_w, (y + height) % o_h),
                ((x + width) % o_w, (y + height) % o_h),
            ];
            points.extend(toroidal_points.iter().cloned());
        }
        points
    }

    fn calculate_regions(&mut self) {
        let h = self.distance;
        let w = self.distance;

        let north_y = self.pos_y + self.height - self.distance;
        let east_x = self.pos_x + self.width - self.distance;
        let south_y = self.pos_y;
        let west_x = self.pos_x;

        let north = Block {
            id: 0,
            x: self.pos_x,
            y: north_y,
            width: self.width,
            height: h,
        };
        let east = Block {
            id: 1,
            x: east_x,
            y: self.pos_y,
            width: w,
            height: self.height,
        };
        let south = Block {
            id: 2,
            x: self.pos_x,
            y: south_y,
            width: self.width,
            height: h,
        };
        let west = Block {
            id: 3,
            x: west_x,
            y: self.pos_y,
            width: w,
            height: self.height,
        };

        self.halo_regions.push(north);
        self.halo_regions.push(east);
        self.halo_regions.push(south);
        self.halo_regions.push(west);
    }

    fn calculate_neighbor_regions(&mut self) {
        let world = universe.world();

        for region in self.halo_regions.iter() {
            let mut region_points = self.get_block_boundary_points(region, true);
            let mut neighbor_blocks_region = Vec::new();
            for block in self.subtrees.iter() {
                if block.id as i32 != world.rank() {
                    let block_points = self.get_block_boundary_points(block, true);
                    if region_points.iter().any(|&region_points| {
                        block_points
                            .iter()
                            .any(|&block_points| region_points == block_points)
                    }) {
                        neighbor_blocks_region.push((region.id as i32, block.id as i32))
                    }
                }
            }
            if neighbor_blocks_region.len() > 0 {
                self.neighbors_halo_regions.push(neighbor_blocks_region);
            }
        }
    }

    fn split(&mut self, direction: &Axis) -> (Kdtree<O>, Kdtree<O>) {
        let mut id = self.id.clone();
        let mut node_x = self.pos_x;
        let mut node_y = self.pos_y;
        let mut node_w = self.width;
        let mut node_h = self.height;

        let mut n1 = self.clone();
        n1.locs.clear();

        match direction {
            Axis::Vertical => {
                n1.width = n1.width / 2.0;
                node_x = self.pos_x + self.width / 2.0;
                node_w = self.width / 2.0;
            }
            Axis::Horizontal => {
                self.height = self.height / 2.0;
                n1.height = self.height;
                node_y = self.pos_y + self.height;
                node_h = self.height;
            }
        }
        let agents: Vec<(O, f32, f32)> = Vec::new();
        let p = self.processors;
        let mut n2 = Kdtree::new(
            id,
            node_x,
            node_y,
            node_w,
            node_h,
            self.discretization,
            self.distance,
        );

        return (n1, n2);
    }

    pub fn get_block_by_location(&self, x: f32, y: f32) -> i32 {
        let world = universe.world();
        if world.size() == 1 {
            return 0;
        }
        for block in self.subtrees.iter() {
            if block.x <= x
                && x <= block.x + block.width
                && block.y <= y
                && y <= block.y + block.height
            {
                return block.id as i32;
            }
        }
        panic!("Block for location {};{} not found! This should not happen!", x, y);
        -1
    }

    pub fn insert(&mut self, agent: O, loc: Real2D) {
        let world = universe.world();
        let bag = self.discretize(&loc);
        let index = ((bag.x * self.dh) + bag.y) as usize;
        let mut bags = self.locs[self.write].borrow_mut();
        bags[index].push(agent);

        for region in &self.halo_regions {
            if (region.x <= loc.x
                && loc.x <= region.x + region.width
                && region.y <= loc.y
                && loc.y <= region.y + region.height)
            {
                self.neighbors[region.id as usize].push(agent);
                break;
            }
        }
        if !self.density_estimation_check {
            *self.nagents.borrow_mut() += 1;
        }
        drop(bags);
    }

    pub fn insert_read(&mut self, agent: O, loc: Real2D) {
        let world = universe.world();
        let bag = self.discretize(&loc);
        let index = ((bag.x * self.dh) + bag.y) as usize;
        let mut bags = self.locs[self.read].borrow_mut();
        bags[index].push(agent);

        if !self.density_estimation_check {
            *self.nagents.borrow_mut() += 1;
        }
    }

    pub fn remove_object_location(&self, object: O, loc: Real2D) {
        let bag = self.discretize(&loc);
        let index = ((bag.x * self.dh) + bag.y) as usize;
        let mut bags = self.locs[self.write].borrow_mut();
        if !bags[index].is_empty() {
            let before = bags[index].len();
            bags[index].retain(|&x| x != object);
            let after = bags[index].len();

            if !self.density_estimation_check {
                *self.nagents.borrow_mut() -= before - after;
            }
        }
    }

    fn contains(&self, x: f32, y: f32) -> bool {
        self.pos_x <= x
            && x < self.pos_x + self.width
            && self.pos_y <= y
            && y < self.pos_y + self.height
    }

    fn calculate_median(&self, agents: &Vec<(O, i32, i32)>) -> i32 {
        let len = agents.len();

        if len >= 1 && len % 2 == 1 {
            return agents[len / 2].1;
        } else if len >= 1 {
            return (agents[len / 2 - 1].1 + agents[(len / 2)].1) / 2;
        } else {
            return 0;
        }
    }

    fn calculate_median_on_y(&self, agents: &Vec<(O, i32, i32)>) -> i32 {
        let len = agents.len();

        if len > 1 && len % 2 == 1 {
            return agents[len / 2].2;
        } else if len > 1 {
            return (agents[len / 2 - 1].2 + agents[(len / 2)].2) / 2;
        } else {
            return 0;
        }
    }

    fn discretize(&self, loc: &Real2D) -> Int2D {
        let x_floor = (loc.x / self.discretization).floor();
        let x_floor = x_floor as i32;

        let y_floor = (loc.y / self.discretization).floor();
        let y_floor = y_floor as i32;

        Int2D {
            x: x_floor,
            y: y_floor,
        }
    }

    pub fn get_neighbors_within_distance(&self, loc: Real2D, dist: f32) -> Vec<O> {
        let mut neighbors: Vec<O>;

        neighbors = Vec::new();

        if dist <= 0.0 {
            return neighbors;
        }

        let disc_dist = (dist / self.discretization).floor() as i32;
        let disc_loc = self.discretize(&loc);
        let max_x = (self.original_width / self.discretization).ceil() as i32;
        let max_y = (self.original_height / self.discretization).ceil() as i32;

        let mut min_i = disc_loc.x - disc_dist;
        let mut max_i = disc_loc.x + disc_dist;
        let mut min_j = disc_loc.y - disc_dist;
        let mut max_j = disc_loc.y + disc_dist;

        min_i = cmp::max(0, min_i);
        max_i = cmp::min(max_i, max_x - 1);
        min_j = cmp::max(0, min_j);
        max_j = cmp::min(max_j, max_y - 1);

        for i in min_i..max_i + 1 {
            for j in min_j..max_j + 1 {
                let bag_id = Int2D {
                    x: t_transform(i, max_x),
                    y: t_transform(j, max_y),
                };

                let check = check_circle(
                    &bag_id,
                    self.discretization,
                    self.original_width,
                    self.original_height,
                    &loc,
                    dist,
                    true,
                );

                let index = ((bag_id.x * self.dh) + bag_id.y) as usize;
                // let bags = self.rbags.borrow();
                let bags = self.locs[self.read].borrow();

                for elem in &bags[index] {
                    if ((check == 0
                        && distance(
                            &loc,
                            &(elem.get_location()),
                            self.original_width,
                            self.original_height,
                            true,
                        ) <= dist)
                        || check == 1)
                        && elem.get_location() != loc
                    {
                        neighbors.push(*elem);
                    }
                }
            }
        }
        neighbors
    }

    pub fn message_exchange(
        &self,
        agents_to_send: &Vec<Vec<O>>,
        dummy: O,
        with_regions: bool,
    ) -> Vec<Vec<O>> {
        let world = universe.world();
        let mut received_messages = vec![0; world.size() as usize];
        let mut send_vec = vec![0; world.size() as usize];
        let mut send_agent_vec: Vec<Vec<O>> = vec![vec![]; world.size() as usize];

        if with_regions {
            for region in self.neighbors_halo_regions.iter() {
                for neighbor in region.iter() {
                    send_vec[neighbor.1 as usize] += agents_to_send[neighbor.0 as usize].len();
                    send_agent_vec[neighbor.1 as usize]
                        .extend(agents_to_send[neighbor.0 as usize].iter())
                }
            }
        } else {
            for neighbor in &self.neighbor_trees {
                send_vec[*neighbor as usize] += self.agents_to_send[*neighbor as usize].len();
                send_agent_vec[*neighbor as usize]
                    .extend(self.agents_to_send[*neighbor as usize].iter())
            }
        }

        //I make a receive of messages from all my neighbors and send to all my neighbors. A message contains the number of agents i will receive.
        for neighbor in &self.neighbor_trees {
            mpi::request::scope(|scope| {
                let ln = &send_vec[*neighbor as usize];
                let rreq = WaitGuard::from(
                    world
                        .process_at_rank(*neighbor)
                        .immediate_receive_into_with_tag(
                            scope,
                            &mut received_messages[*neighbor as usize],
                            *neighbor,
                        ),
                );
                let sreq = WaitGuard::from(
                    world
                        .process_at_rank(*neighbor)
                        .immediate_ready_send_with_tag(scope, ln, world.rank()),
                );
            });
        }

        let mut vec: Vec<Vec<O>> = vec![vec![]; world.size() as usize];
        if received_messages.len() > 0 {
            for i in &self.neighbor_trees {
                if received_messages[*i as usize] != 0 {
                    vec[*i as usize].append(&mut vec![dummy; received_messages[*i as usize]]);
                } else {
                    vec[*i as usize].append((&mut vec![]));
                }
            }
        }

        // I receive the agents from my neighbors and send my agents to them.
        mpi::request::multiple_scope(world.size() as usize, |scope, coll| {
            for (id, buffer) in vec.iter_mut().enumerate() {
                if received_messages[id as usize] != 0 {
                    let rreq = world
                        .process_at_rank(id as i32)
                        .immediate_receive_into_with_tag(scope, &mut buffer[..], world.rank() + 50);
                    coll.add(rreq);
                }
            }

            for id in self.neighbor_trees.iter() {
                if send_agent_vec[*id as usize].len() != 0 {
                    let mut sreq = world.process_at_rank(*id).immediate_send_with_tag(
                        scope,
                        &send_agent_vec[*id as usize][..],
                        *id + 50,
                    );
                    coll.add(sreq);
                }
            }
            let mut out = vec![];
            coll.wait_all(&mut out);
        });
        return vec;
    }

    pub fn get_distributed_neighbors_within_relax_distance(
        &mut self,
        loc: Real2D,
        dist: f32,
        agent: O,
    ) -> Vec<O> {
        let world = universe.world();

        let mut neighbors: Vec<O>;

        neighbors = Vec::new();

        if dist <= 0.0 {
            return neighbors;
        }

        let disc_dist = (dist / self.discretization).floor() as i32;
        let disc_loc = self.discretize(&loc);
        let max_x = (self.original_width / self.discretization).ceil() as i32;
        let max_y = (self.original_height / self.discretization).ceil() as i32;

        let mut min_i = disc_loc.x - disc_dist;
        let mut max_i = disc_loc.x + disc_dist;
        let mut min_j = disc_loc.y - disc_dist;
        let mut max_j = disc_loc.y + disc_dist;

        min_i = cmp::max(0, min_i);
        max_i = cmp::min(max_i, max_x - 1);
        min_j = cmp::max(0, min_j);
        max_j = cmp::min(max_j, max_y - 1);

        for i in min_i..max_i + 1 {
            for j in min_j..max_j + 1 {
                let bag_id = Int2D {
                    x: t_transform(i, max_x),
                    y: t_transform(j, max_y),
                };

                let index = ((bag_id.x * self.dh) + bag_id.y) as usize;
                let bags = self.locs[self.read].borrow();

                for elem in &bags[index] {
                    if (elem.get_location() != loc) {
                        neighbors.push(*elem);
                    }
                }
            }
        }
        neighbors
    }
}

impl<O: Location2D<Real2D> + Clone + Copy + PartialEq + std::fmt::Display> Drop for Kdtree<O> {
    fn drop(&mut self) {}
}

impl<O: Location2D<Real2D> + Eq + Clone + Copy + std::fmt::Display> Field for Kdtree<O> {
    fn lazy_update(&mut self) {
        self.prec_neighbors = Vec::new();
        self.prec_neighbors.append(&mut self.neighbors);
        self.neighbors = vec![vec![]; 4];
        self.agents_to_send = vec![vec![]; universe.world().size() as usize];
        self.received_neighbors.clear();
        std::mem::swap(&mut self.read, &mut self.write);

        if !self.density_estimation_check {
            self.density_estimation = (*self.nagents.borrow_mut()) / ((self.dw * self.dh) as usize);
            self.density_estimation_check = true;
            self.locs[self.write] = RefCell::new(
                std::iter::repeat_with(|| Vec::with_capacity(self.density_estimation))
                    .take((self.dw * self.dh) as usize)
                    .collect(),
            );
        } else {
            let mut bags = self.locs[self.write].borrow_mut();
            for b in 0..bags.len() {
                bags[b].clear();
            }
        }
    }

    fn update(&mut self) {}
}

fn t_transform(n: i32, size: i32) -> i32 {
    if n >= 0 {
        n % size
    } else {
        (n % size) + size
    }
}

fn check_circle(
    bag: &Int2D,
    discretization: f32,
    width: f32,
    height: f32,
    loc: &Real2D,
    dis: f32,
    tor: bool,
) -> i8 {
    let nw = Real2D {
        x: (bag.x as f32) * discretization,
        y: (bag.y as f32) * discretization,
    };
    let ne = Real2D {
        x: nw.x,
        y: (nw.y + discretization).min(height),
    };
    let sw = Real2D {
        x: (nw.x + discretization).min(width),
        y: nw.y,
    };
    let se = Real2D { x: sw.x, y: ne.y };

    if distance(&nw, loc, width, height, tor) <= dis
        && distance(&ne, loc, width, height, tor) <= dis
        && distance(&sw, loc, width, height, tor) <= dis
        && distance(&se, loc, width, height, tor) <= dis
    {
        1
    } else if distance(&nw, loc, width, height, tor) > dis
        && distance(&ne, loc, width, height, tor) > dis
        && distance(&sw, loc, width, height, tor) > dis
        && distance(&se, loc, width, height, tor) > dis
    {
        -1
    } else {
        0
    }
}

fn distance(loc1: &Real2D, loc2: &Real2D, dim1: f32, dim2: f32, tor: bool) -> f32 {
    let dx;
    let dy;

    if tor {
        dx = toroidal_distance(loc1.x, loc2.x, dim1);
        dy = toroidal_distance(loc1.y, loc2.y, dim2);
    } else {
        dx = loc1.x - loc2.x;
        dy = loc1.y - loc2.y;
    }
    (dx * dx + dy * dy).sqrt()
}

pub fn toroidal_distance(val1: f32, val2: f32, dim: f32) -> f32 {
    if (val1 - val2).abs() <= dim / 2.0 {
        return val1 - val2;
    }

    let d = toroidal_transform(val1, dim) - toroidal_transform(val2, dim);

    if d * 2.0 > dim {
        d - dim
    } else if d * 2.0 < -dim {
        d + dim
    } else {
        d
    }
}

pub fn toroidal_transform(val: f32, dim: f32) -> f32 {
    if val >= 0.0 && val < dim {
        val
    } else {
        let mut val = val % dim;
        if val < 0.0 {
            val += dim;
        }
        val
    }
}
