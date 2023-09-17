use crate::{components::Context, Component};

use super::AsyncState;

pub struct AsyncExecutor;

impl<T, X: Clone> Component<X, &mut AsyncState<T>> for AsyncExecutor {
    type Input = ();

    fn instantiate(_: Self::Input) -> Self
    where
        Self: Sized,
    {
        Self
    }

    fn process<'c>(&mut self, _: &Context, state: &'c mut AsyncState<T>) -> &'c mut AsyncState<T> {
        state.process();
        state
    }
}
