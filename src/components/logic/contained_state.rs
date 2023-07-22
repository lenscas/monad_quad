use crate::Component;
/// like [StateFull](crate::components::StateFull) if it was a Component itself.
///
/// Unlike [StateFull](crate::components::StateFull) it does not have a built in render loop.
///
/// Should be used _very_ sparingly. As it pretty much breaks the idea of how monad_quad manages state
pub struct ContainedState<T, Child: Component<T>> {
    static_value: T,
    child: Child,
}
impl<U, T, Child: Component<T>> Component<U> for ContainedState<T, Child> {
    fn process(&mut self, _: &mut U) {
        self.child.process(&mut self.static_value)
    }

    fn render(&self, _: &U) {
        self.child.render(&self.static_value)
    }

    type Input = (T, Child);

    fn instantiate(input: Self::Input) -> Self {
        Self {
            static_value: input.0,
            child: input.1,
        }
    }
    fn ui(&mut self, ui: &mut macroquad::ui::Ui, _: &mut U) {
        self.child.ui(ui, &mut self.static_value)
    }
}
