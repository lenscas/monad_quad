use macroquad::{prelude::Color, window::clear_background};

use crate::Component;
/// Sets the background, make sure to have one of these components as one of the first things that get rendered
pub struct Background;
impl Component<Color> for Background {
    fn process(&mut self, _: &mut Color) {}

    fn render(&self, props: &Color) {
        clear_background(*props)
    }

    type Input = ();

    fn instantiate(_: Self::Input) -> Self {
        Self
    }
    fn ui(&mut self, _: &mut macroquad::ui::Ui, _: &mut Color) {}
}
