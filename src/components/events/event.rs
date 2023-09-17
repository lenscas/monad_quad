use crate::components::{Component, Context};

pub struct Event<CheckEvent, MapIntoState> {
    check_event: CheckEvent,
    map: MapIntoState,
}

impl<CheckEvent, MapIntoState> Event<CheckEvent, MapIntoState> {
    pub fn new<StateIn, CreatedEvent>(check_event: CheckEvent, mapper: MapIntoState) -> Self
    where
        CheckEvent: Fn(&StateIn) -> CreatedEvent,
        MapIntoState: Fn(CreatedEvent, &mut StateIn),
    {
        <Self as Component<&StateIn, &mut StateIn>>::instantiate((check_event, mapper))
    }
}
impl<
        StateIn,
        CreatedEvent,
        CheckEvent: Fn(&StateIn) -> CreatedEvent,
        MapIntoState: Fn(CreatedEvent, &mut StateIn),
    > Component<&StateIn, &mut StateIn> for Event<CheckEvent, MapIntoState>
{
    type Input = (CheckEvent, MapIntoState);

    fn instantiate((check_event, map): Self::Input) -> Self {
        Self { check_event, map }
    }

    fn process<'c>(&mut self, _: &Context, state: &'c mut StateIn) -> &'c mut StateIn {
        let x = (self.check_event)(state);
        (self.map)(x, state);
        state
    }
}
