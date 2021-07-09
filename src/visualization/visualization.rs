use bevy::prelude::*;
use bevy::DefaultPlugins;

use crate::engine::agent::Agent;
use crate::engine::schedule::Schedule;
use crate::visualization::on_state_init::OnStateInit;
use crate::visualization::renderable::Render;
use crate::visualization::simulation_descriptor::SimulationDescriptor;
use crate::visualization::sprite_render_factory::SpriteRenderFactory;
use crate::visualization::systems::init_system::init_system;
use crate::visualization::systems::renderer_system::renderer_system;
use crate::visualization::systems::simulation_system::simulation_system;

/// The application main struct, used to build and start the event loop. Offers several methods in a builder-pattern style
/// to allow for basic customization, such as background color, asset path and custom systems. Right now the framework
/// supports the automatic visualization of a single type of agents, for ease of implementation.
///
/// REQUIREMENTS:
/// 1) In the root of the project, a folder called `assets` must be created. The emoji icons used will
///     have to be copied there. In future, this limitation will be removed.
pub struct Visualization {
    width: f32,
    height: f32,
    sim_width: f32,
    sim_height: f32,
    window_name: &'static str,
    background_color: Color,
}

impl Visualization {
    /// Specify width and height of the window where the visualization will appear. Defaults to 500x300.
    pub fn with_window_dimensions(mut self, width: f32, height: f32) -> Visualization {
        self.width = width;
        self.height = height;
        self
    }

    /// Specify width and height of the simulation. This should not be smaller than the window dimension,
    /// or else the simulation won't be fully visible. Defaults to 500.300
    pub fn with_simulation_dimensions(mut self, width: f32, height: f32) -> Visualization {
        self.sim_width = width;
        self.sim_height = height;
        self
    }

    /// Specify the name of the window. Defaults to the project name defined in the cargo manifest.
    pub fn with_name(mut self, name: &'static str) -> Visualization {
        self.window_name = name;
        self
    }

    /// Specify the background color of the window. Defaults to black.
    pub fn with_background_color(mut self, color: Color) -> Visualization {
        self.background_color = color;
        self
    }

    /// Create the application and start it immediately. Requires a startup callback defined as a struct
    /// that implements [OnStateInit], along with the state and the schedule, which you manually create.
    pub fn start<A: 'static + Agent + Render + Clone + Send, I: OnStateInit<A> + 'static>(
        self,
        init_call: I,
        state: A::SimState,
        schedule: Schedule<A>,
    ) {
        let mut app_builder = self.setup(init_call, state, schedule);
        app_builder.run()
    }

    /// Sets up the application, exposing the [AppBuilder]. Useful if you want to directly interface Bevy
    /// and add plugins, resources or systems yourself.
    pub fn setup<A: 'static + Agent + Render + Clone + Send, I: OnStateInit<A> + 'static>(
        &self,
        init_call: I,
        state: A::SimState,
        schedule: Schedule<A>,
    ) -> AppBuilder {
        let mut app = App::build();
        app.add_plugins(DefaultPlugins);
        #[cfg(target_arch = "wasm32")]
        app.add_plugin(bevy_webgl2::WebGL2Plugin);
        #[cfg(feature = "canvas")]
        app.add_plugin(bevy_canvas::CanvasPlugin);

        app.insert_resource(WindowDescriptor {
            title: self.window_name.parse().unwrap(),
            width: self.width,
            height: self.height,
            vsync: true,
            ..Default::default()
        })
        .insert_resource(SimulationDescriptor {
            width: self.sim_width,
            height: self.sim_height,
            center_x: (self.width * 0.5) - (self.width - self.sim_width as f32) / 2.,
            center_y: (self.height * 0.5) - (self.height - self.sim_height as f32) / 2.,
        })
        .insert_resource(ClearColor(self.background_color))
        .insert_resource(SpriteRenderFactory::new())
        .insert_resource(init_call)
        .insert_resource(state)
        .insert_resource(schedule)
        .add_startup_system(init_system::<A, I>.system())
        .add_system(renderer_system::<A>.system().label("render"))
        .add_system(simulation_system::<A>.system().before("render"));

        app
    }
}

impl Default for Visualization {
    fn default() -> Self {
        Visualization {
            width: 500.,
            height: 300.,
            sim_width: 500.,
            sim_height: 300.,
            window_name: env!("CARGO_PKG_NAME"),
            background_color: Color::rgb(1., 1., 1.),
        }
    }
}
