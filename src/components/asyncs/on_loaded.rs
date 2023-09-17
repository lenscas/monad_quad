use std::{cell::RefCell, rc::Rc};

use crate::{components::Context, Component};

use super::{AsyncExecutor, AsyncState};

pub struct OnLoaded<Mapper, Child> {
    executor: AsyncExecutor,
    mapper: Mapper,
    child: Child,
}
impl<
        T,
        Async,
        Mapper: Fn(&T) -> AsyncState<Async>,
        Child: for<'b> Component<
            &'b (Rc<RefCell<Async>>, bool, &'b T),
            &'b mut (Rc<RefCell<Async>>, bool, &'b mut T),
        >,
    > Component<&T, &mut T> for OnLoaded<Mapper, Child>
{
    type Input = (Mapper, Child);

    fn instantiate((mapper, child): Self::Input) -> Self
    where
        Self: Sized,
    {
        Self {
            executor: AsyncExecutor,
            mapper,
            child,
        }
    }

    fn process<'c>(&mut self, context: &Context, state: &'c mut T) -> &'c mut T {
        let mut task = (self.mapper)(state);
        <AsyncExecutor as Component<&T, &mut AsyncState<Async>>>::process(
            &mut self.executor,
            context,
            &mut task,
        );
        let (is_loaded, value) = match task {
            AsyncState::Unloaded | AsyncState::Loading(None, _) => return state,
            AsyncState::Loading(Some(x), _) => (false, x),
            AsyncState::Loaded(x) => (true, x),
        };
        self.child.process(context, &mut (value, is_loaded, state));
        state
    }
    fn render(&self, context: &Context, state: &T) {
        let task = (self.mapper)(state);
        let (is_loaded, value) = match task {
            AsyncState::Unloaded | AsyncState::Loading(None, _) => {
                return;
            }
            AsyncState::Loading(Some(x), _) => (false, x),
            AsyncState::Loaded(x) => (true, x),
        };
        self.child.render(context, &(value, is_loaded, state));
    }
    fn ui<'b>(
        &mut self,
        context: &Context,
        ui: &mut macroquad::ui::Ui,
        state: &'b mut T,
    ) -> &'b mut T {
        let task = (self.mapper)(state);
        let (is_loaded, value) = match task {
            AsyncState::Unloaded | AsyncState::Loading(None, _) => {
                return state;
            }
            AsyncState::Loading(Some(x), _) => (false, x),
            AsyncState::Loaded(x) => (true, x),
        };
        self.child.ui(context, ui, &mut (value, is_loaded, state));
        state
    }
}
