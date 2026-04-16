use bevy::prelude::{Commands, Query, Res, ResMut, Sprite, Transform, Visibility};

use crate::engine::state::State;

use crate::visualization::{
    agent_render::{AgentRenderComponent, SpriteType},
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
        &mut AgentRenderComponent,
        &mut Transform,
        &mut Visibility,
        &mut Sprite,
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

        for (mut agent_render_component, mut transform, mut visible, mut sprite) in query.iter_mut()
        {
            let agent_render = &mut agent_render_component.0;
            let state = state_wrapper.0.lock().expect("error on lock");
            if let Some(agent) = vis_state.get_agent(agent_render.as_ref(), &Box::new(state.as_state())) {
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
                if sprite.image != new_material {
                    sprite.image = new_material;
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
