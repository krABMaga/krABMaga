use bevy::prelude::{Commands, OrthographicCameraBundle, Res, ResMut, WindowDescriptor};

use crate::engine::agent::Agent;
use crate::engine::schedule::Schedule;
use crate::visualization::on_state_init::OnStateInit;
use crate::visualization::renderable::Render;
use crate::visualization::simulation_descriptor::SimulationDescriptor;
use crate::visualization::sprite_render_factory::SpriteFactoryResource;

/// The main startup system which boostraps a simple orthographic camera, centers it to aim at the simulation,
/// then calls the user provided init callback.
pub fn init_system<A: 'static + Agent + Render + Clone + Send, I: OnStateInit<A> + 'static>(
    on_init: Res<I>,
    sprite_factory: SpriteFactoryResource,
    mut commands: Commands,
    state: ResMut<A::SimState>,
    schedule: ResMut<Schedule<A>>,
    window: Res<WindowDescriptor>,
    sim: ResMut<SimulationDescriptor>,
) {
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.transform.translation.x = sim.center_x;
    camera_bundle.transform.translation.y = sim.center_y;
    camera_bundle.transform.scale.x = sim.width / window.width + 0.1;
    camera_bundle.transform.scale.y = sim.height / window.height + 0.1;
    commands.spawn_bundle(camera_bundle);
    on_init.on_init(commands, sprite_factory, state, schedule, sim);
}
