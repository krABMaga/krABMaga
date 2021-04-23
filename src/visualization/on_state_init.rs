use bevy::prelude::{Commands, ResMut};

use crate::engine::agent::Agent;
use crate::engine::schedule::Schedule;
use crate::visualization::renderable::Render;
use crate::visualization::simulation_descriptor::SimulationDescriptor;
use crate::visualization::sprite_render_factory::SpriteFactoryResource;
use std::hash::Hash;

/// A simple trait which lets the developer set up the visualization components of his simulation.
/// This method will be called in a Bevy startup system.
pub trait OnStateInit<A: 'static + Agent + Render + Clone + Send + Hash + Eq>: Send + Sync {
    /// The method that will be called during the visualization inizialization.
    ///
    /// # Arguments
    ///
    /// * `commands` - Bevy [Commands](bevy::prelude::Commands), used mainly to create entities.
    /// * `sprite_render_factory` - A [bundle](crate::visualization::sprite_render_factory::SpriteFactoryResource) offering sprite-related resources.
    /// * `state` - The state of the simulation, available as a resource.
    /// * `schedule` - The schedule of the simulation, available as a resource.
    /// * `sim` - Data related to the simulation, for example width, height and center x/y.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_ab::visualization::on_state_init::OnStateInit;
    /// use bevy::prelude::{Commands, ResMut};
    /// use rust_ab::visualization::sprite_render_factory::SpriteFactoryResource;
    /// use rust_ab::visualization::simulation_descriptor::SimulationDescriptor;
    /// use rust_ab::visualization::renderable::{SpriteType, Render};
    /// # use rust_ab::engine::state::State;
    /// # use rust_ab::engine::agent::Agent;
    /// use rust_ab::bevy::prelude::Transform;
    /// use rust_ab::engine::schedule::Schedule;
    ///
    /// # struct MyState;
    /// # impl State for MyState{};
    ///
    /// # #[derive(Clone, Copy)]
    /// # struct MyAgent;
    /// # impl Agent for MyAgent{
    /// #    type SimState = MyState;
    /// #    fn step(&mut self,state: &Self::SimState) {}
    /// # }
    ///
    /// # impl Render for MyAgent{
    /// #   fn sprite(&self) -> SpriteType {
    /// #       SpriteType::Emoji(String::from("bird"))
    /// #   }
    /// #   fn position(&self,state: &Self::SimState) -> (f32, f32, f32) {
    /// #       (0.,0.,0.)
    /// #   }
    /// #   fn scale(&self) -> (f32, f32) {
    /// #       (1.,1.)
    /// #   }
    /// #   fn rotation(&self) -> f32 {
    /// #       0.
    /// #   }
    /// #   fn update(&mut self,transform: &mut Transform,state: &Self::SimState) {
    /// #       
    /// #   }
    /// # }
    /// pub struct VisState;
    ///
    /// impl OnStateInit<MyAgent> for VisState {
    ///     fn on_init(&self, mut commands: Commands, mut sprite_render_factory: SpriteFactoryResource, mut state: ResMut<MyState>, mut schedule: ResMut<Schedule<MyAgent>>, mut sim: ResMut<SimulationDescriptor>) {
    ///         let agent = MyAgent;
    ///         schedule.schedule_repeating(agent, 0., 0);
    ///
    ///         let SpriteType::Emoji(emoji_code) = agent.sprite();
    ///         let sprite_render =
    ///             sprite_render_factory.get_emoji_loader(emoji_code);
    ///         agent.setup_graphics(sprite_render, &mut commands, &state);
    ///     }
    /// }
    /// ```
    fn on_init(
        &self,
        commands: Commands,
        sprite_render_factory: SpriteFactoryResource,
        state: ResMut<A::SimState>,
        schedule: ResMut<Schedule<A>>,
        sim: ResMut<SimulationDescriptor>,
    );
}
