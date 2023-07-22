use macroquad::hash;

use crate::Component;

pub struct InputProperties {
    pub is_password: bool,
    pub label: String,
    pub data: String,
}

pub struct Input;
impl Component<InputProperties> for Input {
    type Input = ();

    fn instantiate((): Self::Input) -> Self
    where
        Self: Sized,
    {
        Self
    }

    fn process(&mut self, _: &mut InputProperties) {}

    fn render(&self, _: &InputProperties) {}

    fn ui(&mut self, ui: &mut macroquad::ui::Ui, state: &mut InputProperties) {
        if state.is_password {
            ui.input_password(hash!(), &state.label, &mut state.data)
        } else {
            ui.input_text(hash!(), &state.label, &mut state.data)
        }
    }
}
