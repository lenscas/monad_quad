use macroquad::hash;

use crate::Component;

pub struct CheckboxProperties {
    pub state: bool,
    pub label: String,
}
pub struct CheckBox;

impl Component<CheckboxProperties> for CheckBox {
    type Input = ();

    fn instantiate(_: Self::Input) -> Self
    where
        Self: Sized,
    {
        Self
    }

    fn process(&mut self, _: &mut CheckboxProperties) {}

    fn render(&self, _: &CheckboxProperties) {}

    fn ui(&mut self, ui: &mut macroquad::ui::Ui, state: &mut CheckboxProperties) {
        ui.checkbox(hash!(), &state.label, &mut state.state)
    }
}
