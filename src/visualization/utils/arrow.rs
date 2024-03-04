use bevy_prototype_lyon::prelude::tess::geom::point;
use bevy_prototype_lyon::prelude::tess::path::path::Builder;
use bevy_prototype_lyon::prelude::Geometry;

use crate::bevy::prelude::Vec2;

// An arrow segment consisting of a main line, along with two small lines starting at the end point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Arrow(pub Vec2, pub Vec2, pub f32, pub f32);

impl Geometry for Arrow {
    fn add_geometry(&self, b: &mut Builder) {
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

        let start_point = point(start.x, start.y);
        let end_point = point(end.x, end.y);
        let head_point_one = point(x1, y1);
        let head_point_two = point(x2, y2);

        b.begin(start_point);
        b.line_to(end_point);
        b.line_to(head_point_one);
        b.line_to(end_point);
        b.line_to(head_point_two);
    }
}
