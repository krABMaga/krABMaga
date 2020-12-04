use amethyst::{
    core::Transform,
    ecs::{Join, System, WriteExpect, WriteStorage},
    shred::ReadExpect,
};

use crate::environment::TintEvent;
use crate::{
    agent_adapter::AgentAdapter, resources::AntsGrid, resources::ObstaclesGrid,
    resources::SitesGrid, resources::ToFoodGrid, resources::ToHomeGrid,
};
use amethyst::core::ecs::shrev::EventChannel;
use amethyst::core::ecs::Write;
use amethyst::renderer::resources::Tint;

pub const EVAPORATION: f64 = 0.999;

/// The main system of the simulation. It handles the ants' step actions, along with updating the ants'
/// tint according to whether they are holding food or not, and it also handles triggering the pheromones'
/// evaporation once all ants have moved and deposited their pheromones.
pub struct AntSystem;

// Transform our struct in an actual Amethyst System.
impl<'s> System<'s> for AntSystem {
    // Specify what data we are going to operate on and in which way. Amethyst will give it to us and it will build
    // an optimized execution schedule to parallelize systems as much as possible.
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, AgentAdapter>,
        WriteExpect<'s, AntsGrid>,
        ReadExpect<'s, ObstaclesGrid>,
        ReadExpect<'s, SitesGrid>,
        WriteExpect<'s, ToFoodGrid>,
        WriteExpect<'s, ToHomeGrid>,
        Write<'s, EventChannel<TintEvent>>,
        WriteStorage<'s, Tint>,
    );

    fn run(
        &mut self,
        (
            mut transforms,
            mut agent_adapters,
            mut ants_grid,
            obstacles_grid,
            sites_grid,
            mut to_food_grid,
            mut to_home_grid,
            mut event_channel,
            mut tint_storage,
        ): Self::SystemData,
    ) {
        // Loop all agents with their transform and tint
        for (agent, transform, tint) in
            (&mut agent_adapters, &mut transforms, &mut tint_storage).join()
        {
            agent.deposit_pheromone(&mut to_home_grid, &mut to_food_grid, &mut event_channel);
            agent.act(
                &mut ants_grid,
                &obstacles_grid,
                &sites_grid,
                &to_home_grid,
                &to_food_grid,
            );
            // Ants holding food are represented with a red tint.
            if agent.has_food {
                tint.0.red = 1.;
            } else {
                tint.0.red = 0.;
            }

            // Mirror the changes to the transform, so that the graphics update as well.
            transform.set_translation_xyz(agent.loc.x as f32, agent.loc.y as f32, 0.);
        }
        // Trigger the pheromones' evaporation.
        to_food_grid.multiply(EVAPORATION, &mut event_channel);
        to_home_grid.multiply(EVAPORATION, &mut event_channel);
    }
}
