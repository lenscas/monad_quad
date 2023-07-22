use macroquad::{
    prelude::{Color, Vec2},
    shapes::draw_rectangle,
};

use crate::Component;

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

impl Component<RectangleProps> for Rectangle {
    fn process(&mut self, _: &mut RectangleProps) {
        //let mut mapped = (self.func)(state);
    }

    fn render(&self, props: &RectangleProps) {
        draw_rectangle(
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
    fn ui(&mut self, _: &mut macroquad::ui::Ui, _: &mut RectangleProps) {}
}
