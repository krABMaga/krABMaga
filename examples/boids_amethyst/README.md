# Rust Flockers
An initial attempt at implementing a visualization layer for an agent based simulation, by relying on RustAB for part of the logic.
Flockers was chosen since it's one of the easiest simulations.

![](dlnJqGql3M.gif)

# How to run
Simply run `cargo run --example boids_amethyst --features amethyst_vulkan`, or add the  `--release` flag for a slower to compile, faster to execute option.
If you're on macOS, you have to use the metal renderer backend. To do so, swap `amethyst_vulkan` with `amethyst_metal`.

# Dependencies
- If you're on linux, follow the instructions [here](https://github.com/amethyst/amethyst#dependencies).
- On macOS, it may be required to update Xcode to the latest version, to allow for gfx-backend-metal to compile the required shaders.
- As of currently (11/12/2020), rustc 1.48.0 is not supported due to a [bug](https://github.com/amethyst/amethyst/issues/2524) with the winit version Amethyst uses.
For this reason, the project forces the default rustc toolchain version to be used to the 1.47.0.

# Current issues
- The sprite bounding box seems to not be properly aligned to the sprite itself. The collision actually
    happens with a smaller bounding box.
    
	- SOLVED: The Field2D implementation is point-based, so the agents have no area. To account for this,
	    the neighbors distance value was raised from 10.0 to 12.5, and the cohesion multiplier was severely lowered from 1.0 to 0.1.

- The movements look sudden, sometimes flockers flying together try to collide with each other slightly while moving.
    This might be caused by the use of set_translation_xyz() instead of appending the vector movement, thus causing
    the translations to not look perfectly smooth.
	- PARTIALLY SOLVED: The conflicts between consistent flockers were mainly caused by the bounding box.
	    The only issue that remains is that, when a collision happens with a group of agents,
	    the agent's rotation matrix goes slightly crazy while trying to choose the next direction to pick.

- Some dead agents seem to have no collision activated for unknown reasons, resulting in agents just going through them.
    Seems to happen more with dead agents near the perimeter of the Field2D: maybe it's related to some toroidal calculation?

# References:
- https://github.com/spagnuolocarmine/abm/blob/master/examples/boids_ui.rs
- https://github.com/eclab/mason/tree/f89201872a91c2176e5dcbcdd1960d3fa6fe1f91/mason/src/main/java/sim/app/flockers