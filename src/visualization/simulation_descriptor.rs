// A resource containing data about the simulation, for ease of access during initialization.
use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(any(feature = "visualization", feature = "visualization_wasm"))] {
        use crate::bevy::ecs::system::Resource;
        use crate::bevy::ecs::system::SystemParam;

        #[derive(Resource)]
        pub struct SimulationDescriptor {
            pub title: String,
            pub width: f32,
            pub height: f32,
            pub center_x: f32,
            pub center_y: f32,
            pub paused: bool,
            pub ui_width: f32,
        }

        // impl SimulationDescriptor {
        //     pub fn new(title: String, width: f32, height: f32, center_x: f32, center_y: f32, ui_width: f32) -> Self {
        //         Self {
        //             title,
        //             width,
        //             height,
        //             center_x,
        //             center_y,
        //             paused: false,
        //             ui_width,
        //         }
        //     }
        // }

        // impl FromWorld for MyFancyResource {
        //     fn from_world(world: &mut World) -> Self {
        //         // You have full access to anything in the ECS from here.
        //         // For instance, you can mutate other resources:
        //         let mut x = world.get_resource_mut::<MyOtherResource>().unwrap();
        //         x.do_mut_stuff();
        
        //         MyFancyResource { /* stuff */ }
        //     }
        // }

        //write a simulation descriptor with height and width that implement IntoSystem 
        // pub fn simulation_descriptor(
        //     mut commands: Commands,
        //     mut windows: ResMut<Windows>,
        //     sim: Res<SimulationDescriptor>,
        // ) {
        //     let window = windows.get_primary_mut().unwrap();
        //     window.set_title(&sim.title);
        //     window.set_resizable(true);
        //     window.set_inner_size(PhysicalSize::new(sim.width as u32, sim.height as u32));
        //     window.set_position(PhysicalPosition::new(sim.center_x, sim.center_y));
        //     commands.insert_resource(sim);
        // }
    }
}
