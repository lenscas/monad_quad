use std::{cell::RefCell, rc::Rc};

use crate::{components::Context, Component};

use super::{AsyncExecutor, AsyncState};

pub struct WithLoading<Mapper, LoadedChild, LoadingChild> {
    executor: AsyncExecutor,
    mapper: Mapper,
    loaded_child: LoadedChild,
    loading_child: LoadingChild,
}
impl<Mapper, LoadedChild, LoadingChild> WithLoading<Mapper, LoadedChild, LoadingChild> {
    pub fn new<T, Async>(map: Mapper, loaded_chid: LoadedChild, loading_child: LoadingChild) -> Self
    where
        Mapper: Fn(&T) -> AsyncState<Async>,
        LoadedChild: for<'b> Component<
            &'b (Rc<RefCell<Async>>, bool, &'b T),
            &'b mut (Rc<RefCell<Async>>, bool, &'b mut T),
        >,
        LoadingChild: for<'b> Component<&'b T, &'b mut T>,
    {
        Self::instantiate((map, loaded_chid, loading_child))
    }
}
impl<
        T,
        Async,
        Mapper: Fn(&T) -> AsyncState<Async>,
        LoadedChild: for<'b> Component<
            &'b (Rc<RefCell<Async>>, bool, &'b T),
            &'b mut (Rc<RefCell<Async>>, bool, &'b mut T),
        >,
        LoadingChild: for<'b> Component<&'b T, &'b mut T>,
    > Component<&T, &mut T> for WithLoading<Mapper, LoadedChild, LoadingChild>
{
    type Input = (Mapper, LoadedChild, LoadingChild);

    fn instantiate((mapper, loaded_child, loading_child): Self::Input) -> Self
    where
        Self: Sized,
    {
        Self {
            executor: AsyncExecutor,
            mapper,
            loaded_child,
            loading_child,
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
            AsyncState::Unloaded | AsyncState::Loading(None, _) => {
                return self.loading_child.process(context, state)
            }
            AsyncState::Loading(Some(x), _) => (false, x),
            AsyncState::Loaded(x) => (true, x),
        };
        self.loaded_child
            .process(context, &mut (value, is_loaded, state));
        state
    }
    fn render(&self, context: &Context, state: &T) {
        let task = (self.mapper)(state);
        let (is_loaded, value) = match task {
            AsyncState::Unloaded | AsyncState::Loading(None, _) => {
                return self.loading_child.render(context, state)
            }
            AsyncState::Loading(Some(x), _) => (false, x),
            AsyncState::Loaded(x) => (true, x),
        };
        self.loaded_child
            .render(context, &(value, is_loaded, state));
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
                return self.loading_child.ui(context, ui, state)
            }
            AsyncState::Loading(Some(x), _) => (false, x),
            AsyncState::Loaded(x) => (true, x),
        };
        self.loaded_child
            .ui(context, ui, &mut (value, is_loaded, state));
        state
    }
}
