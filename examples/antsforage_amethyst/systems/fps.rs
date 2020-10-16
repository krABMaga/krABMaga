use amethyst::{core::Time, shred::Read, shred::System, utils::fps_counter::FpsCounter};


pub struct FPSSystem {
	pub print_elapsed: f32
}

impl<'s> System<'s> for FPSSystem {
	type SystemData = (
		Read<'s, Time>,
		Read<'s, FpsCounter>,
	);

	fn run(&mut self, data: Self::SystemData) {
        let (
            time,
            fps_counter,
        ) = data;

        self.print_elapsed += time.delta_seconds();

        if self.print_elapsed > 2.0 {
            let fps = fps_counter.sampled_fps();
            println!("FPS: {}\n", fps);
            self.print_elapsed = 0.0;
        }
    }
}
