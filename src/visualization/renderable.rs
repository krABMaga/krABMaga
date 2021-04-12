use bevy::prelude::{Commands, Quat, SpriteBundle, Transform, Vec3};

use crate::engine::agent::Agent;

pub trait Render: Agent + Send + Sync + Sized + 'static {
    /// Specifies the asset to use when visualizing the agent.
    /// This should be overwritten to return a string which can point to two things:
    /// 1) An emoji code, a list of compatible ones can be found here: https://www.webfx.com/tools/emoji-cheat-sheet/
    /// 2) (NOT YET SUPPORTED) A filename pointing to a file within the project's asset folder, which should be located in the project root.
    /// As for now, the emoji asset must be present in your project in an "assets" folder located in the root of the project itself.
    /// This requirement will be likely removed in future updates by bundling the emoji asset in the executable.
    fn sprite(&self) -> SpriteType;

    /// Specifies the position of the sprite in the window.
    /// This is separate from Location2D because we require f32s, and the user may want to separate window
    /// position from the actual model's.
    ///
    /// IMPORTANT:
    /// Do NOT rely on local fields of the struct implementing Render to save the location of the agent:
    /// they will NOT be automatically updated when the RustAB scheduler steps. Instead, use the state
    /// passed as argument to fetch the agent position and act on that.
    fn position(&self, state: &Self::SimState) -> (f32, f32, f32);

    /// Specifies the scale of the sprite in the window.
    fn scale(&self) -> (f32, f32);

    /// Rotation of the sprite in radians.
    fn rotation(&self) -> f32;

    /// Generate an entity and automatically insert it into the world, with a Transform with
    /// the given position, scale and rotation defined by the three previous methods.
    fn setup_graphics(
        self,
        mut sprite_bundle: SpriteBundle,
        commands: &mut Commands,
        state: &Self::SimState,
    ) {
        let (x, y, z) = self.position(state);
        let (scale_x, scale_y) = self.scale();
        let rotation = self.rotation();

        let mut transform = Transform::from_translation(Vec3::new(x, y, z));
        transform.scale.x = scale_x;
        transform.scale.y = scale_y;
        transform.rotation = Quat::from_rotation_z(rotation);

        sprite_bundle.transform = transform;
        commands
            .spawn()
            .insert(self)
            .insert(transform)
            .insert_bundle(sprite_bundle);
    }

    /// Update the graphical variables based on the information coming from the model through the state.
    fn update(&mut self, transform: &mut Transform, state: &Self::SimState);
}

pub enum SpriteType {
    Emoji(String),
    // File(String), TODO
}
