use std::marker::PhantomData;

use macroquad::{input::KeyCode, prelude::is_key_down};

use crate::components::Component;

/// This component listens to the given keycode and fires off an event every process to tell the current state of said key
pub struct KeyDown<State, CheckFor: Fn(&State) -> KeyCode, OnEvent: Fn(bool, &mut State)> {
    _state: PhantomData<State>,
    check: CheckFor,
    on_event: OnEvent,
}
impl<State, CheckFor: Fn(&State) -> KeyCode, OnEvent: Fn(bool, &mut State)>
    KeyDown<State, CheckFor, OnEvent>
{
    pub fn new(check: CheckFor, on_event: OnEvent) -> Self {
        Self::instantiate((check, on_event))
    }
}

impl<State, CheckFor: Fn(&State) -> KeyCode, OnEvent: Fn(bool, &mut State)> Component<State>
    for KeyDown<State, CheckFor, OnEvent>
{
    type Input = (CheckFor, OnEvent);

    fn instantiate((check, on_event): Self::Input) -> Self {
        Self {
            _state: PhantomData,
            check,
            on_event,
        }
    }

    fn process(&mut self, state: &mut State) {
        let res = is_key_down((self.check)(state));
        (self.on_event)(res, state)
    }

    fn render(&self, _: &State) {}
    fn ui(&mut self, _: &mut macroquad::ui::Ui, _: &mut State) {}
}
