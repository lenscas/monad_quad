use macroquad::hash;

use crate::{components::Context, Component};

pub struct InputProperties {
    pub is_password: bool,
    pub label: String,
    pub data: String,
}

pub struct Input;
impl Component<&InputProperties, &mut InputProperties> for Input {
    type Input = ();

    fn instantiate((): Self::Input) -> Self
    where
        Self: Sized,
    {
        Self
    }

    fn ui<'c>(
        &mut self,
        _: &Context,
        ui: &mut macroquad::ui::Ui,
        state: &'c mut InputProperties,
    ) -> &'c mut InputProperties {
        if state.is_password {
            ui.input_password(hash!(), &state.label, &mut state.data)
        } else {
            ui.input_text(hash!(), &state.label, &mut state.data)
        }
        state
    }
}
