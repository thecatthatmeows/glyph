use std::f32::consts::PI;

use crate::types::vec2::Vec2;

pub mod line;
pub mod rectangle;
pub mod triangle;

#[derive(Clone, Copy)]
pub enum Orientation {
    Up,
    Down,
    Left,
    Right,
    Custom(f32),
}

impl Orientation {
    pub fn opposite(&self) -> Self {
        match self {
            Orientation::Up => Orientation::Down,
            Orientation::Down => Orientation::Up,
            Orientation::Left => Orientation::Right,
            Orientation::Right => Orientation::Left,
            Orientation::Custom(v) => Orientation::Custom((v + PI) % (2.0 * PI)),
        }
    }
}

/// if it outputs 0.0, its inside the triangle
pub fn edge(a: Vec2<f32>, b: Vec2<f32>, p: Vec2<f32>) -> f32 {
    (p.x - a.x)*(b.y - a.y) - (p.y - a.y)*(b.x - a.x)
}

pub fn inside_triangle(a: Vec2<f32>, b: Vec2<f32>, c: Vec2<f32>, p: Vec2<f32>) -> bool {
    let e1 = edge(a, b, p);
    let e2 = edge(b, c, p);
    let e3 = edge(c, a, p);

    (e1 >= 0.0 && e2 >= 0.0 && e3 >= 0.0 ) ||
    (e1 <= 0.0 && e2 <= 0.0 && e3 <= 0.0 )
}
