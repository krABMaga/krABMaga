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
//!- [How to run your first example simulaton](#how-to-run-your-first-example-simulaton)
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
//!# How to run your first example simulaton
//!First of all, install latest version of [Rust](https://www.rust-lang.org/tools/install). Follow steps to setup Rust toolchain (*cargo*, *rustc* and *rustup*).
//!
//!Now, you can download/clone all available krABMaga examples from our github repository called [examples](https://github.com/krABMaga/examples).
//!
//!To run a simulation, go to root directory of a model, for example `/path/to/examples/flockers`. With command `ls`, you should be able to see a typcal krABMaga simulation struct:
//!- `src`: main folder with code. It contains `main.rs` file and two directories for model and visulization components.
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
//!In addition to the classical visualization, you can run your krABMaga simulation inside your browser using (*Web Assembly*)[https://webassembly.org].
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
//!krABMaga = { git="https://github.com/krABMaga/krABMaga.git" }
//!
//![features]
//!visualization = ["krABMaga/visualization"]
//!visualization_wasm = ["krABMaga/visualization_wasm"]
//!```
//!
//!We **strongly** recommend to use [Template](https://github.com/krABMaga/examples/tree/main/template) or any other example as base of a new project, especially if you want to provide any visualization.
//!
//!Each krABMaga model needs structs that implements our *Traits*, one for *State* and the other for *Agent*. In the *State* struct you have to put *Agent* field(s), because it represents the ecosystem of a simulation. More details for each krABMaga componenet are in the [Architecture](#architecture) section.
//!
//!The simplest part is `main.rs`, because is similar for each example.
//!You can define two *main* functions using **cfg** directive, that can remove code based on which features are (not) enabled.  
//!Without visualization, you have only to use *simulate!* to run simulation, passing a state, step number and how may time repeat your simulation.
//!With visualization, you have to set graphical settings (like dimension or background) and call *start* method.
//!```rs
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
//!| **parallel**  | Speed-up a single simulation parallelizing agent scheduling during a step.| 🦀  |   |   |
//!
//!</div>
//!
//!---
//!# Macros for playing with Simulation Terminal
//!
//!`Simulation Terminal` is enabled by default using macro `simulate!`, so can be used passing a state, step number and how may time repeat your simulation..
//!That macro has a fourth optional parameter, a boolean. When `false` is passed, `Simulation Terminal` is disabled.
//!```rs
//!($s:expr, $step:expr, $reps:expr $(, $flag:expr)?) => {{
//!      // Macro code
//!}}
//!```
//!
//!You can create tabs and plot your data using two macro:
//!- `addplot!` let you create a new plot that will be displayed in its own tab.
//!```rs
//!addplot!(String::from("Chart Name"), String::from("xxxx"), String::from("yyyyy"));
//!```
//!- `plot!` to add a point to a plot. Points can be added during simulation execution, for example inside `after_step` method.
//!  You have to pass plot name, series name, x value and y value. Coordinate values need to be `f64`.
//!```rs
//!plot!(String::from("Chart name"), String::from("s1"), x, y);
//!```
//!
//!On Terminal home page there is also a *log section*, you can plot log messages when some event needs to be noticed.
//!You can navigate among all logs using ↑↓ arrows.
//!To add a log use the macro `log!`, passing a `LogType` (an enum) and the log message.
//!```rs
//! log!(LogType::Info, String::from("Log Message"));
//!```
//!
//!Are available four type of Logs:
//!```rs
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
//!If you want to test, add or change something inside krABMaga engine, you can clone [main repo](https://github.com/krABMaga/krABMaga) locally, and change dependecy inside `Cargo.toml` of your examples:
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
//!```
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

#[doc(hidden)]
/// Module for model exploration
pub mod explore;

#[doc(hidden)]
pub mod utils;

#[doc(hidden)]
pub use {
    ::lazy_static::*,
    chrono,
    core::fmt,
    csv::{Reader, Writer},
    hashbrown,
    indicatif::ProgressBar,
    rand, rand_pcg, rayon,
    rayon::prelude::*,
    std::collections::HashMap,
    std::error::Error,
    std::fs,
    std::fs::File,
    std::fs::OpenOptions,
    std::io,
    std::io::prelude::*,
    std::io::Write,
    std::process::{Command, Stdio},
    std::sync::{Arc, Mutex},
    std::thread,
    std::time::Duration,
    std::time::Instant,
};

#[cfg(any(feature = "visualization", feature = "visualization_wasm",))]
pub mod visualization;

#[cfg(any(feature = "visualization", feature = "visualization_wasm",))]
pub use bevy;

#[doc(hidden)]
pub use rand::{
    distributions::{Distribution, Uniform},
    thread_rng, Rng,
};

#[doc(hidden)]
#[cfg(not(feature = "visualization_wasm"))]
pub use {
    crate::utils::monitoring::ui::UI,
    crossterm,
    crossterm::event::poll,
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    plotters,
    sysinfo::*,
    tui::{
        backend::{Backend, CrosstermBackend},
        Terminal,
    },
};

#[cfg(feature = "distributed_mpi")]
pub use {
    memoffset::{offset_of, span_of},
    mpi_fork_fnsp::datatype::DynBufferMut,
    mpi_fork_fnsp::datatype::PartitionMut,
    mpi_fork_fnsp::point_to_point as p2p,
    mpi_fork_fnsp::Count,
    mpi_fork_fnsp::{datatype::UserDatatype, traits::*, Address},
};

#[cfg(feature = "distributed_mpi")]
pub extern crate mpi_fork_fnsp;

#[cfg(feature = "web_plot")]
pub extern crate walkdir;

#[cfg(feature = "web_plot")]
pub extern crate csv;

#[cfg(feature = "web_plot")]
pub extern crate notify;

#[cfg(feature = "web_plot")]
pub extern crate serde;

#[cfg(feature = "web_plot")]
pub extern crate serde_json;

#[cfg(feature = "web_plot")]
pub extern crate tungstenite;

#[cfg(feature = "web_plot")]
pub use {
    rocket::http::Header,
    rocket::fairing::{Fairing, Info, Kind},
    rocket::fs::NamedFile,
    std::path::{Path, PathBuf},
    
    
    //TokioTungstenite for web socket
    std::{net::TcpListener, thread::spawn},
    
    tungstenite::{
        accept_hdr,
        handshake::server::{Request, Response},
    },
    
    //WalkDir to inspect a directory
    walkdir::WalkDir,
    
    //csv
    
    //serde
    rocket::serde::{Serialize, json::Json},
    serde::*,
    
    //notify to inspect a directory
    notify::{Watcher, RecursiveMode, RawEvent, raw_watcher},
};

#[doc(hidden)]
#[cfg(any(feature = "bayesian"))]
pub use {argmin, finitediff, friedrich, statrs};

#[doc(hidden)]
#[cfg(feature = "aws")]
pub use {
    aws_config,
    aws_sdk_lambda,
    aws_sdk_sqs,
    futures::executor::block_on,
    lambda_runtime,
    serde_json,
    serde_json::{json, Value},
    std::fs,
    std::io::BufReader,
    tokio,
    tokio::runtime::Runtime, // 0.3.5
};

/// Options of `old_simulate!` macro
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum InfoSim {
    Verbose,
    Normal,
}

///
/// Model Exploration modes
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ExploreMode {
    /// Exaustive: Brute force parameter exploration
    /// Matched: explore every input with the same indexes
    Exaustive,
    Matched,
}

#[doc(hidden)]
#[derive(Clone)]
/// Struct to manage plots inside `Simulation Terminal`
pub struct PlotData {
    /// Title of the plot
    pub name: String,
    /// Data of a plot. Managed using `HashMap`: the key is series name, the value is a vector of couples (x, y) representing the data of the series.
    pub series: HashMap<String, Vec<(f64, f64)>>,
    /// Min value of x axis
    pub min_x: f64,
    /// Max value of x axis
    pub max_x: f64,
    /// Min value of y axis
    pub min_y: f64,
    /// Max value of y axis
    pub max_y: f64,
    /// Label of x axis
    pub xlabel: String,
    /// Label of y axis
    pub ylabel: String,
    /// If true: the plot is stored as a PNG file
    pub to_be_stored: bool,
}

#[doc(hidden)]
impl PlotData {
    /// Create new Plot
    pub fn new(name: String, xlabel: String, ylabel: String, to_be_stored: bool) -> PlotData {
        PlotData {
            name,
            series: HashMap::new(),
            min_x: f64::MAX,
            max_x: f64::MIN,
            min_y: f64::MAX,
            max_y: f64::MIN,
            xlabel,
            ylabel,
            to_be_stored,
        }
    }

    #[cfg(not(feature = "visualization_wasm"))]
    pub fn store_plot(&self, rep: u64) {
        let n_markers = 3;

        let colors = [
            RED,
            RGBColor(0, 95, 106), // Petrol Green
            BLACK,
            MAGENTA,
            GREEN,
            BLUE,
        ];

        use plotters::prelude::*;

        let date = CURRENT_DATE.clone();
        let path = format!("output/{}/{}", date, self.name.replace("/", "-"));

        // Create directory if it doesn't exist
        fs::create_dir_all(&path).expect("Can't create folder");

        let output_name = format!("{}/{}_{}.png", &path, self.name.replace("/", "-"), rep);

        let root = BitMapBackend::new(&output_name, (1024, 768)).into_drawing_area();
        root.fill(&WHITE).expect("Can't fill the canvas");

        let mut scatter_ctx = ChartBuilder::on(&root)
            .caption(self.name.clone(), ("sans-serif", 30))
            .margin(5)
            .x_label_area_size(60)
            .y_label_area_size(60)
            .build_cartesian_2d(self.min_x..self.max_x, self.min_y..self.max_y)
            .expect("Error Creating Chart");

        scatter_ctx
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .y_desc(self.ylabel.clone())
            .x_desc(self.xlabel.clone())
            .draw()
            .expect("Cant't draw mesh");

        let mut marker_id = 0;
        let mut color_id = 0;
        for (series_name, series) in &self.series {
            match marker_id {
                0 => scatter_ctx
                    .draw_series(
                        series
                            .iter()
                            .map(|(x, y)| Circle::new((*x, *y), 2.0, colors[color_id].filled())),
                    )
                    .expect("Can't draw series")
                    .label(series_name)
                    .legend(move |(x, y)| Circle::new((x, y), 3.0, colors[color_id].filled())),
                1 => scatter_ctx
                    .draw_series(
                        series
                            .iter()
                            .map(|(x, y)| Cross::new((*x, *y), 3.0, colors[color_id].filled())),
                    )
                    .expect("Can't draw series")
                    .label(series_name)
                    .legend(move |(x, y)| Cross::new((x, y), 3.0, colors[color_id].filled())),
                2 => scatter_ctx
                    .draw_series(series.iter().map(|(x, y)| {
                        TriangleMarker::new((*x, *y), 3.0, colors[color_id].filled())
                    }))
                    .expect("Can't draw series")
                    .label(series_name)
                    .legend(move |(x, y)| {
                        TriangleMarker::new((x, y), 3.0, colors[color_id].filled())
                    }),
                _ => scatter_ctx
                    .draw_series(
                        series
                            .iter()
                            .map(|(x, y)| Circle::new((*x, *y), 2.0, colors[color_id].filled())),
                    )
                    .expect("Can't draw series")
                    .label(series_name)
                    .legend(move |(x, y)| Circle::new((x, y), 3.0, colors[color_id].filled())),
            };

            scatter_ctx
                .draw_series(LineSeries::new(
                    series.iter().map(|(x, y)| (*x, *y)),
                    colors[color_id],
                ))
                .expect("Can't draw series curve");

            marker_id = (marker_id + 1) % n_markers;
            color_id = (color_id + 1) % colors.len();
        }

        scatter_ctx
            .configure_series_labels()
            .position(SeriesLabelPosition::UpperRight)
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()
            .expect("Can't draw series labels");

        root.present()
            .unwrap_or_else(|_| panic!("Unable to write result to file: {}", output_name))
        //.expect(format!("Unable to write result to file: {}", output_name).as_str());
    }
}

/// Available log types to use for `Simulation Terminal` log mechanism.
#[derive(Copy, Clone, Debug)]
pub enum LogType {
    Info,
    Warning,
    Error,
    Critical,
}

impl fmt::Display for LogType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LogType::Info => write!(f, "Info: "),
            LogType::Warning => write!(f, "Warning: "),
            LogType::Error => write!(f, "Error: "),
            LogType::Critical => write!(f, "Critical: "),
        }
    }
}

#[doc(hidden)]
pub struct Log {
    /// One of 4 availbale types
    pub ltype: LogType,
    /// Log message to display
    pub body: String,
    /// If true, Log will be stored in a log file
    pub to_be_stored: bool,
}

// Implements Display for Log
impl fmt::Display for Log {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.ltype, self.body)
    }
}

use std::sync::mpsc::Sender;
lazy_static! {
    /// static HashMap to manage plots of the whole simulation. Used to create tabs and plot inside `UI` module.
    #[doc(hidden)]
    pub static ref DATA: Mutex<HashMap<String, PlotData>> = Mutex::new(HashMap::new());
    // /// static HashMap to manage plots of the whole simulation. Used to create tabs and plot inside `UI` module.
    // #[doc(hidden)]
    pub static ref CSV_SENDER: Mutex<Option<Sender<MessageType>>> = Mutex::new(None);
    pub static ref PLOT_NAMES: Mutex<std::collections::HashSet<(String, String, String)>> = Mutex::new(std::collections::HashSet::new());
    /// static Vec to store all Logs and make it availables inside terminal.
    #[doc(hidden)]
    pub static ref LOGS: Mutex<Vec<Vec<Log>>> = Mutex::new(Vec::new());
    /// static String to save Model description to show as a popup. Press 's' on `Simulation Terminal.
    #[doc(hidden)]
    pub static ref DESCR: Mutex<String> = Mutex::new(String::new());
    /// Current date to manage plot storage
    #[doc(hidden)]
    pub static ref CURRENT_DATE: String = chrono::Local::now().format("%Y-%m-%d %H-%M-%S").to_string();
}

#[doc(hidden)]
/// struct to store machine system info during the simulation.
pub struct Monitoring {
    /// Percentage of memory used
    pub mem_used: Vec<f64>,
    /// Percentage of cpu used
    pub cpu_used: Vec<f64>,
}

#[doc(hidden)]
#[derive(Clone)]
pub enum MessageType {
    AfterRep(u64, u64),
    AfterStep(u64, f64),
    Clear,
    Consumed,
    EndOfSimulation,
    Quit,
    Step,
    Plot(String, String, f64, f64),
}

#[doc(hidden)]
impl Monitoring {
    pub fn new() -> Self {
        Monitoring {
            mem_used: Vec::new(),
            cpu_used: Vec::new(),
        }
    }
}

impl Default for Monitoring {
    fn default() -> Self {
        Self::new()
    }
}

lazy_static! {
    /// static object to collect data of monitoring
    #[doc(hidden)]
    pub static ref MONITOR: Arc<Mutex<Monitoring>> = Arc::new(Mutex::new(Monitoring::new()));
}

#[doc(hidden)]
pub use std::sync::mpsc::{self, RecvError, TryRecvError};

/// Run simulation directly using this macro. By default, `Simulation Terminal` is used
///
/// # Arguments
///
/// *`s` - istance of state of simulation
///
/// *`step`- number of steps to run
///
/// *`reps`- number of repetitions to run
///
/// *`flag` - to abilitate TUI (optional, default true)
#[cfg(not(feature = "visualization_wasm"))]
#[macro_export]
macro_rules! simulate {
    ($s:expr, $step:expr, $reps:expr $(, $flag:expr)?) => {{

        let mut flag = true;
        $(
            flag = $flag;
        )?

        if flag {

            let mut monitor = Arc::clone(&MONITOR);
            let (sender_monitoring, recv_monitoring) = mpsc::channel();
            let (sender_ui, recv_ui) = mpsc::channel();

            let pid_main = match get_current_pid() {
                Ok(pid) => pid,
                Err(_) => panic!("Unable to get current pid"),
            };

            thread::spawn(move ||

                loop {
                // System info - Monitoring CPU and Memory used

                let mut sys = System::new_all();
                sys.refresh_all();

                match sys.process(pid_main) {
                    Some(process) => {
                        let mem_used: f64 = ( sys.used_memory() as f64 / sys.total_memory() as f64) * 100.0;
                        // log!(LogType::Info, format!("Memory used: {}%", mem * 100.0 ));
                        // log!(LogType::Critical, format!("cpu usage {}", process.cpu_usage() as f64 / sys.cpus().len() as f64));

                        let cpu_used: f64 = process.cpu_usage() as f64 / sys.cpus().len() as f64;

                        {
                            let mut monitor = monitor.lock().unwrap();

                            if monitor.mem_used.len()>100 {
                                monitor.mem_used.remove(0);
                                monitor.cpu_used.remove(0);
                            }

                            monitor.mem_used.push(mem_used);
                            monitor.cpu_used.push(cpu_used);
                        }
                    },
                    None => {
                        log!(LogType::Critical, format!("Error on finding main pid"))
                    }
                };


                match recv_monitoring.try_recv() {
                    Ok(_) | Err(TryRecvError::Disconnected) => {
                        break;
                    }
                    Err(TryRecvError::Empty) => {}
                }
            });



            let mut tui_operation: Arc<Mutex<MessageType>> = Arc::new(Mutex::new(MessageType::Consumed));
            let mut tui_reps: Arc<Mutex<MessageType>> = Arc::new(Mutex::new(MessageType::Consumed));

            let c_tui_operation = Arc::clone(&tui_operation);
            let c_tui_reps = Arc::clone(&tui_reps);
            let terminal_thread = thread::spawn(move || {
                let tick_rate = Duration::from_millis(250);
                let _ = enable_raw_mode();
                let mut stdout = io::stdout();
                let _ = execute!(stdout, EnterAlternateScreen, EnableMouseCapture);
                let backend = CrosstermBackend::new(stdout);
                let mut terminal = Terminal::new(backend).unwrap();
                let mut last_tick = Instant::now();
                let mut ui = UI::new($step, $reps);
                loop {
                    terminal.draw(|f| ui.draw(f));
                    let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));
                    //check for keyboard input
                    if crossterm::event::poll(timeout).unwrap() {
                        //?
                        if let Event::Key(key) = event::read().unwrap(){
                            //?
                            match key.code {
                                KeyCode::Char(c) => ui.on_key(c),
                                KeyCode::Left => ui.on_left(),
                                KeyCode::Up => ui.on_up(),
                                KeyCode::Right => ui.on_right(),
                                KeyCode::Down => ui.on_down(),
                                _ => {
                                    log!(LogType::Critical, format!("Invalid key pressed!"));
                                }
                            }
                        }
                    }
                    if ui.should_quit {
                        disable_raw_mode();
                        execute!(
                            terminal.backend_mut(),
                            LeaveAlternateScreen,
                            DisableMouseCapture
                        );
                        terminal.show_cursor();
                        break;
                    }

                    match recv_ui.try_recv() {
                        Ok(_) | Err(TryRecvError::Disconnected) => {
                            let op;
                            let rep;
                            {
                                op = c_tui_operation.lock().unwrap().clone();
                                rep = c_tui_reps.lock().unwrap().clone();
                            }

                            match op {

                                MessageType::AfterStep(step, progress) => {
                                    ui.on_tick(step, progress);
                                    {
                                        *c_tui_operation.lock().unwrap() = MessageType::Consumed;
                                    }
                                },

                                MessageType::Clear => {
                                    terminal.clear();
                                },

                                MessageType::Quit => {
                                    terminal.clear();
                                    disable_raw_mode();
                                    execute!(
                                        terminal.backend_mut(),
                                        LeaveAlternateScreen,
                                        DisableMouseCapture
                                    );
                                    terminal.show_cursor();
                                    break;
                                },
                                _ => {},
                                // MessageType::Step => {
                                //     terminal.draw(|f| ui.draw(f));
                                // },
                            };

                            match rep {
                                MessageType::AfterRep(r, time) => {
                                    ui.on_rep(
                                        r,
                                        time,
                                    );

                                    {
                                        *c_tui_reps.lock().unwrap() = MessageType::Consumed;
                                    }
                                },
                                _ => {},
                            }
                        },
                        Err(TryRecvError::Empty) => {}
                    }
                };

            });


            let csv_recv: krabmaga::mpsc::Receiver<MessageType>;
            let (s, r)  = mpsc::channel();
            {
                let mut csv_send = CSV_SENDER.lock().expect("Error on lock");
                *csv_send = Some(s.clone());
                csv_recv = r;
            }

            let csv_thread = thread::spawn(move || {

                let open_files = |rep_counter: &u32| {

                    let mut csv_writers: Vec<(String, Writer<File>)> = PLOT_NAMES.lock().unwrap().iter().map(|(name, x, y)| {
                        let date = CURRENT_DATE.clone();
                        let path = format!("output/{}/{}", date, name.replace("/", "-"));

                        // Create directory if it doesn't exist
                        fs::create_dir_all(&path).expect("Can't create folder");

                        let csv_name = format!("{}/{}_{}.csv", path, name.replace("/", "-"), rep_counter);
                        let mut writer = Writer::from_path(csv_name).expect("error on open the file path");
                        writer.write_record(&["series", &x, &y]).unwrap();
                        (name.replace("/", "-"), writer)
                    }).collect();
                    csv_writers
                };

                let mut csv_writers = open_files(&0);
                let mut rep_counter = 0;
                loop {
                    match csv_recv.recv(){
                        Ok(message) => {
                            match message {
                                MessageType::Plot(name, series, x, y) => {
                                    for (n, writer) in &mut csv_writers {
                                        if name.replace("/", "-") == *n {
                                            writer.write_record(&[&series, &x.to_string(), &y.to_string()]).unwrap();
                                            writer.flush().unwrap();
                                        }
                                    }
                                },
                                MessageType::EndOfSimulation => {
                                    rep_counter += 1;
                                    csv_writers = open_files(&rep_counter);
                                },
                                _ => break,
                            }
                        },
                        Err(_) => {
                        }
                    };
                };
            });


            let sim_thread = thread::spawn(move || {
                let mut s = $s;
                let mut state = s.as_state_mut();
                let n_step: u64 = $step;

                for r in 0..$reps {
                    {
                        let mut logs = LOGS.lock().unwrap();
                        logs.insert(0, Vec::new());
                    }
                    //clean data structure for UI
                    { DATA.lock().unwrap().clear(); }
                    // terminal.clear();
                    {
                        let mut tui_operation = tui_operation.lock().unwrap();
                        *tui_operation = MessageType::Clear;
                    }

                    sender_ui.send(()).expect("Simulation interrupted by user. Quitting...");

                    let start = std::time::Instant::now();
                    let mut schedule: Schedule = Schedule::new();
                    state.init(&mut schedule);


                    log!(LogType::Info, format!("#{} Simulation started", r), true);
                    //simulation loop
                    for i in 0..n_step {

                        schedule.step(state);

                        //send after step to UI
                        {
                            let mut tui_operation = tui_operation.lock().unwrap();
                            *tui_operation = MessageType::AfterStep(
                                i,
                                (i + 1) as f64 / n_step as f64
                            );
                        }

                        sender_ui.send(()).expect("Simulation interrupted by user. Quitting...");
                        if state.end_condition(&mut schedule) {
                            {
                                let mut tui_operation = tui_operation.lock().unwrap();
                                *tui_operation = MessageType::Quit;
                            }
                            sender_ui.send(()).expect("Simulation interrupted by user. Quitting...");
                            break;
                        }

                    } //end simulation loop

                    {
                        CSV_SENDER.lock().unwrap().as_ref().unwrap().send(MessageType::EndOfSimulation);
                    }

                    log!(LogType::Info, format!("#{} Simulation ended", r), true);

                    {
                        let data = DATA.lock().unwrap();
                        // iterate on data values and save to file
                        for (key, plot) in data.iter() {
                            if plot.to_be_stored {
                                plot.store_plot(r)
                            }
                        }

                    }

                    let run_duration = start.elapsed();
                    {
                        let mut tui_reps = tui_reps.lock().unwrap();
                        *tui_reps = MessageType::AfterRep(
                            r,
                            ((schedule.step as f32 / (run_duration.as_nanos() as f32 * 1e-9)) as u64),
                        );
                    }

                    sender_ui.send(()).expect("Simulation interrupted by user. Quitting...");
                } //end of repetitions
                {
                    CSV_SENDER.lock().unwrap().as_ref().unwrap().send(MessageType::Quit);
                }

            });

            sim_thread.join();
            csv_thread.join();
            let _ = sender_monitoring.send(());


            {
                let mut logs = LOGS.lock().unwrap();

                // iter on logs and save to file

                let date = CURRENT_DATE.clone();
                // Create directory if it doesn't exist
                fs::create_dir_all("output").expect("Can't create folder");
                let log_path = format!("output/{}.log", date);
                let mut f = File::create(log_path).expect("Can't create log file");
                for log in logs.iter().flatten() {
                    if log.to_be_stored {
                    write!(f, "{}\n", log).expect("Can't write to log file");
                    }
                }

            }

            terminal_thread.join();
            
        } else {

            let mut s = $s;
            let mut state = s.as_state_mut();
            let n_step: u64 = $step;
            //basic simulation without UI
            for r in 0..$reps {
                let mut schedule: Schedule = Schedule::new();
                state.init(&mut schedule);
                //simulation loop
                for i in 0..n_step {
                    schedule.step(state);
                    if state.end_condition(&mut schedule) {
                        break;
                    }
                } //end simulation loop
            } //end of repetitions
            println!("Simulation finished!");
        } //enf if/else flag

    }}; // end pattern macro
} //end macro

/// Add a description to your simulation. You can show a popup (pressing `s`) with this message.
///
/// # Arguments
///
/// * `description` - The description to be shown.
#[macro_export]
macro_rules! description {
    ($description:expr) => {{
        *DESCR.lock().unwrap() = $description.clone();
    }};
}

/// Add a point to a series of an existing plot.
///
/// # Arguments
///
/// * `name` - name of the plot
///
/// * `series` - name of the series
///
/// * `x` - x value
///
/// * `y` - y value
#[macro_export]
macro_rules! plot {
    ($name:expr, $serie:expr, $x:expr, $y:expr) => {{
        let mut data = DATA.lock().unwrap();
        if data.contains_key(&$name) {
            let mut pdata = data.get_mut(&$name).unwrap();
            if !pdata.series.contains_key(&$serie) {
                pdata.series.insert($serie.clone(), Vec::new());
            }
            let serie = pdata.series.get_mut(&$serie).unwrap();
            serie.push(($x, $y));

            if $x < pdata.min_x {
                pdata.min_x = $x
            };
            if $x > pdata.max_x {
                pdata.max_x = $x
            };
            if $y < pdata.min_y {
                pdata.min_y = $y
            };
            if $y > pdata.max_y {
                pdata.max_y = $y
            };
        }
    }};
}

#[macro_export]
macro_rules! plot_csv {
    ($name:expr, $serie:expr, $x:expr, $y:expr) => {{
        let mut data = DATA.lock().unwrap();
        if data.contains_key(&$name) {
            let mut pdata = data.get_mut(&$name).unwrap();
            if !pdata.series.contains_key(&$serie) {
                pdata.series.insert($serie.clone(), Vec::new());
            }
            let serie = pdata.series.get_mut(&$serie).unwrap();
            serie.push(($x, $y));

            if $x < pdata.min_x {
                pdata.min_x = $x
            };
            if $x > pdata.max_x {
                pdata.max_x = $x
            };
            if $y < pdata.min_y {
                pdata.min_y = $y
            };
            if $y > pdata.max_y {
                pdata.max_y = $y
            };

            //send Plot Messsage on send csv channel
            {
                let send = CSV_SENDER
                    .lock()
                    .unwrap()
                    .as_ref()
                    .unwrap()
                    .send(MessageType::Plot($name.clone(), $serie.clone(), $x, $y))
                    .expect("Can't send to csv channel");
            }
        }
    }};
}

/// Create new plot for your simulation.
///
/// # Arguments
///
/// * `name`- name of the plot.
///  
/// * `x_label`- label for the x axis.
///  
/// * `y_label`- label for the y axis.
///  
/// * `to_be_stored`- if true, the plot will be saved in the output folder.
#[macro_export]
macro_rules! addplot {
    ($name:expr, $xlabel:expr, $ylabel:expr $(, $to_be_stored: expr)? ) => {{

        let mut to_be_stored = false;
        $(
            to_be_stored = $to_be_stored;
        )?

        let mut data = DATA.lock().unwrap();
        if !data.contains_key(&$name) {
            data.insert($name, PlotData::new($name, $xlabel, $ylabel, to_be_stored));
        }
    }};
}

#[macro_export]
macro_rules! setup_csv {
    ($name:expr, $xlabel:expr, $ylabel:expr  ) => {{
        let mut names = PLOT_NAMES.lock().unwrap();
        names.insert(($name, $xlabel, $ylabel));
    }};
}

/// Add a log to the simulation logger.
///
/// # Arguments
///
/// * `ltype` - LogType paramater to specify the type of log. See `LogType` enum for more information.
///
/// * `message` - message to be logged.
#[macro_export]
macro_rules! log {
    ($ltype:expr, $message:expr $(, $to_be_stored: expr)? ) => {{
        //TODO: Avoid From String

        let to_be_stored = false;
        $(
            let to_be_stored = $to_be_stored;
        )?

        {
            let mut logs = LOGS.lock().unwrap();
            if logs.is_empty() { logs.push(Vec::new()) }
            logs[0].insert(
                0,
                Log {
                    ltype: $ltype,
                    body: $message,
                    to_be_stored,
                },
            );
        }
    }};
}

#[macro_export]
/// Run simulation directly using this macro. Not based on `Simulation Terminal`.
///
/// # Arguments
///
/// * `step` - number of steps to be simulated
///
/// * `s` - istance of state of simulation
///  
/// * `reps` - number of repetitions
///  
/// * `info` - type of info you want to display during and after simulation. See `InfoSim` enum for more information.
macro_rules! simulate_old {
    ($step:expr, $s:expr, $reps:expr, $info:expr) => {{
        let mut s = $s;
        let mut state = s.as_state_mut();
        let n_step: u64 = $step;

        let mut results: Vec<(Duration, f32)> = Vec::new();
        let option = $info;

        match option {
            InfoSim::Verbose => {
                // println!("\u{1F980} krABMaga v1.0\n");
                // println!(
                //     "{0: >10}|{1: >9}|    {2: >11}|{3: >10}|",
                //     "#Rep", "Steps", "Steps/Seconds", "Time"
                // );
                // println!("--------------------------------------------------");
            }
            InfoSim::Normal => {
                println!("{esc}c", esc = 27 as char);
                println!("\u{1F980} krABMaga v1.0\n");
                println!(
                    "{0: >10}|{1: >9}|    {2: >11}|{3: >10}|",
                    "#Rep", "Steps", "Avg. Steps/Seconds", "Avg. Time"
                );
                println!("----------------------------------------------------------------");
            }
        }
        // print!("{:width$}|", 1, width = 14 - $reps.to_string().len());
        // println!(
        //     "{:width$}|",
        //     n_step,
        //     width = 15 - n_step.to_string().len() - $reps.to_string().len()
        // );

        match option {
            InfoSim::Verbose => {}
            InfoSim::Normal => {
                println!("{esc}c", esc = 27 as char);
            }
        }

        for r in 0..$reps {
            let mut schedule: Schedule = Schedule::new();
            state.init(&mut schedule);
            let start = std::time::Instant::now();
            //let pb = ProgressBar::new(n_step);
            for i in 0..n_step {
                schedule.step(state);
                if state.end_condition(&mut schedule) {
                    break;
                }
                //pb.inc(1);
            }
            //pb.finish_with_message("\u{1F980}");

            let run_duration = start.elapsed();

            match option {
                InfoSim::Verbose => {}
                InfoSim::Normal => {
                    println!("{esc}c", esc = 27 as char);
                    println!("\u{1F980} krABMaga v1.0\n");
                    println!(
                        "{0: >10}|{1: >9}|    {2: >11}|{3: >10}|",
                        "#Rep", "Steps", "Avg. Steps/Seconds", "Avg. Time"
                    );
                    println!("----------------------------------------------------------------");
                }
            }

            // let step_seconds =
            //     format!("{:.0}", schedule.step as f32 / (run_duration.as_secs_f32()));
            // let time = format!("{:.4}", run_duration.as_secs_f32());
            // print!("{:width$}|", (r + 1), width = 14 - $reps.to_string().len());
            // print!(
            //     "{:width$}|",
            //     schedule.step,
            //     width = 15 - n_step.to_string().len() - $reps.to_string().len()
            // );
            // print!("{:width$}", "", width = 13 - step_seconds.len());

            results.push((
                run_duration,
                schedule.step as f32 / (run_duration.as_nanos() as f32 * 1e-9),
            ));

            match option {
                InfoSim::Verbose => {
                    // print!("{}|", step_seconds);
                    // print!("{:width$}", "", width = 9 - time.len());
                    // println!("{}s|", time);
                }
                InfoSim::Normal => {
                    let mut avg_time = 0.0;
                    let mut avg_step_seconds = 0.0;
                    for (time, step_seconds) in &results {
                        avg_time += time.as_secs_f32();
                        avg_step_seconds += step_seconds;
                    }
                    avg_time /= results.len() as f32;
                    avg_step_seconds /= results.len() as f32;
                    let avg_step_seconds = format!("{:.2}", avg_step_seconds);
                    let avg_time = format!("{:.4}", avg_time);
                    print!("{}|", avg_step_seconds);
                    print!("{:width$}", "", width = 9 - avg_time.len());
                    println!("{}s|", avg_time);
                }
            }
        }
        results
    }};
}

#[macro_use]
mod no_exported {
    #[doc(hidden)]
    #[macro_export]
    macro_rules! replace_expr {
        ($_t:tt $sub:expr) => {
            $sub
        };
    }

    //Used to count tokens of an expansion
    #[doc(hidden)]
    #[macro_export]
    macro_rules! count_tts {
        ($($tts:tt)*) => {<[()]>::len(&[$(replace_expr!($tts ())),*])};
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! build_configurations{

        ($n_conf: expr, $( $input:ident )*) =>{{
        let mut config_table_index:Vec<Vec<usize>> = Vec::new();
        let mut input_size:usize = 0;
        let mut rep = $n_conf;
        {
            $(
                let mut row:Vec<usize> = Vec::with_capacity($n_conf);
                input_size = $input.len();
                rep /= input_size;
                let mut i = 0;
                for _ in 0..$n_conf{
                    for _ in 0..rep{
                            row.push(i);
                    }
                    i = (i + 1) % input_size;
                }
                config_table_index.push(row);
            )*
        }

        config_table_index
        }};

    }
}

///Create a csv file with the experiment results
///
///"DataFrame" trait allow the function to know field names and
///
///params list + output list for each configuration runned
///
/// # Arguments
/// * `name` - filename to save the csv file
/// * `dataframe` - dataframe with the configurations and results

pub fn write_csv<A: DataFrame>(name: &str, dataframe: &[A]) -> Result<(), Box<dyn Error>> {
    let csv_name = format!("{}.csv", name);
    let mut wtr = Writer::from_path(csv_name).expect("error on open the file path");
    //define column name
    wtr.write_record(A::field_names())?;

    for row in dataframe {
        wtr.serialize(row.to_string())?;
    }

    Ok(())
}

#[doc(hidden)]
//Trait implemented dynamically for our dataframe struct.
//Used into "export_dataframe" function
pub trait DataFrame {
    fn field_names() -> &'static [&'static str];
    fn to_string(&self) -> Vec<String>;
}

///Generate parameter values using a Uniform Distribution.
///
/// # Arguments
/// * `type` - The type of the values to sample.
/// * `min` - The minimum value of the range.
/// * `max` - The maximum value of the range.
/// * `n` - The number of values to sample.
#[macro_export]
macro_rules! gen_param {
    ( $type:ty, $min:expr, $max:expr, $n:expr) => {{
        let minimum: $type;
        let maximum: $type;
        minimum = $min;
        maximum = $max;
        let mut n = $n as usize;

        // Check parameters range to avoid error with Distribution
        let (minimum, maximum) = if minimum > maximum {
            (maximum, minimum)
        } else if minimum == maximum {
            (minimum, maximum + 1 as $type)
        } else {
            (minimum, maximum)
        };

        if n == 0 {
            n = 1;
        }

        let between = Uniform::from(minimum..maximum);
        let mut rng = rand::thread_rng();
        let dist: Vec<$type> = between.sample_iter(&mut rng).take($n).collect();

        dist
    }};

    // gen a single value
    (  $type:ty, $min:expr, $max:expr) => {{
        gen_param!($type, $min, $max, 1)
    }};
}

/// Load parameters from a csv.
///
/// # Arguments
///
/// * `input_file` - path to the csv
///
/// * `x` and `x_ty`, couples of field names and their types.
#[macro_export]
macro_rules! load_csv {

    ($input_file: expr, $( $x:ident: $x_ty: ty ),*) =>{{
        let mut rdr = Reader::from_path($input_file).expect("error on read a file from path");
        $(
            let mut $x: Vec<$x_ty> = Vec::new();
        )*
        for result in rdr.records() {
            let record = result.expect("error on unwrap the record in csv file");
            let mut i = 0;
            $(
                let x : $x_ty = record[i].parse().expect("error on parsing the record");
                $x.push(x);
                i += 1;
            )*
        }
        let v = ($( $x, )*);
        v
    }};
}



#[macro_export]
macro_rules! plots {
    () => {
        #[macro_use] extern crate rocket;
        
        //struct for the final response, The client will get a Vector of FinalResponse. Each response represtents the information of 1 file.
        #[derive(Deserialize,Serialize,Debug)]
        struct FinalResponse{
            file : String,
            data : DataSet,
        }
        
        //struct for a single ChartData
        #[derive(Debug,Deserialize,Serialize)]
        struct ChartData{
            //Example 'Wolfs'
            label:String,
            //Example ['40','57,'42']
            data:Vec<String>,
        }
        
        impl ChartData{
            //Push a value to the data vector
            fn add_data(&mut self,data : String) {
                self.data.push(data);
            }
        }
        
        //struct for a complete Dataset
        #[derive(Debug,Deserialize,Serialize)]
        struct DataSet{
            datasets: Vec<ChartData>,
            labels: Vec<String>,
        }
        
        //struct for message to send to the client
        #[derive(Debug,Deserialize,Serialize)]
        struct WsMessage{
            op: String,
            response: FinalResponse,
        }
        
        //struct for message to send to the client for a Remove Operation
        #[derive(Debug,Deserialize,Serialize)]
        struct RemoveStruct{
            op: String,
            file: String,
        }
        
        
        //Struct For CORS Settings
        pub struct CORS;
        #[rocket::async_trait]
        impl Fairing for CORS {
            fn info(&self) -> Info {
                Info {
                    name: "Add CORS headers to responses",
                    kind: Kind::Response
                }
            }
        
            async fn on_response<'r>(&self, _request: &'r rocket::Request<'_>, response: &mut rocket::Response<'r>) {
                response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
                response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
                response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
                response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
            }
        }
        
        //next 3 function to serve the front end
        #[get("/<file..>")]
        async fn serve_front_end(file: PathBuf) -> Option<NamedFile> {
            NamedFile::open(Path::new("./src/bin/build/").join(file)).await.ok()
        }
        
        #[get("/Chart/<_file..>")]
        async fn serve_front_end_2(_file: PathBuf) -> Option<NamedFile> {
            NamedFile::open("./src/bin/build/index.html").await.ok()
        }
        
        #[get("/")]
        async fn index() -> Option<NamedFile> {
            NamedFile::open("./src/bin/build/index.html").await.ok()
        }
        
        //Endpoint for single chart display
        #[get("/buildsingledata/<filename>")]
        async fn get_single_data(filename: String) -> Json<Vec<FinalResponse>>{
            let path = "./output";
            let mut final_res = Vec::new();
            for entry in WalkDir::new(path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok()) {
                let f_name = entry.file_name().to_string_lossy();
                if f_name.ends_with(".csv"){
                    if f_name == filename{
                        let file_name = format!("{}",entry.path().display());
                        let contents = read_file(&file_name);
                        let datasets = build_dataset(&contents);
                        final_res.push(FinalResponse{file:f_name.to_string(),data:datasets});
                    }
                }
            }
            Json(final_res)
        }
        
        
        //endpoint for get data
        //the server parses the data and returns a vector of FinalResponse
        //each response represents the information about 1 file
        #[get("/getcsvdata")]
        async fn get_csv_data() -> Json<Vec<FinalResponse>> {
            let response = compute();
            Json(response)
        }
        
        //Main Function
        //Get paths to the files and build datasets
        fn compute() -> Vec<FinalResponse>{
            let mut final_json : Vec<FinalResponse> = Vec::new();
            let path = "./output";
            for entry in WalkDir::new(path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok()) {
                let f_name = entry.file_name().to_string_lossy();
                if f_name.ends_with(".csv"){
                    let filename = format!("{}",entry.path().display());
                    let contents = read_file(&filename);
                    let datasets = build_dataset(&contents);
                    let final_res = FinalResponse{file:f_name.to_string(),data:datasets};
                    final_json.push(final_res);
                }
            }
            final_json
        }
        
        //function to read a file
        fn read_file(filename: &str)-> String{
            let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
            contents
        }
        
        //function to build a Vector of ChartData for each file in the directory
        fn build_dataset(contents: &str)->DataSet{
            let mut dataset : Vec<ChartData> = Vec::new();
            let mut labels : Vec<String> = Vec::new();
            let mut max = 0;
            let mut rdr = csv::Reader::from_reader(contents.as_bytes());
            for result in rdr.records() {
                let mut insert = true;
                let record = result.expect("Error Reading Records");
                for entry in &mut dataset{
                    if entry.label == record[0].to_string(){
                        entry.add_data(record[2].to_string());
                        insert = false;
                    }
                }
                if insert{
                    dataset.push(ChartData{
                        label:record[0].to_string(),
                        data:Vec::new(),
                    });
                    for entry in &mut dataset{
                        if entry.label == record[0].to_string(){
                            entry.add_data(record[2].to_string());
                        }
                    }
                }
            }
            for entry in &dataset{
                if entry.data.len() > max{
                    max = entry.data.len();
                }
            }
            for n in 0..max{
                labels.push(n.to_string());
            }
            DataSet{labels:labels,datasets:dataset}
        }
        
        #[launch]
        async fn rocket() -> _ {
            
            spawn(||{
            let server = TcpListener::bind("127.0.0.1:3012").unwrap();
            println!("{:?}",std::env::current_dir());
            println!("listening on port 127.0.0.1:3012");
            for stream in server.incoming() {
                spawn(move || {
                    let callback = |req: &Request,response: Response| {
                        println!("Received a new ws handshake");
                        println!("The request's path is: {}", req.uri().path());
        
                        //For print Request's Headers
                        /*println!("The request's headers are:");
                        for (ref header, _value) in req.headers() {
                            println!("* {}", header);
                        }*/
        
                        Ok(response)
                    };
                    let mut websocket = accept_hdr(stream.unwrap(), callback).unwrap();
                    let (tx, rx) = std::sync::mpsc::channel();
        
                    // Create a watcher object, delivering raw events.
                    // The notification back-end is selected based on the platform.
        
                    let mut watcher = raw_watcher(tx).unwrap();
        
                    // Add a path to be watched. All files and directories at that path and
                    // below will be monitored for changes.
        
                    watcher.watch("./output", RecursiveMode::Recursive).unwrap();
                    println!("watching path : ./output");
                    loop {
                        match rx.recv() {
                            Ok(RawEvent{path: Some(path), op: Ok(op), cookie}) => {
                                println!("{:?} {:?} ({:?})", op, path, cookie);
                                let filename = path.file_name().unwrap().to_str().unwrap().to_string();
                                let file_path = path.clone().into_os_string().into_string().unwrap();
                                if file_path.ends_with(".csv")
                                {
                                    //Create different message for client, based on operation
                                    let operation;
                                    match op {
                                        notify::op::Op::WRITE => operation="WRITE".to_string(),
                                        notify::op::Op::CREATE => operation="CREATE".to_string(),
                                        notify::op::Op::REMOVE => operation="REMOVE".to_string(),
                                        _ => operation="DEFAULT".to_string(),
                                    }
                                    if operation == "REMOVE"{
                                        let response = {RemoveStruct{op:operation,file:filename}};
                                        let json = serde_json::to_string(&response).unwrap();
                                        websocket.write_message(tungstenite::Message::Text(json)).unwrap();
                                    }else{
                                        let contents = read_file(&file_path);
                                        let datasets = build_dataset(&contents);
                                        let final_res = FinalResponse{file:filename,data:datasets};
                                        let ws_message = WsMessage{op:operation,response:final_res};
                                        let json = serde_json::to_string(&ws_message).unwrap();
                                        websocket.write_message(tungstenite::Message::Text(json)).unwrap();
                                    }
                            }
                            },
                            Ok(event) => println!("broken event: {:?}", event),
                            Err(e) => println!("watch error: {:?}", e),
                        }
                    }
                });
            }});
        
            //starting the server
            rocket::build()
                    .mount("/", rocket::routes![
                        index,
                        serve_front_end,
                        serve_front_end_2,
                        get_csv_data,
                        get_single_data
                    ]).attach(CORS)
        }
    }
}
