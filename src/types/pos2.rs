use crate::types::vec2::Vec2;

pub enum Pos2 {
    Absolute(Vec2<f32>),
    Relative(Vec2<f32>),
}
