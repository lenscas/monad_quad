use std::marker::PhantomData;

use crate::components::Component;

pub struct Event<
    StateIn,
    CreatedEvent,
    CheckEvent: Fn(&StateIn) -> CreatedEvent,
    MapIntoState: Fn(CreatedEvent, &mut StateIn),
> {
    _state: PhantomData<StateIn>,
    _event: PhantomData<CreatedEvent>,
    check_event: CheckEvent,
    map: MapIntoState,
}

impl<
        StateIn,
        CreatedEvent,
        CheckEvent: Fn(&StateIn) -> CreatedEvent,
        MapIntoState: Fn(CreatedEvent, &mut StateIn),
    > Event<StateIn, CreatedEvent, CheckEvent, MapIntoState>
{
    pub fn new(check_event: CheckEvent, mapper: MapIntoState) -> Self {
        Self::instantiate((check_event, mapper))
    }
}
impl<
        StateIn,
        CreatedEvent,
        CheckEvent: Fn(&StateIn) -> CreatedEvent,
        MapIntoState: Fn(CreatedEvent, &mut StateIn),
    > Component<StateIn> for Event<StateIn, CreatedEvent, CheckEvent, MapIntoState>
{
    type Input = (CheckEvent, MapIntoState);

    fn instantiate((check_event, map): Self::Input) -> Self {
        Self {
            _state: PhantomData,
            _event: PhantomData,
            check_event,
            map,
        }
    }

    fn process(&mut self, state: &mut StateIn) {
        let x = (self.check_event)(state);
        (self.map)(x, state);
    }

    fn render(&self, _: &StateIn) {}

    fn ui(&mut self, _: &mut macroquad::ui::Ui, _: &mut StateIn) {}
}
