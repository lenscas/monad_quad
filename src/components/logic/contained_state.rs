use crate::{components::Context, Component};
/// like [StateFull](crate::components::StateFull) if it was a Component itself.
///
/// Unlike [StateFull](crate::components::StateFull) it does not have a built in render loop.
///
/// Should be used _very_ sparingly. As it pretty much breaks the idea of how monad_quad manages state
pub struct ContainedState<T, Child> {
    static_value: T,
    child: Child,
}
impl<U: Clone, X, T, Child: for<'a> Component<&'a T, &'a mut T>> Component<U, X>
    for ContainedState<T, Child>
{
    fn process<'c>(&mut self, context: &Context, state: X) -> X {
        self.child.process(context, &mut self.static_value);
        state
    }

    fn render(&self, context: &Context, _: U) {
        self.child.render(context, &self.static_value)
    }

    type Input = (T, Child);

    fn instantiate(input: Self::Input) -> Self {
        Self {
            static_value: input.0,
            child: input.1,
        }
    }
    fn ui<'c>(&mut self, context: &Context, ui: &mut macroquad::ui::Ui, state: X) -> X {
        self.child.ui(context, ui, &mut self.static_value);
        state
    }
}
