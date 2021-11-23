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
use bevy_canvas::CanvasPlugin;
use bevy_egui::EguiPlugin;

use std::sync::{Arc, Mutex};

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
    pub fn start<I: VisualizationState<S> + 'static + Clone, S: State>(
        self,
        init_call: I,
        state: S,
    ) {
        let mut app_builder = self.setup(init_call, state);
        app_builder.run()
    }

    // Sets up the application, exposing the [AppBuilder]. Useful if you want to directly interface Bevy
    // and add plugins, resources or systems yourself.
    pub fn setup<I: VisualizationState<S> + Clone + 'static, S: State>(
        &self,
        init_call: I,
        mut state: S,
    ) -> AppBuilder {
        // Minimum constraints taking into account a 300 x 300 simulation window + a 300 width UI panel
        let mut window_constraints = WindowResizeConstraints::default();
        window_constraints.min_width = 600.;
        window_constraints.min_height = 300.;

        let window_descriptor = WindowDescriptor {
            title: self.window_name.parse().unwrap(),
            width: self.width,
            height: self.height,
            vsync: true,
            resize_constraints: window_constraints,
            ..Default::default()
        };

        let mut app = App::build();
        let mut schedule = Schedule::new();
        state.init(&mut schedule);
        let cloned_init_call = init_call.clone();

        app.insert_resource(window_descriptor)
            .add_plugins(DefaultPlugins)
            .add_plugin(EguiPlugin);

        #[cfg(target_arch = "wasm32")]
        app.add_plugin(bevy_webgl2::WebGL2Plugin);

        // #[cfg(feature = "canvas")]
        app.add_plugin(CanvasPlugin);

        app.insert_resource(SimulationDescriptor {
            title: self.window_name.parse().unwrap(),
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
        .add_startup_system(init_system::<I, S>.system())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_system(renderer_system::<I, S>.system().label("render"))
        .add_system(simulation_system::<S>.system().before("render"))
        .add_system(ui_system::<I, S>.system().before("render"))
        .add_system(camera_system.system());

        app
    }
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
