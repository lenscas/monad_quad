use crate::{components::Context, Component};

/// Acts as [ContainedState](crate::components::ContainedState) but does NOT run the process method of its child
///
/// Thus ensuring that the given value remains static
pub struct StaticValue<T, Child> {
    static_value: T,
    child: Child,
}
impl<'a, T: 'a, Child: Component<&'a T, &'a mut T>> StaticValue<T, Child> {
    pub fn new(static_value: T, child: Child) -> Self {
        Self {
            static_value,
            child,
        }
    }
}
impl<U: Clone, Z, T, Child: for<'a> Component<&'a T, &'a mut T>> Component<U, Z>
    for StaticValue<T, Child>
{
    fn render(&self, context: &Context, _: U) {
        self.child.render(context, &self.static_value)
    }

    type Input = (T, Child);

    fn instantiate(input: Self::Input) -> Self {
        Self::new(input.0, input.1)
    }
}
