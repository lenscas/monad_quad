use std::marker::PhantomData;

use crate::Component;

pub struct OnlyRenderOn<State, Child: Component<State>, Decider: Fn(&State) -> bool> {
    _state: PhantomData<State>,
    child: Child,
    decider: Decider,
}
impl<State, Child: Component<State>, Decider: Fn(&State) -> bool>
    OnlyRenderOn<State, Child, Decider>
{
    pub fn new(child: Child, decider: Decider) -> Self {
        Self {
            _state: PhantomData,
            child,
            decider,
        }
    }
    pub fn process(&mut self, state: &mut State) -> bool {
        if self.should_process(state) {
            self.child.process(state);
            true
        } else {
            false
        }
    }
    pub fn ui(&mut self, ui: &mut macroquad::ui::Ui, state: &mut State) -> bool {
        if self.should_process(state) {
            self.child.ui(ui, state);
            true
        } else {
            false
        }
    }
    pub fn should_process(&self, state: &State) -> bool {
        (self.decider)(state)
    }
}

impl<State, Child: Component<State>, Decider: Fn(&State) -> bool> Component<State>
    for OnlyRenderOn<State, Child, Decider>
{
    type Input = (Child, Decider);

    fn instantiate((child, decider): Self::Input) -> Self
    where
        Self: Sized,
    {
        Self::new(child, decider)
    }

    fn process(&mut self, state: &mut State) {
        self.process(state);
    }

    fn render(&self, props: &State) {
        self.child.render(props)
    }
    fn ui(&mut self, ui: &mut macroquad::ui::Ui, state: &mut State) {
        self.ui(ui, state);
    }
}

pub struct OnlyRenderWith<
    State,
    Child: Component<State>,
    Decider: Fn(&State) -> bool,
    OnPausedChild: Component<State>,
> {
    child: OnlyRenderOn<State, Child, Decider>,
    on_paused_child: OnPausedChild,
}
impl<
        State,
        Child: Component<State>,
        Decider: Fn(&State) -> bool,
        OnPausedChild: Component<State>,
    > OnlyRenderWith<State, Child, Decider, OnPausedChild>
{
    pub fn new(decider: Decider, child: Child, on_paused_child: OnPausedChild) -> Self {
        Self {
            child: OnlyRenderOn::new(child, decider),
            on_paused_child,
        }
    }
}

impl<
        State,
        Child: Component<State>,
        Decider: Fn(&State) -> bool,
        OnPausedChild: Component<State>,
    > Component<State> for OnlyRenderWith<State, Child, Decider, OnPausedChild>
{
    type Input = (Decider, Child, OnPausedChild);

    fn instantiate((decider, child, paused_child): Self::Input) -> Self
    where
        Self: Sized,
    {
        Self::new(decider, child, paused_child)
    }

    fn process(&mut self, state: &mut State) {
        if !self.child.process(state) {
            self.on_paused_child.process(state)
        }
    }

    fn render(&self, props: &State) {
        self.child.render(props);
        if !self.child.should_process(props) {
            self.on_paused_child.render(props)
        }
    }

    fn ui(&mut self, ui: &mut macroquad::ui::Ui, state: &mut State) {
        self.child.ui(ui, state);
        if !self.child.should_process(state) {
            self.on_paused_child.ui(ui, state)
        }
    }
}
