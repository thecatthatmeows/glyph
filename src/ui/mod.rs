pub mod text;
pub mod container;
pub mod style;

use crate::{shapes::Shape, types::{pos2::Pos2, vec2::Vec2}};

pub enum ElementState {
    Active,
    Inactive,
    Disabled,
}

pub trait UIElement {
    fn pos(&self) -> Pos2;
    fn size(&self) -> Vec2<f32>;
    fn draw(&self);
    fn update(&mut self);
}

pub trait InteractiveUIElement: UIElement {
    fn state(&self) -> &ElementState;
}

pub trait ShapedElement: UIElement {
    fn shape(&self) -> &dyn Shape;
}
