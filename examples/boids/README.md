# Rust Flockers
A simple implementation of the Flockers simulation, both with and without UI, fully based on the RustAB framework.
There are currently two versions:
- boids: Outputs the time elapsed for given a number of steps and number of agents (currently hardcoded), along with the step for seconds.
- boids_ui: Shows a graphical interface describing the flockers moving in the environments, casually grouping together and 
avoiding other flockers. The simulation never stops.

![](6IBb1CCxZj.gif)

# How to run
Simply run `cargo run --example boids_ui` for the simple version,
or `cargo run --example boids_ui --features "amethyst_vulkan"` for the UI version on Windows or Linux, or
`cargo run --example boids_ui --features "amethyst_metal"` on MacOS.
If you encounter any issue, refer to the [FAQ](../../FAQ.md).
You can also add the `--release` flag for a slower to compile, faster to execute option.
