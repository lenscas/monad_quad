use macroquad::hash;

use crate::{components::Context, Component};

pub struct ComboBoxProperties {
    pub label: String,
    pub variants: Vec<String>,
    pub chosen: Option<usize>,
}
pub struct ComboBox;
impl Component<&ComboBoxProperties, &mut ComboBoxProperties> for ComboBox {
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
        state: &'c mut ComboBoxProperties,
    ) -> &'c mut ComboBoxProperties {
        let x = state
            .variants
            .iter()
            .map(String::as_ref)
            .collect::<Vec<_>>();
        ui.combo_box(hash!(), &state.label, &x, &mut state.chosen);
        state
    }
}
