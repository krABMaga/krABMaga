# Rust AntsForaging
An initial attempt at implementing the ants foraging simulation, with a visualization of it as well.

![](eELHp8s7FW.gif)

# How to run
Simply run `cargo run --example antsforage_amethyst --features amethyst_vulkan`, or add the `--release` flag for a slower to compile, faster to execute option.
If you're on macOS, you have to use the metal renderer backend. To do so, swap `amethyst_vulkan` with `amethyst_metal`.

# Dependencies
If you're on linux, follow the instructions [here](https://github.com/amethyst/amethyst#dependencies)

# Current issues
- [x] Obstacles are not implemented yet; Implemented on 18/10/2020
- [x] Pheromones lack some sort of visualization; Implemented on 18/10/2020
- [x] Pheromones do not evaporate over time. Implemented on 18/11/2020, optimized on 25/11/2020

# References:
- https://github.com/eclab/mason/tree/master/mason/src/main/java/sim/app/antsforage
