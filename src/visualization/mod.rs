cfg_if::cfg_if! {
if #[cfg(any(feature = "amethyst_metal", feature = "amethyst_vulkan"))] {
extern crate amethyst;

pub mod renderable;
pub mod visualization_state;
pub mod visualization;
pub mod on_state_init;
pub mod sprite_render_factory;

mod main_system_bundle;
mod systems;
}
}
/*
 What do we do here?
 The user will definitely start:
     - An initial state (might be unique, might not), with initialization code
     - A scheduler
 After this, he will loop for a number of steps, stepping the schedule and calling code pre/post step
 Finally, after stepping through the whole simulation, he will write some sort of ending code.

 STRUCTURE
 1) Model structs initialization: those must be done by the user, because he defines the model
 2) Stepping: This must be handled by us, because we want to be able to fully control the schedule.
     We must let the user write code prior or after doing a step, he must have access to the current step
     so we must let the user inject two closures.
     The model schedule will act as its own system, whereas Amethyst's schedule schedules the model vs
     the rendering. So we basically have two levels: Application wide (Model and Rendering), and
     Model wide (The Rust-AB schedule).
 3) Ending code: a closure to be executed prior to the simulation's end.

 FEATURES
 1) The user must map an agent to a particular sprite somehow. The user will optimally define a
     folder containing the UI-only code (the model must NOT be touched). This folder will contain the
     main_ui, which initializes the model structs and wraps them with our visualization wrapper,
     and one file per agent, to implement the required traits to render the agents correctly.
     The initial idea is to make the user implement a Renderable trait, which specifies either an
     emoji's code or a sprite filename (assuming the file to be located in an assets folder, as a png
     containing the single sprite).
 1a) This also means the user must be able to specify properties that the model uses which need to
     be represented in the visualization. The most common example is positioning: Rust-AB has a
     Location2D trait implemented by agents. The visualization framework can leverage this trait to
     fetch the coordinates to apply to the sprites' transforms.
 2) The user must be able to also set the environment's renderable properties. This can be easily done
     by exposing a proxy method on the wrapper that allows access to with_clear, to paint the background
     with a specific color.
 3) An agent's direction must be mapped somehow. How? Models usually don't bother with directions.
     This could be solved with a Directed trait that lets the user specify how to calculate the direction,
     expressed in radians (for example, by using lastd with the current position).
 4) The visualization's wrapper must expose a panel that allows basic operations on the simulation
     (start, stop, pause, set velocity) and on the agents themselves (view, edit properties).
     There must also be a way to disable/re-enable the whole visualization framework, for example if
     the user is only interested in late stages of his simulation. The easy choice would be to simply
     turn the renderer system offline. A better choice would be to unhook it from the Amethyst's schedule,
     to then reattach it when the user turns rendering on again, to ensure no impact on the model schedule.
 4a) This implies the visualization's window must be big enough to fit the rendered simulation in it,
     along with a panel to control the simulation. The simulation size must be specified in the user's
     main_ui.rs file.

 REQUIRED USER INPUT:
 --) AGENT
     Sprite (User must implement the Renderable trait on a struct that exposes an Agent)
     Direction (method that must return a radians value)
 --) ENVIRONMENT
     backgroundColor (expose srgba?)

*/
