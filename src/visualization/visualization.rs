use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(any(feature = "visualization", feature = "visualization_wasm"))] {
        use crate::bevy::diagnostic::FrameTimeDiagnosticsPlugin;

        use crate::engine::{schedule::Schedule, state::State};

        use crate::visualization::{
            asset_handle_factory::AssetHandleFactory,
            simulation_descriptor::SimulationDescriptor,
            systems::{
                camera_system::camera_system, init_system::init_system, renderer_system::renderer_system,
                simulation_system::simulation_system, ui_system::ui_system,
            },
            visualization_state::VisualizationState,
            wrappers::{ActiveSchedule, ActiveState, Initializer},
        };

        use bevy::{prelude::*, window::WindowResizeConstraints, DefaultPlugins};
        use bevy_egui::EguiPlugin;

        use crate::visualization::utils::fixed_timestep::{FixedTimestep, FixedTimestepState};
        use crate::visualization::utils::updated_time::{time_system, Time};
        use bevy_prototype_lyon::prelude::ShapePlugin;
        use std::sync::{Arc, Mutex};
        use bevy_inspector_egui::bevy_inspector;

        // The application main struct, used to build and start the event loop. Offers several methods in a builder-pattern style
        // to allow for basic customization, such as background color, asset path and custom systems. Right now the framework
        // supports the automatic visualization of a single type of agents, for ease of implementation.
        //
        // REQUIREMENTS:
        // 1) In the root of the project, a folder called `assets` must be created. The emoji icons used will
        //     have to be copied there. In future, this limitation will be removed.
        pub struct Visualization {
            width: f32,
            height: f32,
            sim_width: f32,
            sim_height: f32,
            window_name: &'static str,
            background_color: Color,
        }

        impl Visualization {
            // Specify width and height of the window where the visualization will appear. Defaults to 500x300.
            pub fn with_window_dimensions(mut self, width: f32, height: f32) -> Visualization {
                self.width = width;
                self.height = height;
                self
            }

            // Specify width and height of the simulation. This should not be smaller than the window dimension,
            // or else the simulation won't be fully visible. Defaults to 500.300
            pub fn with_simulation_dimensions(mut self, width: f32, height: f32) -> Visualization {
                self.sim_width = width;
                self.sim_height = height;
                self
            }

            // Specify the name of the window. Defaults to the project name defined in the cargo manifest.
            pub fn with_name(mut self, name: &'static str) -> Visualization {
                self.window_name = name;
                self
            }

            // Specify the background color of the window. Defaults to black.
            pub fn with_background_color(mut self, color: Color) -> Visualization {
                self.background_color = color;
                self
            }

            // Create the application and start it immediately. Requires a startup callback defined as a struct
            // that implements [OnStateInit], along with the state and the schedule, which you manually create.
            pub fn start<I: VisualizationState<S> + 'static + bevy::prelude::Resource + Clone, S: State>(
                self,
                init_call: I,
                state: S,
            ) {
                let mut app_builder = self.setup(init_call, state);
                app_builder.run()
            }

            // Sets up the application, exposing the [AppBuilder]. Useful if you want to directly interface Bevy
            // and add plugins, resources or systems yourself.
            pub fn setup<I: VisualizationState<S> + Clone + 'static + bevy::prelude::Resource, S: State>(
                &self,
                init_call: I,
                mut state: S,
            ) -> App {
                // Minimum constraints taking into account a 300 x 300 simulation window + a 300 width UI panel
                let mut window_constraints = WindowResizeConstraints::default();
                window_constraints.min_width = 600.;
                window_constraints.min_height = 300.;

                let mut app = App::new();
                let mut schedule = Schedule::new();
                state.init(&mut schedule);
                let cloned_init_call = init_call.clone();

                app.add_plugins(DefaultPlugins.set(WindowPlugin {
                    window: WindowDescriptor {
                        // width: 400.0,
                        ..default()
                    },
                    ..default()
                    }))
                    .add_plugin(EguiPlugin);


                // Required for network visualization
                app.add_plugin(ShapePlugin);
                app.add_plugin(bevy_inspector_egui::DefaultInspectorConfigPlugin); // adds default options and `InspectorEguiImpl`s

                app.insert_resource(SimulationDescriptor {
                    title: self
                        .window_name
                        .parse()
                        .expect("Error: can't parse window name"),
                    width: self.sim_width,
                    height: self.sim_height,
                    center_x: (self.width * 0.5) - (self.width - self.sim_width as f32) / 2.,
                    center_y: (self.height * 0.5) - (self.height - self.sim_height as f32) / 2.,
                    paused: true,
                    ui_width: 300.,
                })
                .insert_resource(ClearColor(self.background_color))
                .insert_resource(AssetHandleFactory::new())
                .insert_resource(init_call)
                .insert_resource(ActiveState(Arc::new(Mutex::new(state))))
                .insert_resource(ActiveSchedule(Arc::new(Mutex::new(schedule))))
                .insert_resource(Initializer(cloned_init_call, Default::default()))
                .init_resource::<Time>()
                .init_resource::<FixedTimestepState>()
                .add_startup_system(init_system::<I, S>)
                .add_startup_system(set_initial_timestep)
                .add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_system_set(
                    SystemSet::new()
                        .with_run_criteria(FixedTimestep::step)
                        .with_system(renderer_system::<I, S>.label("render"))
                        .with_system(simulation_system::<S>.before("render")),
                )
                .add_system(ui_system::<I, S>.before("render"))
                .add_system(camera_system)
                .add_system(inspector_ui)
                .add_system_to_stage(CoreStage::First, time_system);

                app
            }
        }

        fn set_initial_timestep(mut time: ResMut<Time>) {
            time.set_steps_per_second(60.);
        }

        fn inspector_ui(world: &mut World) {
            let egui_context = world.resource_mut::<bevy_inspector_egui::bevy_egui::EguiContext>().ctx_mut().clone();
        
            bevy_inspector_egui::egui::Window::new("UI").show(&egui_context, |ui| {
                bevy_inspector_egui::egui::ScrollArea::vertical().show(ui, |ui| {
                    // equivalent to `WorldInspectorPlugin`
                    bevy_inspector::ui_for_world(world, ui);
                     
                    // works with any `Reflect` value, including `Handle`s
                    let mut any_reflect_value: i32 = 5;
                    bevy_inspector::ui_for_value(&mut any_reflect_value, ui, world);
        
                    bevy_inspector_egui::egui::CollapsingHeader::new("Materials").show(ui, |ui| {
                        bevy_inspector::ui_for_assets::<bevy::pbr::StandardMaterial>(world, ui);
                    });
        
                    ui.heading("Entities");
                    bevy_inspector::ui_for_world_entities(world, ui);
                });
            });
        }

        impl Default for Visualization {
            fn default() -> Self {
                Visualization {
                    width: 600.,
                    height: 300.,
                    sim_width: 300.,
                    sim_height: 300.,
                    window_name: env!("CARGO_PKG_NAME"),
                    background_color: Color::rgb(1., 1., 1.),
                }
            }
        }
    }
}
