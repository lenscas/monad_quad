use macroquad::prelude::Vec2;

use crate::Component;

pub struct LabelProperties {
    pub location: Vec2,
    pub label: String,
}

pub struct Label;

impl Component<LabelProperties> for Label {
    type Input = ();

    fn instantiate(_: Self::Input) -> Self
    where
        Self: Sized,
    {
        Self
    }

    fn process(&mut self, _: &mut LabelProperties) {}

    fn render(&self, _: &LabelProperties) {}

    fn ui(&mut self, ui: &mut macroquad::ui::Ui, state: &mut LabelProperties) {
        ui.label(state.location, &state.label)
    }
}
