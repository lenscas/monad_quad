use crate::Component;

/// Acts as [ContainedState](crate::components::ContainedState) but does NOT run the process method of its child
///
/// Thus ensuring that the given value remains static
pub struct StaticValue<T, Child: Component<T>> {
    static_value: T,
    child: Child,
}
impl<T, Child: Component<T>> StaticValue<T, Child> {
    pub fn new(static_value: T, child: Child) -> Self {
        Self {
            static_value,
            child,
        }
    }
}
impl<U, T, Child: Component<T>> Component<U> for StaticValue<T, Child> {
    fn process(&mut self, _: &mut U) {}

    fn render(&self, _: &U) {
        self.child.render(&self.static_value)
    }

    type Input = (T, Child);

    fn instantiate(input: Self::Input) -> Self {
        Self::new(input.0, input.1)
    }
    fn ui(&mut self, _: &mut macroquad::ui::Ui, _: &mut U) {
        //self.child.ui(ui, state)
    }
}
