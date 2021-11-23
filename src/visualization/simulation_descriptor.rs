// A resource containing data about the simulation, for ease of access during initialization.
pub struct SimulationDescriptor {
    pub title: String,
    pub width: f32,
    pub height: f32,
    pub center_x: f32,
    pub center_y: f32,
    pub paused: bool,
    pub ui_width: f32,
}
