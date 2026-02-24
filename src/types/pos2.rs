use std::ops::{Add, Sub, Mul, Div};
use crate::types::vec2::Vec2;

#[derive(Clone, Copy)]
pub enum Pos2 {
    Absolute(Vec2<f32>),
    Relative(Vec2<f32>),
}

impl Into<Vec2<f32>> for Pos2 {
    fn into(self) -> Vec2<f32> {
        match self {
            Pos2::Absolute(pos) => pos,
            Pos2::Relative(pos) => pos,
        }
    }
}

impl Add for Pos2 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Pos2::Absolute(a), Pos2::Absolute(b)) => Pos2::Absolute(a + b),
            (Pos2::Relative(a), Pos2::Relative(b)) => Pos2::Relative(a + b),
            (Pos2::Absolute(a), Pos2::Relative(b)) => Pos2::Absolute(a + b),
            (Pos2::Relative(a), Pos2::Absolute(b)) => Pos2::Absolute(a + b),
        }
    }
}

impl Sub for Pos2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (Pos2::Absolute(a), Pos2::Absolute(b)) => Pos2::Absolute(a - b),
            (Pos2::Relative(a), Pos2::Relative(b)) => Pos2::Relative(a - b),
            (Pos2::Absolute(a), Pos2::Relative(b)) => Pos2::Absolute(a - b),
            (Pos2::Relative(a), Pos2::Absolute(b)) => Pos2::Absolute(a - b),
        }
    }
}

impl Mul<f32> for Pos2 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        match self {
            Pos2::Absolute(a) => Pos2::Absolute(a * scalar),
            Pos2::Relative(a) => Pos2::Relative(a * scalar),
        }
    }
}

impl Div<f32> for Pos2 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self::Output {
        match self {
            Pos2::Absolute(a) => Pos2::Absolute(a / scalar),
            Pos2::Relative(a) => Pos2::Relative(a / scalar),
        }
    }
}
