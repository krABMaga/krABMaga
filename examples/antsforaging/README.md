# Rust AntsForaging
A simple implementation of the Ants Foraging simulation, fully based on the RustAB framework.
There are currently two versions:
- antsforaging: The simulation without the visualization framework. Outputs the number of steps
required for the ants to find the food and return to their nest for the first time;
- antsforaging_ui: The simulation with the visualization framework enabled. Allows the viewer to see the random
paths taken by the ants while they look for food and avoid obstacles, and the pheromone distribution around the grid hotspots
  (nest and food sites).

![](E2lM5gktFl.gif)

![](aj3Hxwh3fI.gif)

# How to run
Simply run `cargo run --example antsforaging` for the simple version,
or `cargo run --example antsforaging_ui --features "amethyst_vulkan"` for the UI version.
If the visualization is being executed in a macOS environment, switch the `amethyst_vulkan` feature with the `amethyst_metal` one. 
You can also add the `--release` flag for a slower to compile, faster to execute option.

# References:
- https://github.com/eclab/mason/tree/master/mason/src/main/java/sim/app/antsforage