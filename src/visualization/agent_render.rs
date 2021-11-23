use bevy::prelude::{Transform, Visible};

use crate::engine::{agent::Agent, state::State};

use downcast_rs::{impl_downcast, Downcast};

pub trait AgentRender: Downcast + Send + Sync + 'static {
    // Specifies the asset to use when visualizing the agent.
    // This should be overwritten to return a string which can point to two things:
    // 1) An emoji code, a list of compatible ones can be found here: https://www.webfx.com/tools/emoji-cheat-sheet/
    // 2) (NOT YET SUPPORTED) A filename pointing to a file within the project's asset folder, which should be located in the project root.
    // As for now, the emoji asset must be present in your project in an "assets" folder located in the root of the project itself.
    // This requirement will be likely removed in future updates by bundling the emoji asset in the executable.
    fn sprite(&self, agent: &Box<dyn Agent>, state: &Box<&dyn State>) -> SpriteType;

    // Specifies the position of the sprite in the window.
    // This is separate from Location2D because we require f32s, and the user may want to separate window
    // position from the actual model's.
    //
    // IMPORTANT:
    // Do NOT rely on local fields of the struct implementing Render to save the location of the agent:
    // they will NOT be automatically updated when the RustAB scheduler steps. Instead, use the state
    // passed as argument to fetch the agent position and act on that.
    fn position(&self, agent: &Box<dyn Agent>, state: &Box<&dyn State>) -> (f32, f32, f32);

    /// Specifies the scale of the sprite in the window.
    fn scale(&self, agent: &Box<dyn Agent>, state: &Box<&dyn State>) -> (f32, f32);

    // Rotation of the sprite in radians.
    fn rotation(&self, agent: &Box<dyn Agent>, state: &Box<&dyn State>) -> f32;

    // Update the graphical variables based on the information coming from the model through the state.
    fn update(
        &mut self,
        agent: &Box<dyn Agent>,
        transform: &mut Transform,
        state: &Box<&dyn State>,
        visible: &mut Visible,
    );

    fn get_id(&self) -> u32;
}

impl_downcast!(AgentRender);

#[derive(Clone)]
pub enum SpriteType {
    Emoji(String),
    // File(String), TODO
}
