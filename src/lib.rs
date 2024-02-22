#![doc(
    html_logo_url = "https://raw.githubusercontent.com/krABMaga/krABMaga.github.io/main/static/images/krabmaga_docs.png"
)]

//!
//![krABMaga](https://github.com/krABMaga/krABMaga) is a discrete events simulation engine for developing ABM simulation
//!written in the [Rust language](https://www.rust-lang.org/).
//!
//![krABMaga](https://github.com/krABMaga/krABMaga) is designed to be a ready-to-use tool for the ABM community and for this
//!reason the architectural concepts of the well-adopted [MASON library](https://cs.gmu.edu/~eclab/projects/mason/) were
//!re-engineered to exploit the Rust peculiarities and programming model, in particular by keeping the visualization and the
//!simulation subsystems fully separated.
//!
//! *Developed by [IsisLab](https://www.isislab.it)*
//!  <img alt="ISISLab Logo" src="https://raw.githubusercontent.com/krABMaga/krABMaga.github.io/main/static/images/isislab.png">
//!
//!---
//!
//!## Table of contents
//!<!-- no toc -->
//!- [Table of contents](#table-of-contents)
//!- [Dependencies](#dependencies)
//!- [How to run your first example simulation](#how-to-run-your-first-example-simulation)
//!- [How to write your first model](#how-to-write-your-first-model)
//!- [Available features](#available-features)
//!- [Macros for playing with Simulation Terminal](#macros-for-playing-with-simulation-terminal)
//!- [How to contribute](#how-to-contribute)
//!- [Architecture](#architecture)
//!  - [Agents](#agents)
//!  - [Simulation state](#simulation-state)
//!  - [Schedule](#schedule)
//!  - [Data structures](#data-structures)
//!
//!---
//!
//!# Dependencies
//!
//!The visualization framework requires certain dependencies to run the simulation properly.
//!- 💻 Windows: [VS2019 build tools](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16)
//!- 🍎 MacOS: No dependencies needed.
//!- 🐧 Linux: A few dependencies are needed. Check [here](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md) for a list based on your distribution.
//!---
//!# How to run your first example simulation
//!First of all, install latest version of [Rust](https://www.rust-lang.org/tools/install). Follow steps to setup Rust toolchain (*cargo*, *rustc* and *rustup*).
//!
//!Now, you can download/clone all available krABMaga examples from our github repository called [examples](https://github.com/krABMaga/examples).
//!
//!To run a simulation, go to root directory of a model, for example `/path/to/examples/flockers`. With command `ls`, you should be able to see a typical krABMaga simulation struct:
//!- `src`: main folder with code. It contains `main.rs` file and two directories for model and visualization components.
//!- `Cargo.toml`: Configuration file for Rust project, with dependencies and features.
//!- `assets`: an images folder. It contains all the icons that can be used for visualization.
//!- `Makefile.toml`: another configuration file, necessary to a correct execution of visualization.
//!
//!Inside the root directory of model that you choose, you can run a models with or without visualization.
//!
//!To simply run your simulation, with no visualization:
//!```sh
//!cargo run --release
//!```
//!Running in this way, you can see our `Simulation Terminal` (better known as `Simulation Monitor`)) based on [tui-rs](https://github.com/fdehau/tui-rs), a rust library that provides components to create terminal with an interface. As a modelist, you can use krABMaga macros to create several plots, print logs and add a model description (shown using a popup)
//!
//!
//!<style>
//!* {
//!  box-sizing: border-box;
//!}
//!.column {
//!  height: auto;
//!  min-height: 100%;
//!  /* width: 45.0%; */
//!  min-width: 200px;
//!  padding: 5px;
//!  display:inline-block;
//!  text-align: center;
//!  vertical-align:middle;
//!}
//!
//!  @media screen and (max-width: 400px) {
//!    .column{
//!        width: 45%;
//!    }
//!  }
//!
//!
//!/* Clearfix (clear floats) */
//!.row::after {  
//!  content: "";
//!  clear: both;
//!  display: table;
//!}
//!
//!.row{
//!    text-align: center;
//!
//!}
//!</style>
//!
//!
//!<div class="row">
//!  <div class="column" >
//!    <img style="margin-left: auto;" src="https://raw.githubusercontent.com/krABMaga/krABMaga.github.io/main/static/images/tui-wsg.gif"/>
//!  </div>
//!  <div class="column">
//!    <img style="margin-left: auto;" src="https://raw.githubusercontent.com/krABMaga/krABMaga.github.io/main/static/images/ant.gif"/>
//!  </div>
//!</div>
//!
//!
//!Based on [Bevy game engine](https://bevyengine.org/), it's possible to run simulation with visualization. It's also available a menu to start and stop simulations and a slider to set simulation speed.
//!To run a model with visualization enabled, you have to start the simulation with the command:
//!```sh
//!cargo run --release --features  visualization
//!
//!# Alternative command. Requires 'cargo make' installed
//!cargo make run --release
//!```
//!
//!In addition to the classical visualization, you can run your krABMaga simulation inside your browser using [*Web Assembly*](https://webassembly.org).
//!This is possible with the command:
//!```sh
//!# Requires 'cargo make' installed
//!cargo make serve --release
//!```
//!
//!
//!---
//!# How to write your first model
//!
//!If you don't start from our [Template](https://github.com/krABMaga/examples/tree/main/template), add this to your `Cargo.toml`:
//!```toml
//![dependencies]
//!krABMaga = 0.1.*
//!
//![features]
//!visualization = ["krABMaga/visualization"]
//!visualization_wasm = ["krABMaga/visualization_wasm"]
//!```
//!
//!We **strongly** recommend to use [Template](https://github.com/krABMaga/examples/tree/main/template) or any other example as base of a new project, especially if you want to provide any visualization.
//!
//!Each krABMaga model needs structs that implements our *Traits*, one for *State* and the other for *Agent*. In the *State* struct you have to put *Agent* field(s), because it represents the ecosystem of a simulation. More details for each krABMaga component are in the [Architecture](#architecture) section.
//!
//!The simplest part is `main.rs`, because is similar for each example.
//!You can define two *main* functions using **cfg** directive, that can remove code based on which features are (not) enabled.  
//!Without visualization, you have only to use *simulate!* to run simulation, passing a state, step number and how may time repeat your simulation.
//!With visualization, you have to set graphical settings (like dimension or background) and call *start* method.
//!```rust
//!// Main used when only the simulation should run, without any visualization.
//!#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
//!fn main() {
//!  let dim = (200., 200.);
//!  let state = Flocker::new(dim, num_agents);
//!  let step = 10;
//!  let reps = 1;
//!  let num_agents = 100;  
//!  let _ = simulate!(state, step, reps);
//!}
//!
//!// Main used when a visualization feature is applied.
//!#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
//!fn main() {
//!  let dim = (200., 200.);
//!  let num_agents = 100;
//!  let state = Flocker::new(dim, num_agents);
//!  Visualization::default()
//!      .with_window_dimensions(1000., 700.)
//!      .with_simulation_dimensions(dim.0 as f32, dim.1 as f32)
//!      .with_background_color(Color::rgb(0., 0., 0.))
//!      .with_name("Flockers")
//!      .start::<VisState, Flocker>(VisState, state);
//!}
//!
//!```
//!---
//!
//!# Available features
//!
//!<style>
//!  table{
//!    word-wrap: break-word;
//!    table-layout: auto;
//!    width: 100%;
//!    
//!  }
//!</style>
//!
//!This library offers some features to make your simulation more interesting and to avoid to install many dependencies that are not needed for basic simulation.
//!```sh
//!cargo run --release --features <name_feature>
//!```
//!
//!<div  style="overflow-x:auto;">
//!
//!| Compilation Feature  | Description |  Experimental | Release Candidate  | Stable  |
//!|:------:|:-------:|:---:|:---:|:---:|
//!| **No Features** | Possibility to run model using `Simulation Terminal` and setup model-exploration experiments (Parameter Sweeping, Genetic and Random) in sequential/parallel mode. It's enough to create your base simulations. |   |   | 🦀 |
//!| **visualization**  | Based on `Bevy engine`, it makes possible to visualize your model elements, to understand better the behavior of your simulation. |   | 🦀 |   |
//!| **visualization-wasm** | Based on `Web Assembly`, give you the possibility to execute your visualized simulation inside your own browser. |   | 🦀 |   |
//!| **distributed-mpi** | Enable distributed model exploration using MPI. At each iteration, the amount of configurations are balanced among your nodes.  |   |  🦀 |   |
//!| **bayesian**  | Use ML Rust libraries to use/create function to use `Bayesian Optimization`.|   | 🦀  |   |
//!| **parallel**  | Speed-up a single simulation by parallelizing agent scheduling during a step.| 🦀  |   |   |
//!
//!</div>
//!
//!---
//!# Macros for playing with Simulation Terminal
//!
//!`Simulation Terminal` is enabled by default using macro `simulate!`, so can be used passing a state, step number and how may time repeat your simulation..
//!That macro has a fourth optional parameter, a boolean. When `false` is passed, `Simulation Terminal` is disabled.
//!```rust
//!($s:expr, $step:expr, $reps:expr $(, $flag:expr)?) => {{
//!      // Macro code
//!}}
//!```
//!
//!You can create tabs and plot your data using two macro:
//!- `addplot!` let you create a new plot that will be displayed in its own tab.
//!```rust
//!addplot!(String::from("Chart Name"), String::from("xxxx"), String::from("yyyyy"));
//!```
//!- `plot!` to add a point to a plot. Points can be added during simulation execution, for example inside `after_step` method.
//!  You have to pass plot name, series name, x value and y value. Coordinate values need to be `f64`.
//!```rust
//!plot!(String::from("Chart name"), String::from("s1"), x, y);
//!```
//!
//!On Terminal home page there is also a *log section*, you can plot log messages when some event needs to be noticed.
//!You can navigate among all logs using ↑↓ arrows.
//!To add a log use the macro `log!`, passing a `LogType` (an enum) and the log message.
//!```rust
//! log!(LogType::Info, String::from("Log Message"));
//!```
//!
//!Are available four type of Logs:
//!```rust
//!pub enum LogType {
//!    Info,
//!    Warning,
//!    Error,
//!    Critical,
//!}
//!```
//!
//!---
//!# How to contribute
//!
//!If you want to test, add or change something inside krABMaga engine, you can clone [main repo](https://github.com/krABMaga/krABMaga) locally, and change dependency inside `Cargo.toml` of your examples:
//!
//!```toml
//![dependencies]
//!# krABMaga = { git="https://github.com/krABMaga/krABMaga.git" }
//!krABMaga = { path="path/to/krABMaga"}
//!```
//!
//!---
//!# Architecture
//!
//!## Agents
//!
//!The krABMaga framework defines a trait `Agent` that can be implemented on a struct to define `Agent` specific functionalities,
//!mainly the `step` method which specifies how the agent behaves for each simulation step, and the `get_id` method,
//!to uniquely identify an agent. There are also other methods, with default implementation, to improve agent control:
//!
//!- `is_stopped` notify the scheduler if a specific agent should be removed or not, based on some condition.
//!- `before_step` and `after_step` to implement some operations before/after a step.
//!
//!The krABMaga framework allow multi-agent implementations: you can define multiple 'Agent' that
//!implement the trait, and [Wolf, Sheep & Grass](https://krABMaga.github.io/wolfsheepgrass/) is the main example of this feature.
//!
//!---
//!## Simulation state
//!
//!The simulation state can be considered as the single source of truth of the simulation, where data resides and is updated.
//!Like `Agent`, krABMaga exposes a `State` trait to let the user mark a particular structure as a simulation state, along with
//!exposing an `update` method to define logic to execute once for each simulation step. The simulation state is the perfect
//!structure to put field definitions on (such as 2D continuous fields, grids and so on). An important effect of the state being
//!the single source of truth forces agents to update (and most importantly read) their own location by interacting with the
//!state, even though they can store their own location locally in the agent structure too. Although, to be sure one is interacting
//!with the latest computed data, it is considered a good practice to update both an agent own location field and its copy on the
//!state structure.
//!
//!---
//!## Schedule
//!
//!The simulation timeline is controlled by a Schedule structure that takes care of notifying all the scheduled agents, and the
//!simulation state that a step has been taken. For this reason, agents should be scheduled so that they can be notified when
//!a step has been taken.
//!The scheduler works as a priority queue, where the agents are sorted according to their scheduled time
//!and a priority value - an integer. The simulation time - a real value - starts from the scheduling time of the first agent.
//!The schedule structure exposed by the krABMaga framework provides two methods to do so:
//!- `schedule_once` to insert an agent in the schedule for a specific simulation step. The scheduling time and the
//!  priority are given as parameters. The priority is used to sort all agents within the same simulation time.
//!  
//!- `schedule_repeating` which acts like schedule once, with the difference that the agent will be scheduled for all
//!  subsequent simulation steps.
//!
//!The schedule provides the `step` method which allows executing one simulation step. In this way, the programmer can
//!easily design his/her simulation by looping for a certain number of step or for a given amount of CPU time.
//!
//!---
//!
//!## Data structures
//!
//!<!-- The krABMaga framework exposes a few data structures based on the `DBDashMap`, a customized version of the
//![Rust HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html) that implements a double
//!buffering technique to avoid indeterminism caused by the lack of knowledge of the agents' step execution order within a step.
//!The `DBDashMap` implements the interior mutability pattern, which allows the user to safely write in it without having an actual
//!mutable reference to the structure, because the reads are done on a different memory block than the writes. Only the `update`
//!method actually requires a mutable reference, to swap the read and the write buffers and commit the changes. -->
//!
//!The currently implemented structures are:
//!
//!- `Field2D`, a sparse matrix structure modelling agent interactions on a
//!  2D real space with coordinates represented by 2D f64 tuples (`Real2D`).
//!  
//!- `Grid2D`, a discrete field representing agents locations as 2D i64 tuples (`Int2D`). This structure keeps two copies of a DBDashMap in sync,
//!  one the inverse of the other, to allow constant time access both by key (agent) and by value (position). There are two kind of Grid based on density, `SparseGrid2D` and `DenseGrid2D`.
//!  
//!- `NumberGrid2D`, a simpler version of the `Grid2D` to use with simpler values. This is useful to represent simulation spaces
//!  covered by a simple entity that can be represented with a non-agent structure. This data structure can be used with any
//!  structure that can be cloned, most notably simple primitive values such as f64s. As the previous grid, there are two implementations: `SparseNumberGrid2D` and `DenseNumberGrid2D`.
//!  
//!- `Network` and `HNetwork` to connect any kind of nodes using `Edge`/`HEdge`. With `Network` you can define both directed and undirected graphs and connect a couple of nodes with an edge with label and/or weight. `HNetwork` is a generalization of a `Network` to represent hypergraph. In this case, `HEdge` is an `HashSet` of nodes.
//!  With this fields you can reproduce any kind of graph or network, such as for our example [`Virus on a Network`](/virusnetwork).
//!
//!---
//!
//!# Support conference paper
//!
//!If you find this code useful in your research, please consider citing:
//!
//!```bibtex
//!@ARTICLE{AntelmiASIASIM2019,
//!  author={Antelmi, A. and Cordasco, G. and D’Auria, M. and De Vinco, D. and Negro, A. and Spagnuolo, C.},
//!  title={On Evaluating Rust as a Programming Language for the Future of Massive Agent-Based Simulations},
//!  journal={Communications in Computer and Information Science},
//!  note={Conference of 19th Asia Simulation Conference, AsiaSim 2019 ; Conference Date: 30 October 2019 Through 1 November 2019;  Conference Code:233729},
//!  year={2019},
//!  volume={1094},
//!  pages={15-28},
//!  doi={10.1007/978-981-15-1078-6_2},
//!  issn={18650929},
//!  isbn={9789811510779},
//!}
//!
//!```
//!

/// Main module, with structs for Agents, Fields and Schedule
pub mod engine;

// TODO refactor macros one by one
// TODO reimplement visualization
