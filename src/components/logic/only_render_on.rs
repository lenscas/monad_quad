use crate::{components::Context, Component};

pub struct OnlyRenderOn<Child, Decider> {
    child: Child,
    decider: Decider,
}
impl<Child, Decider> OnlyRenderOn<Child, Decider> {
    pub fn new<State>(child: Child, decider: Decider) -> Self
    where
        Child: for<'a> Component<&'a State, &'a mut State>,
        Decider: Fn(&State) -> bool,
    {
        Self { child, decider }
    }
    pub fn process<State>(&mut self, context: &Context, state: &mut State) -> bool
    where
        Child: for<'a> Component<&'a State, &'a mut State>,
        Decider: Fn(&State) -> bool,
    {
        if self.should_process(state) {
            self.child.process(context, state);
            true
        } else {
            false
        }
    }
    pub fn ui<'a, State>(
        &mut self,
        context: &Context,
        ui: &mut macroquad::ui::Ui,
        state: &'a mut State,
    ) -> bool
    where
        Child: Component<&'a State, &'a mut State>,
        Decider: Fn(&State) -> bool,
    {
        if self.should_process(state) {
            self.child.ui(context, ui, state);
            true
        } else {
            false
        }
    }
    pub fn should_process<'a, 'b: 'a, State: 'b>(&self, state: &State) -> bool
    where
        Child: Component<&'a State, &'a mut State>,
        Decider: Fn(&State) -> bool,
    {
        (self.decider)(state)
    }
}

impl<State, Child: for<'a> Component<&'a State, &'a mut State>, Decider: Fn(&State) -> bool>
    Component<&State, &mut State> for OnlyRenderOn<Child, Decider>
{
    type Input = (Child, Decider);

    fn instantiate((child, decider): Self::Input) -> Self
    where
        Self: Sized,
    {
        Self::new(child, decider)
    }

    fn process<'c>(&mut self, context: &Context, state: &'c mut State) -> &'c mut State {
        self.process(context, state);
        state
    }

    fn render(&self, context: &Context, props: &State) {
        self.child.render(context, props)
    }
    fn ui<'c>(
        &mut self,
        context: &Context,
        ui: &mut macroquad::ui::Ui,
        state: &'c mut State,
    ) -> &'c mut State {
        self.ui(context, ui, state);
        state
    }
}

pub struct OnlyRenderWith<Child, Decider, OnPausedChild> {
    child: OnlyRenderOn<Child, Decider>,
    on_paused_child: OnPausedChild,
}
impl<Child, Decider, OnPausedChild> OnlyRenderWith<Child, Decider, OnPausedChild> {
    pub fn new<State>(decider: Decider, child: Child, on_paused_child: OnPausedChild) -> Self
    where
        Child: for<'a> Component<&'a State, &'a mut State>,
        Decider: Fn(&State) -> bool,
        OnPausedChild: for<'a> Component<&'a State, &'a mut State>,
    {
        Self {
            child: OnlyRenderOn::new(child, decider),
            on_paused_child,
        }
    }
}

impl<
        State,
        Child: for<'a> Component<&'a State, &'a mut State>,
        Decider: Fn(&State) -> bool,
        OnPausedChild: for<'a> Component<&'a State, &'a mut State>,
    > Component<&State, &mut State> for OnlyRenderWith<Child, Decider, OnPausedChild>
{
    type Input = (Decider, Child, OnPausedChild);

    fn instantiate((decider, child, paused_child): Self::Input) -> Self
    where
        Self: Sized,
    {
        Self::new(decider, child, paused_child)
    }

    fn process<'c>(&mut self, context: &Context, state: &'c mut State) -> &'c mut State {
        if !self.child.process(context, state) {
            self.on_paused_child.process(context, state);
        }
        state
    }

    fn render(&self, context: &Context, props: &State) {
        self.child.render(context, props);
        if !self.child.should_process(props) {
            self.on_paused_child.render(context, props)
        }
    }

    fn ui<'c>(
        &mut self,
        context: &Context,
        ui: &mut macroquad::ui::Ui,
        state: &'c mut State,
    ) -> &'c mut State {
        self.child.ui(context, ui, state);
        if !self.child.should_process(state) {
            self.on_paused_child.ui(context, ui, state);
        }
        state
    }
}
