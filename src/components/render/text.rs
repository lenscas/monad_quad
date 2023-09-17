use macroquad::prelude::{Color, Vec2};

use crate::{components::Context, Component};

/// The properties needed to render text
pub struct TextProperties {
    pub text: String,
    pub location: Vec2,
    pub font_size: f32,
    pub color: Color,
}

/// A component to render some text
pub struct Text;
impl Component<&TextProperties, &mut TextProperties> for Text {
    type Input = ();

    fn instantiate(_: Self::Input) -> Self {
        Self
    }

    fn render(&self, context: &Context, props: &TextProperties) {
        context.draw_text(
            &props.text,
            props.location.x,
            props.location.y,
            props.font_size,
            props.color,
        )
    }
}
