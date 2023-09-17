use macroquad::{input::KeyCode, prelude::is_key_down};

use crate::components::{Component, Context};

/// This component listens to the given keycode and fires off an event every process to tell the current state of said key
pub struct KeyDown<CheckFor, OnEvent> {
    check: CheckFor,
    on_event: OnEvent,
}
impl<CheckFor, OnEvent> KeyDown<CheckFor, OnEvent> {
    pub fn new<State>(check: CheckFor, on_event: OnEvent) -> Self
    where
        CheckFor: Fn(&State) -> KeyCode,
        OnEvent: Fn(bool, &mut State),
    {
        <Self as Component<&State, &mut State>>::instantiate((check, on_event))
    }
}

impl<State, CheckFor: Fn(&State) -> KeyCode, OnEvent: Fn(bool, &mut State)>
    Component<&State, &mut State> for KeyDown<CheckFor, OnEvent>
{
    type Input = (CheckFor, OnEvent);

    fn instantiate((check, on_event): Self::Input) -> Self {
        Self { check, on_event }
    }

    fn process<'c>(&mut self, _: &Context, state: &'c mut State) -> &'c mut State {
        let res = is_key_down((self.check)(state));
        (self.on_event)(res, state);
        state
    }
}
