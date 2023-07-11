<div align="center">
  <img src="https://raw.githubusercontent.com/krABMaga/krABMaga.github.io/main/static/images/krabmaga.gif" alt="krabmaga animated logo" width="150" height="130">
  <h1> krABMaga </h1>
  <h4> <i>A modern developing art for reliable and efficient ABM simulation with the Rust language</i></h4>
  <img alt="Crates.io" src="https://img.shields.io/crates/l/krabmaga">
  <a href="https://crates.io/crates/krabmaga"><img alt="Crates.io" src="https://img.shields.io/crates/v/krabmaga"> </a>
  <a href="https://docs.rs/krabmaga/latest/krabmaga"><img alt="docs.rs" src="https://img.shields.io/docsrs/krabmaga"> </a>
  <img alt="Rust CI" src="https://github.com/krABMaga/krABMaga/workflows/Rust%20CI/badge.svg">
  <a href="https://codecov.io/gh/krABMaga/krABMaga"><img alt="codecov" src="https://codecov.io/gh/krABMaga/krABMaga/branch/main/graph/badge.svg?token=GWYP2UBPIZ"> </a>
</div>

(Notice that the *parallel* and *visualization* components are excluded from _codecov_ as are experimental ore release candidate)

**krABMaga** (Previously named Rust-AB) is a discrete events simulation engine for developing ABM simulation that is written in Rust language. 

krABMaga is designed to be a _ready-to-use_ tool for the ABM community and for this reason the architectural concepts of the well-adopted MASON library were re-engineered to exploit the Rust peculiarities and programming model.

**Developed by** [![ISISLab](https://raw.githubusercontent.com/krABMaga/krABMaga.github.io/main/static/images/isislab.png)](https://www.isislab.it)

# Examples

All the examples are hosted in a separate repository [here](https://github.com/krABMaga/examples).
- [Ants Foraging](https://github.com/krABMaga/examples/tree/main/antsforaging)
- [Flockers](https://github.com/krABMaga/examples/tree/main/flockers)
- [Forest Fire](https://github.com/krABMaga/examples/tree/main/forestfire)
- [Schelling Model](https://github.com/krABMaga/examples/tree/main/schelling)
- [Virus on Network](https://github.com/krABMaga/examples/tree/main/virusnetwork)
- [Wolf Sheep Grass](https://github.com/krABMaga/examples/tree/main/wolfsheepgrass)

### Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
krabmaga = 0.4.*
```

To get started using krABMaga, see [the examples](https://github.com/krABMaga/examples).
There's also a template to set up the correct project structure and the required files [here](https://github.com/krABMaga/examples/tree/main/template).

**Model Visualization with Bevy Game Engine**

Based on [Bevy game engine](https://bevyengine.org/), it's possible to run simulation with visualization. It's also available a menu to start and stop simulations and a slider to set simulation speed.
To run a model with visualization enabled, you have to start the simulation with the command:
```sh
cargo run --release --features  visualization

# Alternative command. Requires 'cargo make' installed
cargo make run --release 
```

In addition to the classical visualization, you can run your krABMaga simulation inside your browser using [*Web Assembly*](https://webassembly.org). 
This is possible with the command:
```sh
# Requires 'cargo make' installed
cargo make serve --release 
```

***Visualization FAQs***

In case you have troubles compiling your visualization, consult this following list of common errors first before making
an issue:
- [Wasm related errors due to `bevy_log`](https://github.com/bevyengine/bevy/issues/3099): run this command to force the `tracing-wasm` dependency to 0.2.0:
```sh
cargo update -p tracing-wasm --precise 0.2.0
```
- "Data remaining" issue or "len is 0 but index is 0" when running a simulation on the web: Force update your wasm-bindgen-cli local installation to version 0.2.79.
- Out of memory error when running a simulation on the web, in chrome: run your simulation with the release profile.

### Dependencies
The visualization framework requires certain dependencies to run the simulation properly.
- :desktop_computer: Windows: [VS2019 build tools](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16)
- :apple: MacOS: No dependencies needed.
- :penguin: Linux: A few dependencies are needed. Check [here](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md) for a list based on your distribution.

---

### How to write your first model

If you don't start from our [Template](https://github.com/krABMaga/examples/tree/main/template), add this to your `Cargo.toml`:
```toml
[dependencies]
krabmaga = 0.4.*

[features]
visualization = ["krabmaga/visualization"]
visualization_wasm = ["krabmaga/visualization_wasm"]
```

We **strongly** recommend to use [Template](https://github.com/krABMaga/examples/tree/main/template) or any other example as base of a new project, especially if you want to provide any visualization.

Each krABMaga model needs structs that implements our *Traits*, one for *State* and the other for *Agent*. In the *State* struct you have to put *Agent* field(s), because it represents the ecosystem of a simulation. More details for each krABMaga componenet are in the [Architecture](#architecture) section.

The simplest part is `main.rs`, because is similar for each example.
You can define two *main* functions using **cfg** directive, that can remove code based on which features are (not) enabled.  
Without visualization, you have only to use *simulate!* to run simulation, passing a state, step number and how may time repeat your simulation. 
With visualization, you have to set graphical settings (like dimension or background) and call *start* method.
```rs
// Main used when only the simulation should run, without any visualization.
#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn main() {
  let dim = (200., 200.);
  let num_agents = 100;  
  let state = Flocker::new(dim, num_agents);
  let step = 10;
  let reps = 1;
  let _ = simulate!(state, step, reps);
}

// Main used when a visualization feature is applied.
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
fn main() {
  let dim = (200., 200.);
  let num_agents = 100;
  let state = Flocker::new(dim, num_agents);
  Visualization::default()
      .with_window_dimensions(1000., 700.)
      .with_simulation_dimensions(dim.0 as f32, dim.1 as f32)
      .with_background_color(Color::rgb(0., 0., 0.))
      .with_name("Flockers")
      .start::<VisState, Flocker>(VisState, state);
}

```
---

### Available features

| Compilation Feature  | Description |  Experimental | Release Candidate  | Stable  |
|:----:|:---------:|:---:|:---:|:---:|
| **No Features** | Possibility to run model using `Simulation Terminal` and setup model-exploration experiments (Parameter Sweeping, Genetic and Random) in sequential/parallel mode. It's enough to create your base simulations. |   |   | 🦀 |
| **visualization**  | Based on `Bevy engine`, it makes possible to visualize your model elements, to understand better the behavior of your simulation. |   | 🦀 |   |
| **visualization-wasm** | Based on `Web Assembly`, give you the possibility to execute your visualized simulation inside your own browser. |   | 🦀 |   |
| **distributed-mpi** | Enable distributed model exploration using MPI. At each iteration, the amount of configurations are balanced among your nodes.  |   |  🦀 |   |
| **bayesian**  | Use ML Rust libraries to use/create function to use `Bayesian Optimization`.|   | 🦀  |   |
| **parallel**  | Speed-up a single simulation parallelizing agent scheduling during a step.| 🦀  |   |   |


---

### Macros for playing with Simulation Terminal

`Simulation Terminal` is enabled by default using macro `simulate!`, so can be used passing a state, step number and how may time repeat your simulation..
That macro has a fourth optional parameter, a boolean. When `false` is passed, `Simulation Terminal` is disabled.
```rs
($s:expr, $step:expr, $reps:expr $(, $flag:expr)?) => {{
      // Macro code 
}}
```

You can create tabs and plot your data using two macro:
- `addplot!` let you create a new plot that will be displayed in its own tab.
```rs
addplot!(String::from("Chart Name"), String::from("xxxx"), String::from("yyyyy"));
```
- `plot!` to add a point to a plot. Points can be added during simulation execution, for example inside `after_step` method.
  You have to pass plot name, series name, x value and y value. Coordinate values need to be `f64`.
```rs
plot!(String::from("Chart name"), String::from("s1"), x, y);
```

On Terminal home page there is also a *log section*, you can plot log messages when some event needs to be noticed.
You can navigate among all logs using ↑↓ arrows.
To add a log use the macro `log!`, passing a `LogType` (an enum) and the log message.
```rs
 log!(LogType::Info, String::from("Log Message"));
```

Are available four type of Logs:
```rs
pub enum LogType {
    Info,
    Warning,
    Error,
    Critical,
}
```


### [Contributing FAQ](CONTRIBUTING.md)
 
## Support conference paper

If you find this code useful in your research, please consider citing:

```
@ARTICLE{AntelmiASIASIM2019,
  author={Antelmi, A. and Cordasco, G. and D’Auria, M. and De Vinco, D. and Negro, A. and Spagnuolo, C.},
  title={On Evaluating Rust as a Programming Language for the Future of Massive Agent-Based Simulations},
  journal={Communications in Computer and Information Science},
  note={Conference of 19th Asia Simulation Conference, AsiaSim 2019 ; Conference Date: 30 October 2019 Through 1 November 2019;  Conference Code:233729},
  year={2019},
  volume={1094},
  pages={15-28},
  doi={10.1007/978-981-15-1078-6_2},
  issn={18650929},
  isbn={9789811510779},
}

```
🏆 Best Paper Nominee


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
