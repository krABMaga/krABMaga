# Rust-AB: An Agent-Based Simulation engine in Rust

[![Build Status](https://travis-ci.org/spagnuolocarmine/abm.svg?branch=master)](https://travis-ci.org/spagnuolocarmine/abm)

**Rust-AB** is a discrete events simulation engine for developing ABM simulation that is written in a novel programming language named Rust. Rust-AB is designed to be a _ready-to-use_ tool for the ABM community and for this reason the architectural concepts of the well-adopted MASON library were re-engineered to exploit the Rust peculiarities.


## Boids Simulation Example

The Boids model by C. Raynolds, 1986, is a steering behavior ABM for autonomous agents, which simulates the flocking behavior of birds. The agent behavior is derived by a linear combination of three independent rules: _Separation_: steer in order to avoid crowding local flockmates; _Alignment_: steer towards the average heading of local flockmates; _Cohesion_: steer to move towards the average position (center of mass) of local flockmates.

### Agent definition

A Rust-AB agent is a struct contains all the local agent data. For our exampel, we have to define a new struct named _Bird_ that emulate the concept of a bird in a flock. The struct definition, in Rust-AB, must implements the trait  _Agent_ and the traits _Eq_ and _Hash_. According to the model specification, each agent in each simulation time has to compute three steering rules according to its neighboring agents. For this reason, it will be placed in a Rust-AB _Field2D_, a bi-dimensional environment. Consequentially, the agent definition must implements the trait _Location2D_, and also the traits _Clone_ and _Copy_, instead of developing they can be automatically computed using the Rust macro ```#derive[(\_)]```.

The steering behavior model can be developed by storing the position of the agent in the previous time and in current time, the agent position can be modeled using a Rust-AB struct named _Real2D_. Furthermore, an unique identify is stored in the agent in order to easily develop the trait _Hash_. 

```rust
#[derive(Clone, Copy)]
pub struct Bird{
    pub id: u128,
    pub pos: Real2D,
    pub last_d: Real2D,
}
```

The agent logic is placed in the _step_ function, however, in order to develop more robust code, we designed agent logic using three sub-functions defined in the agent implementation. Listing _code2_ shows the agent implementation code. Lines 1-8 defines the object _Bird_, by providing the object constructor, and three functions: avoidance, cohesion, and consistency, corresponding to the steering model rules. Each function takes as input parameter the reference to a vector of agents (the agent neighborhood) and returns a new Real2D, which is the force computed according to the neighbors. Moreover, lines 9-12 shows the code for implementing the trait _Location2D_ trait, which enables to place the agent in the _Field2D_ environment. Lines 13-20 shows the code for implementing the Rust traits _Hash_ and _Eq_, notice that in order to develop the Rust _Eq_ trait, it is needed to develop also the trait _PartialEq_, which is developed by exploiting the unique agent identifier.

Finally, the agent _step_ function is defined. Lines 21-39 shows the code of the agent logic, that enables to simulate the steering behavior of the model. The agent computes the neighboring agents (line 23) and using the sub-functions compute its new position. The computed position is used to update the status of the environment (line 37). Notice that in order to access to the simulation state, are used a particular Rust mechanism.

```rust
impl Bird {
    pub fn new(id: u128, pos: Real2D, last_d: Real2D) -> Self {
        Bird {id, pos, last_d}
    }
    pub fn avoidance (self, vec: &Vec<Bird>) -> Real2D {..}
    pub fn cohesion (self, vec: &Vec<Bird>) -> Real2D {..}
    pub fn consistency (self, vec: &Vec<Bird>) -> Real2D {..}
 }
impl Location2D for Bird {
    fn get_location(self) -> Real2D { self.pos }
    fn set_location(&mut self, loc: Real2D) { self.pos = loc; }
}
impl Hash for Bird {
    fn hash<H>(&self, state: &mut H) where H: Hasher,
    { state.write_u128(self.id); state.finish();}
}
impl Eq for Bird {}
impl PartialEq for Bird {
    fn eq(&self, other: &Bird) -> bool {self.id == other.id}
}
impl Agent for Bird {
    fn step(&mut self) {
        let vec = GLOBAL_STATE.lock().unwrap().field1.get_neighbors_within_distance(self.pos,10.0);
        let avoid = self.avoidance(&vec);
        let cohe  = self.cohesion(&vec);
        let rand  = self.randomness();
        let cons  = self.consistency(&vec);
        let mom   = self.last_d;
        let mut dx = COHESION*cohe.x + AVOIDANCE*avoid.x + CONSISTENCY*cons.x + RANDOMNESS*rand.x + MOMENTUM*mom.x;
        let mut dy = COHESION*cohe.y + AVOIDANCE*avoid.y + CONSISTENCY*cons.y + RANDOMNESS*rand.y + MOMENTUM*mom.y;
        let dis = (dx*dx + dy*dy).sqrt();
        if dis > 0.0 { dx = dx/dis*JUMP; dy = dy/dis*JUMP;}
        let _lastd = Real2D {x: dx, y:dy};
        let loc_x = toroidal_transform(self.pos.x + dx, WIDTH);
        let loc_y = toroidal_transform(self.pos.y + dy, HEIGHT);
        self.pos = Real2D{x: loc_x, y: loc_y};
        GLOBAL_STATE.lock().unwrap().field1.set_object_location(*self, Real2D{x: loc_x, y: loc_y});
    }
}
```

### Model definition



## License

The MIT License

Copyright (c) ISISLab, Università degli Studi di Salerno 2019.

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
