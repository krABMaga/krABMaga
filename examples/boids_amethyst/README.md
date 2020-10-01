# Rust Flockers
An initial attempt at implementing visualization for an agent based simulation, by relying on RustAB for part of the logic.
Flockers was chosen since it's one of the easiest simulations.

![](47y8baYdYg.gif)

# How to run
Simply run `cargo run`, or `cargo run --release` for a slower to compile, faster to execute option.

# Current issues
- The sprite bounding box seems to not be properly aligned to the sprite itself. The collision actually happens with a smaller bounding box.
	- SOLVED: The Field2D implementation is point-based, so the agents have no area. To account for this, the neighbors distance value was raised from 10.0 to 12.5, and the cohesion multiplier was severely lowered from 1.0 to 0.1.

- The movements look sudden, sometimes flockers flying together try to collide with each other slightly while moving. This might be caused by the use of set_translation_xyz() instead of appending the vector movement, thus causing the translations to not look perfectly smooth.
	- PARTIALLY SOLVED: The conflicts between consistent flockers were mainly caused by the bounding box. The only issue that remains is that, when a collision happens with a group of agents, the agent's rotation matrix goes slightly crazy while trying to choose the next direction to pick.

# References:
- https://github.com/spagnuolocarmine/abm/blob/master/examples/boids_ui.rs
- https://github.com/eclab/mason/tree/f89201872a91c2176e5dcbcdd1960d3fa6fe1f91/mason/src/main/java/sim/app/flockers