use macroquad::prelude::Vec2;

use crate::{components::Context, Component};

pub struct LabelProperties {
    pub location: Vec2,
    pub label: String,
}

pub struct Label;

impl Component<&LabelProperties, &mut LabelProperties> for Label {
    type Input = ();

    fn instantiate(_: Self::Input) -> Self
    where
        Self: Sized,
    {
        Self
    }

    fn ui<'c>(
        &mut self,
        _: &Context,
        ui: &mut macroquad::ui::Ui,
        state: &'c mut LabelProperties,
    ) -> &'c mut LabelProperties {
        ui.label(state.location, &state.label);
        state
    }
}
