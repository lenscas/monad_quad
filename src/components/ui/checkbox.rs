use macroquad::hash;

use crate::{components::Context, Component};

pub struct CheckboxProperties {
    pub state: bool,
    pub label: String,
}
pub struct CheckBox;

impl Component<&CheckboxProperties, &mut CheckboxProperties> for CheckBox {
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
        state: &'c mut CheckboxProperties,
    ) -> &'c mut CheckboxProperties {
        ui.checkbox(hash!(), &state.label, &mut state.state);
        state
    }
}
