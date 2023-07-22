use macroquad::hash;

use crate::Component;

pub struct ComboBoxProperties {
    label: String,
    variants: Vec<String>,
    chosen: Option<usize>,
}
pub struct ComboBox;
impl Component<ComboBoxProperties> for ComboBox {
    type Input = ();

    fn instantiate(_: Self::Input) -> Self
    where
        Self: Sized,
    {
        Self
    }

    fn process(&mut self, _: &mut ComboBoxProperties) {}

    fn render(&self, _: &ComboBoxProperties) {}

    fn ui(&mut self, ui: &mut macroquad::ui::Ui, state: &mut ComboBoxProperties) {
        let x = state
            .variants
            .iter()
            .map(String::as_ref)
            .collect::<Vec<_>>();
        ui.combo_box(hash!(), &state.label, &x, &mut state.chosen);
    }
}
