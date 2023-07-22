use macroquad::{
    prelude::{Color, Vec2},
    text::draw_text,
};

use crate::Component;

/// The properties needed to render text
pub struct TextProperties {
    pub text: String,
    pub location: Vec2,
    pub font_size: f32,
    pub color: Color,
}

/// A component to render some text
pub struct Text;
impl Component<TextProperties> for Text {
    type Input = ();

    fn instantiate(_: Self::Input) -> Self {
        Self
    }

    fn process(&mut self, _: &mut TextProperties) {}

    fn render(&self, props: &TextProperties) {
        draw_text(
            &props.text,
            props.location.x,
            props.location.y,
            props.font_size,
            props.color,
        )
    }
    fn ui(&mut self, _: &mut macroquad::ui::Ui, _: &mut TextProperties) {}
}
