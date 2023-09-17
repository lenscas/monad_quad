use macroquad::prelude::{Color, Vec2};

use crate::{components::Context, Component};

/// Properties to render a Rectangle

#[derive(Clone)]
pub struct RectangleProps {
    pub size: Vec2,
    pub color: Color,
    pub location: Vec2,
}

/// Draws a rectangle
pub struct Rectangle;

impl Rectangle {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Self::new()
    }
}

impl Component<&RectangleProps, &mut RectangleProps> for Rectangle {
    fn render(&self, context: &Context, props: &RectangleProps) {
        context.draw_rectangle(
            props.location.x,
            props.location.y,
            props.size.x,
            props.size.y,
            props.color,
        );
    }

    type Input = ();

    fn instantiate(_: Self::Input) -> Self {
        Self::new()
    }
}
