use macroquad::prelude::Color;

use crate::{components::Context, Component};
/// Sets the background, make sure to have one of these components as one of the first things that get rendered
pub struct Background;
impl Component<&Color, &mut Color> for Background {
    fn render(&self, context: &Context, props: &Color) {
        context.clear_background(*props)
    }

    type Input = ();

    fn instantiate(_: Self::Input) -> Self {
        Self
    }
}
