use bevy::prelude::{Handle, Image, Query, Res, Transform, Visibility};

use crate::bevy::prelude::{Commands, ResMut};
use crate::engine::state::State;
use crate::visualization::{
    agent_render::{AgentRender, SpriteType},
    asset_handle_factory::AssetHandleFactoryResource,
    simulation_descriptor::SimulationDescriptor,
    visualization_state::VisualizationState,
    wrappers::{ActiveSchedule, ActiveState},
};

// The system that updates the visual representation of each agent of our simulation.
pub fn renderer_system<
    I: VisualizationState<S> + Clone + 'static + bevy::prelude::Resource,
    S: State,
>(
    mut query: Query<(
        &mut Box<dyn AgentRender>,
        &mut Transform,
        &mut Visibility,
        &mut Handle<Image>,
    )>,
    state_wrapper: ResMut<ActiveState<S>>,
    schedule_wrapper: Res<ActiveSchedule>,
    mut sprite_factory: AssetHandleFactoryResource,
    mut commands: Commands,
    mut vis_state: ResMut<I>,
    sim_data: Res<SimulationDescriptor>,
) {
    if !sim_data.paused {
        vis_state.before_render(
            &mut state_wrapper.0.lock().expect("error on lock"),
            &schedule_wrapper.0.lock().expect("error on lock"),
            &mut commands,
            &mut sprite_factory,
        );

        for (mut agent_render, mut transform, mut visible, mut material) in query.iter_mut() {
            let state = state_wrapper.0.lock().expect("error on lock");
            if let Some(agent) = vis_state.get_agent(&agent_render, &Box::new(state.as_state())) {
                agent_render.update(
                    &agent,
                    &mut *transform,
                    &Box::new(state.as_state()),
                    &mut *visible,
                );
                *visible = Visibility::Visible;
                // transform.translation.x = 0.5;
                transform.translation.y += 0.5;
                let SpriteType::Emoji(emoji_code) =
                    agent_render.sprite(&agent, &Box::new(state.as_state()));
                let new_material = sprite_factory.get_material_handle(emoji_code);
                if *material != new_material {
                    *material = new_material;
                }
            } else {
                let schedule = schedule_wrapper.0.lock().expect("error on lock");
                let step = schedule.step;
                if step != 0 {
                    *visible = Visibility::Hidden;
                }
            }
        }
    }
}
