use bevy::prelude::{Commands, SpriteBundle};

use crate::bevy::ecs::system::Resource;
use crate::bevy::prelude::{Quat, Transform, Vec3};
use crate::engine::{agent::Agent, schedule::Schedule, state::State};
use crate::visualization::{
    agent_render::{AgentRender, SpriteType},
    asset_handle_factory::AssetHandleFactoryResource,
    simulation_descriptor::SimulationDescriptor,
};

// A simple trait which lets the developer set up the visualization components of his simulation.
// This method will be called in a Bevy startup system.
pub trait VisualizationState<S: State>: Send + Sync + Resource {
    fn on_init(
        &self,
        commands: &mut Commands,
        sprite_render_factory: &mut AssetHandleFactoryResource,
        state: &mut S,
        schedule: &mut Schedule,
        sim: &mut SimulationDescriptor,
    );

    fn setup_graphics(
        &self,
        schedule: &mut Schedule,
        commands: &mut Commands,
        state: &mut S,
        mut sprite_render_factory: AssetHandleFactoryResource,
    ) {
        for (agent_impl, _) in schedule.events.iter() {
            let agent_render = self.get_agent_render(&agent_impl.agent, state);
            match agent_render {
                Some(agent_render) => {
                    let boxed_state = Box::new(state.as_state());
                    let SpriteType::Emoji(emoji_code) =
                        agent_render.sprite(&agent_impl.agent, &boxed_state);
                    let sprite_render = sprite_render_factory.get_emoji_loader(emoji_code);
                    self.setup_agent_graphics(
                        &agent_impl.agent,
                        agent_render,
                        sprite_render,
                        commands,
                        &boxed_state,
                    );
                }
                None => {}
            }
        }
    }

    fn setup_agent_graphics(
        &self,
        agent: &Box<dyn Agent>,
        agent_render: Box<dyn AgentRender>,
        mut sprite_bundle: SpriteBundle,
        commands: &mut Commands,
        state: &Box<&dyn State>,
    ) {
        // AgentVis separate object which accepts an agent reference
        let (x, y, z) = agent_render.location(agent, state);
        let (scale_x, scale_y) = agent_render.scale(agent, state);
        let rotation = agent_render.rotation(agent, state);

        let mut transform = Transform::from_translation(Vec3::new(x, y + 0.5, z));
        transform.scale.x = scale_x;
        transform.scale.y = scale_y;
        transform.rotation = Quat::from_rotation_z(rotation);

        sprite_bundle.transform = transform;
        commands
            .spawn(sprite_bundle)
            .insert(agent_render)
            .insert(transform);
    }

    // The user must specify which AgentRender is associated to which Agent through this method
    // TODO: how can the developer connect the two? Type string identifier?
    fn get_agent_render(&self, agent: &Box<dyn Agent>, state: &S) -> Option<Box<dyn AgentRender>>;

    fn get_agent(
        &self,
        agent_render: &Box<dyn AgentRender>,
        state: &Box<&dyn State>,
    ) -> Option<Box<dyn Agent>>;

    fn before_render(
        &mut self,
        _state: &mut S,
        _schedule: &Schedule,
        _commands: &mut Commands,
        _sprite_factory: &mut AssetHandleFactoryResource,
    ) {
    }
}
