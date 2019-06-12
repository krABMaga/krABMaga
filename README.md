# Rust-AB: An Agent-Based Simulation engine in Rust

[![Build Status](https://travis-ci.org/spagnuolocarmine/abm.svg?branch=master)](https://travis-ci.org/spagnuolocarmine/abm)

Rust-AN is a discrete events simulation engine for developing ABM simulation that is written in a novel programming language named Rust. Rust-AB is designed to be a _ready-to-use_ tool for the ABM community and for this reason the architectural concepts of the well-adopted MASON library were re-engineered.


## Examples: Boids Simulation

The Boids model by C. Raynolds, 1986, is a steering behavior ABM for autonomous agents, which simulates the flocking behavior of birds. The agent behavior is derived by a linear combination of three independent rules: _Separation_: steer in order to avoid crowding local flockmates; _Alignment_: steer towards the average heading of local flockmates; _Cohesion_: steer to move towards the average position (center of mass) of local flockmates.

### Agent definition

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
