pub mod text;

use crate::{shapes::Shape, types::{pos2::Pos2, vec2::Vec2}};

pub enum ElementState {
    Active,
    Inactive,
    Disabled,
}

pub(crate) trait UIElement {
    fn pos(&self) -> Pos2;
    fn size(&self) -> Vec2<f32>;
}

pub(crate) trait InteractiveUIElement: UIElement {
    fn state(&self) -> &ElementState;
}

pub(crate) trait ShapedElement: UIElement {
    fn shape(&self) -> &dyn Shape;
}
