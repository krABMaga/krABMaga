use amethyst::{core::Time, shred::Read, shred::System, utils::fps_counter::FpsCounter};

/// A simple system to keep track of the amount of frames rendered per second.
pub struct FPSSystem {
    pub print_elapsed: f32,
}

impl<'s> System<'s> for FPSSystem {
    type SystemData = (Read<'s, Time>, Read<'s, FpsCounter>);

    fn run(&mut self, data: Self::SystemData) {
        let (time, fps_counter) = data;

        self.print_elapsed += time.delta_seconds();

        if self.print_elapsed > 2. {
            let fps = fps_counter.sampled_fps();
            println!("FPS: {}\n", fps);
            self.print_elapsed = 0.;
        }
    }
}
