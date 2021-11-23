use crate::bevy::prelude::Vec2;
pub use bevy_canvas::{Geometry, Path, PathBuilder};

// An arrow segment consisting of a main line, along with two small lines starting at the end point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Arrow(pub Vec2, pub Vec2, pub f32, pub f32);

impl Geometry for Arrow {
    fn generate_path(&self) -> Path {
        let mut b = PathBuilder::new();

        let start = self.0;
        let end = self.1;

        let head_length = self.2;
        let head_angle = self.3;

        let angle = (end.y - start.y).atan2(end.x - start.x);
        let first_head_angle = angle - head_angle;
        let second_head_angle = angle + head_angle;

        let x1 = end.x - head_length * first_head_angle.cos();
        let x2 = end.x - head_length * second_head_angle.cos();

        let y1 = end.y - head_length * first_head_angle.sin();
        let y2 = end.y - head_length * second_head_angle.sin();

        let head_point_one = Vec2::new(x1, y1);
        let head_point_two = Vec2::new(x2, y2);

        b.move_to(start);
        b.line_to(end);
        b.line_to(head_point_one);
        b.move_to(end);
        b.line_to(head_point_two);

        b.build()
    }
}
